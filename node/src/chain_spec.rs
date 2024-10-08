use std::{env, time::Duration};

use sc_service::{ChainType, Properties};
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

use argon_node_runtime::{opaque::SessionKeys, AccountId, Balance, Signature, WASM_BINARY};
use argon_primitives::{
	bitcoin::BitcoinNetwork,
	block_seal::{MiningRegistration, RewardDestination},
	block_vote::VoteMinimum,
	notary::{GenesisNotary, NotaryPublic},
	tick::{Ticker, TICK_MILLIS},
	BlockSealAuthorityId, ComputeDifficulty, ADDRESS_PREFIX,
};
// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

fn session_keys(block_seal_authority: BlockSealAuthorityId, grandpa: GrandpaId) -> SessionKeys {
	SessionKeys { block_seal_authority, grandpa }
}
/// Generate a BlockSeal authority key.
pub fn authority_keys_from_seed(s: &str) -> (BlockSealAuthorityId, GrandpaId) {
	(get_from_seed::<BlockSealAuthorityId>(s), get_from_seed::<GrandpaId>(s))
}
/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

pub fn development_config() -> Result<ChainSpec, String> {
	let mut properties = Properties::new();
	properties.insert("tokenDecimals".into(), 3.into());
	properties.insert("tokenSymbol".into(), "ARGON".into());
	properties.insert("ss58Format".into(), ADDRESS_PREFIX.into());

	const HASHES_PER_SECOND: u64 = 100;

	Ok(ChainSpec::builder(
		WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?,
		None,
	)
	.with_name("Development")
	.with_id("dev")
	.with_chain_type(ChainType::Development)
	.with_properties(properties)
	.with_genesis_config_patch(testnet_genesis(
		// You have to have an authority to start the chain
		vec![(
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			authority_keys_from_seed("Alice"),
		)],
		// Sudo account
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		BitcoinNetwork::Regtest,
		// Bitcoin utxo tip operator
		get_account_id_from_seed::<sr25519::Public>("Dave"),
		// Price index operator
		get_account_id_from_seed::<sr25519::Public>("Eve"),
		// Pre-funded accounts
		vec![
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			get_account_id_from_seed::<sr25519::Public>("Bob"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie"),
			get_account_id_from_seed::<sr25519::Public>("Eve"),
			get_account_id_from_seed::<sr25519::Public>("Dave"),
		],
		500,
		(TICK_MILLIS * HASHES_PER_SECOND / 1_000) as ComputeDifficulty,
		TICK_MILLIS,
		vec![], // No notaries
	))
	.build())
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let mut properties = Properties::new();
	properties.insert("tokenDecimals".into(), 3.into());

	let notary_host = env::var("ARGON_LOCAL_TESTNET_NOTARY_URL")
		.unwrap_or("ws://127.0.0.1:9925".to_string())
		.into();
	const HASHES_PER_SECOND: u64 = 1_000;
	Ok(ChainSpec::builder(
		WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?,
		None,
	)
	.with_name("Local Testnet")
	.with_id("local_testnet")
	.with_chain_type(ChainType::Local)
	.with_properties(properties)
	.with_genesis_config_patch(testnet_genesis(
		// Initial BlockSeal authorities
		vec![(
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			authority_keys_from_seed("Alice"),
		)],
		// Sudo account
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		BitcoinNetwork::Regtest,
		// Bitcoin utxo tip operator
		get_account_id_from_seed::<sr25519::Public>("Dave"),
		// Price index operator
		get_account_id_from_seed::<sr25519::Public>("Eve"),
		// Pre-funded accounts
		vec![
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			get_account_id_from_seed::<sr25519::Public>("Bob"),
			get_account_id_from_seed::<sr25519::Public>("Charlie"),
			get_account_id_from_seed::<sr25519::Public>("Dave"),
			get_account_id_from_seed::<sr25519::Public>("Eve"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie//notary"),
		],
		500,
		(TICK_MILLIS * HASHES_PER_SECOND / 1_000) as ComputeDifficulty,
		TICK_MILLIS,
		vec![GenesisNotary {
			account_id: get_account_id_from_seed::<sr25519::Public>("Ferdie"),
			public: get_from_seed::<NotaryPublic>("Ferdie//notary"),
			hosts: vec![notary_host],
			name: "FerdieStamp".into(),
		}],
	))
	.build())
}

#[allow(clippy::too_many_arguments)]
/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	initial_authorities: Vec<(AccountId, (BlockSealAuthorityId, GrandpaId))>,
	root_key: AccountId,
	bitcoin_network: BitcoinNetwork,
	bitcoin_tip_operator: AccountId,
	price_index_operator: AccountId,
	endowed_accounts: Vec<AccountId>,
	initial_vote_minimum: VoteMinimum,
	initial_difficulty: ComputeDifficulty,
	tick_millis: u64,
	initial_notaries: Vec<GenesisNotary<AccountId>>,
) -> serde_json::Value {
	let authority_zero = initial_authorities[0].clone();
	let ticker = Ticker::start(Duration::from_millis(tick_millis));
	let miner_zero = MiningRegistration::<AccountId, Balance> {
		account_id: authority_zero.0.clone(),
		bond_id: None,
		reward_destination: RewardDestination::Owner,
		bond_amount: 0u32.into(),
		ownership_tokens: 0u32.into(),
		reward_sharing: None,
	};

	serde_json::json!({
		"argonBalances": {
			"balances": endowed_accounts.iter().cloned().map(|k| (k, 100_000_000)).collect::<Vec<_>>(),
		},
		"shareBalances": {
			"balances": endowed_accounts.iter().cloned().map(|k| (k, 10_000)).collect::<Vec<_>>(),
		},
		"priceIndex": {
			"operator": Some(price_index_operator),
		},
		"bitcoinUtxos": {
			"operator": Some(bitcoin_tip_operator),
			"network": bitcoin_network,
		},
		"miningSlot":  {
			"minerZero": Some(miner_zero),
		},
		"sudo":  {
			// Assign network admin rights.
			"key": Some(root_key),
		},
		"ticks":  {
			"tickDurationMillis": tick_millis,
			"genesisUtcTime": ticker.genesis_utc_time,
		},
		"blockSealSpec":  {
			"initialVoteMinimum": initial_vote_minimum,
			"initialComputeDifficulty": initial_difficulty,
		},
		"notaries": {
			"list": initial_notaries,
		},
		"session":  {
			"keys": initial_authorities
				.iter()
				.map(|(id, (block_seal_id, grandpa_id))| {
					(
						id.clone(),
						id.clone(),
						session_keys(block_seal_id.clone(), grandpa_id.clone()),
					)
				})
				.collect::<Vec<_>>(),
		},
	})
}
