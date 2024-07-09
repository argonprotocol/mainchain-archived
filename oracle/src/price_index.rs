use std::{thread::spawn, time::Duration};

use anyhow::anyhow;
use chrono::DateTime;
use tokio::{join, time::sleep};
use tracing::info;

use ulixee_client::{
	api::{
		price_index::calls::types::submit::Index,
		runtime_types::sp_arithmetic::fixed_point::{FixedI128, FixedU128},
		tx,
	},
	signer::{KeystoreSigner, Signer},
	MainchainClient, ReconnectingClient,
};

use crate::{argon_cpi::calculate_argon_cpi, argon_price, btc_price, us_cpi::UsCpiRetriever};

pub async fn price_index_loop(
	trusted_rpc_url: String,
	signer: KeystoreSigner,
	use_simulated_schedule: bool,
) -> anyhow::Result<()> {
	let mut mainchain_client = ReconnectingClient::new(vec![trusted_rpc_url.clone()]);
	let mut ticker = mainchain_client.get().await?.lookup_ticker().await?;
	if !cfg!(test) {
		ticker
			.lookup_ntp_offset("pool.ntp.org")
			.await
			.map_err(|e| anyhow!("Unable to synchronize time {e:?}"))?;
	}

	let interval = Duration::from_millis(ticker.tick_duration_millis)
		.saturating_sub(Duration::from_secs(10))
		.max(Duration::from_secs(5));

	let mut us_cpi = UsCpiRetriever::new(interval).await?;
	let btc_price_lookup = btc_price::BtcPriceLookup::new();
	let mut argon_price_lookup =
		argon_price::ArgonPriceLookup::new(use_simulated_schedule, interval.as_millis() as u64);

	info!("Oracle Started.");
	let account_id = signer.account_id();

	loop {
		let (btc_price, _) = join!(btc_price_lookup.get_btc_price(), us_cpi.refresh());
		let btc_price = match btc_price {
			Ok(x) => x,
			Err(e) => {
				tracing::warn!("Couldn't update btc prices {:?}", e);
				continue;
			},
		};

		let timestamp = ticker.time_for_tick(ticker.current());
		let us_cpi_ratio = us_cpi.get_us_cpi_ratio(timestamp);
		let target_price = argon_price_lookup.get_target_price(us_cpi_ratio);
		let argon_usd_price =
			match argon_price_lookup.get_argon_price(us_cpi_ratio, timestamp).await {
				Ok(x) => x,
				Err(e) => {
					tracing::warn!("Couldn't update argon prices {:?}", e);
					continue;
				},
			};

		let argon_cpi = calculate_argon_cpi(target_price, argon_usd_price);

		info!(
			"Current CPI: {:?}, argon price {:?} at {:?}",
			argon_cpi,
			argon_usd_price,
			DateTime::from_timestamp_millis(timestamp as i64)
		);

		let price_index = tx().price_index().submit(Index {
			argon_cpi: FixedI128(argon_cpi.into_inner()),
			timestamp,
			argon_usd_price: FixedU128(argon_usd_price.into_inner()),
			btc_usd_price: FixedU128(btc_price.into_inner()),
		});
		{
			let client = mainchain_client.get().await?;

			let nonce = client.get_account_nonce_subxt(&account_id).await?;
			let params = MainchainClient::ext_params_builder().nonce(nonce.into()).build();
			let progress = client
				.live
				.tx()
				.sign_and_submit_then_watch(&price_index, &signer, params)
				.await?;

			info!("Submitted price index with progress: {:?}", progress);
			spawn(move || {
				if let Err(res) =
					futures::executor::block_on(MainchainClient::wait_for_ext_in_block(progress))
				{
					panic!("Error processing price index!! {:?}", res)
				}
			});
		}

		let sleep_time = Duration::from_millis(ticker.time_for_tick(ticker.next())).min(interval);
		sleep(sleep_time).await;
	}
}

#[cfg(test)]
mod tests {
	use sp_core::{
		crypto::{key_types::ACCOUNT, AccountId32},
		sr25519, Pair,
	};
	use sp_keystore::{testing::MemoryKeystore, Keystore};
	use sp_runtime::FixedU128;
	use tokio::spawn;

	use ulixee_client::{api, signer::KeystoreSigner};
	use ulx_primitives::CryptoType;
	use ulx_testing::start_ulx_test_node;

	use crate::{btc_price::use_mock_btc_price, price_index_loop, us_cpi::use_mock_cpi_values};

	#[tokio::test]
	async fn can_submit_multiple_price_indices() {
		let node = start_ulx_test_node().await;
		let keystore = MemoryKeystore::new();
		let keypair = sr25519::Pair::from_string("//Alice", None).unwrap();
		keystore.insert(ACCOUNT, "//Alice", &keypair.public().0).unwrap();
		let account_id: AccountId32 = keypair.public().into();

		let signer = KeystoreSigner::new(keystore.into(), account_id, CryptoType::Sr25519);
		spawn(price_index_loop(node.client.url.clone(), signer, true));

		let mut block_sub = node.client.live.blocks().subscribe_best().await.unwrap();

		use_mock_btc_price(FixedU128::from_float(62_000.23));
		use_mock_cpi_values(vec![0.2, 0.1, -0.1, 0.3]).await;
		let mut counter = 0;
		let mut blocks = 0;
		while let Some(Ok(block)) = block_sub.next().await {
			blocks += 1;
			let price_index = block
				.events()
				.await
				.unwrap()
				.find_first::<api::price_index::events::NewIndex>()
				.unwrap();
			if price_index.is_some() {
				counter += 1;
				if counter > 3 {
					break;
				}
			}
			if blocks > 10 {
				break;
			}
		}
		assert!(counter >= 3);
	}
}
