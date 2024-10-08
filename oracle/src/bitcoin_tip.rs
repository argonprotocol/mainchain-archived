use std::time::Duration;

use bitcoincore_rpc::{Auth, Client, RpcApi};
use tokio::time::sleep;

use argon_client::{
	api::{runtime_types::argon_primitives::bitcoin as bitcoin_primitives_subxt, tx},
	signer::Signer,
	ArgonConfig, MainchainClient, ReconnectingClient,
};
use argon_primitives::bitcoin::H256Le;

pub async fn bitcoin_loop(
	bitcoin_rpc_url: String,
	bitcoin_rpc_auth: Option<(String, String)>,
	mainchain_rpc_url: String,
	signer: impl Signer<ArgonConfig> + Send + Sync + 'static,
) -> anyhow::Result<()> {
	let mut mainchain_client = ReconnectingClient::new(vec![mainchain_rpc_url.clone()]);
	let auth = if let Some((username, password)) = bitcoin_rpc_auth {
		Auth::UserPass(username, password)
	} else {
		Auth::None
	};
	let client = Client::new(&bitcoin_rpc_url, auth)?;
	tracing::info!("Oracle Started. Connected to bitcoin at {}", bitcoin_rpc_url);

	let mut last_tip = None;
	let account_id = signer.account_id();
	loop {
		let bitcoin_tip = client.get_best_block_hash()?;
		if Some(bitcoin_tip) == last_tip {
			sleep(Duration::from_secs(10)).await;
			continue;
		}
		last_tip = Some(bitcoin_tip);
		let header = client.get_block_header_info(&bitcoin_tip)?;
		tracing::info!("New bitcoin tip: {} {:?}", header.height, bitcoin_tip);
		let bitcoin_height = header.height as u64;
		let bitcoin_tip: H256Le = bitcoin_tip.into();

		let latest_block = tx()
			.bitcoin_utxos()
			.set_confirmed_block(bitcoin_height, bitcoin_primitives_subxt::H256Le(bitcoin_tip.0));

		let client = mainchain_client.get().await?;
		let nonce = client.get_account_nonce_subxt(&account_id).await?;
		let params = MainchainClient::ext_params_builder().nonce(nonce.into()).build();
		let progress = client
			.live
			.tx()
			.sign_and_submit_then_watch(&latest_block, &signer, params)
			.await?;
		tracing::info!("Submitted bitcoin tip {bitcoin_height} with progress: {:?}", progress);
		MainchainClient::wait_for_ext_in_block(progress).await?;
	}
}

#[cfg(test)]
mod tests {
	use argon_client::{
		api::{
			bitcoin_utxos::storage::types::confirmed_bitcoin_block_tip::ConfirmedBitcoinBlockTip,
			storage,
		},
		signer::Sr25519Signer,
	};
	use argon_primitives::bitcoin::BitcoinNetwork;
	use argon_testing::ArgonTestNode;
	use bitcoin::Network;
	use sp_core::{sr25519, Pair};

	use super::*;

	#[tokio::test]
	async fn test_bitcoin_loop() {
		let _ = env_logger::builder().is_test(true).try_init();

		let alice = sr25519::Pair::from_string("//Dave", None).unwrap();

		let signer = Sr25519Signer::new(alice);
		let argon_node =
			ArgonTestNode::start("alice".into()).await.expect("Failed to start argon-node");
		let bitcoind = argon_node.bitcoind.as_ref().expect("Bitcoind not started");
		let network: BitcoinNetwork = argon_node
			.client
			.fetch_storage(&storage().bitcoin_utxos().bitcoin_network(), None)
			.await
			.unwrap()
			.expect("Expected network")
			.into();
		let network: Network = network.into();

		let address = bitcoind
			.client
			.get_new_address(None, None)
			.unwrap()
			.require_network(network)
			.unwrap();
		bitcoind.client.generate_to_address(5, &address).unwrap();

		let (rpc_url, auth) = argon_node.get_bitcoin_url();
		let auth = match auth {
			Auth::None => None,
			Auth::UserPass(u, p) => Some((u, p)),
			Auth::CookieFile(_) => None,
		};
		assert!(get_confirmed_block(&argon_node.client).await.is_none());
		let task = bitcoin_loop(rpc_url.to_string(), auth, argon_node.client.url.clone(), signer);
		let handle = tokio::spawn(task);

		let mut block_watch = argon_node.client.live.blocks().subscribe_best().await.unwrap();
		while let Some(Ok(block)) = block_watch.next().await {
			if block.number() == 5 {
				assert!(get_confirmed_block(&argon_node.client).await.is_some());
				break;
			}
		}

		assert_eq!(get_confirmed_block(&argon_node.client).await.unwrap().block_height, 5);
		assert_eq!(
			get_confirmed_block(&argon_node.client).await.unwrap().block_hash.0,
			bitcoind.client.get_best_block_hash().unwrap().as_raw_hash().as_ref()
		);

		handle.abort();
	}

	async fn get_confirmed_block(client: &MainchainClient) -> Option<ConfirmedBitcoinBlockTip> {
		let best_block = client.best_block_hash().await.unwrap();
		client
			.fetch_storage(
				&storage().bitcoin_utxos().confirmed_bitcoin_block_tip(),
				Some(best_block),
			)
			.await
			.expect("Expected tip")
	}
}
