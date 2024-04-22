#[allow(dead_code, unused_imports, non_camel_case_types)]
#[allow(clippy::all)]
#[allow(rustdoc::broken_intra_doc_links)]
pub mod api {
	#[allow(unused_imports)]
	mod root_mod {
		pub use super::*;
	}
	pub static PALLETS: [&str; 23usize] = [
		"System",
		"Timestamp",
		"Ticks",
		"MiningSlot",
		"Bond",
		"Notaries",
		"Notebook",
		"ChainTransfer",
		"BlockSealSpec",
		"DataDomain",
		"Authorship",
		"Historical",
		"Session",
		"BlockSeal",
		"BlockRewards",
		"Grandpa",
		"Offences",
		"ArgonBalances",
		"Mint",
		"UlixeeBalances",
		"TxPause",
		"TransactionPayment",
		"Sudo",
	];
	pub static RUNTIME_APIS: [&str; 17usize] = [
		"Core",
		"Metadata",
		"BlockBuilder",
		"TaggedTransactionQueue",
		"OffchainWorkerApi",
		"AccountNonceApi",
		"SessionKeys",
		"TransactionPaymentApi",
		"TransactionPaymentCallApi",
		"MiningApis",
		"BlockSealApis",
		"NotaryApis",
		"MiningSlotApi",
		"NotebookApis",
		"TickApis",
		"GrandpaApi",
		"GenesisBuilder",
	];
	#[doc = r" The error type returned when there is a runtime issue."]
	pub type DispatchError = runtime_types::sp_runtime::DispatchError;
	#[doc = r" The outer event enum."]
	pub type Event = runtime_types::ulx_node_runtime::RuntimeEvent;
	#[doc = r" The outer extrinsic enum."]
	pub type Call = runtime_types::ulx_node_runtime::RuntimeCall;
	#[doc = r" The outer error enum representing the DispatchError's Module variant."]
	pub type Error = runtime_types::ulx_node_runtime::RuntimeError;
	pub fn constants() -> ConstantsApi {
		ConstantsApi
	}
	pub fn storage() -> StorageApi {
		StorageApi
	}
	pub fn tx() -> TransactionApi {
		TransactionApi
	}
	pub fn apis() -> runtime_apis::RuntimeApi {
		runtime_apis::RuntimeApi
	}
	pub mod runtime_apis {
		use super::root_mod;
		use super::runtime_types;
		use ::subxt::ext::codec::Encode;
		pub struct RuntimeApi;
		impl RuntimeApi {
			pub fn core(&self) -> core::Core {
				core::Core
			}
			pub fn metadata(&self) -> metadata::Metadata {
				metadata::Metadata
			}
			pub fn block_builder(&self) -> block_builder::BlockBuilder {
				block_builder::BlockBuilder
			}
			pub fn tagged_transaction_queue(
				&self,
			) -> tagged_transaction_queue::TaggedTransactionQueue {
				tagged_transaction_queue::TaggedTransactionQueue
			}
			pub fn offchain_worker_api(&self) -> offchain_worker_api::OffchainWorkerApi {
				offchain_worker_api::OffchainWorkerApi
			}
			pub fn account_nonce_api(&self) -> account_nonce_api::AccountNonceApi {
				account_nonce_api::AccountNonceApi
			}
			pub fn session_keys(&self) -> session_keys::SessionKeys {
				session_keys::SessionKeys
			}
			pub fn transaction_payment_api(
				&self,
			) -> transaction_payment_api::TransactionPaymentApi {
				transaction_payment_api::TransactionPaymentApi
			}
			pub fn transaction_payment_call_api(
				&self,
			) -> transaction_payment_call_api::TransactionPaymentCallApi {
				transaction_payment_call_api::TransactionPaymentCallApi
			}
			pub fn mining_apis(&self) -> mining_apis::MiningApis {
				mining_apis::MiningApis
			}
			pub fn block_seal_apis(&self) -> block_seal_apis::BlockSealApis {
				block_seal_apis::BlockSealApis
			}
			pub fn notary_apis(&self) -> notary_apis::NotaryApis {
				notary_apis::NotaryApis
			}
			pub fn mining_slot_api(&self) -> mining_slot_api::MiningSlotApi {
				mining_slot_api::MiningSlotApi
			}
			pub fn notebook_apis(&self) -> notebook_apis::NotebookApis {
				notebook_apis::NotebookApis
			}
			pub fn tick_apis(&self) -> tick_apis::TickApis {
				tick_apis::TickApis
			}
			pub fn grandpa_api(&self) -> grandpa_api::GrandpaApi {
				grandpa_api::GrandpaApi
			}
			pub fn genesis_builder(&self) -> genesis_builder::GenesisBuilder {
				genesis_builder::GenesisBuilder
			}
		}
		pub mod core {
			use super::root_mod;
			use super::runtime_types;
			#[doc = " The `Core` runtime api that every Substrate runtime needs to implement."]
			pub struct Core;
			impl Core {
				#[doc = " Returns the version of the runtime."]
				pub fn version(
					&self,
				) -> ::subxt::runtime_api::Payload<types::Version, types::version::output::Output> {
					::subxt::runtime_api::Payload::new_static(
						"Core",
						"version",
						types::Version {},
						[
							76u8, 202u8, 17u8, 117u8, 189u8, 237u8, 239u8, 237u8, 151u8, 17u8,
							125u8, 159u8, 218u8, 92u8, 57u8, 238u8, 64u8, 147u8, 40u8, 72u8, 157u8,
							116u8, 37u8, 195u8, 156u8, 27u8, 123u8, 173u8, 178u8, 102u8, 136u8,
							6u8,
						],
					)
				}
				#[doc = " Execute the given block."]
				pub fn execute_block(
					&self,
					block: types::execute_block::Block,
				) -> ::subxt::runtime_api::Payload<
					types::ExecuteBlock,
					types::execute_block::output::Output,
				> {
					::subxt::runtime_api::Payload::new_static(
						"Core",
						"execute_block",
						types::ExecuteBlock { block },
						[
							133u8, 135u8, 228u8, 65u8, 106u8, 27u8, 85u8, 158u8, 112u8, 254u8,
							93u8, 26u8, 102u8, 201u8, 118u8, 216u8, 249u8, 247u8, 91u8, 74u8, 56u8,
							208u8, 231u8, 115u8, 131u8, 29u8, 209u8, 6u8, 65u8, 57u8, 214u8, 125u8,
						],
					)
				}
				#[doc = " Initialize a block with the given header and return the runtime executive mode."]
				pub fn initialize_block(
					&self,
					header: types::initialize_block::Header,
				) -> ::subxt::runtime_api::Payload<
					types::InitializeBlock,
					types::initialize_block::output::Output,
				> {
					::subxt::runtime_api::Payload::new_static(
						"Core",
						"initialize_block",
						types::InitializeBlock { header },
						[
							132u8, 169u8, 113u8, 112u8, 80u8, 139u8, 113u8, 35u8, 41u8, 81u8, 36u8,
							35u8, 37u8, 202u8, 29u8, 207u8, 205u8, 229u8, 145u8, 7u8, 133u8, 94u8,
							25u8, 108u8, 233u8, 86u8, 234u8, 29u8, 236u8, 57u8, 56u8, 186u8,
						],
					)
				}
			}
			pub mod types {
				use super::runtime_types;
				pub mod version {
					use super::runtime_types;
					pub mod output {
						use super::runtime_types;
						pub type Output = runtime_types::sp_version::RuntimeVersion;
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Version {}
				pub mod execute_block {
					use super::runtime_types;
					pub type Block = runtime_types :: sp_runtime :: generic :: block :: Block < runtime_types :: sp_runtime :: generic :: header :: Header < :: core :: primitive :: u32 > , :: subxt :: utils :: UncheckedExtrinsic < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , () > , runtime_types :: ulx_node_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment ,) > > ;
					pub mod output {
						use super::runtime_types;
						pub type Output = ();
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ExecuteBlock {
					pub block: execute_block::Block,
				}
				pub mod initialize_block {
					use super::runtime_types;
					pub type Header =
						runtime_types::sp_runtime::generic::header::Header<::core::primitive::u32>;
					pub mod output {
						use super::runtime_types;
						pub type Output = runtime_types::sp_runtime::ExtrinsicInclusionMode;
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct InitializeBlock {
					pub header: initialize_block::Header,
				}
			}
		}
		pub mod metadata {
			use super::root_mod;
			use super::runtime_types;
			#[doc = " The `Metadata` api trait that returns metadata for the runtime."]
			pub struct Metadata;
			impl Metadata {
				#[doc = " Returns the metadata of a runtime."]
				pub fn metadata(
					&self,
				) -> ::subxt::runtime_api::Payload<types::Metadata, types::metadata::output::Output>
				{
					::subxt::runtime_api::Payload::new_static(
						"Metadata",
						"metadata",
						types::Metadata {},
						[
							231u8, 24u8, 67u8, 152u8, 23u8, 26u8, 188u8, 82u8, 229u8, 6u8, 185u8,
							27u8, 175u8, 68u8, 83u8, 122u8, 69u8, 89u8, 185u8, 74u8, 248u8, 87u8,
							217u8, 124u8, 193u8, 252u8, 199u8, 186u8, 196u8, 179u8, 179u8, 96u8,
						],
					)
				}
				#[doc = " Returns the metadata at a given version."]
				#[doc = ""]
				#[doc = " If the given `version` isn't supported, this will return `None`."]
				#[doc = " Use [`Self::metadata_versions`] to find out about supported metadata version of the runtime."]
				pub fn metadata_at_version(
					&self,
					version: types::metadata_at_version::Version,
				) -> ::subxt::runtime_api::Payload<
					types::MetadataAtVersion,
					types::metadata_at_version::output::Output,
				> {
					::subxt::runtime_api::Payload::new_static(
						"Metadata",
						"metadata_at_version",
						types::MetadataAtVersion { version },
						[
							131u8, 53u8, 212u8, 234u8, 16u8, 25u8, 120u8, 252u8, 153u8, 153u8,
							216u8, 28u8, 54u8, 113u8, 52u8, 236u8, 146u8, 68u8, 142u8, 8u8, 10u8,
							169u8, 131u8, 142u8, 204u8, 38u8, 48u8, 108u8, 134u8, 86u8, 226u8,
							61u8,
						],
					)
				}
				#[doc = " Returns the supported metadata versions."]
				#[doc = ""]
				#[doc = " This can be used to call `metadata_at_version`."]
				pub fn metadata_versions(
					&self,
				) -> ::subxt::runtime_api::Payload<
					types::MetadataVersions,
					types::metadata_versions::output::Output,
				> {
					::subxt::runtime_api::Payload::new_static(
						"Metadata",
						"metadata_versions",
						types::MetadataVersions {},
						[
							23u8, 144u8, 137u8, 91u8, 188u8, 39u8, 231u8, 208u8, 252u8, 218u8,
							224u8, 176u8, 77u8, 32u8, 130u8, 212u8, 223u8, 76u8, 100u8, 190u8,
							82u8, 94u8, 190u8, 8u8, 82u8, 244u8, 225u8, 179u8, 85u8, 176u8, 56u8,
							16u8,
						],
					)
				}
			}
			pub mod types {
				use super::runtime_types;
				pub mod metadata {
					use super::runtime_types;
					pub mod output {
						use super::runtime_types;
						pub type Output = runtime_types::sp_core::OpaqueMetadata;
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Metadata {}
				pub mod metadata_at_version {
					use super::runtime_types;
					pub type Version = ::core::primitive::u32;
					pub mod output {
						use super::runtime_types;
						pub type Output =
							::core::option::Option<runtime_types::sp_core::OpaqueMetadata>;
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct MetadataAtVersion {
					pub version: metadata_at_version::Version,
				}
				pub mod metadata_versions {
					use super::runtime_types;
					pub mod output {
						use super::runtime_types;
						pub type Output = ::std::vec::Vec<::core::primitive::u32>;
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct MetadataVersions {}
			}
		}
		pub mod block_builder {
			use super::root_mod;
			use super::runtime_types;
			#[doc = " The `BlockBuilder` api trait that provides the required functionality for building a block."]
			pub struct BlockBuilder;
			impl BlockBuilder {
				#[doc = " Apply the given extrinsic."]
				#[doc = ""]
				#[doc = " Returns an inclusion outcome which specifies if this extrinsic is included in"]
				#[doc = " this block or not."]
				pub fn apply_extrinsic(
					&self,
					extrinsic: types::apply_extrinsic::Extrinsic,
				) -> ::subxt::runtime_api::Payload<
					types::ApplyExtrinsic,
					types::apply_extrinsic::output::Output,
				> {
					::subxt::runtime_api::Payload::new_static(
						"BlockBuilder",
						"apply_extrinsic",
						types::ApplyExtrinsic { extrinsic },
						[
							72u8, 54u8, 139u8, 3u8, 118u8, 136u8, 65u8, 47u8, 6u8, 105u8, 125u8,
							223u8, 160u8, 29u8, 103u8, 74u8, 79u8, 149u8, 48u8, 90u8, 237u8, 2u8,
							97u8, 201u8, 123u8, 34u8, 167u8, 37u8, 187u8, 35u8, 176u8, 97u8,
						],
					)
				}
				#[doc = " Finish the current block."]
				pub fn finalize_block(
					&self,
				) -> ::subxt::runtime_api::Payload<
					types::FinalizeBlock,
					types::finalize_block::output::Output,
				> {
					::subxt::runtime_api::Payload::new_static(
						"BlockBuilder",
						"finalize_block",
						types::FinalizeBlock {},
						[
							244u8, 207u8, 24u8, 33u8, 13u8, 69u8, 9u8, 249u8, 145u8, 143u8, 122u8,
							96u8, 197u8, 55u8, 64u8, 111u8, 238u8, 224u8, 34u8, 201u8, 27u8, 146u8,
							232u8, 99u8, 191u8, 30u8, 114u8, 16u8, 32u8, 220u8, 58u8, 62u8,
						],
					)
				}
				#[doc = " Generate inherent extrinsics. The inherent data will vary from chain to chain."]
				pub fn inherent_extrinsics(
					&self,
					inherent: types::inherent_extrinsics::Inherent,
				) -> ::subxt::runtime_api::Payload<
					types::InherentExtrinsics,
					types::inherent_extrinsics::output::Output,
				> {
					::subxt::runtime_api::Payload::new_static(
						"BlockBuilder",
						"inherent_extrinsics",
						types::InherentExtrinsics { inherent },
						[
							254u8, 110u8, 245u8, 201u8, 250u8, 192u8, 27u8, 228u8, 151u8, 213u8,
							166u8, 89u8, 94u8, 81u8, 189u8, 234u8, 64u8, 18u8, 245u8, 80u8, 29u8,
							18u8, 140u8, 129u8, 113u8, 236u8, 135u8, 55u8, 79u8, 159u8, 175u8,
							183u8,
						],
					)
				}
				#[doc = " Check that the inherents are valid. The inherent data will vary from chain to chain."]
				pub fn check_inherents(
					&self,
					block: types::check_inherents::Block,
					data: types::check_inherents::Data,
				) -> ::subxt::runtime_api::Payload<
					types::CheckInherents,
					types::check_inherents::output::Output,
				> {
					::subxt::runtime_api::Payload::new_static(
						"BlockBuilder",
						"check_inherents",
						types::CheckInherents { block, data },
						[
							153u8, 134u8, 1u8, 215u8, 139u8, 11u8, 53u8, 51u8, 210u8, 175u8, 197u8,
							28u8, 38u8, 209u8, 175u8, 247u8, 142u8, 157u8, 50u8, 151u8, 164u8,
							191u8, 181u8, 118u8, 80u8, 97u8, 160u8, 248u8, 110u8, 217u8, 181u8,
							234u8,
						],
					)
				}
			}
			pub mod types {
				use super::runtime_types;
				pub mod apply_extrinsic {
					use super::runtime_types;
					pub type Extrinsic = :: subxt :: utils :: UncheckedExtrinsic < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , () > , runtime_types :: ulx_node_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment ,) > ;
					pub mod output {
						use super::runtime_types;
						pub type Output = :: core :: result :: Result < :: core :: result :: Result < () , runtime_types :: sp_runtime :: DispatchError > , runtime_types :: sp_runtime :: transaction_validity :: TransactionValidityError > ;
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ApplyExtrinsic {
					pub extrinsic: apply_extrinsic::Extrinsic,
				}
				pub mod finalize_block {
					use super::runtime_types;
					pub mod output {
						use super::runtime_types;
						pub type Output = runtime_types::sp_runtime::generic::header::Header<
							::core::primitive::u32,
						>;
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct FinalizeBlock {}
				pub mod inherent_extrinsics {
					use super::runtime_types;
					pub type Inherent = runtime_types::sp_inherents::InherentData;
					pub mod output {
						use super::runtime_types;
						pub type Output = :: std :: vec :: Vec < :: subxt :: utils :: UncheckedExtrinsic < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , () > , runtime_types :: ulx_node_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment ,) > > ;
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct InherentExtrinsics {
					pub inherent: inherent_extrinsics::Inherent,
				}
				pub mod check_inherents {
					use super::runtime_types;
					pub type Block = runtime_types :: sp_runtime :: generic :: block :: Block < runtime_types :: sp_runtime :: generic :: header :: Header < :: core :: primitive :: u32 > , :: subxt :: utils :: UncheckedExtrinsic < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , () > , runtime_types :: ulx_node_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment ,) > > ;
					pub type Data = runtime_types::sp_inherents::InherentData;
					pub mod output {
						use super::runtime_types;
						pub type Output = runtime_types::sp_inherents::CheckInherentsResult;
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct CheckInherents {
					pub block: check_inherents::Block,
					pub data: check_inherents::Data,
				}
			}
		}
		pub mod tagged_transaction_queue {
			use super::root_mod;
			use super::runtime_types;
			#[doc = " The `TaggedTransactionQueue` api trait for interfering with the transaction queue."]
			pub struct TaggedTransactionQueue;
			impl TaggedTransactionQueue {
				#[doc = " Validate the transaction."]
				#[doc = ""]
				#[doc = " This method is invoked by the transaction pool to learn details about given transaction."]
				#[doc = " The implementation should make sure to verify the correctness of the transaction"]
				#[doc = " against current state. The given `block_hash` corresponds to the hash of the block"]
				#[doc = " that is used as current state."]
				#[doc = ""]
				#[doc = " Note that this call may be performed by the pool multiple times and transactions"]
				#[doc = " might be verified in any possible order."]
				pub fn validate_transaction(
					&self,
					source: types::validate_transaction::Source,
					tx: types::validate_transaction::Tx,
					block_hash: types::validate_transaction::BlockHash,
				) -> ::subxt::runtime_api::Payload<
					types::ValidateTransaction,
					types::validate_transaction::output::Output,
				> {
					::subxt::runtime_api::Payload::new_static(
						"TaggedTransactionQueue",
						"validate_transaction",
						types::ValidateTransaction { source, tx, block_hash },
						[
							196u8, 50u8, 90u8, 49u8, 109u8, 251u8, 200u8, 35u8, 23u8, 150u8, 140u8,
							143u8, 232u8, 164u8, 133u8, 89u8, 32u8, 240u8, 115u8, 39u8, 95u8, 70u8,
							162u8, 76u8, 122u8, 73u8, 151u8, 144u8, 234u8, 120u8, 100u8, 29u8,
						],
					)
				}
			}
			pub mod types {
				use super::runtime_types;
				pub mod validate_transaction {
					use super::runtime_types;
					pub type Source =
						runtime_types::sp_runtime::transaction_validity::TransactionSource;
					pub type Tx = :: subxt :: utils :: UncheckedExtrinsic < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , () > , runtime_types :: ulx_node_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment ,) > ;
					pub type BlockHash = ::subxt::utils::H256;
					pub mod output {
						use super::runtime_types;
						pub type Output = :: core :: result :: Result < runtime_types :: sp_runtime :: transaction_validity :: ValidTransaction , runtime_types :: sp_runtime :: transaction_validity :: TransactionValidityError > ;
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ValidateTransaction {
					pub source: validate_transaction::Source,
					pub tx: validate_transaction::Tx,
					pub block_hash: validate_transaction::BlockHash,
				}
			}
		}
		pub mod offchain_worker_api {
			use super::root_mod;
			use super::runtime_types;
			#[doc = " The offchain worker api."]
			pub struct OffchainWorkerApi;
			impl OffchainWorkerApi {
				#[doc = " Starts the off-chain task for given block header."]
				pub fn offchain_worker(
					&self,
					header: types::offchain_worker::Header,
				) -> ::subxt::runtime_api::Payload<
					types::OffchainWorker,
					types::offchain_worker::output::Output,
				> {
					::subxt::runtime_api::Payload::new_static(
						"OffchainWorkerApi",
						"offchain_worker",
						types::OffchainWorker { header },
						[
							10u8, 135u8, 19u8, 153u8, 33u8, 216u8, 18u8, 242u8, 33u8, 140u8, 4u8,
							223u8, 200u8, 130u8, 103u8, 118u8, 137u8, 24u8, 19u8, 127u8, 161u8,
							29u8, 184u8, 111u8, 222u8, 111u8, 253u8, 73u8, 45u8, 31u8, 79u8, 60u8,
						],
					)
				}
			}
			pub mod types {
				use super::runtime_types;
				pub mod offchain_worker {
					use super::runtime_types;
					pub type Header =
						runtime_types::sp_runtime::generic::header::Header<::core::primitive::u32>;
					pub mod output {
						use super::runtime_types;
						pub type Output = ();
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct OffchainWorker {
					pub header: offchain_worker::Header,
				}
			}
		}
		pub mod account_nonce_api {
			use super::root_mod;
			use super::runtime_types;
			#[doc = " The API to query account nonce."]
			pub struct AccountNonceApi;
			impl AccountNonceApi {
				#[doc = " Get current account nonce of given `AccountId`."]
				pub fn account_nonce(
					&self,
					account: types::account_nonce::Account,
				) -> ::subxt::runtime_api::Payload<
					types::AccountNonce,
					types::account_nonce::output::Output,
				> {
					::subxt::runtime_api::Payload::new_static(
						"AccountNonceApi",
						"account_nonce",
						types::AccountNonce { account },
						[
							231u8, 82u8, 7u8, 227u8, 131u8, 2u8, 215u8, 252u8, 173u8, 82u8, 11u8,
							103u8, 200u8, 25u8, 114u8, 116u8, 79u8, 229u8, 152u8, 150u8, 236u8,
							37u8, 101u8, 26u8, 220u8, 146u8, 182u8, 101u8, 73u8, 55u8, 191u8,
							171u8,
						],
					)
				}
			}
			pub mod types {
				use super::runtime_types;
				pub mod account_nonce {
					use super::runtime_types;
					pub type Account = ::subxt::utils::AccountId32;
					pub mod output {
						use super::runtime_types;
						pub type Output = ::core::primitive::u32;
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct AccountNonce {
					pub account: account_nonce::Account,
				}
			}
		}
		pub mod session_keys {
			use super::root_mod;
			use super::runtime_types;
			#[doc = " Session keys runtime api."]
			pub struct SessionKeys;
			impl SessionKeys {
				#[doc = " Generate a set of session keys with optionally using the given seed."]
				#[doc = " The keys should be stored within the keystore exposed via runtime"]
				#[doc = " externalities."]
				#[doc = ""]
				#[doc = " The seed needs to be a valid `utf8` string."]
				#[doc = ""]
				#[doc = " Returns the concatenated SCALE encoded public keys."]
				pub fn generate_session_keys(
					&self,
					seed: types::generate_session_keys::Seed,
				) -> ::subxt::runtime_api::Payload<
					types::GenerateSessionKeys,
					types::generate_session_keys::output::Output,
				> {
					::subxt::runtime_api::Payload::new_static(
						"SessionKeys",
						"generate_session_keys",
						types::GenerateSessionKeys { seed },
						[
							96u8, 171u8, 164u8, 166u8, 175u8, 102u8, 101u8, 47u8, 133u8, 95u8,
							102u8, 202u8, 83u8, 26u8, 238u8, 47u8, 126u8, 132u8, 22u8, 11u8, 33u8,
							190u8, 175u8, 94u8, 58u8, 245u8, 46u8, 80u8, 195u8, 184u8, 107u8, 65u8,
						],
					)
				}
				#[doc = " Decode the given public session keys."]
				#[doc = ""]
				#[doc = " Returns the list of public raw public keys + key type."]
				pub fn decode_session_keys(
					&self,
					encoded: types::decode_session_keys::Encoded,
				) -> ::subxt::runtime_api::Payload<
					types::DecodeSessionKeys,
					types::decode_session_keys::output::Output,
				> {
					::subxt::runtime_api::Payload::new_static(
						"SessionKeys",
						"decode_session_keys",
						types::DecodeSessionKeys { encoded },
						[
							57u8, 242u8, 18u8, 51u8, 132u8, 110u8, 238u8, 255u8, 39u8, 194u8, 8u8,
							54u8, 198u8, 178u8, 75u8, 151u8, 148u8, 176u8, 144u8, 197u8, 87u8,
							29u8, 179u8, 235u8, 176u8, 78u8, 252u8, 103u8, 72u8, 203u8, 151u8,
							248u8,
						],
					)
				}
			}
			pub mod types {
				use super::runtime_types;
				pub mod generate_session_keys {
					use super::runtime_types;
					pub type Seed = ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>;
					pub mod output {
						use super::runtime_types;
						pub type Output = ::std::vec::Vec<::core::primitive::u8>;
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct GenerateSessionKeys {
					pub seed: generate_session_keys::Seed,
				}
				pub mod decode_session_keys {
					use super::runtime_types;
					pub type Encoded = ::std::vec::Vec<::core::primitive::u8>;
					pub mod output {
						use super::runtime_types;
						pub type Output = ::core::option::Option<
							::std::vec::Vec<(
								::std::vec::Vec<::core::primitive::u8>,
								runtime_types::sp_core::crypto::KeyTypeId,
							)>,
						>;
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct DecodeSessionKeys {
					pub encoded: decode_session_keys::Encoded,
				}
			}
		}
		pub mod transaction_payment_api {
			use super::root_mod;
			use super::runtime_types;
			pub struct TransactionPaymentApi;
			impl TransactionPaymentApi {
				pub fn query_info(
					&self,
					uxt: types::query_info::Uxt,
					len: types::query_info::Len,
				) -> ::subxt::runtime_api::Payload<
					types::QueryInfo,
					types::query_info::output::Output,
				> {
					::subxt::runtime_api::Payload::new_static(
						"TransactionPaymentApi",
						"query_info",
						types::QueryInfo { uxt, len },
						[
							56u8, 30u8, 174u8, 34u8, 202u8, 24u8, 177u8, 189u8, 145u8, 36u8, 1u8,
							156u8, 98u8, 209u8, 178u8, 49u8, 198u8, 23u8, 150u8, 173u8, 35u8,
							205u8, 147u8, 129u8, 42u8, 22u8, 69u8, 3u8, 129u8, 8u8, 196u8, 139u8,
						],
					)
				}
				pub fn query_fee_details(
					&self,
					uxt: types::query_fee_details::Uxt,
					len: types::query_fee_details::Len,
				) -> ::subxt::runtime_api::Payload<
					types::QueryFeeDetails,
					types::query_fee_details::output::Output,
				> {
					::subxt::runtime_api::Payload::new_static(
						"TransactionPaymentApi",
						"query_fee_details",
						types::QueryFeeDetails { uxt, len },
						[
							117u8, 60u8, 137u8, 159u8, 237u8, 252u8, 216u8, 238u8, 232u8, 1u8,
							100u8, 152u8, 26u8, 185u8, 145u8, 125u8, 68u8, 189u8, 4u8, 30u8, 125u8,
							7u8, 196u8, 153u8, 235u8, 51u8, 219u8, 108u8, 185u8, 254u8, 100u8,
							201u8,
						],
					)
				}
				pub fn query_weight_to_fee(
					&self,
					weight: types::query_weight_to_fee::Weight,
				) -> ::subxt::runtime_api::Payload<
					types::QueryWeightToFee,
					types::query_weight_to_fee::output::Output,
				> {
					::subxt::runtime_api::Payload::new_static(
						"TransactionPaymentApi",
						"query_weight_to_fee",
						types::QueryWeightToFee { weight },
						[
							206u8, 243u8, 189u8, 83u8, 231u8, 244u8, 247u8, 52u8, 126u8, 208u8,
							224u8, 5u8, 163u8, 108u8, 254u8, 114u8, 214u8, 156u8, 227u8, 217u8,
							211u8, 198u8, 121u8, 164u8, 110u8, 54u8, 181u8, 146u8, 50u8, 146u8,
							146u8, 23u8,
						],
					)
				}
				pub fn query_length_to_fee(
					&self,
					length: types::query_length_to_fee::Length,
				) -> ::subxt::runtime_api::Payload<
					types::QueryLengthToFee,
					types::query_length_to_fee::output::Output,
				> {
					::subxt::runtime_api::Payload::new_static(
						"TransactionPaymentApi",
						"query_length_to_fee",
						types::QueryLengthToFee { length },
						[
							92u8, 132u8, 29u8, 119u8, 66u8, 11u8, 196u8, 224u8, 129u8, 23u8, 249u8,
							12u8, 32u8, 28u8, 92u8, 50u8, 188u8, 101u8, 203u8, 229u8, 248u8, 216u8,
							130u8, 150u8, 212u8, 161u8, 81u8, 254u8, 116u8, 89u8, 162u8, 48u8,
						],
					)
				}
			}
			pub mod types {
				use super::runtime_types;
				pub mod query_info {
					use super::runtime_types;
					pub type Uxt = :: subxt :: utils :: UncheckedExtrinsic < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , () > , runtime_types :: ulx_node_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment ,) > ;
					pub type Len = ::core::primitive::u32;
					pub mod output {
						use super::runtime_types;
						pub type Output =
							runtime_types::pallet_transaction_payment::types::RuntimeDispatchInfo<
								::core::primitive::u128,
								runtime_types::sp_weights::weight_v2::Weight,
							>;
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct QueryInfo {
					pub uxt: query_info::Uxt,
					pub len: query_info::Len,
				}
				pub mod query_fee_details {
					use super::runtime_types;
					pub type Uxt = :: subxt :: utils :: UncheckedExtrinsic < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , () > , runtime_types :: ulx_node_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment ,) > ;
					pub type Len = ::core::primitive::u32;
					pub mod output {
						use super::runtime_types;
						pub type Output =
							runtime_types::pallet_transaction_payment::types::FeeDetails<
								::core::primitive::u128,
							>;
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct QueryFeeDetails {
					pub uxt: query_fee_details::Uxt,
					pub len: query_fee_details::Len,
				}
				pub mod query_weight_to_fee {
					use super::runtime_types;
					pub type Weight = runtime_types::sp_weights::weight_v2::Weight;
					pub mod output {
						use super::runtime_types;
						pub type Output = ::core::primitive::u128;
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct QueryWeightToFee {
					pub weight: query_weight_to_fee::Weight,
				}
				pub mod query_length_to_fee {
					use super::runtime_types;
					pub type Length = ::core::primitive::u32;
					pub mod output {
						use super::runtime_types;
						pub type Output = ::core::primitive::u128;
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct QueryLengthToFee {
					pub length: query_length_to_fee::Length,
				}
			}
		}
		pub mod transaction_payment_call_api {
			use super::root_mod;
			use super::runtime_types;
			pub struct TransactionPaymentCallApi;
			impl TransactionPaymentCallApi {
				#[doc = " Query information of a dispatch class, weight, and fee of a given encoded `Call`."]
				pub fn query_call_info(
					&self,
					call: types::query_call_info::Call,
					len: types::query_call_info::Len,
				) -> ::subxt::runtime_api::Payload<
					types::QueryCallInfo,
					types::query_call_info::output::Output,
				> {
					::subxt::runtime_api::Payload::new_static(
						"TransactionPaymentCallApi",
						"query_call_info",
						types::QueryCallInfo { call, len },
						[
							41u8, 88u8, 107u8, 173u8, 84u8, 218u8, 118u8, 142u8, 132u8, 255u8,
							12u8, 60u8, 242u8, 255u8, 151u8, 228u8, 143u8, 175u8, 251u8, 119u8,
							223u8, 205u8, 24u8, 63u8, 39u8, 2u8, 155u8, 105u8, 157u8, 51u8, 183u8,
							174u8,
						],
					)
				}
				#[doc = " Query fee details of a given encoded `Call`."]
				pub fn query_call_fee_details(
					&self,
					call: types::query_call_fee_details::Call,
					len: types::query_call_fee_details::Len,
				) -> ::subxt::runtime_api::Payload<
					types::QueryCallFeeDetails,
					types::query_call_fee_details::output::Output,
				> {
					::subxt::runtime_api::Payload::new_static(
						"TransactionPaymentCallApi",
						"query_call_fee_details",
						types::QueryCallFeeDetails { call, len },
						[
							169u8, 220u8, 56u8, 116u8, 51u8, 35u8, 150u8, 245u8, 188u8, 94u8,
							188u8, 9u8, 191u8, 131u8, 163u8, 84u8, 140u8, 206u8, 161u8, 135u8,
							207u8, 147u8, 71u8, 200u8, 176u8, 89u8, 6u8, 210u8, 187u8, 241u8,
							210u8, 97u8,
						],
					)
				}
				#[doc = " Query the output of the current `WeightToFee` given some input."]
				pub fn query_weight_to_fee(
					&self,
					weight: types::query_weight_to_fee::Weight,
				) -> ::subxt::runtime_api::Payload<
					types::QueryWeightToFee,
					types::query_weight_to_fee::output::Output,
				> {
					::subxt::runtime_api::Payload::new_static(
						"TransactionPaymentCallApi",
						"query_weight_to_fee",
						types::QueryWeightToFee { weight },
						[
							117u8, 91u8, 94u8, 22u8, 248u8, 212u8, 15u8, 23u8, 97u8, 116u8, 64u8,
							228u8, 83u8, 123u8, 87u8, 77u8, 97u8, 7u8, 98u8, 181u8, 6u8, 165u8,
							114u8, 141u8, 164u8, 113u8, 126u8, 88u8, 174u8, 171u8, 224u8, 35u8,
						],
					)
				}
				#[doc = " Query the output of the current `LengthToFee` given some input."]
				pub fn query_length_to_fee(
					&self,
					length: types::query_length_to_fee::Length,
				) -> ::subxt::runtime_api::Payload<
					types::QueryLengthToFee,
					types::query_length_to_fee::output::Output,
				> {
					::subxt::runtime_api::Payload::new_static(
						"TransactionPaymentCallApi",
						"query_length_to_fee",
						types::QueryLengthToFee { length },
						[
							246u8, 40u8, 4u8, 160u8, 152u8, 94u8, 170u8, 53u8, 205u8, 122u8, 5u8,
							69u8, 70u8, 25u8, 128u8, 156u8, 119u8, 134u8, 116u8, 147u8, 14u8,
							164u8, 65u8, 140u8, 86u8, 13u8, 250u8, 218u8, 89u8, 95u8, 234u8, 228u8,
						],
					)
				}
			}
			pub mod types {
				use super::runtime_types;
				pub mod query_call_info {
					use super::runtime_types;
					pub type Call = runtime_types::ulx_node_runtime::RuntimeCall;
					pub type Len = ::core::primitive::u32;
					pub mod output {
						use super::runtime_types;
						pub type Output =
							runtime_types::pallet_transaction_payment::types::RuntimeDispatchInfo<
								::core::primitive::u128,
								runtime_types::sp_weights::weight_v2::Weight,
							>;
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct QueryCallInfo {
					pub call: query_call_info::Call,
					pub len: query_call_info::Len,
				}
				pub mod query_call_fee_details {
					use super::runtime_types;
					pub type Call = runtime_types::ulx_node_runtime::RuntimeCall;
					pub type Len = ::core::primitive::u32;
					pub mod output {
						use super::runtime_types;
						pub type Output =
							runtime_types::pallet_transaction_payment::types::FeeDetails<
								::core::primitive::u128,
							>;
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct QueryCallFeeDetails {
					pub call: query_call_fee_details::Call,
					pub len: query_call_fee_details::Len,
				}
				pub mod query_weight_to_fee {
					use super::runtime_types;
					pub type Weight = runtime_types::sp_weights::weight_v2::Weight;
					pub mod output {
						use super::runtime_types;
						pub type Output = ::core::primitive::u128;
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct QueryWeightToFee {
					pub weight: query_weight_to_fee::Weight,
				}
				pub mod query_length_to_fee {
					use super::runtime_types;
					pub type Length = ::core::primitive::u32;
					pub mod output {
						use super::runtime_types;
						pub type Output = ::core::primitive::u128;
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct QueryLengthToFee {
					pub length: query_length_to_fee::Length,
				}
			}
		}
		pub mod mining_apis {
			use super::root_mod;
			use super::runtime_types;
			pub struct MiningApis;
			impl MiningApis {
				pub fn get_authority_id(
					&self,
					account_id: types::get_authority_id::AccountId,
				) -> ::subxt::runtime_api::Payload<
					types::GetAuthorityId,
					types::get_authority_id::output::Output,
				> {
					::subxt::runtime_api::Payload::new_static(
						"MiningApis",
						"get_authority_id",
						types::GetAuthorityId { account_id },
						[
							77u8, 76u8, 252u8, 255u8, 70u8, 110u8, 251u8, 108u8, 92u8, 141u8, 6u8,
							122u8, 191u8, 248u8, 214u8, 19u8, 136u8, 46u8, 207u8, 152u8, 27u8,
							241u8, 131u8, 117u8, 28u8, 251u8, 178u8, 207u8, 247u8, 136u8, 204u8,
							164u8,
						],
					)
				}
			}
			pub mod types {
				use super::runtime_types;
				pub mod get_authority_id {
					use super::runtime_types;
					pub type AccountId = ::subxt::utils::AccountId32;
					pub mod output {
						use super::runtime_types;
						pub type Output = ::core::option::Option<
							runtime_types::ulx_primitives::block_seal::MiningAuthority<
								runtime_types::ulx_primitives::block_seal::app::Public,
								::subxt::utils::AccountId32,
							>,
						>;
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct GetAuthorityId {
					pub account_id: get_authority_id::AccountId,
				}
			}
		}
		pub mod block_seal_apis {
			use super::root_mod;
			use super::runtime_types;
			pub struct BlockSealApis;
			impl BlockSealApis {
				pub fn vote_minimum(
					&self,
				) -> ::subxt::runtime_api::Payload<
					types::VoteMinimum,
					types::vote_minimum::output::Output,
				> {
					::subxt::runtime_api::Payload::new_static(
						"BlockSealApis",
						"vote_minimum",
						types::VoteMinimum {},
						[
							34u8, 243u8, 211u8, 48u8, 245u8, 6u8, 82u8, 51u8, 6u8, 166u8, 211u8,
							255u8, 49u8, 101u8, 124u8, 196u8, 54u8, 25u8, 202u8, 165u8, 171u8,
							83u8, 168u8, 132u8, 181u8, 92u8, 125u8, 47u8, 37u8, 172u8, 208u8, 46u8,
						],
					)
				}
				pub fn compute_difficulty(
					&self,
				) -> ::subxt::runtime_api::Payload<
					types::ComputeDifficulty,
					types::compute_difficulty::output::Output,
				> {
					::subxt::runtime_api::Payload::new_static(
						"BlockSealApis",
						"compute_difficulty",
						types::ComputeDifficulty {},
						[
							149u8, 83u8, 109u8, 227u8, 84u8, 55u8, 195u8, 204u8, 71u8, 92u8, 148u8,
							180u8, 227u8, 192u8, 22u8, 15u8, 33u8, 41u8, 176u8, 238u8, 15u8, 218u8,
							52u8, 183u8, 182u8, 199u8, 174u8, 83u8, 84u8, 180u8, 176u8, 57u8,
						],
					)
				}
				pub fn create_vote_digest(
					&self,
					tick: types::create_vote_digest::Tick,
					included_notebooks: types::create_vote_digest::IncludedNotebooks,
				) -> ::subxt::runtime_api::Payload<
					types::CreateVoteDigest,
					types::create_vote_digest::output::Output,
				> {
					::subxt::runtime_api::Payload::new_static(
						"BlockSealApis",
						"create_vote_digest",
						types::CreateVoteDigest { tick, included_notebooks },
						[
							212u8, 58u8, 178u8, 158u8, 47u8, 84u8, 233u8, 9u8, 218u8, 195u8, 151u8,
							229u8, 77u8, 46u8, 81u8, 95u8, 40u8, 152u8, 181u8, 94u8, 27u8, 112u8,
							56u8, 152u8, 11u8, 35u8, 209u8, 138u8, 79u8, 24u8, 30u8, 94u8,
						],
					)
				}
				pub fn find_vote_block_seals(
					&self,
					votes: types::find_vote_block_seals::Votes,
					with_better_strength: types::find_vote_block_seals::WithBetterStrength,
				) -> ::subxt::runtime_api::Payload<
					types::FindVoteBlockSeals,
					types::find_vote_block_seals::output::Output,
				> {
					::subxt::runtime_api::Payload::new_static(
						"BlockSealApis",
						"find_vote_block_seals",
						types::FindVoteBlockSeals { votes, with_better_strength },
						[
							11u8, 172u8, 29u8, 248u8, 46u8, 146u8, 69u8, 138u8, 206u8, 18u8, 2u8,
							200u8, 125u8, 106u8, 244u8, 88u8, 44u8, 56u8, 57u8, 91u8, 130u8, 39u8,
							101u8, 78u8, 29u8, 159u8, 40u8, 45u8, 107u8, 180u8, 86u8, 140u8,
						],
					)
				}
			}
			pub mod types {
				use super::runtime_types;
				pub mod vote_minimum {
					use super::runtime_types;
					pub mod output {
						use super::runtime_types;
						pub type Output = ::core::primitive::u128;
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct VoteMinimum {}
				pub mod compute_difficulty {
					use super::runtime_types;
					pub mod output {
						use super::runtime_types;
						pub type Output = ::core::primitive::u128;
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ComputeDifficulty {}
				pub mod create_vote_digest {
					use super::runtime_types;
					pub type Tick = ::core::primitive::u32;
					pub type IncludedNotebooks = ::std::vec::Vec<
						runtime_types::ulx_primitives::notary::NotaryNotebookVoteDigestDetails,
					>;
					pub mod output {
						use super::runtime_types;
						pub type Output = runtime_types::ulx_primitives::digests::BlockVoteDigest;
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct CreateVoteDigest {
					pub tick: create_vote_digest::Tick,
					pub included_notebooks: create_vote_digest::IncludedNotebooks,
				}
				pub mod find_vote_block_seals {
					use super::runtime_types;
					pub type Votes =
						::std::vec::Vec<runtime_types::ulx_primitives::apis::NotaryNotebookVotes>;
					pub type WithBetterStrength = runtime_types::primitive_types::U256;
					pub mod output {
						use super::runtime_types;
						pub type Output = ::core::result::Result<
							runtime_types::bounded_collections::bounded_vec::BoundedVec<
								runtime_types::ulx_primitives::block_vote::BestBlockVoteSeal<
									::subxt::utils::AccountId32,
									runtime_types::ulx_primitives::block_seal::app::Public,
								>,
							>,
							runtime_types::sp_runtime::DispatchError,
						>;
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct FindVoteBlockSeals {
					pub votes: find_vote_block_seals::Votes,
					pub with_better_strength: find_vote_block_seals::WithBetterStrength,
				}
			}
		}
		pub mod notary_apis {
			use super::root_mod;
			use super::runtime_types;
			pub struct NotaryApis;
			impl NotaryApis {
				pub fn notary_by_id(
					&self,
					notary_id: types::notary_by_id::NotaryId,
				) -> ::subxt::runtime_api::Payload<
					types::NotaryById,
					types::notary_by_id::output::Output,
				> {
					::subxt::runtime_api::Payload::new_static(
						"NotaryApis",
						"notary_by_id",
						types::NotaryById { notary_id },
						[
							222u8, 166u8, 35u8, 137u8, 223u8, 254u8, 225u8, 124u8, 123u8, 149u8,
							173u8, 2u8, 25u8, 223u8, 46u8, 144u8, 115u8, 2u8, 67u8, 116u8, 143u8,
							201u8, 188u8, 146u8, 238u8, 5u8, 52u8, 233u8, 202u8, 236u8, 41u8, 2u8,
						],
					)
				}
				pub fn notaries(
					&self,
				) -> ::subxt::runtime_api::Payload<types::Notaries, types::notaries::output::Output>
				{
					::subxt::runtime_api::Payload::new_static(
						"NotaryApis",
						"notaries",
						types::Notaries {},
						[
							19u8, 198u8, 120u8, 130u8, 98u8, 0u8, 101u8, 197u8, 91u8, 98u8, 148u8,
							69u8, 101u8, 146u8, 244u8, 190u8, 156u8, 167u8, 110u8, 63u8, 40u8,
							194u8, 11u8, 52u8, 60u8, 126u8, 244u8, 211u8, 72u8, 143u8, 224u8,
							109u8,
						],
					)
				}
			}
			pub mod types {
				use super::runtime_types;
				pub mod notary_by_id {
					use super::runtime_types;
					pub type NotaryId = ::core::primitive::u32;
					pub mod output {
						use super::runtime_types;
						pub type Output = ::core::option::Option<
							runtime_types::ulx_primitives::notary::NotaryRecord<
								::subxt::utils::AccountId32,
								::core::primitive::u32,
							>,
						>;
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct NotaryById {
					pub notary_id: notary_by_id::NotaryId,
				}
				pub mod notaries {
					use super::runtime_types;
					pub mod output {
						use super::runtime_types;
						pub type Output = ::std::vec::Vec<
							runtime_types::ulx_primitives::notary::NotaryRecord<
								::subxt::utils::AccountId32,
								::core::primitive::u32,
							>,
						>;
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Notaries {}
			}
		}
		pub mod mining_slot_api {
			use super::root_mod;
			use super::runtime_types;
			#[doc = " This runtime api allows people to query the upcoming mining_slot"]
			pub struct MiningSlotApi;
			impl MiningSlotApi {
				pub fn next_slot_era(
					&self,
				) -> ::subxt::runtime_api::Payload<
					types::NextSlotEra,
					types::next_slot_era::output::Output,
				> {
					::subxt::runtime_api::Payload::new_static(
						"MiningSlotApi",
						"next_slot_era",
						types::NextSlotEra {},
						[
							101u8, 27u8, 62u8, 52u8, 219u8, 204u8, 173u8, 178u8, 17u8, 87u8, 198u8,
							186u8, 162u8, 215u8, 162u8, 64u8, 214u8, 87u8, 101u8, 112u8, 128u8,
							160u8, 134u8, 183u8, 46u8, 141u8, 167u8, 2u8, 15u8, 77u8, 5u8, 224u8,
						],
					)
				}
			}
			pub mod types {
				use super::runtime_types;
				pub mod next_slot_era {
					use super::runtime_types;
					pub mod output {
						use super::runtime_types;
						pub type Output = (::core::primitive::u32, ::core::primitive::u32);
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct NextSlotEra {}
			}
		}
		pub mod notebook_apis {
			use super::root_mod;
			use super::runtime_types;
			pub struct NotebookApis;
			impl NotebookApis {
				pub fn audit_notebook_and_get_votes(
					&self,
					version: types::audit_notebook_and_get_votes::Version,
					notary_id: types::audit_notebook_and_get_votes::NotaryId,
					notebook_number: types::audit_notebook_and_get_votes::NotebookNumber,
					header_hash: types::audit_notebook_and_get_votes::HeaderHash,
					vote_minimums: types::audit_notebook_and_get_votes::VoteMinimums,
					bytes: types::audit_notebook_and_get_votes::Bytes,
					audit_dependency_summaries : types :: audit_notebook_and_get_votes :: AuditDependencySummaries,
				) -> ::subxt::runtime_api::Payload<
					types::AuditNotebookAndGetVotes,
					types::audit_notebook_and_get_votes::output::Output,
				> {
					::subxt::runtime_api::Payload::new_static(
						"NotebookApis",
						"audit_notebook_and_get_votes",
						types::AuditNotebookAndGetVotes {
							version,
							notary_id,
							notebook_number,
							header_hash,
							vote_minimums,
							bytes,
							audit_dependency_summaries,
						},
						[
							216u8, 94u8, 202u8, 134u8, 38u8, 173u8, 164u8, 235u8, 124u8, 165u8,
							206u8, 40u8, 249u8, 40u8, 205u8, 200u8, 78u8, 240u8, 67u8, 83u8, 24u8,
							131u8, 172u8, 216u8, 146u8, 47u8, 70u8, 219u8, 219u8, 199u8, 37u8,
							22u8,
						],
					)
				}
				pub fn decode_signed_raw_notebook_header(
					&self,
					raw_header: types::decode_signed_raw_notebook_header::RawHeader,
				) -> ::subxt::runtime_api::Payload<
					types::DecodeSignedRawNotebookHeader,
					types::decode_signed_raw_notebook_header::output::Output,
				> {
					::subxt::runtime_api::Payload::new_static(
						"NotebookApis",
						"decode_signed_raw_notebook_header",
						types::DecodeSignedRawNotebookHeader { raw_header },
						[
							13u8, 81u8, 253u8, 91u8, 223u8, 71u8, 227u8, 7u8, 31u8, 197u8, 139u8,
							211u8, 168u8, 241u8, 101u8, 203u8, 183u8, 69u8, 120u8, 170u8, 223u8,
							148u8, 215u8, 81u8, 151u8, 177u8, 165u8, 250u8, 119u8, 66u8, 29u8,
							96u8,
						],
					)
				}
				pub fn latest_notebook_by_notary(
					&self,
				) -> ::subxt::runtime_api::Payload<
					types::LatestNotebookByNotary,
					types::latest_notebook_by_notary::output::Output,
				> {
					::subxt::runtime_api::Payload::new_static(
						"NotebookApis",
						"latest_notebook_by_notary",
						types::LatestNotebookByNotary {},
						[
							85u8, 20u8, 202u8, 169u8, 17u8, 113u8, 81u8, 236u8, 115u8, 197u8,
							120u8, 136u8, 102u8, 113u8, 49u8, 102u8, 175u8, 238u8, 64u8, 34u8,
							88u8, 80u8, 194u8, 239u8, 232u8, 40u8, 227u8, 162u8, 135u8, 203u8,
							122u8, 236u8,
						],
					)
				}
			}
			pub mod types {
				use super::runtime_types;
				pub mod audit_notebook_and_get_votes {
					use super::runtime_types;
					pub type Version = ::core::primitive::u32;
					pub type NotaryId = ::core::primitive::u32;
					pub type NotebookNumber = ::core::primitive::u32;
					pub type HeaderHash = ::subxt::utils::H256;
					pub type VoteMinimums =
						::subxt::utils::KeyedVec<::subxt::utils::H256, ::core::primitive::u128>;
					pub type Bytes = ::std::vec::Vec<::core::primitive::u8>;
					pub type AuditDependencySummaries =
						::std::vec::Vec<runtime_types::ulx_primitives::apis::NotebookAuditSummary>;
					pub mod output {
						use super::runtime_types;
						pub type Output = ::core::result::Result<
							runtime_types::ulx_primitives::apis::NotebookAuditResult,
							runtime_types::ulx_notary_audit::error::VerifyError,
						>;
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct AuditNotebookAndGetVotes {
					pub version: audit_notebook_and_get_votes::Version,
					pub notary_id: audit_notebook_and_get_votes::NotaryId,
					pub notebook_number: audit_notebook_and_get_votes::NotebookNumber,
					pub header_hash: audit_notebook_and_get_votes::HeaderHash,
					pub vote_minimums: audit_notebook_and_get_votes::VoteMinimums,
					pub bytes: audit_notebook_and_get_votes::Bytes,
					pub audit_dependency_summaries:
						audit_notebook_and_get_votes::AuditDependencySummaries,
				}
				pub mod decode_signed_raw_notebook_header {
					use super::runtime_types;
					pub type RawHeader = ::std::vec::Vec<::core::primitive::u8>;
					pub mod output {
						use super::runtime_types;
						pub type Output = ::core::result::Result<
							runtime_types::ulx_primitives::notary::NotaryNotebookVoteDetails<
								::subxt::utils::H256,
							>,
							runtime_types::sp_runtime::DispatchError,
						>;
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct DecodeSignedRawNotebookHeader {
					pub raw_header: decode_signed_raw_notebook_header::RawHeader,
				}
				pub mod latest_notebook_by_notary {
					use super::runtime_types;
					pub mod output {
						use super::runtime_types;
						pub type Output = ::subxt::utils::KeyedVec<
							::core::primitive::u32,
							(::core::primitive::u32, ::core::primitive::u32),
						>;
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct LatestNotebookByNotary {}
			}
		}
		pub mod tick_apis {
			use super::root_mod;
			use super::runtime_types;
			pub struct TickApis;
			impl TickApis {
				pub fn current_tick(
					&self,
				) -> ::subxt::runtime_api::Payload<
					types::CurrentTick,
					types::current_tick::output::Output,
				> {
					::subxt::runtime_api::Payload::new_static(
						"TickApis",
						"current_tick",
						types::CurrentTick {},
						[
							14u8, 164u8, 187u8, 5u8, 165u8, 232u8, 115u8, 62u8, 28u8, 152u8, 59u8,
							125u8, 52u8, 220u8, 63u8, 169u8, 198u8, 88u8, 58u8, 185u8, 7u8, 214u8,
							232u8, 65u8, 163u8, 38u8, 161u8, 233u8, 164u8, 129u8, 67u8, 193u8,
						],
					)
				}
				pub fn ticker(
					&self,
				) -> ::subxt::runtime_api::Payload<types::Ticker, types::ticker::output::Output> {
					::subxt::runtime_api::Payload::new_static(
						"TickApis",
						"ticker",
						types::Ticker {},
						[
							242u8, 50u8, 78u8, 194u8, 192u8, 155u8, 42u8, 156u8, 182u8, 142u8, 8u8,
							147u8, 11u8, 233u8, 105u8, 22u8, 191u8, 183u8, 38u8, 35u8, 161u8, 21u8,
							187u8, 143u8, 253u8, 24u8, 219u8, 219u8, 215u8, 48u8, 217u8, 18u8,
						],
					)
				}
				pub fn blocks_at_tick(
					&self,
					tick: types::blocks_at_tick::Tick,
				) -> ::subxt::runtime_api::Payload<
					types::BlocksAtTick,
					types::blocks_at_tick::output::Output,
				> {
					::subxt::runtime_api::Payload::new_static(
						"TickApis",
						"blocks_at_tick",
						types::BlocksAtTick { tick },
						[
							24u8, 144u8, 142u8, 178u8, 118u8, 93u8, 62u8, 204u8, 18u8, 106u8, 41u8,
							140u8, 137u8, 26u8, 109u8, 47u8, 252u8, 163u8, 76u8, 164u8, 253u8,
							248u8, 114u8, 130u8, 199u8, 246u8, 96u8, 13u8, 96u8, 242u8, 159u8,
							47u8,
						],
					)
				}
			}
			pub mod types {
				use super::runtime_types;
				pub mod current_tick {
					use super::runtime_types;
					pub mod output {
						use super::runtime_types;
						pub type Output = ::core::primitive::u32;
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct CurrentTick {}
				pub mod ticker {
					use super::runtime_types;
					pub mod output {
						use super::runtime_types;
						pub type Output = runtime_types::ulx_primitives::tick::Ticker;
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Ticker {}
				pub mod blocks_at_tick {
					use super::runtime_types;
					pub type Tick = ::core::primitive::u32;
					pub mod output {
						use super::runtime_types;
						pub type Output = ::std::vec::Vec<::subxt::utils::H256>;
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct BlocksAtTick {
					pub tick: blocks_at_tick::Tick,
				}
			}
		}
		pub mod grandpa_api {
			use super::root_mod;
			use super::runtime_types;
			#[doc = " APIs for integrating the GRANDPA finality gadget into runtimes."]
			#[doc = " This should be implemented on the runtime side."]
			#[doc = ""]
			#[doc = " This is primarily used for negotiating authority-set changes for the"]
			#[doc = " gadget. GRANDPA uses a signaling model of changing authority sets:"]
			#[doc = " changes should be signaled with a delay of N blocks, and then automatically"]
			#[doc = " applied in the runtime after those N blocks have passed."]
			#[doc = ""]
			#[doc = " The consensus protocol will coordinate the handoff externally."]
			pub struct GrandpaApi;
			impl GrandpaApi {
				#[doc = " Get the current GRANDPA authorities and weights. This should not change except"]
				#[doc = " for when changes are scheduled and the corresponding delay has passed."]
				#[doc = ""]
				#[doc = " When called at block B, it will return the set of authorities that should be"]
				#[doc = " used to finalize descendants of this block (B+1, B+2, ...). The block B itself"]
				#[doc = " is finalized by the authorities from block B-1."]
				pub fn grandpa_authorities(
					&self,
				) -> ::subxt::runtime_api::Payload<
					types::GrandpaAuthorities,
					types::grandpa_authorities::output::Output,
				> {
					::subxt::runtime_api::Payload::new_static(
						"GrandpaApi",
						"grandpa_authorities",
						types::GrandpaAuthorities {},
						[
							8u8, 1u8, 99u8, 227u8, 52u8, 95u8, 230u8, 139u8, 198u8, 90u8, 159u8,
							146u8, 193u8, 81u8, 37u8, 27u8, 216u8, 227u8, 108u8, 126u8, 12u8, 94u8,
							125u8, 183u8, 143u8, 231u8, 87u8, 101u8, 114u8, 190u8, 193u8, 180u8,
						],
					)
				}
				#[doc = " Submits an unsigned extrinsic to report an equivocation. The caller"]
				#[doc = " must provide the equivocation proof and a key ownership proof"]
				#[doc = " (should be obtained using `generate_key_ownership_proof`). The"]
				#[doc = " extrinsic will be unsigned and should only be accepted for local"]
				#[doc = " authorship (not to be broadcast to the network). This method returns"]
				#[doc = " `None` when creation of the extrinsic fails, e.g. if equivocation"]
				#[doc = " reporting is disabled for the given runtime (i.e. this method is"]
				#[doc = " hardcoded to return `None`). Only useful in an offchain context."]
				pub fn submit_report_equivocation_unsigned_extrinsic(
					&self,
					equivocation_proof : types :: submit_report_equivocation_unsigned_extrinsic :: EquivocationProof,
					key_owner_proof : types :: submit_report_equivocation_unsigned_extrinsic :: KeyOwnerProof,
				) -> ::subxt::runtime_api::Payload<
					types::SubmitReportEquivocationUnsignedExtrinsic,
					types::submit_report_equivocation_unsigned_extrinsic::output::Output,
				> {
					::subxt::runtime_api::Payload::new_static(
						"GrandpaApi",
						"submit_report_equivocation_unsigned_extrinsic",
						types::SubmitReportEquivocationUnsignedExtrinsic {
							equivocation_proof,
							key_owner_proof,
						},
						[
							27u8, 32u8, 16u8, 79u8, 172u8, 124u8, 44u8, 13u8, 176u8, 89u8, 69u8,
							60u8, 45u8, 176u8, 72u8, 151u8, 252u8, 5u8, 243u8, 82u8, 170u8, 51u8,
							179u8, 197u8, 117u8, 177u8, 110u8, 111u8, 97u8, 15u8, 109u8, 169u8,
						],
					)
				}
				#[doc = " Generates a proof of key ownership for the given authority in the"]
				#[doc = " given set. An example usage of this module is coupled with the"]
				#[doc = " session historical module to prove that a given authority key is"]
				#[doc = " tied to a given staking identity during a specific session. Proofs"]
				#[doc = " of key ownership are necessary for submitting equivocation reports."]
				#[doc = " NOTE: even though the API takes a `set_id` as parameter the current"]
				#[doc = " implementations ignore this parameter and instead rely on this"]
				#[doc = " method being called at the correct block height, i.e. any point at"]
				#[doc = " which the given set id is live on-chain. Future implementations will"]
				#[doc = " instead use indexed data through an offchain worker, not requiring"]
				#[doc = " older states to be available."]
				pub fn generate_key_ownership_proof(
					&self,
					set_id: types::generate_key_ownership_proof::SetId,
					authority_id: types::generate_key_ownership_proof::AuthorityId,
				) -> ::subxt::runtime_api::Payload<
					types::GenerateKeyOwnershipProof,
					types::generate_key_ownership_proof::output::Output,
				> {
					::subxt::runtime_api::Payload::new_static(
						"GrandpaApi",
						"generate_key_ownership_proof",
						types::GenerateKeyOwnershipProof { set_id, authority_id },
						[
							13u8, 144u8, 66u8, 235u8, 24u8, 190u8, 39u8, 75u8, 29u8, 157u8, 215u8,
							181u8, 173u8, 145u8, 224u8, 244u8, 189u8, 79u8, 6u8, 116u8, 139u8,
							196u8, 54u8, 16u8, 89u8, 190u8, 121u8, 43u8, 137u8, 150u8, 117u8, 68u8,
						],
					)
				}
				#[doc = " Get current GRANDPA authority set id."]
				pub fn current_set_id(
					&self,
				) -> ::subxt::runtime_api::Payload<
					types::CurrentSetId,
					types::current_set_id::output::Output,
				> {
					::subxt::runtime_api::Payload::new_static(
						"GrandpaApi",
						"current_set_id",
						types::CurrentSetId {},
						[
							42u8, 230u8, 120u8, 211u8, 156u8, 245u8, 109u8, 86u8, 100u8, 146u8,
							234u8, 205u8, 41u8, 183u8, 109u8, 42u8, 17u8, 33u8, 156u8, 25u8, 139u8,
							84u8, 101u8, 75u8, 232u8, 198u8, 87u8, 136u8, 218u8, 233u8, 103u8,
							156u8,
						],
					)
				}
			}
			pub mod types {
				use super::runtime_types;
				pub mod grandpa_authorities {
					use super::runtime_types;
					pub mod output {
						use super::runtime_types;
						pub type Output = ::std::vec::Vec<(
							runtime_types::sp_consensus_grandpa::app::Public,
							::core::primitive::u64,
						)>;
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct GrandpaAuthorities {}
				pub mod submit_report_equivocation_unsigned_extrinsic {
					use super::runtime_types;
					pub type EquivocationProof =
						runtime_types::sp_consensus_grandpa::EquivocationProof<
							::subxt::utils::H256,
							::core::primitive::u32,
						>;
					pub type KeyOwnerProof =
						runtime_types::sp_consensus_grandpa::OpaqueKeyOwnershipProof;
					pub mod output {
						use super::runtime_types;
						pub type Output = ::core::option::Option<()>;
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SubmitReportEquivocationUnsignedExtrinsic {
					pub equivocation_proof:
						submit_report_equivocation_unsigned_extrinsic::EquivocationProof,
					pub key_owner_proof:
						submit_report_equivocation_unsigned_extrinsic::KeyOwnerProof,
				}
				pub mod generate_key_ownership_proof {
					use super::runtime_types;
					pub type SetId = ::core::primitive::u64;
					pub type AuthorityId = runtime_types::sp_consensus_grandpa::app::Public;
					pub mod output {
						use super::runtime_types;
						pub type Output = ::core::option::Option<
							runtime_types::sp_consensus_grandpa::OpaqueKeyOwnershipProof,
						>;
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct GenerateKeyOwnershipProof {
					pub set_id: generate_key_ownership_proof::SetId,
					pub authority_id: generate_key_ownership_proof::AuthorityId,
				}
				pub mod current_set_id {
					use super::runtime_types;
					pub mod output {
						use super::runtime_types;
						pub type Output = ::core::primitive::u64;
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct CurrentSetId {}
			}
		}
		pub mod genesis_builder {
			use super::root_mod;
			use super::runtime_types;
			#[doc = " API to interact with RuntimeGenesisConfig for the runtime"]
			pub struct GenesisBuilder;
			impl GenesisBuilder {
				#[doc = " Creates the default `RuntimeGenesisConfig` and returns it as a JSON blob."]
				#[doc = ""]
				#[doc = " This function instantiates the default `RuntimeGenesisConfig` struct for the runtime and serializes it into a JSON"]
				#[doc = " blob. It returns a `Vec<u8>` containing the JSON representation of the default `RuntimeGenesisConfig`."]
				pub fn create_default_config(
					&self,
				) -> ::subxt::runtime_api::Payload<
					types::CreateDefaultConfig,
					types::create_default_config::output::Output,
				> {
					::subxt::runtime_api::Payload::new_static(
						"GenesisBuilder",
						"create_default_config",
						types::CreateDefaultConfig {},
						[
							238u8, 5u8, 139u8, 81u8, 184u8, 155u8, 221u8, 118u8, 190u8, 76u8,
							229u8, 67u8, 132u8, 89u8, 83u8, 80u8, 56u8, 171u8, 169u8, 64u8, 123u8,
							20u8, 129u8, 159u8, 28u8, 135u8, 84u8, 52u8, 192u8, 98u8, 104u8, 214u8,
						],
					)
				}
				#[doc = " Build `RuntimeGenesisConfig` from a JSON blob not using any defaults and store it in the storage."]
				#[doc = ""]
				#[doc = " This function deserializes the full `RuntimeGenesisConfig` from the given JSON blob and puts it into the storage."]
				#[doc = " If the provided JSON blob is incorrect or incomplete or the deserialization fails, an error is returned."]
				#[doc = " It is recommended to log any errors encountered during the process."]
				#[doc = ""]
				#[doc = " Please note that provided json blob must contain all `RuntimeGenesisConfig` fields, no defaults will be used."]
				pub fn build_config(
					&self,
					json: types::build_config::Json,
				) -> ::subxt::runtime_api::Payload<
					types::BuildConfig,
					types::build_config::output::Output,
				> {
					::subxt::runtime_api::Payload::new_static(
						"GenesisBuilder",
						"build_config",
						types::BuildConfig { json },
						[
							6u8, 98u8, 68u8, 125u8, 157u8, 26u8, 107u8, 86u8, 213u8, 227u8, 26u8,
							229u8, 122u8, 161u8, 229u8, 114u8, 123u8, 192u8, 66u8, 231u8, 148u8,
							175u8, 5u8, 185u8, 248u8, 88u8, 40u8, 122u8, 230u8, 209u8, 170u8,
							254u8,
						],
					)
				}
			}
			pub mod types {
				use super::runtime_types;
				pub mod create_default_config {
					use super::runtime_types;
					pub mod output {
						use super::runtime_types;
						pub type Output = ::std::vec::Vec<::core::primitive::u8>;
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct CreateDefaultConfig {}
				pub mod build_config {
					use super::runtime_types;
					pub type Json = ::std::vec::Vec<::core::primitive::u8>;
					pub mod output {
						use super::runtime_types;
						pub type Output = ::core::result::Result<(), ::std::string::String>;
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct BuildConfig {
					pub json: build_config::Json,
				}
			}
		}
	}
	pub fn custom() -> CustomValuesApi {
		CustomValuesApi
	}
	pub struct CustomValuesApi;
	impl CustomValuesApi {}
	pub struct ConstantsApi;
	impl ConstantsApi {
		pub fn system(&self) -> system::constants::ConstantsApi {
			system::constants::ConstantsApi
		}
		pub fn timestamp(&self) -> timestamp::constants::ConstantsApi {
			timestamp::constants::ConstantsApi
		}
		pub fn mining_slot(&self) -> mining_slot::constants::ConstantsApi {
			mining_slot::constants::ConstantsApi
		}
		pub fn bond(&self) -> bond::constants::ConstantsApi {
			bond::constants::ConstantsApi
		}
		pub fn notaries(&self) -> notaries::constants::ConstantsApi {
			notaries::constants::ConstantsApi
		}
		pub fn chain_transfer(&self) -> chain_transfer::constants::ConstantsApi {
			chain_transfer::constants::ConstantsApi
		}
		pub fn block_seal_spec(&self) -> block_seal_spec::constants::ConstantsApi {
			block_seal_spec::constants::ConstantsApi
		}
		pub fn block_rewards(&self) -> block_rewards::constants::ConstantsApi {
			block_rewards::constants::ConstantsApi
		}
		pub fn grandpa(&self) -> grandpa::constants::ConstantsApi {
			grandpa::constants::ConstantsApi
		}
		pub fn argon_balances(&self) -> argon_balances::constants::ConstantsApi {
			argon_balances::constants::ConstantsApi
		}
		pub fn ulixee_balances(&self) -> ulixee_balances::constants::ConstantsApi {
			ulixee_balances::constants::ConstantsApi
		}
		pub fn tx_pause(&self) -> tx_pause::constants::ConstantsApi {
			tx_pause::constants::ConstantsApi
		}
		pub fn transaction_payment(&self) -> transaction_payment::constants::ConstantsApi {
			transaction_payment::constants::ConstantsApi
		}
	}
	pub struct StorageApi;
	impl StorageApi {
		pub fn system(&self) -> system::storage::StorageApi {
			system::storage::StorageApi
		}
		pub fn timestamp(&self) -> timestamp::storage::StorageApi {
			timestamp::storage::StorageApi
		}
		pub fn ticks(&self) -> ticks::storage::StorageApi {
			ticks::storage::StorageApi
		}
		pub fn mining_slot(&self) -> mining_slot::storage::StorageApi {
			mining_slot::storage::StorageApi
		}
		pub fn bond(&self) -> bond::storage::StorageApi {
			bond::storage::StorageApi
		}
		pub fn notaries(&self) -> notaries::storage::StorageApi {
			notaries::storage::StorageApi
		}
		pub fn notebook(&self) -> notebook::storage::StorageApi {
			notebook::storage::StorageApi
		}
		pub fn chain_transfer(&self) -> chain_transfer::storage::StorageApi {
			chain_transfer::storage::StorageApi
		}
		pub fn block_seal_spec(&self) -> block_seal_spec::storage::StorageApi {
			block_seal_spec::storage::StorageApi
		}
		pub fn data_domain(&self) -> data_domain::storage::StorageApi {
			data_domain::storage::StorageApi
		}
		pub fn authorship(&self) -> authorship::storage::StorageApi {
			authorship::storage::StorageApi
		}
		pub fn historical(&self) -> historical::storage::StorageApi {
			historical::storage::StorageApi
		}
		pub fn session(&self) -> session::storage::StorageApi {
			session::storage::StorageApi
		}
		pub fn block_seal(&self) -> block_seal::storage::StorageApi {
			block_seal::storage::StorageApi
		}
		pub fn block_rewards(&self) -> block_rewards::storage::StorageApi {
			block_rewards::storage::StorageApi
		}
		pub fn grandpa(&self) -> grandpa::storage::StorageApi {
			grandpa::storage::StorageApi
		}
		pub fn offences(&self) -> offences::storage::StorageApi {
			offences::storage::StorageApi
		}
		pub fn argon_balances(&self) -> argon_balances::storage::StorageApi {
			argon_balances::storage::StorageApi
		}
		pub fn mint(&self) -> mint::storage::StorageApi {
			mint::storage::StorageApi
		}
		pub fn ulixee_balances(&self) -> ulixee_balances::storage::StorageApi {
			ulixee_balances::storage::StorageApi
		}
		pub fn tx_pause(&self) -> tx_pause::storage::StorageApi {
			tx_pause::storage::StorageApi
		}
		pub fn transaction_payment(&self) -> transaction_payment::storage::StorageApi {
			transaction_payment::storage::StorageApi
		}
		pub fn sudo(&self) -> sudo::storage::StorageApi {
			sudo::storage::StorageApi
		}
	}
	pub struct TransactionApi;
	impl TransactionApi {
		pub fn system(&self) -> system::calls::TransactionApi {
			system::calls::TransactionApi
		}
		pub fn timestamp(&self) -> timestamp::calls::TransactionApi {
			timestamp::calls::TransactionApi
		}
		pub fn ticks(&self) -> ticks::calls::TransactionApi {
			ticks::calls::TransactionApi
		}
		pub fn mining_slot(&self) -> mining_slot::calls::TransactionApi {
			mining_slot::calls::TransactionApi
		}
		pub fn bond(&self) -> bond::calls::TransactionApi {
			bond::calls::TransactionApi
		}
		pub fn notaries(&self) -> notaries::calls::TransactionApi {
			notaries::calls::TransactionApi
		}
		pub fn notebook(&self) -> notebook::calls::TransactionApi {
			notebook::calls::TransactionApi
		}
		pub fn chain_transfer(&self) -> chain_transfer::calls::TransactionApi {
			chain_transfer::calls::TransactionApi
		}
		pub fn block_seal_spec(&self) -> block_seal_spec::calls::TransactionApi {
			block_seal_spec::calls::TransactionApi
		}
		pub fn data_domain(&self) -> data_domain::calls::TransactionApi {
			data_domain::calls::TransactionApi
		}
		pub fn session(&self) -> session::calls::TransactionApi {
			session::calls::TransactionApi
		}
		pub fn block_seal(&self) -> block_seal::calls::TransactionApi {
			block_seal::calls::TransactionApi
		}
		pub fn block_rewards(&self) -> block_rewards::calls::TransactionApi {
			block_rewards::calls::TransactionApi
		}
		pub fn grandpa(&self) -> grandpa::calls::TransactionApi {
			grandpa::calls::TransactionApi
		}
		pub fn argon_balances(&self) -> argon_balances::calls::TransactionApi {
			argon_balances::calls::TransactionApi
		}
		pub fn mint(&self) -> mint::calls::TransactionApi {
			mint::calls::TransactionApi
		}
		pub fn ulixee_balances(&self) -> ulixee_balances::calls::TransactionApi {
			ulixee_balances::calls::TransactionApi
		}
		pub fn tx_pause(&self) -> tx_pause::calls::TransactionApi {
			tx_pause::calls::TransactionApi
		}
		pub fn sudo(&self) -> sudo::calls::TransactionApi {
			sudo::calls::TransactionApi
		}
	}
	#[doc = r" check whether the metadata provided is aligned with this statically generated code."]
	pub fn is_codegen_valid_for(metadata: &::subxt::Metadata) -> bool {
		let runtime_metadata_hash = metadata
			.hasher()
			.only_these_pallets(&PALLETS)
			.only_these_runtime_apis(&RUNTIME_APIS)
			.hash();
		runtime_metadata_hash
			== [
				221u8, 163u8, 174u8, 173u8, 245u8, 117u8, 70u8, 2u8, 226u8, 16u8, 56u8, 215u8,
				39u8, 197u8, 116u8, 96u8, 119u8, 153u8, 11u8, 80u8, 15u8, 131u8, 188u8, 149u8,
				187u8, 223u8, 251u8, 141u8, 35u8, 123u8, 0u8, 98u8,
			]
	}
	pub mod system {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "Error for the System pallet"]
		pub type Error = runtime_types::frame_system::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::frame_system::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Make some on-chain remark."]
				#[doc = ""]
				#[doc = "Can be executed by every `origin`."]
				pub struct Remark {
					pub remark: remark::Remark,
				}
				pub mod remark {
					use super::runtime_types;
					pub type Remark = ::std::vec::Vec<::core::primitive::u8>;
				}
				impl ::subxt::blocks::StaticExtrinsic for Remark {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "remark";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Set the number of pages in the WebAssembly environment's heap."]
				pub struct SetHeapPages {
					pub pages: set_heap_pages::Pages,
				}
				pub mod set_heap_pages {
					use super::runtime_types;
					pub type Pages = ::core::primitive::u64;
				}
				impl ::subxt::blocks::StaticExtrinsic for SetHeapPages {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "set_heap_pages";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Set the new runtime code."]
				pub struct SetCode {
					pub code: set_code::Code,
				}
				pub mod set_code {
					use super::runtime_types;
					pub type Code = ::std::vec::Vec<::core::primitive::u8>;
				}
				impl ::subxt::blocks::StaticExtrinsic for SetCode {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "set_code";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Set the new runtime code without doing any checks of the given `code`."]
				#[doc = ""]
				#[doc = "Note that runtime upgrades will not run if this is called with a not-increasing spec"]
				#[doc = "version!"]
				pub struct SetCodeWithoutChecks {
					pub code: set_code_without_checks::Code,
				}
				pub mod set_code_without_checks {
					use super::runtime_types;
					pub type Code = ::std::vec::Vec<::core::primitive::u8>;
				}
				impl ::subxt::blocks::StaticExtrinsic for SetCodeWithoutChecks {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "set_code_without_checks";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Set some items of storage."]
				pub struct SetStorage {
					pub items: set_storage::Items,
				}
				pub mod set_storage {
					use super::runtime_types;
					pub type Items = ::std::vec::Vec<(
						::std::vec::Vec<::core::primitive::u8>,
						::std::vec::Vec<::core::primitive::u8>,
					)>;
				}
				impl ::subxt::blocks::StaticExtrinsic for SetStorage {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "set_storage";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Kill some items from storage."]
				pub struct KillStorage {
					pub keys: kill_storage::Keys,
				}
				pub mod kill_storage {
					use super::runtime_types;
					pub type Keys = ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>;
				}
				impl ::subxt::blocks::StaticExtrinsic for KillStorage {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "kill_storage";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Kill all storage items with a key that starts with the given prefix."]
				#[doc = ""]
				#[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
				#[doc = "the prefix we are removing to accurately calculate the weight of this function."]
				pub struct KillPrefix {
					pub prefix: kill_prefix::Prefix,
					pub subkeys: kill_prefix::Subkeys,
				}
				pub mod kill_prefix {
					use super::runtime_types;
					pub type Prefix = ::std::vec::Vec<::core::primitive::u8>;
					pub type Subkeys = ::core::primitive::u32;
				}
				impl ::subxt::blocks::StaticExtrinsic for KillPrefix {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "kill_prefix";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Make some on-chain remark and emit event."]
				pub struct RemarkWithEvent {
					pub remark: remark_with_event::Remark,
				}
				pub mod remark_with_event {
					use super::runtime_types;
					pub type Remark = ::std::vec::Vec<::core::primitive::u8>;
				}
				impl ::subxt::blocks::StaticExtrinsic for RemarkWithEvent {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "remark_with_event";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied"]
				#[doc = "later."]
				#[doc = ""]
				#[doc = "This call requires Root origin."]
				pub struct AuthorizeUpgrade {
					pub code_hash: authorize_upgrade::CodeHash,
				}
				pub mod authorize_upgrade {
					use super::runtime_types;
					pub type CodeHash = ::subxt::utils::H256;
				}
				impl ::subxt::blocks::StaticExtrinsic for AuthorizeUpgrade {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "authorize_upgrade";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied"]
				#[doc = "later."]
				#[doc = ""]
				#[doc = "WARNING: This authorizes an upgrade that will take place without any safety checks, for"]
				#[doc = "example that the spec name remains the same and that the version number increases. Not"]
				#[doc = "recommended for normal use. Use `authorize_upgrade` instead."]
				#[doc = ""]
				#[doc = "This call requires Root origin."]
				pub struct AuthorizeUpgradeWithoutChecks {
					pub code_hash: authorize_upgrade_without_checks::CodeHash,
				}
				pub mod authorize_upgrade_without_checks {
					use super::runtime_types;
					pub type CodeHash = ::subxt::utils::H256;
				}
				impl ::subxt::blocks::StaticExtrinsic for AuthorizeUpgradeWithoutChecks {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "authorize_upgrade_without_checks";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Provide the preimage (runtime binary) `code` for an upgrade that has been authorized."]
				#[doc = ""]
				#[doc = "If the authorization required a version check, this call will ensure the spec name"]
				#[doc = "remains unchanged and that the spec version has increased."]
				#[doc = ""]
				#[doc = "Depending on the runtime's `OnSetCode` configuration, this function may directly apply"]
				#[doc = "the new `code` in the same block or attempt to schedule the upgrade."]
				#[doc = ""]
				#[doc = "All origins are allowed."]
				pub struct ApplyAuthorizedUpgrade {
					pub code: apply_authorized_upgrade::Code,
				}
				pub mod apply_authorized_upgrade {
					use super::runtime_types;
					pub type Code = ::std::vec::Vec<::core::primitive::u8>;
				}
				impl ::subxt::blocks::StaticExtrinsic for ApplyAuthorizedUpgrade {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "apply_authorized_upgrade";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Make some on-chain remark."]
				#[doc = ""]
				#[doc = "Can be executed by every `origin`."]
				pub fn remark(
					&self,
					remark: types::remark::Remark,
				) -> ::subxt::tx::Payload<types::Remark> {
					::subxt::tx::Payload::new_static(
						"System",
						"remark",
						types::Remark { remark },
						[
							43u8, 126u8, 180u8, 174u8, 141u8, 48u8, 52u8, 125u8, 166u8, 212u8,
							216u8, 98u8, 100u8, 24u8, 132u8, 71u8, 101u8, 64u8, 246u8, 169u8, 33u8,
							250u8, 147u8, 208u8, 2u8, 40u8, 129u8, 209u8, 232u8, 207u8, 207u8,
							13u8,
						],
					)
				}
				#[doc = "Set the number of pages in the WebAssembly environment's heap."]
				pub fn set_heap_pages(
					&self,
					pages: types::set_heap_pages::Pages,
				) -> ::subxt::tx::Payload<types::SetHeapPages> {
					::subxt::tx::Payload::new_static(
						"System",
						"set_heap_pages",
						types::SetHeapPages { pages },
						[
							188u8, 191u8, 99u8, 216u8, 219u8, 109u8, 141u8, 50u8, 78u8, 235u8,
							215u8, 242u8, 195u8, 24u8, 111u8, 76u8, 229u8, 64u8, 99u8, 225u8,
							134u8, 121u8, 81u8, 209u8, 127u8, 223u8, 98u8, 215u8, 150u8, 70u8,
							57u8, 147u8,
						],
					)
				}
				#[doc = "Set the new runtime code."]
				pub fn set_code(
					&self,
					code: types::set_code::Code,
				) -> ::subxt::tx::Payload<types::SetCode> {
					::subxt::tx::Payload::new_static(
						"System",
						"set_code",
						types::SetCode { code },
						[
							233u8, 248u8, 88u8, 245u8, 28u8, 65u8, 25u8, 169u8, 35u8, 237u8, 19u8,
							203u8, 136u8, 160u8, 18u8, 3u8, 20u8, 197u8, 81u8, 169u8, 244u8, 188u8,
							27u8, 147u8, 147u8, 236u8, 65u8, 25u8, 3u8, 143u8, 182u8, 22u8,
						],
					)
				}
				#[doc = "Set the new runtime code without doing any checks of the given `code`."]
				#[doc = ""]
				#[doc = "Note that runtime upgrades will not run if this is called with a not-increasing spec"]
				#[doc = "version!"]
				pub fn set_code_without_checks(
					&self,
					code: types::set_code_without_checks::Code,
				) -> ::subxt::tx::Payload<types::SetCodeWithoutChecks> {
					::subxt::tx::Payload::new_static(
						"System",
						"set_code_without_checks",
						types::SetCodeWithoutChecks { code },
						[
							82u8, 212u8, 157u8, 44u8, 70u8, 0u8, 143u8, 15u8, 109u8, 109u8, 107u8,
							157u8, 141u8, 42u8, 169u8, 11u8, 15u8, 186u8, 252u8, 138u8, 10u8,
							147u8, 15u8, 178u8, 247u8, 229u8, 213u8, 98u8, 207u8, 231u8, 119u8,
							115u8,
						],
					)
				}
				#[doc = "Set some items of storage."]
				pub fn set_storage(
					&self,
					items: types::set_storage::Items,
				) -> ::subxt::tx::Payload<types::SetStorage> {
					::subxt::tx::Payload::new_static(
						"System",
						"set_storage",
						types::SetStorage { items },
						[
							141u8, 216u8, 52u8, 222u8, 223u8, 136u8, 123u8, 181u8, 19u8, 75u8,
							163u8, 102u8, 229u8, 189u8, 158u8, 142u8, 95u8, 235u8, 240u8, 49u8,
							150u8, 76u8, 78u8, 137u8, 126u8, 88u8, 183u8, 88u8, 231u8, 146u8,
							234u8, 43u8,
						],
					)
				}
				#[doc = "Kill some items from storage."]
				pub fn kill_storage(
					&self,
					keys: types::kill_storage::Keys,
				) -> ::subxt::tx::Payload<types::KillStorage> {
					::subxt::tx::Payload::new_static(
						"System",
						"kill_storage",
						types::KillStorage { keys },
						[
							73u8, 63u8, 196u8, 36u8, 144u8, 114u8, 34u8, 213u8, 108u8, 93u8, 209u8,
							234u8, 153u8, 185u8, 33u8, 91u8, 187u8, 195u8, 223u8, 130u8, 58u8,
							156u8, 63u8, 47u8, 228u8, 249u8, 216u8, 139u8, 143u8, 177u8, 41u8,
							35u8,
						],
					)
				}
				#[doc = "Kill all storage items with a key that starts with the given prefix."]
				#[doc = ""]
				#[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
				#[doc = "the prefix we are removing to accurately calculate the weight of this function."]
				pub fn kill_prefix(
					&self,
					prefix: types::kill_prefix::Prefix,
					subkeys: types::kill_prefix::Subkeys,
				) -> ::subxt::tx::Payload<types::KillPrefix> {
					::subxt::tx::Payload::new_static(
						"System",
						"kill_prefix",
						types::KillPrefix { prefix, subkeys },
						[
							184u8, 57u8, 139u8, 24u8, 208u8, 87u8, 108u8, 215u8, 198u8, 189u8,
							175u8, 242u8, 167u8, 215u8, 97u8, 63u8, 110u8, 166u8, 238u8, 98u8,
							67u8, 236u8, 111u8, 110u8, 234u8, 81u8, 102u8, 5u8, 182u8, 5u8, 214u8,
							85u8,
						],
					)
				}
				#[doc = "Make some on-chain remark and emit event."]
				pub fn remark_with_event(
					&self,
					remark: types::remark_with_event::Remark,
				) -> ::subxt::tx::Payload<types::RemarkWithEvent> {
					::subxt::tx::Payload::new_static(
						"System",
						"remark_with_event",
						types::RemarkWithEvent { remark },
						[
							120u8, 120u8, 153u8, 92u8, 184u8, 85u8, 34u8, 2u8, 174u8, 206u8, 105u8,
							228u8, 233u8, 130u8, 80u8, 246u8, 228u8, 59u8, 234u8, 240u8, 4u8, 49u8,
							147u8, 170u8, 115u8, 91u8, 149u8, 200u8, 228u8, 181u8, 8u8, 154u8,
						],
					)
				}
				#[doc = "Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied"]
				#[doc = "later."]
				#[doc = ""]
				#[doc = "This call requires Root origin."]
				pub fn authorize_upgrade(
					&self,
					code_hash: types::authorize_upgrade::CodeHash,
				) -> ::subxt::tx::Payload<types::AuthorizeUpgrade> {
					::subxt::tx::Payload::new_static(
						"System",
						"authorize_upgrade",
						types::AuthorizeUpgrade { code_hash },
						[
							4u8, 14u8, 76u8, 107u8, 209u8, 129u8, 9u8, 39u8, 193u8, 17u8, 84u8,
							254u8, 170u8, 214u8, 24u8, 155u8, 29u8, 184u8, 249u8, 241u8, 109u8,
							58u8, 145u8, 131u8, 109u8, 63u8, 38u8, 165u8, 107u8, 215u8, 217u8,
							172u8,
						],
					)
				}
				#[doc = "Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied"]
				#[doc = "later."]
				#[doc = ""]
				#[doc = "WARNING: This authorizes an upgrade that will take place without any safety checks, for"]
				#[doc = "example that the spec name remains the same and that the version number increases. Not"]
				#[doc = "recommended for normal use. Use `authorize_upgrade` instead."]
				#[doc = ""]
				#[doc = "This call requires Root origin."]
				pub fn authorize_upgrade_without_checks(
					&self,
					code_hash: types::authorize_upgrade_without_checks::CodeHash,
				) -> ::subxt::tx::Payload<types::AuthorizeUpgradeWithoutChecks> {
					::subxt::tx::Payload::new_static(
						"System",
						"authorize_upgrade_without_checks",
						types::AuthorizeUpgradeWithoutChecks { code_hash },
						[
							126u8, 126u8, 55u8, 26u8, 47u8, 55u8, 66u8, 8u8, 167u8, 18u8, 29u8,
							136u8, 146u8, 14u8, 189u8, 117u8, 16u8, 227u8, 162u8, 61u8, 149u8,
							197u8, 104u8, 184u8, 185u8, 161u8, 99u8, 154u8, 80u8, 125u8, 181u8,
							233u8,
						],
					)
				}
				#[doc = "Provide the preimage (runtime binary) `code` for an upgrade that has been authorized."]
				#[doc = ""]
				#[doc = "If the authorization required a version check, this call will ensure the spec name"]
				#[doc = "remains unchanged and that the spec version has increased."]
				#[doc = ""]
				#[doc = "Depending on the runtime's `OnSetCode` configuration, this function may directly apply"]
				#[doc = "the new `code` in the same block or attempt to schedule the upgrade."]
				#[doc = ""]
				#[doc = "All origins are allowed."]
				pub fn apply_authorized_upgrade(
					&self,
					code: types::apply_authorized_upgrade::Code,
				) -> ::subxt::tx::Payload<types::ApplyAuthorizedUpgrade> {
					::subxt::tx::Payload::new_static(
						"System",
						"apply_authorized_upgrade",
						types::ApplyAuthorizedUpgrade { code },
						[
							232u8, 107u8, 127u8, 38u8, 230u8, 29u8, 97u8, 4u8, 160u8, 191u8, 222u8,
							156u8, 245u8, 102u8, 196u8, 141u8, 44u8, 163u8, 98u8, 68u8, 125u8,
							32u8, 124u8, 101u8, 108u8, 93u8, 211u8, 52u8, 0u8, 231u8, 33u8, 227u8,
						],
					)
				}
			}
		}
		#[doc = "Event for the System pallet."]
		pub type Event = runtime_types::frame_system::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "An extrinsic completed successfully."]
			pub struct ExtrinsicSuccess {
				pub dispatch_info: extrinsic_success::DispatchInfo,
			}
			pub mod extrinsic_success {
				use super::runtime_types;
				pub type DispatchInfo = runtime_types::frame_support::dispatch::DispatchInfo;
			}
			impl ::subxt::events::StaticEvent for ExtrinsicSuccess {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "ExtrinsicSuccess";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "An extrinsic failed."]
			pub struct ExtrinsicFailed {
				pub dispatch_error: extrinsic_failed::DispatchError,
				pub dispatch_info: extrinsic_failed::DispatchInfo,
			}
			pub mod extrinsic_failed {
				use super::runtime_types;
				pub type DispatchError = runtime_types::sp_runtime::DispatchError;
				pub type DispatchInfo = runtime_types::frame_support::dispatch::DispatchInfo;
			}
			impl ::subxt::events::StaticEvent for ExtrinsicFailed {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "ExtrinsicFailed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "`:code` was updated."]
			pub struct CodeUpdated;
			impl ::subxt::events::StaticEvent for CodeUpdated {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "CodeUpdated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A new account was created."]
			pub struct NewAccount {
				pub account: new_account::Account,
			}
			pub mod new_account {
				use super::runtime_types;
				pub type Account = ::subxt::utils::AccountId32;
			}
			impl ::subxt::events::StaticEvent for NewAccount {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "NewAccount";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "An account was reaped."]
			pub struct KilledAccount {
				pub account: killed_account::Account,
			}
			pub mod killed_account {
				use super::runtime_types;
				pub type Account = ::subxt::utils::AccountId32;
			}
			impl ::subxt::events::StaticEvent for KilledAccount {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "KilledAccount";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "On on-chain remark happened."]
			pub struct Remarked {
				pub sender: remarked::Sender,
				pub hash: remarked::Hash,
			}
			pub mod remarked {
				use super::runtime_types;
				pub type Sender = ::subxt::utils::AccountId32;
				pub type Hash = ::subxt::utils::H256;
			}
			impl ::subxt::events::StaticEvent for Remarked {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "Remarked";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "An upgrade was authorized."]
			pub struct UpgradeAuthorized {
				pub code_hash: upgrade_authorized::CodeHash,
				pub check_version: upgrade_authorized::CheckVersion,
			}
			pub mod upgrade_authorized {
				use super::runtime_types;
				pub type CodeHash = ::subxt::utils::H256;
				pub type CheckVersion = ::core::primitive::bool;
			}
			impl ::subxt::events::StaticEvent for UpgradeAuthorized {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "UpgradeAuthorized";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub mod types {
				use super::runtime_types;
				pub mod account {
					use super::runtime_types;
					pub type Account = runtime_types::frame_system::AccountInfo<
						::core::primitive::u32,
						runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
					>;
					pub type Param0 = ::subxt::utils::AccountId32;
				}
				pub mod extrinsic_count {
					use super::runtime_types;
					pub type ExtrinsicCount = ::core::primitive::u32;
				}
				pub mod inherents_applied {
					use super::runtime_types;
					pub type InherentsApplied = ::core::primitive::bool;
				}
				pub mod block_weight {
					use super::runtime_types;
					pub type BlockWeight = runtime_types::frame_support::dispatch::PerDispatchClass<
						runtime_types::sp_weights::weight_v2::Weight,
					>;
				}
				pub mod all_extrinsics_len {
					use super::runtime_types;
					pub type AllExtrinsicsLen = ::core::primitive::u32;
				}
				pub mod block_hash {
					use super::runtime_types;
					pub type BlockHash = ::subxt::utils::H256;
					pub type Param0 = ::core::primitive::u32;
				}
				pub mod extrinsic_data {
					use super::runtime_types;
					pub type ExtrinsicData = ::std::vec::Vec<::core::primitive::u8>;
					pub type Param0 = ::core::primitive::u32;
				}
				pub mod number {
					use super::runtime_types;
					pub type Number = ::core::primitive::u32;
				}
				pub mod parent_hash {
					use super::runtime_types;
					pub type ParentHash = ::subxt::utils::H256;
				}
				pub mod digest {
					use super::runtime_types;
					pub type Digest = runtime_types::sp_runtime::generic::digest::Digest;
				}
				pub mod events {
					use super::runtime_types;
					pub type Events = ::std::vec::Vec<
						runtime_types::frame_system::EventRecord<
							runtime_types::ulx_node_runtime::RuntimeEvent,
							::subxt::utils::H256,
						>,
					>;
				}
				pub mod event_count {
					use super::runtime_types;
					pub type EventCount = ::core::primitive::u32;
				}
				pub mod event_topics {
					use super::runtime_types;
					pub type EventTopics =
						::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>;
					pub type Param0 = ::subxt::utils::H256;
				}
				pub mod last_runtime_upgrade {
					use super::runtime_types;
					pub type LastRuntimeUpgrade =
						runtime_types::frame_system::LastRuntimeUpgradeInfo;
				}
				pub mod upgraded_to_u32_ref_count {
					use super::runtime_types;
					pub type UpgradedToU32RefCount = ::core::primitive::bool;
				}
				pub mod upgraded_to_triple_ref_count {
					use super::runtime_types;
					pub type UpgradedToTripleRefCount = ::core::primitive::bool;
				}
				pub mod execution_phase {
					use super::runtime_types;
					pub type ExecutionPhase = runtime_types::frame_system::Phase;
				}
				pub mod authorized_upgrade {
					use super::runtime_types;
					pub type AuthorizedUpgrade =
						runtime_types::frame_system::CodeUpgradeAuthorization;
				}
			}
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The full account information for a particular account ID."]
				pub fn account_iter(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::account::Account,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"Account",
						(),
						[
							14u8, 233u8, 115u8, 214u8, 0u8, 109u8, 222u8, 121u8, 162u8, 65u8, 60u8,
							175u8, 209u8, 79u8, 222u8, 124u8, 22u8, 235u8, 138u8, 176u8, 133u8,
							124u8, 90u8, 158u8, 85u8, 45u8, 37u8, 174u8, 47u8, 79u8, 47u8, 166u8,
						],
					)
				}
				#[doc = " The full account information for a particular account ID."]
				pub fn account(
					&self,
					_0: impl ::std::borrow::Borrow<types::account::Param0>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageKey<types::account::Param0>,
					types::account::Account,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"Account",
						::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							14u8, 233u8, 115u8, 214u8, 0u8, 109u8, 222u8, 121u8, 162u8, 65u8, 60u8,
							175u8, 209u8, 79u8, 222u8, 124u8, 22u8, 235u8, 138u8, 176u8, 133u8,
							124u8, 90u8, 158u8, 85u8, 45u8, 37u8, 174u8, 47u8, 79u8, 47u8, 166u8,
						],
					)
				}
				#[doc = " Total extrinsics count for the current block."]
				pub fn extrinsic_count(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::extrinsic_count::ExtrinsicCount,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"ExtrinsicCount",
						(),
						[
							102u8, 76u8, 236u8, 42u8, 40u8, 231u8, 33u8, 222u8, 123u8, 147u8,
							153u8, 148u8, 234u8, 203u8, 181u8, 119u8, 6u8, 187u8, 177u8, 199u8,
							120u8, 47u8, 137u8, 254u8, 96u8, 100u8, 165u8, 182u8, 249u8, 230u8,
							159u8, 79u8,
						],
					)
				}
				#[doc = " Whether all inherents have been applied."]
				pub fn inherents_applied(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::inherents_applied::InherentsApplied,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"InherentsApplied",
						(),
						[
							132u8, 249u8, 142u8, 252u8, 8u8, 103u8, 80u8, 120u8, 50u8, 6u8, 188u8,
							223u8, 101u8, 55u8, 165u8, 189u8, 172u8, 249u8, 165u8, 230u8, 183u8,
							109u8, 34u8, 65u8, 185u8, 150u8, 29u8, 8u8, 186u8, 129u8, 135u8, 239u8,
						],
					)
				}
				#[doc = " The current weight for the block."]
				pub fn block_weight(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::block_weight::BlockWeight,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"BlockWeight",
						(),
						[
							158u8, 46u8, 228u8, 89u8, 210u8, 214u8, 84u8, 154u8, 50u8, 68u8, 63u8,
							62u8, 43u8, 42u8, 99u8, 27u8, 54u8, 42u8, 146u8, 44u8, 241u8, 216u8,
							229u8, 30u8, 216u8, 255u8, 165u8, 238u8, 181u8, 130u8, 36u8, 102u8,
						],
					)
				}
				#[doc = " Total length (in bytes) for all extrinsics put together, for the current block."]
				pub fn all_extrinsics_len(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::all_extrinsics_len::AllExtrinsicsLen,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"AllExtrinsicsLen",
						(),
						[
							117u8, 86u8, 61u8, 243u8, 41u8, 51u8, 102u8, 214u8, 137u8, 100u8,
							243u8, 185u8, 122u8, 174u8, 187u8, 117u8, 86u8, 189u8, 63u8, 135u8,
							101u8, 218u8, 203u8, 201u8, 237u8, 254u8, 128u8, 183u8, 169u8, 221u8,
							242u8, 65u8,
						],
					)
				}
				#[doc = " Map of block numbers to block hashes."]
				pub fn block_hash_iter(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::block_hash::BlockHash,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"BlockHash",
						(),
						[
							217u8, 32u8, 215u8, 253u8, 24u8, 182u8, 207u8, 178u8, 157u8, 24u8,
							103u8, 100u8, 195u8, 165u8, 69u8, 152u8, 112u8, 181u8, 56u8, 192u8,
							164u8, 16u8, 20u8, 222u8, 28u8, 214u8, 144u8, 142u8, 146u8, 69u8,
							202u8, 118u8,
						],
					)
				}
				#[doc = " Map of block numbers to block hashes."]
				pub fn block_hash(
					&self,
					_0: impl ::std::borrow::Borrow<types::block_hash::Param0>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageKey<types::block_hash::Param0>,
					types::block_hash::BlockHash,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"BlockHash",
						::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							217u8, 32u8, 215u8, 253u8, 24u8, 182u8, 207u8, 178u8, 157u8, 24u8,
							103u8, 100u8, 195u8, 165u8, 69u8, 152u8, 112u8, 181u8, 56u8, 192u8,
							164u8, 16u8, 20u8, 222u8, 28u8, 214u8, 144u8, 142u8, 146u8, 69u8,
							202u8, 118u8,
						],
					)
				}
				#[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
				pub fn extrinsic_data_iter(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::extrinsic_data::ExtrinsicData,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"ExtrinsicData",
						(),
						[
							160u8, 180u8, 122u8, 18u8, 196u8, 26u8, 2u8, 37u8, 115u8, 232u8, 133u8,
							220u8, 106u8, 245u8, 4u8, 129u8, 42u8, 84u8, 241u8, 45u8, 199u8, 179u8,
							128u8, 61u8, 170u8, 137u8, 231u8, 156u8, 247u8, 57u8, 47u8, 38u8,
						],
					)
				}
				#[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
				pub fn extrinsic_data(
					&self,
					_0: impl ::std::borrow::Borrow<types::extrinsic_data::Param0>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageKey<types::extrinsic_data::Param0>,
					types::extrinsic_data::ExtrinsicData,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"ExtrinsicData",
						::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							160u8, 180u8, 122u8, 18u8, 196u8, 26u8, 2u8, 37u8, 115u8, 232u8, 133u8,
							220u8, 106u8, 245u8, 4u8, 129u8, 42u8, 84u8, 241u8, 45u8, 199u8, 179u8,
							128u8, 61u8, 170u8, 137u8, 231u8, 156u8, 247u8, 57u8, 47u8, 38u8,
						],
					)
				}
				#[doc = " The current block number being processed. Set by `execute_block`."]
				pub fn number(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::number::Number,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"Number",
						(),
						[
							30u8, 194u8, 177u8, 90u8, 194u8, 232u8, 46u8, 180u8, 85u8, 129u8, 14u8,
							9u8, 8u8, 8u8, 23u8, 95u8, 230u8, 5u8, 13u8, 105u8, 125u8, 2u8, 22u8,
							200u8, 78u8, 93u8, 115u8, 28u8, 150u8, 113u8, 48u8, 53u8,
						],
					)
				}
				#[doc = " Hash of the previous block."]
				pub fn parent_hash(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::parent_hash::ParentHash,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"ParentHash",
						(),
						[
							26u8, 130u8, 11u8, 216u8, 155u8, 71u8, 128u8, 170u8, 30u8, 153u8, 21u8,
							192u8, 62u8, 93u8, 137u8, 80u8, 120u8, 81u8, 202u8, 94u8, 248u8, 125u8,
							71u8, 82u8, 141u8, 229u8, 32u8, 56u8, 73u8, 50u8, 101u8, 78u8,
						],
					)
				}
				#[doc = " Digest of the current block, also part of the block header."]
				pub fn digest(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::digest::Digest,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"Digest",
						(),
						[
							61u8, 64u8, 237u8, 91u8, 145u8, 232u8, 17u8, 254u8, 181u8, 16u8, 234u8,
							91u8, 51u8, 140u8, 254u8, 131u8, 98u8, 135u8, 21u8, 37u8, 251u8, 20u8,
							58u8, 92u8, 123u8, 141u8, 14u8, 227u8, 146u8, 46u8, 222u8, 117u8,
						],
					)
				}
				#[doc = " Events deposited for the current block."]
				#[doc = ""]
				#[doc = " NOTE: The item is unbound and should therefore never be read on chain."]
				#[doc = " It could otherwise inflate the PoV size of a block."]
				#[doc = ""]
				#[doc = " Events have a large in-memory size. Box the events to not go out-of-memory"]
				#[doc = " just in case someone still reads them from within the runtime."]
				pub fn events(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::events::Events,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"Events",
						(),
						[
							204u8, 231u8, 223u8, 173u8, 74u8, 173u8, 229u8, 160u8, 202u8, 154u8,
							83u8, 212u8, 175u8, 167u8, 66u8, 19u8, 129u8, 120u8, 72u8, 255u8,
							118u8, 209u8, 175u8, 29u8, 85u8, 234u8, 202u8, 57u8, 114u8, 83u8,
							197u8, 35u8,
						],
					)
				}
				#[doc = " The number of events in the `Events<T>` list."]
				pub fn event_count(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::event_count::EventCount,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"EventCount",
						(),
						[
							175u8, 24u8, 252u8, 184u8, 210u8, 167u8, 146u8, 143u8, 164u8, 80u8,
							151u8, 205u8, 189u8, 189u8, 55u8, 220u8, 47u8, 101u8, 181u8, 33u8,
							254u8, 131u8, 13u8, 143u8, 3u8, 244u8, 245u8, 45u8, 2u8, 210u8, 79u8,
							133u8,
						],
					)
				}
				#[doc = " Mapping between a topic (represented by T::Hash) and a vector of indexes"]
				#[doc = " of events in the `<Events<T>>` list."]
				#[doc = ""]
				#[doc = " All topic vectors have deterministic storage locations depending on the topic. This"]
				#[doc = " allows light-clients to leverage the changes trie storage tracking mechanism and"]
				#[doc = " in case of changes fetch the list of events of interest."]
				#[doc = ""]
				#[doc = " The value has the type `(BlockNumberFor<T>, EventIndex)` because if we used only just"]
				#[doc = " the `EventIndex` then in case if the topic has the same contents on the next block"]
				#[doc = " no notification will be triggered thus the event might be lost."]
				pub fn event_topics_iter(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::event_topics::EventTopics,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"EventTopics",
						(),
						[
							40u8, 225u8, 14u8, 75u8, 44u8, 176u8, 76u8, 34u8, 143u8, 107u8, 69u8,
							133u8, 114u8, 13u8, 172u8, 250u8, 141u8, 73u8, 12u8, 65u8, 217u8, 63u8,
							120u8, 241u8, 48u8, 106u8, 143u8, 161u8, 128u8, 100u8, 166u8, 59u8,
						],
					)
				}
				#[doc = " Mapping between a topic (represented by T::Hash) and a vector of indexes"]
				#[doc = " of events in the `<Events<T>>` list."]
				#[doc = ""]
				#[doc = " All topic vectors have deterministic storage locations depending on the topic. This"]
				#[doc = " allows light-clients to leverage the changes trie storage tracking mechanism and"]
				#[doc = " in case of changes fetch the list of events of interest."]
				#[doc = ""]
				#[doc = " The value has the type `(BlockNumberFor<T>, EventIndex)` because if we used only just"]
				#[doc = " the `EventIndex` then in case if the topic has the same contents on the next block"]
				#[doc = " no notification will be triggered thus the event might be lost."]
				pub fn event_topics(
					&self,
					_0: impl ::std::borrow::Borrow<types::event_topics::Param0>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageKey<types::event_topics::Param0>,
					types::event_topics::EventTopics,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"EventTopics",
						::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							40u8, 225u8, 14u8, 75u8, 44u8, 176u8, 76u8, 34u8, 143u8, 107u8, 69u8,
							133u8, 114u8, 13u8, 172u8, 250u8, 141u8, 73u8, 12u8, 65u8, 217u8, 63u8,
							120u8, 241u8, 48u8, 106u8, 143u8, 161u8, 128u8, 100u8, 166u8, 59u8,
						],
					)
				}
				#[doc = " Stores the `spec_version` and `spec_name` of when the last runtime upgrade happened."]
				pub fn last_runtime_upgrade(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::last_runtime_upgrade::LastRuntimeUpgrade,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"LastRuntimeUpgrade",
						(),
						[
							137u8, 29u8, 175u8, 75u8, 197u8, 208u8, 91u8, 207u8, 156u8, 87u8,
							148u8, 68u8, 91u8, 140u8, 22u8, 233u8, 1u8, 229u8, 56u8, 34u8, 40u8,
							194u8, 253u8, 30u8, 163u8, 39u8, 54u8, 209u8, 13u8, 27u8, 139u8, 184u8,
						],
					)
				}
				#[doc = " True if we have upgraded so that `type RefCount` is `u32`. False (default) if not."]
				pub fn upgraded_to_u32_ref_count(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::upgraded_to_u32_ref_count::UpgradedToU32RefCount,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"UpgradedToU32RefCount",
						(),
						[
							229u8, 73u8, 9u8, 132u8, 186u8, 116u8, 151u8, 171u8, 145u8, 29u8, 34u8,
							130u8, 52u8, 146u8, 124u8, 175u8, 79u8, 189u8, 147u8, 230u8, 234u8,
							107u8, 124u8, 31u8, 2u8, 22u8, 86u8, 190u8, 4u8, 147u8, 50u8, 245u8,
						],
					)
				}
				#[doc = " True if we have upgraded so that AccountInfo contains three types of `RefCount`. False"]
				#[doc = " (default) if not."]
				pub fn upgraded_to_triple_ref_count(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::upgraded_to_triple_ref_count::UpgradedToTripleRefCount,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"UpgradedToTripleRefCount",
						(),
						[
							97u8, 66u8, 124u8, 243u8, 27u8, 167u8, 147u8, 81u8, 254u8, 201u8,
							101u8, 24u8, 40u8, 231u8, 14u8, 179u8, 154u8, 163u8, 71u8, 81u8, 185u8,
							167u8, 82u8, 254u8, 189u8, 3u8, 101u8, 207u8, 206u8, 194u8, 155u8,
							151u8,
						],
					)
				}
				#[doc = " The execution phase of the block."]
				pub fn execution_phase(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::execution_phase::ExecutionPhase,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"ExecutionPhase",
						(),
						[
							191u8, 129u8, 100u8, 134u8, 126u8, 116u8, 154u8, 203u8, 220u8, 200u8,
							0u8, 26u8, 161u8, 250u8, 133u8, 205u8, 146u8, 24u8, 5u8, 156u8, 158u8,
							35u8, 36u8, 253u8, 52u8, 235u8, 86u8, 167u8, 35u8, 100u8, 119u8, 27u8,
						],
					)
				}
				#[doc = " `Some` if a code upgrade has been authorized."]
				pub fn authorized_upgrade(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::authorized_upgrade::AuthorizedUpgrade,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"AuthorizedUpgrade",
						(),
						[
							165u8, 97u8, 27u8, 138u8, 2u8, 28u8, 55u8, 92u8, 96u8, 96u8, 168u8,
							169u8, 55u8, 178u8, 44u8, 127u8, 58u8, 140u8, 206u8, 178u8, 1u8, 37u8,
							214u8, 213u8, 251u8, 123u8, 5u8, 111u8, 90u8, 148u8, 217u8, 135u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " Block & extrinsics weights: base values and limits."]
				pub fn block_weights(
					&self,
				) -> ::subxt::constants::Address<runtime_types::frame_system::limits::BlockWeights>
				{
					::subxt::constants::Address::new_static(
						"System",
						"BlockWeights",
						[
							176u8, 124u8, 225u8, 136u8, 25u8, 73u8, 247u8, 33u8, 82u8, 206u8, 85u8,
							190u8, 127u8, 102u8, 71u8, 11u8, 185u8, 8u8, 58u8, 0u8, 94u8, 55u8,
							163u8, 177u8, 104u8, 59u8, 60u8, 136u8, 246u8, 116u8, 0u8, 239u8,
						],
					)
				}
				#[doc = " The maximum length of a block (in bytes)."]
				pub fn block_length(
					&self,
				) -> ::subxt::constants::Address<runtime_types::frame_system::limits::BlockLength> {
					::subxt::constants::Address::new_static(
						"System",
						"BlockLength",
						[
							23u8, 242u8, 225u8, 39u8, 225u8, 67u8, 152u8, 41u8, 155u8, 104u8, 68u8,
							229u8, 185u8, 133u8, 10u8, 143u8, 184u8, 152u8, 234u8, 44u8, 140u8,
							96u8, 166u8, 235u8, 162u8, 160u8, 72u8, 7u8, 35u8, 194u8, 3u8, 37u8,
						],
					)
				}
				#[doc = " Maximum number of block number to block hash mappings to keep (oldest pruned first)."]
				pub fn block_hash_count(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"System",
						"BlockHashCount",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The weight of runtime database operations the runtime can invoke."]
				pub fn db_weight(
					&self,
				) -> ::subxt::constants::Address<runtime_types::sp_weights::RuntimeDbWeight> {
					::subxt::constants::Address::new_static(
						"System",
						"DbWeight",
						[
							42u8, 43u8, 178u8, 142u8, 243u8, 203u8, 60u8, 173u8, 118u8, 111u8,
							200u8, 170u8, 102u8, 70u8, 237u8, 187u8, 198u8, 120u8, 153u8, 232u8,
							183u8, 76u8, 74u8, 10u8, 70u8, 243u8, 14u8, 218u8, 213u8, 126u8, 29u8,
							177u8,
						],
					)
				}
				#[doc = " Get the chain's in-code version."]
				pub fn version(
					&self,
				) -> ::subxt::constants::Address<runtime_types::sp_version::RuntimeVersion> {
					::subxt::constants::Address::new_static(
						"System",
						"Version",
						[
							219u8, 45u8, 162u8, 245u8, 177u8, 246u8, 48u8, 126u8, 191u8, 157u8,
							228u8, 83u8, 111u8, 133u8, 183u8, 13u8, 148u8, 108u8, 92u8, 102u8,
							72u8, 205u8, 74u8, 242u8, 233u8, 79u8, 20u8, 170u8, 72u8, 202u8, 158u8,
							165u8,
						],
					)
				}
				#[doc = " The designated SS58 prefix of this chain."]
				#[doc = ""]
				#[doc = " This replaces the \"ss58Format\" property declared in the chain spec. Reason is"]
				#[doc = " that the runtime should know about the prefix in order to make use of it as"]
				#[doc = " an identifier of the chain."]
				pub fn ss58_prefix(&self) -> ::subxt::constants::Address<::core::primitive::u16> {
					::subxt::constants::Address::new_static(
						"System",
						"SS58Prefix",
						[
							116u8, 33u8, 2u8, 170u8, 181u8, 147u8, 171u8, 169u8, 167u8, 227u8,
							41u8, 144u8, 11u8, 236u8, 82u8, 100u8, 74u8, 60u8, 184u8, 72u8, 169u8,
							90u8, 208u8, 135u8, 15u8, 117u8, 10u8, 123u8, 128u8, 193u8, 29u8, 70u8,
						],
					)
				}
			}
		}
	}
	pub mod timestamp {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_timestamp::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Set the current time."]
				#[doc = ""]
				#[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
				#[doc = "phase, if this call hasn't been invoked by that time."]
				#[doc = ""]
				#[doc = "The timestamp should be greater than the previous one by the amount specified by"]
				#[doc = "[`Config::MinimumPeriod`]."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _None_."]
				#[doc = ""]
				#[doc = "This dispatch class is _Mandatory_ to ensure it gets executed in the block. Be aware"]
				#[doc = "that changing the complexity of this call could result exhausting the resources in a"]
				#[doc = "block to execute any other calls."]
				#[doc = ""]
				#[doc = "## Complexity"]
				#[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
				#[doc = "- 1 storage read and 1 storage mutation (codec `O(1)` because of `DidUpdate::take` in"]
				#[doc = "  `on_finalize`)"]
				#[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
				pub struct Set {
					#[codec(compact)]
					pub now: set::Now,
				}
				pub mod set {
					use super::runtime_types;
					pub type Now = ::core::primitive::u64;
				}
				impl ::subxt::blocks::StaticExtrinsic for Set {
					const PALLET: &'static str = "Timestamp";
					const CALL: &'static str = "set";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Set the current time."]
				#[doc = ""]
				#[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
				#[doc = "phase, if this call hasn't been invoked by that time."]
				#[doc = ""]
				#[doc = "The timestamp should be greater than the previous one by the amount specified by"]
				#[doc = "[`Config::MinimumPeriod`]."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _None_."]
				#[doc = ""]
				#[doc = "This dispatch class is _Mandatory_ to ensure it gets executed in the block. Be aware"]
				#[doc = "that changing the complexity of this call could result exhausting the resources in a"]
				#[doc = "block to execute any other calls."]
				#[doc = ""]
				#[doc = "## Complexity"]
				#[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
				#[doc = "- 1 storage read and 1 storage mutation (codec `O(1)` because of `DidUpdate::take` in"]
				#[doc = "  `on_finalize`)"]
				#[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
				pub fn set(&self, now: types::set::Now) -> ::subxt::tx::Payload<types::Set> {
					::subxt::tx::Payload::new_static(
						"Timestamp",
						"set",
						types::Set { now },
						[
							37u8, 95u8, 49u8, 218u8, 24u8, 22u8, 0u8, 95u8, 72u8, 35u8, 155u8,
							199u8, 213u8, 54u8, 207u8, 22u8, 185u8, 193u8, 221u8, 70u8, 18u8,
							200u8, 4u8, 231u8, 195u8, 173u8, 6u8, 122u8, 11u8, 203u8, 231u8, 227u8,
						],
					)
				}
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub mod types {
				use super::runtime_types;
				pub mod now {
					use super::runtime_types;
					pub type Now = ::core::primitive::u64;
				}
				pub mod did_update {
					use super::runtime_types;
					pub type DidUpdate = ::core::primitive::bool;
				}
			}
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The current time for the current block."]
				pub fn now(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::now::Now,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Timestamp",
						"Now",
						(),
						[
							44u8, 50u8, 80u8, 30u8, 195u8, 146u8, 123u8, 238u8, 8u8, 163u8, 187u8,
							92u8, 61u8, 39u8, 51u8, 29u8, 173u8, 169u8, 217u8, 158u8, 85u8, 187u8,
							141u8, 26u8, 12u8, 115u8, 51u8, 11u8, 200u8, 244u8, 138u8, 152u8,
						],
					)
				}
				#[doc = " Whether the timestamp has been updated in this block."]
				#[doc = ""]
				#[doc = " This value is updated to `true` upon successful submission of a timestamp by a node."]
				#[doc = " It is then checked at the end of each block execution in the `on_finalize` hook."]
				pub fn did_update(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::did_update::DidUpdate,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Timestamp",
						"DidUpdate",
						(),
						[
							229u8, 175u8, 246u8, 102u8, 237u8, 158u8, 212u8, 229u8, 238u8, 214u8,
							205u8, 160u8, 164u8, 252u8, 195u8, 75u8, 139u8, 110u8, 22u8, 34u8,
							248u8, 204u8, 107u8, 46u8, 20u8, 200u8, 238u8, 167u8, 71u8, 41u8,
							214u8, 140u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The minimum period between blocks."]
				#[doc = ""]
				#[doc = " Be aware that this is different to the *expected* period that the block production"]
				#[doc = " apparatus provides. Your chosen consensus system will generally work with this to"]
				#[doc = " determine a sensible block time. For example, in the Aura pallet it will be double this"]
				#[doc = " period on default settings."]
				pub fn minimum_period(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u64> {
					::subxt::constants::Address::new_static(
						"Timestamp",
						"MinimumPeriod",
						[
							128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
							59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
							103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
							246u8,
						],
					)
				}
			}
		}
	}
	pub mod ticks {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::pallet_ticks::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_ticks::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
			}
			pub struct TransactionApi;
			impl TransactionApi {}
		}
		pub mod storage {
			use super::runtime_types;
			pub mod types {
				use super::runtime_types;
				pub mod current_tick {
					use super::runtime_types;
					pub type CurrentTick = ::core::primitive::u32;
				}
				pub mod tick_duration {
					use super::runtime_types;
					pub type TickDuration = ::core::primitive::u64;
				}
				pub mod genesis_tick_utc_timestamp {
					use super::runtime_types;
					pub type GenesisTickUtcTimestamp = ::core::primitive::u64;
				}
				pub mod recent_blocks_at_ticks {
					use super::runtime_types;
					pub type RecentBlocksAtTicks =
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::subxt::utils::H256,
						>;
					pub type Param0 = ::core::primitive::u32;
				}
			}
			pub struct StorageApi;
			impl StorageApi {
				pub fn current_tick(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::current_tick::CurrentTick,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Ticks",
						"CurrentTick",
						(),
						[
							22u8, 7u8, 231u8, 159u8, 250u8, 169u8, 243u8, 224u8, 215u8, 82u8, 83u8,
							88u8, 83u8, 90u8, 150u8, 36u8, 157u8, 90u8, 223u8, 33u8, 128u8, 179u8,
							239u8, 41u8, 14u8, 89u8, 96u8, 146u8, 165u8, 37u8, 38u8, 232u8,
						],
					)
				}
				pub fn tick_duration(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::tick_duration::TickDuration,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Ticks",
						"TickDuration",
						(),
						[
							55u8, 52u8, 212u8, 174u8, 70u8, 95u8, 106u8, 159u8, 188u8, 126u8,
							123u8, 167u8, 200u8, 162u8, 123u8, 151u8, 208u8, 141u8, 238u8, 30u8,
							2u8, 249u8, 59u8, 144u8, 88u8, 239u8, 82u8, 32u8, 171u8, 142u8, 241u8,
							130u8,
						],
					)
				}
				pub fn genesis_tick_utc_timestamp(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::genesis_tick_utc_timestamp::GenesisTickUtcTimestamp,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Ticks",
						"GenesisTickUtcTimestamp",
						(),
						[
							237u8, 236u8, 104u8, 247u8, 108u8, 221u8, 147u8, 133u8, 46u8, 84u8,
							173u8, 103u8, 141u8, 162u8, 59u8, 108u8, 39u8, 245u8, 68u8, 84u8,
							216u8, 141u8, 150u8, 23u8, 36u8, 174u8, 131u8, 175u8, 249u8, 139u8,
							213u8, 248u8,
						],
					)
				}
				#[doc = " Blocks from the last 100 ticks. Trimmed in on_initialize."]
				#[doc = " NOTE: cannot include the current block hash until next block"]
				pub fn recent_blocks_at_ticks_iter(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::recent_blocks_at_ticks::RecentBlocksAtTicks,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ticks",
						"RecentBlocksAtTicks",
						(),
						[
							87u8, 146u8, 56u8, 118u8, 8u8, 156u8, 127u8, 97u8, 47u8, 118u8, 176u8,
							69u8, 28u8, 88u8, 208u8, 151u8, 231u8, 136u8, 139u8, 247u8, 240u8,
							171u8, 13u8, 89u8, 145u8, 134u8, 81u8, 194u8, 30u8, 219u8, 126u8, 55u8,
						],
					)
				}
				#[doc = " Blocks from the last 100 ticks. Trimmed in on_initialize."]
				#[doc = " NOTE: cannot include the current block hash until next block"]
				pub fn recent_blocks_at_ticks(
					&self,
					_0: impl ::std::borrow::Borrow<types::recent_blocks_at_ticks::Param0>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageKey<
						types::recent_blocks_at_ticks::Param0,
					>,
					types::recent_blocks_at_ticks::RecentBlocksAtTicks,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Ticks",
						"RecentBlocksAtTicks",
						::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							87u8, 146u8, 56u8, 118u8, 8u8, 156u8, 127u8, 97u8, 47u8, 118u8, 176u8,
							69u8, 28u8, 88u8, 208u8, 151u8, 231u8, 136u8, 139u8, 247u8, 240u8,
							171u8, 13u8, 89u8, 145u8, 134u8, 81u8, 194u8, 30u8, 219u8, 126u8, 55u8,
						],
					)
				}
			}
		}
	}
	pub mod mining_slot {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::pallet_mining_slot::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_mining_slot::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Bid {
					pub bond_id: bid::BondId,
					pub reward_destination: bid::RewardDestination,
				}
				pub mod bid {
					use super::runtime_types;
					pub type BondId = ::core::option::Option<::core::primitive::u64>;
					pub type RewardDestination =
						runtime_types::ulx_primitives::block_seal::RewardDestination<
							::subxt::utils::AccountId32,
						>;
				}
				impl ::subxt::blocks::StaticExtrinsic for Bid {
					const PALLET: &'static str = "MiningSlot";
					const CALL: &'static str = "bid";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn bid(
					&self,
					bond_id: types::bid::BondId,
					reward_destination: types::bid::RewardDestination,
				) -> ::subxt::tx::Payload<types::Bid> {
					::subxt::tx::Payload::new_static(
						"MiningSlot",
						"bid",
						types::Bid { bond_id, reward_destination },
						[
							242u8, 76u8, 69u8, 196u8, 222u8, 106u8, 100u8, 160u8, 87u8, 1u8, 218u8,
							17u8, 174u8, 70u8, 231u8, 207u8, 87u8, 65u8, 14u8, 36u8, 186u8, 148u8,
							183u8, 221u8, 95u8, 79u8, 204u8, 251u8, 152u8, 77u8, 2u8, 254u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_mining_slot::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct NewMiners {
				pub start_index: new_miners::StartIndex,
				pub new_miners: new_miners::NewMiners,
			}
			pub mod new_miners {
				use super::runtime_types;
				pub type StartIndex = ::core::primitive::u32;
				pub type NewMiners = runtime_types::bounded_collections::bounded_vec::BoundedVec<
					runtime_types::ulx_primitives::block_seal::MiningRegistration<
						::subxt::utils::AccountId32,
						::core::primitive::u64,
						::core::primitive::u128,
					>,
				>;
			}
			impl ::subxt::events::StaticEvent for NewMiners {
				const PALLET: &'static str = "MiningSlot";
				const EVENT: &'static str = "NewMiners";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct SlotBidderAdded {
				pub account_id: slot_bidder_added::AccountId,
				pub bid_amount: slot_bidder_added::BidAmount,
				pub index: slot_bidder_added::Index,
			}
			pub mod slot_bidder_added {
				use super::runtime_types;
				pub type AccountId = ::subxt::utils::AccountId32;
				pub type BidAmount = ::core::primitive::u128;
				pub type Index = ::core::primitive::u32;
			}
			impl ::subxt::events::StaticEvent for SlotBidderAdded {
				const PALLET: &'static str = "MiningSlot";
				const EVENT: &'static str = "SlotBidderAdded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct SlotBidderReplaced {
				pub account_id: slot_bidder_replaced::AccountId,
				pub bond_id: slot_bidder_replaced::BondId,
				pub kept_ownership_bond: slot_bidder_replaced::KeptOwnershipBond,
			}
			pub mod slot_bidder_replaced {
				use super::runtime_types;
				pub type AccountId = ::subxt::utils::AccountId32;
				pub type BondId = ::core::option::Option<::core::primitive::u64>;
				pub type KeptOwnershipBond = ::core::primitive::bool;
			}
			impl ::subxt::events::StaticEvent for SlotBidderReplaced {
				const PALLET: &'static str = "MiningSlot";
				const EVENT: &'static str = "SlotBidderReplaced";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct UnbondedMiner {
				pub account_id: unbonded_miner::AccountId,
				pub bond_id: unbonded_miner::BondId,
				pub kept_ownership_bond: unbonded_miner::KeptOwnershipBond,
			}
			pub mod unbonded_miner {
				use super::runtime_types;
				pub type AccountId = ::subxt::utils::AccountId32;
				pub type BondId = ::core::option::Option<::core::primitive::u64>;
				pub type KeptOwnershipBond = ::core::primitive::bool;
			}
			impl ::subxt::events::StaticEvent for UnbondedMiner {
				const PALLET: &'static str = "MiningSlot";
				const EVENT: &'static str = "UnbondedMiner";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub mod types {
				use super::runtime_types;
				pub mod active_miners_by_index {
					use super::runtime_types;
					pub type ActiveMinersByIndex =
						runtime_types::ulx_primitives::block_seal::MiningRegistration<
							::subxt::utils::AccountId32,
							::core::primitive::u64,
							::core::primitive::u128,
						>;
					pub type Param0 = ::core::primitive::u32;
				}
				pub mod active_miners_count {
					use super::runtime_types;
					pub type ActiveMinersCount = ::core::primitive::u16;
				}
				pub mod authorities_by_index {
					use super::runtime_types;
					pub type AuthoritiesByIndex =
						runtime_types::bounded_collections::bounded_btree_map::BoundedBTreeMap<
							::core::primitive::u32,
							(
								runtime_types::ulx_primitives::block_seal::app::Public,
								runtime_types::primitive_types::U256,
							),
						>;
				}
				pub mod ownership_bond_amount {
					use super::runtime_types;
					pub type OwnershipBondAmount = ::core::primitive::u128;
				}
				pub mod account_index_lookup {
					use super::runtime_types;
					pub type AccountIndexLookup = ::core::primitive::u32;
					pub type Param0 = ::subxt::utils::AccountId32;
				}
				pub mod next_slot_cohort {
					use super::runtime_types;
					pub type NextSlotCohort =
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							runtime_types::ulx_primitives::block_seal::MiningRegistration<
								::subxt::utils::AccountId32,
								::core::primitive::u64,
								::core::primitive::u128,
							>,
						>;
				}
				pub mod is_next_slot_bidding_open {
					use super::runtime_types;
					pub type IsNextSlotBiddingOpen = ::core::primitive::bool;
				}
				pub mod miner_zero {
					use super::runtime_types;
					pub type MinerZero =
						runtime_types::ulx_primitives::block_seal::MiningRegistration<
							::subxt::utils::AccountId32,
							::core::primitive::u64,
							::core::primitive::u128,
						>;
				}
			}
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Miners that are active in the current block (post initialize)"]
				pub fn active_miners_by_index_iter(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::active_miners_by_index::ActiveMinersByIndex,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"MiningSlot",
						"ActiveMinersByIndex",
						(),
						[
							148u8, 176u8, 145u8, 80u8, 231u8, 88u8, 226u8, 142u8, 143u8, 11u8,
							181u8, 231u8, 111u8, 29u8, 112u8, 101u8, 137u8, 234u8, 106u8, 182u8,
							230u8, 237u8, 109u8, 34u8, 11u8, 113u8, 85u8, 233u8, 183u8, 243u8,
							126u8, 175u8,
						],
					)
				}
				#[doc = " Miners that are active in the current block (post initialize)"]
				pub fn active_miners_by_index(
					&self,
					_0: impl ::std::borrow::Borrow<types::active_miners_by_index::Param0>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageKey<
						types::active_miners_by_index::Param0,
					>,
					types::active_miners_by_index::ActiveMinersByIndex,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"MiningSlot",
						"ActiveMinersByIndex",
						::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							148u8, 176u8, 145u8, 80u8, 231u8, 88u8, 226u8, 142u8, 143u8, 11u8,
							181u8, 231u8, 111u8, 29u8, 112u8, 101u8, 137u8, 234u8, 106u8, 182u8,
							230u8, 237u8, 109u8, 34u8, 11u8, 113u8, 85u8, 233u8, 183u8, 243u8,
							126u8, 175u8,
						],
					)
				}
				pub fn active_miners_count(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::active_miners_count::ActiveMinersCount,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"MiningSlot",
						"ActiveMinersCount",
						(),
						[
							20u8, 227u8, 98u8, 196u8, 103u8, 165u8, 188u8, 15u8, 246u8, 164u8,
							26u8, 233u8, 247u8, 78u8, 25u8, 118u8, 152u8, 76u8, 206u8, 87u8, 147u8,
							226u8, 101u8, 252u8, 77u8, 171u8, 75u8, 4u8, 74u8, 30u8, 72u8, 214u8,
						],
					)
				}
				#[doc = " Authorities are the session keys that are actively participating in the network."]
				#[doc = " The tuple is the authority, and the blake2 256 hash of the authority used for xor lookups"]
				pub fn authorities_by_index(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::authorities_by_index::AuthoritiesByIndex,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"MiningSlot",
						"AuthoritiesByIndex",
						(),
						[
							35u8, 35u8, 17u8, 184u8, 250u8, 250u8, 12u8, 2u8, 78u8, 97u8, 114u8,
							1u8, 33u8, 200u8, 71u8, 158u8, 77u8, 136u8, 234u8, 148u8, 193u8, 5u8,
							111u8, 235u8, 252u8, 16u8, 104u8, 203u8, 219u8, 244u8, 1u8, 98u8,
						],
					)
				}
				#[doc = " Tokens that must be bonded to take a Miner role"]
				pub fn ownership_bond_amount(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::ownership_bond_amount::OwnershipBondAmount,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"MiningSlot",
						"OwnershipBondAmount",
						(),
						[
							45u8, 244u8, 244u8, 223u8, 190u8, 125u8, 131u8, 55u8, 69u8, 254u8,
							146u8, 237u8, 64u8, 61u8, 35u8, 114u8, 66u8, 95u8, 137u8, 138u8, 50u8,
							128u8, 217u8, 131u8, 10u8, 243u8, 1u8, 238u8, 208u8, 214u8, 106u8,
							235u8,
						],
					)
				}
				#[doc = " Lookup by account id to the corresponding index in ActiveMinersByIndex and Authorities"]
				pub fn account_index_lookup_iter(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::account_index_lookup::AccountIndexLookup,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"MiningSlot",
						"AccountIndexLookup",
						(),
						[
							203u8, 195u8, 115u8, 185u8, 125u8, 36u8, 9u8, 40u8, 68u8, 9u8, 52u8,
							60u8, 181u8, 139u8, 145u8, 41u8, 100u8, 62u8, 237u8, 172u8, 108u8,
							227u8, 106u8, 161u8, 59u8, 110u8, 244u8, 142u8, 80u8, 147u8, 188u8,
							190u8,
						],
					)
				}
				#[doc = " Lookup by account id to the corresponding index in ActiveMinersByIndex and Authorities"]
				pub fn account_index_lookup(
					&self,
					_0: impl ::std::borrow::Borrow<types::account_index_lookup::Param0>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageKey<
						types::account_index_lookup::Param0,
					>,
					types::account_index_lookup::AccountIndexLookup,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"MiningSlot",
						"AccountIndexLookup",
						::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							203u8, 195u8, 115u8, 185u8, 125u8, 36u8, 9u8, 40u8, 68u8, 9u8, 52u8,
							60u8, 181u8, 139u8, 145u8, 41u8, 100u8, 62u8, 237u8, 172u8, 108u8,
							227u8, 106u8, 161u8, 59u8, 110u8, 244u8, 142u8, 80u8, 147u8, 188u8,
							190u8,
						],
					)
				}
				#[doc = " The cohort set to go into effect in the next slot. The Vec has all"]
				#[doc = " registrants with their bid amount"]
				pub fn next_slot_cohort(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::next_slot_cohort::NextSlotCohort,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"MiningSlot",
						"NextSlotCohort",
						(),
						[
							78u8, 82u8, 132u8, 80u8, 133u8, 190u8, 160u8, 225u8, 211u8, 129u8,
							36u8, 17u8, 20u8, 191u8, 235u8, 58u8, 55u8, 8u8, 50u8, 24u8, 11u8,
							105u8, 176u8, 212u8, 64u8, 227u8, 86u8, 245u8, 43u8, 132u8, 183u8,
							136u8,
						],
					)
				}
				#[doc = " Is the next slot still open for bids"]
				pub fn is_next_slot_bidding_open(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::is_next_slot_bidding_open::IsNextSlotBiddingOpen,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"MiningSlot",
						"IsNextSlotBiddingOpen",
						(),
						[
							55u8, 45u8, 220u8, 192u8, 255u8, 253u8, 247u8, 75u8, 156u8, 4u8, 133u8,
							132u8, 109u8, 242u8, 64u8, 251u8, 149u8, 180u8, 69u8, 54u8, 150u8, 3u8,
							249u8, 2u8, 167u8, 148u8, 133u8, 221u8, 27u8, 227u8, 85u8, 32u8,
						],
					)
				}
				#[doc = " The configuration for a miner to supply if there are no registered miners"]
				pub fn miner_zero(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::miner_zero::MinerZero,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"MiningSlot",
						"MinerZero",
						(),
						[
							74u8, 211u8, 155u8, 146u8, 91u8, 168u8, 147u8, 91u8, 54u8, 150u8,
							246u8, 118u8, 33u8, 167u8, 174u8, 143u8, 32u8, 97u8, 76u8, 211u8, 71u8,
							180u8, 134u8, 183u8, 104u8, 40u8, 151u8, 49u8, 116u8, 109u8, 48u8,
							10u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The maximum number of Miners that the pallet can hold."]
				pub fn max_miners(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"MiningSlot",
						"MaxMiners",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " How many new miners can be in the cohort for each slot"]
				pub fn max_cohort_size(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"MiningSlot",
						"MaxCohortSize",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " How many blocks transpire between slots"]
				pub fn blocks_between_slots(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"MiningSlot",
						"BlocksBetweenSlots",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " How many session indexes to keep session history"]
				pub fn session_indices_to_keep_in_history(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"MiningSlot",
						"SessionIndicesToKeepInHistory",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " How many blocks buffer shall we use to stop accepting bids for the next period"]
				pub fn blocks_buffer_to_stop_accepting_bids(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"MiningSlot",
						"BlocksBufferToStopAcceptingBids",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The reduction in percent of ownership currency required to secure a slot"]
				pub fn ownership_percent_damper(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"MiningSlot",
						"OwnershipPercentDamper",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod bond {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::pallet_bond::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_bond::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct OfferFund {
					#[codec(compact)]
					pub lease_annual_percent_rate: offer_fund::LeaseAnnualPercentRate,
					#[codec(compact)]
					pub lease_base_fee: offer_fund::LeaseBaseFee,
					#[codec(compact)]
					pub amount_offered: offer_fund::AmountOffered,
					pub expiration_block: offer_fund::ExpirationBlock,
				}
				pub mod offer_fund {
					use super::runtime_types;
					pub type LeaseAnnualPercentRate = ::core::primitive::u32;
					pub type LeaseBaseFee = ::core::primitive::u128;
					pub type AmountOffered = ::core::primitive::u128;
					pub type ExpirationBlock = ::core::primitive::u32;
				}
				impl ::subxt::blocks::StaticExtrinsic for OfferFund {
					const PALLET: &'static str = "Bond";
					const CALL: &'static str = "offer_fund";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Stop offering this fund for new bond. Will not affect existing bond. Unreserved funds"]
				#[doc = "are returned immediately."]
				pub struct EndFund {
					pub bond_fund_id: end_fund::BondFundId,
				}
				pub mod end_fund {
					use super::runtime_types;
					pub type BondFundId = ::core::primitive::u32;
				}
				impl ::subxt::blocks::StaticExtrinsic for EndFund {
					const PALLET: &'static str = "Bond";
					const CALL: &'static str = "end_fund";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Add additional time or funds to the bond fund"]
				pub struct ExtendFund {
					pub bond_fund_id: extend_fund::BondFundId,
					pub total_amount_offered: extend_fund::TotalAmountOffered,
					pub expiration_block: extend_fund::ExpirationBlock,
				}
				pub mod extend_fund {
					use super::runtime_types;
					pub type BondFundId = ::core::primitive::u32;
					pub type TotalAmountOffered = ::core::primitive::u128;
					pub type ExpirationBlock = ::core::primitive::u32;
				}
				impl ::subxt::blocks::StaticExtrinsic for ExtendFund {
					const PALLET: &'static str = "Bond";
					const CALL: &'static str = "extend_fund";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct BondSelf {
					pub amount: bond_self::Amount,
					pub bond_until_block: bond_self::BondUntilBlock,
				}
				pub mod bond_self {
					use super::runtime_types;
					pub type Amount = ::core::primitive::u128;
					pub type BondUntilBlock = ::core::primitive::u32;
				}
				impl ::subxt::blocks::StaticExtrinsic for BondSelf {
					const PALLET: &'static str = "Bond";
					const CALL: &'static str = "bond_self";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Lease {
					pub bond_fund_id: lease::BondFundId,
					pub amount: lease::Amount,
					pub lease_until_block: lease::LeaseUntilBlock,
				}
				pub mod lease {
					use super::runtime_types;
					pub type BondFundId = ::core::primitive::u32;
					pub type Amount = ::core::primitive::u128;
					pub type LeaseUntilBlock = ::core::primitive::u32;
				}
				impl ::subxt::blocks::StaticExtrinsic for Lease {
					const PALLET: &'static str = "Bond";
					const CALL: &'static str = "lease";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ReturnBond {
					pub bond_id: return_bond::BondId,
				}
				pub mod return_bond {
					use super::runtime_types;
					pub type BondId = ::core::primitive::u64;
				}
				impl ::subxt::blocks::StaticExtrinsic for ReturnBond {
					const PALLET: &'static str = "Bond";
					const CALL: &'static str = "return_bond";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ExtendBond {
					pub bond_id: extend_bond::BondId,
					pub total_amount: extend_bond::TotalAmount,
					pub bond_until_block: extend_bond::BondUntilBlock,
				}
				pub mod extend_bond {
					use super::runtime_types;
					pub type BondId = ::core::primitive::u64;
					pub type TotalAmount = ::core::primitive::u128;
					pub type BondUntilBlock = ::core::primitive::u32;
				}
				impl ::subxt::blocks::StaticExtrinsic for ExtendBond {
					const PALLET: &'static str = "Bond";
					const CALL: &'static str = "extend_bond";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn offer_fund(
					&self,
					lease_annual_percent_rate: types::offer_fund::LeaseAnnualPercentRate,
					lease_base_fee: types::offer_fund::LeaseBaseFee,
					amount_offered: types::offer_fund::AmountOffered,
					expiration_block: types::offer_fund::ExpirationBlock,
				) -> ::subxt::tx::Payload<types::OfferFund> {
					::subxt::tx::Payload::new_static(
						"Bond",
						"offer_fund",
						types::OfferFund {
							lease_annual_percent_rate,
							lease_base_fee,
							amount_offered,
							expiration_block,
						},
						[
							125u8, 9u8, 52u8, 63u8, 83u8, 189u8, 114u8, 102u8, 223u8, 44u8, 206u8,
							249u8, 14u8, 217u8, 244u8, 238u8, 128u8, 0u8, 119u8, 53u8, 135u8,
							218u8, 23u8, 76u8, 125u8, 138u8, 135u8, 251u8, 145u8, 65u8, 75u8,
							144u8,
						],
					)
				}
				#[doc = "Stop offering this fund for new bond. Will not affect existing bond. Unreserved funds"]
				#[doc = "are returned immediately."]
				pub fn end_fund(
					&self,
					bond_fund_id: types::end_fund::BondFundId,
				) -> ::subxt::tx::Payload<types::EndFund> {
					::subxt::tx::Payload::new_static(
						"Bond",
						"end_fund",
						types::EndFund { bond_fund_id },
						[
							131u8, 140u8, 43u8, 83u8, 124u8, 77u8, 95u8, 84u8, 250u8, 118u8, 21u8,
							237u8, 240u8, 20u8, 83u8, 167u8, 60u8, 191u8, 90u8, 241u8, 100u8, 39u8,
							7u8, 24u8, 79u8, 149u8, 182u8, 219u8, 240u8, 243u8, 3u8, 70u8,
						],
					)
				}
				#[doc = "Add additional time or funds to the bond fund"]
				pub fn extend_fund(
					&self,
					bond_fund_id: types::extend_fund::BondFundId,
					total_amount_offered: types::extend_fund::TotalAmountOffered,
					expiration_block: types::extend_fund::ExpirationBlock,
				) -> ::subxt::tx::Payload<types::ExtendFund> {
					::subxt::tx::Payload::new_static(
						"Bond",
						"extend_fund",
						types::ExtendFund { bond_fund_id, total_amount_offered, expiration_block },
						[
							8u8, 174u8, 241u8, 111u8, 94u8, 45u8, 104u8, 19u8, 98u8, 42u8, 125u8,
							136u8, 192u8, 84u8, 198u8, 19u8, 30u8, 59u8, 176u8, 154u8, 238u8, 99u8,
							2u8, 35u8, 31u8, 209u8, 248u8, 175u8, 110u8, 156u8, 139u8, 205u8,
						],
					)
				}
				pub fn bond_self(
					&self,
					amount: types::bond_self::Amount,
					bond_until_block: types::bond_self::BondUntilBlock,
				) -> ::subxt::tx::Payload<types::BondSelf> {
					::subxt::tx::Payload::new_static(
						"Bond",
						"bond_self",
						types::BondSelf { amount, bond_until_block },
						[
							162u8, 47u8, 161u8, 45u8, 175u8, 122u8, 229u8, 198u8, 84u8, 116u8,
							72u8, 15u8, 173u8, 186u8, 40u8, 5u8, 127u8, 189u8, 39u8, 40u8, 108u8,
							71u8, 18u8, 16u8, 21u8, 139u8, 135u8, 101u8, 152u8, 202u8, 180u8,
							160u8,
						],
					)
				}
				pub fn lease(
					&self,
					bond_fund_id: types::lease::BondFundId,
					amount: types::lease::Amount,
					lease_until_block: types::lease::LeaseUntilBlock,
				) -> ::subxt::tx::Payload<types::Lease> {
					::subxt::tx::Payload::new_static(
						"Bond",
						"lease",
						types::Lease { bond_fund_id, amount, lease_until_block },
						[
							12u8, 180u8, 215u8, 175u8, 212u8, 46u8, 23u8, 132u8, 251u8, 15u8,
							239u8, 80u8, 170u8, 146u8, 89u8, 25u8, 38u8, 219u8, 91u8, 243u8, 137u8,
							233u8, 134u8, 5u8, 100u8, 14u8, 89u8, 161u8, 162u8, 99u8, 40u8, 212u8,
						],
					)
				}
				pub fn return_bond(
					&self,
					bond_id: types::return_bond::BondId,
				) -> ::subxt::tx::Payload<types::ReturnBond> {
					::subxt::tx::Payload::new_static(
						"Bond",
						"return_bond",
						types::ReturnBond { bond_id },
						[
							19u8, 164u8, 101u8, 219u8, 211u8, 192u8, 51u8, 188u8, 23u8, 171u8,
							188u8, 55u8, 230u8, 117u8, 110u8, 223u8, 129u8, 12u8, 187u8, 178u8,
							152u8, 23u8, 120u8, 20u8, 147u8, 227u8, 96u8, 109u8, 45u8, 28u8, 225u8,
							111u8,
						],
					)
				}
				pub fn extend_bond(
					&self,
					bond_id: types::extend_bond::BondId,
					total_amount: types::extend_bond::TotalAmount,
					bond_until_block: types::extend_bond::BondUntilBlock,
				) -> ::subxt::tx::Payload<types::ExtendBond> {
					::subxt::tx::Payload::new_static(
						"Bond",
						"extend_bond",
						types::ExtendBond { bond_id, total_amount, bond_until_block },
						[
							0u8, 53u8, 253u8, 106u8, 104u8, 68u8, 176u8, 190u8, 93u8, 28u8, 180u8,
							75u8, 95u8, 132u8, 138u8, 16u8, 88u8, 225u8, 149u8, 29u8, 185u8, 211u8,
							38u8, 49u8, 65u8, 14u8, 198u8, 200u8, 192u8, 109u8, 48u8, 145u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_bond::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct BondFundOffered {
				pub bond_fund_id: bond_fund_offered::BondFundId,
				pub amount_offered: bond_fund_offered::AmountOffered,
				pub expiration_block: bond_fund_offered::ExpirationBlock,
				pub offer_account_id: bond_fund_offered::OfferAccountId,
			}
			pub mod bond_fund_offered {
				use super::runtime_types;
				pub type BondFundId = ::core::primitive::u32;
				pub type AmountOffered = ::core::primitive::u128;
				pub type ExpirationBlock = ::core::primitive::u32;
				pub type OfferAccountId = ::subxt::utils::AccountId32;
			}
			impl ::subxt::events::StaticEvent for BondFundOffered {
				const PALLET: &'static str = "Bond";
				const EVENT: &'static str = "BondFundOffered";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct BondFundExtended {
				pub bond_fund_id: bond_fund_extended::BondFundId,
				pub amount_offered: bond_fund_extended::AmountOffered,
				pub expiration_block: bond_fund_extended::ExpirationBlock,
			}
			pub mod bond_fund_extended {
				use super::runtime_types;
				pub type BondFundId = ::core::primitive::u32;
				pub type AmountOffered = ::core::primitive::u128;
				pub type ExpirationBlock = ::core::primitive::u32;
			}
			impl ::subxt::events::StaticEvent for BondFundExtended {
				const PALLET: &'static str = "Bond";
				const EVENT: &'static str = "BondFundExtended";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct BondFundEnded {
				pub bond_fund_id: bond_fund_ended::BondFundId,
				pub amount_still_bonded: bond_fund_ended::AmountStillBonded,
			}
			pub mod bond_fund_ended {
				use super::runtime_types;
				pub type BondFundId = ::core::primitive::u32;
				pub type AmountStillBonded = ::core::primitive::u128;
			}
			impl ::subxt::events::StaticEvent for BondFundEnded {
				const PALLET: &'static str = "Bond";
				const EVENT: &'static str = "BondFundEnded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct BondFundExpired {
				pub bond_fund_id: bond_fund_expired::BondFundId,
				pub offer_account_id: bond_fund_expired::OfferAccountId,
			}
			pub mod bond_fund_expired {
				use super::runtime_types;
				pub type BondFundId = ::core::primitive::u32;
				pub type OfferAccountId = ::subxt::utils::AccountId32;
			}
			impl ::subxt::events::StaticEvent for BondFundExpired {
				const PALLET: &'static str = "Bond";
				const EVENT: &'static str = "BondFundExpired";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct BondedSelf {
				pub bond_id: bonded_self::BondId,
				pub bonded_account_id: bonded_self::BondedAccountId,
				pub amount: bonded_self::Amount,
				pub completion_block: bonded_self::CompletionBlock,
			}
			pub mod bonded_self {
				use super::runtime_types;
				pub type BondId = ::core::primitive::u64;
				pub type BondedAccountId = ::subxt::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
				pub type CompletionBlock = ::core::primitive::u32;
			}
			impl ::subxt::events::StaticEvent for BondedSelf {
				const PALLET: &'static str = "Bond";
				const EVENT: &'static str = "BondedSelf";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct BondLeased {
				pub bond_fund_id: bond_leased::BondFundId,
				pub bond_id: bond_leased::BondId,
				pub bonded_account_id: bond_leased::BondedAccountId,
				pub amount: bond_leased::Amount,
				pub total_fee: bond_leased::TotalFee,
				pub annual_percent_rate: bond_leased::AnnualPercentRate,
				pub completion_block: bond_leased::CompletionBlock,
			}
			pub mod bond_leased {
				use super::runtime_types;
				pub type BondFundId = ::core::primitive::u32;
				pub type BondId = ::core::primitive::u64;
				pub type BondedAccountId = ::subxt::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
				pub type TotalFee = ::core::primitive::u128;
				pub type AnnualPercentRate = ::core::primitive::u32;
				pub type CompletionBlock = ::core::primitive::u32;
			}
			impl ::subxt::events::StaticEvent for BondLeased {
				const PALLET: &'static str = "Bond";
				const EVENT: &'static str = "BondLeased";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct BondExtended {
				pub bond_fund_id: bond_extended::BondFundId,
				pub bond_id: bond_extended::BondId,
				pub amount: bond_extended::Amount,
				pub completion_block: bond_extended::CompletionBlock,
				pub fee_change: bond_extended::FeeChange,
				pub annual_percent_rate: bond_extended::AnnualPercentRate,
			}
			pub mod bond_extended {
				use super::runtime_types;
				pub type BondFundId = ::core::option::Option<::core::primitive::u32>;
				pub type BondId = ::core::primitive::u64;
				pub type Amount = ::core::primitive::u128;
				pub type CompletionBlock = ::core::primitive::u32;
				pub type FeeChange = ::core::primitive::u128;
				pub type AnnualPercentRate = ::core::primitive::u32;
			}
			impl ::subxt::events::StaticEvent for BondExtended {
				const PALLET: &'static str = "Bond";
				const EVENT: &'static str = "BondExtended";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct BondCompleted {
				pub bond_fund_id: bond_completed::BondFundId,
				pub bond_id: bond_completed::BondId,
			}
			pub mod bond_completed {
				use super::runtime_types;
				pub type BondFundId = ::core::option::Option<::core::primitive::u32>;
				pub type BondId = ::core::primitive::u64;
			}
			impl ::subxt::events::StaticEvent for BondCompleted {
				const PALLET: &'static str = "Bond";
				const EVENT: &'static str = "BondCompleted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct BondFeeRefund {
				pub bond_fund_id: bond_fee_refund::BondFundId,
				pub bond_id: bond_fee_refund::BondId,
				pub bonded_account_id: bond_fee_refund::BondedAccountId,
				pub bond_fund_reduction_for_payment: bond_fee_refund::BondFundReductionForPayment,
				pub final_fee: bond_fee_refund::FinalFee,
				pub refund_amount: bond_fee_refund::RefundAmount,
			}
			pub mod bond_fee_refund {
				use super::runtime_types;
				pub type BondFundId = ::core::primitive::u32;
				pub type BondId = ::core::primitive::u64;
				pub type BondedAccountId = ::subxt::utils::AccountId32;
				pub type BondFundReductionForPayment = ::core::primitive::u128;
				pub type FinalFee = ::core::primitive::u128;
				pub type RefundAmount = ::core::primitive::u128;
			}
			impl ::subxt::events::StaticEvent for BondFeeRefund {
				const PALLET: &'static str = "Bond";
				const EVENT: &'static str = "BondFeeRefund";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct BondLocked {
				pub bond_id: bond_locked::BondId,
				pub bonded_account_id: bond_locked::BondedAccountId,
			}
			pub mod bond_locked {
				use super::runtime_types;
				pub type BondId = ::core::primitive::u64;
				pub type BondedAccountId = ::subxt::utils::AccountId32;
			}
			impl ::subxt::events::StaticEvent for BondLocked {
				const PALLET: &'static str = "Bond";
				const EVENT: &'static str = "BondLocked";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct BondUnlocked {
				pub bond_id: bond_unlocked::BondId,
				pub bonded_account_id: bond_unlocked::BondedAccountId,
			}
			pub mod bond_unlocked {
				use super::runtime_types;
				pub type BondId = ::core::primitive::u64;
				pub type BondedAccountId = ::subxt::utils::AccountId32;
			}
			impl ::subxt::events::StaticEvent for BondUnlocked {
				const PALLET: &'static str = "Bond";
				const EVENT: &'static str = "BondUnlocked";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub mod types {
				use super::runtime_types;
				pub mod next_bond_id {
					use super::runtime_types;
					pub type NextBondId = ::core::primitive::u64;
				}
				pub mod next_bond_fund_id {
					use super::runtime_types;
					pub type NextBondFundId = ::core::primitive::u32;
				}
				pub mod bond_funds {
					use super::runtime_types;
					pub type BondFunds = runtime_types::ulx_primitives::bond::BondFund<
						::subxt::utils::AccountId32,
						::core::primitive::u128,
						::core::primitive::u32,
					>;
					pub type Param0 = ::core::primitive::u32;
				}
				pub mod bond_fund_expirations {
					use super::runtime_types;
					pub type BondFundExpirations =
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u32,
						>;
					pub type Param0 = ::core::primitive::u32;
				}
				pub mod bonds {
					use super::runtime_types;
					pub type Bonds = runtime_types::ulx_primitives::bond::Bond<
						::subxt::utils::AccountId32,
						::core::primitive::u128,
						::core::primitive::u32,
						::core::primitive::u32,
					>;
					pub type Param0 = ::core::primitive::u64;
				}
				pub mod bond_completions {
					use super::runtime_types;
					pub type BondCompletions =
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u64,
						>;
					pub type Param0 = ::core::primitive::u32;
				}
			}
			pub struct StorageApi;
			impl StorageApi {
				pub fn next_bond_id(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::next_bond_id::NextBondId,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Bond",
						"NextBondId",
						(),
						[
							5u8, 229u8, 152u8, 112u8, 204u8, 211u8, 171u8, 9u8, 47u8, 162u8, 31u8,
							88u8, 78u8, 187u8, 161u8, 163u8, 70u8, 216u8, 229u8, 145u8, 188u8,
							250u8, 163u8, 102u8, 207u8, 195u8, 149u8, 21u8, 202u8, 216u8, 11u8,
							181u8,
						],
					)
				}
				pub fn next_bond_fund_id(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::next_bond_fund_id::NextBondFundId,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Bond",
						"NextBondFundId",
						(),
						[
							194u8, 59u8, 237u8, 245u8, 182u8, 7u8, 180u8, 225u8, 13u8, 94u8, 214u8,
							166u8, 215u8, 116u8, 117u8, 79u8, 103u8, 219u8, 89u8, 99u8, 37u8, 8u8,
							30u8, 160u8, 24u8, 48u8, 43u8, 81u8, 44u8, 178u8, 93u8, 46u8,
						],
					)
				}
				#[doc = " BondFunds by id"]
				pub fn bond_funds_iter(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::bond_funds::BondFunds,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Bond",
						"BondFunds",
						(),
						[
							201u8, 175u8, 22u8, 184u8, 52u8, 18u8, 133u8, 107u8, 222u8, 91u8,
							177u8, 22u8, 45u8, 39u8, 73u8, 235u8, 89u8, 138u8, 41u8, 250u8, 31u8,
							245u8, 212u8, 105u8, 79u8, 147u8, 126u8, 235u8, 37u8, 21u8, 57u8, 58u8,
						],
					)
				}
				#[doc = " BondFunds by id"]
				pub fn bond_funds(
					&self,
					_0: impl ::std::borrow::Borrow<types::bond_funds::Param0>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageKey<types::bond_funds::Param0>,
					types::bond_funds::BondFunds,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Bond",
						"BondFunds",
						::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							201u8, 175u8, 22u8, 184u8, 52u8, 18u8, 133u8, 107u8, 222u8, 91u8,
							177u8, 22u8, 45u8, 39u8, 73u8, 235u8, 89u8, 138u8, 41u8, 250u8, 31u8,
							245u8, 212u8, 105u8, 79u8, 147u8, 126u8, 235u8, 37u8, 21u8, 57u8, 58u8,
						],
					)
				}
				#[doc = " Expiration block number for each bond fund"]
				pub fn bond_fund_expirations_iter(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::bond_fund_expirations::BondFundExpirations,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Bond",
						"BondFundExpirations",
						(),
						[
							22u8, 162u8, 77u8, 110u8, 141u8, 47u8, 65u8, 85u8, 143u8, 125u8, 17u8,
							130u8, 155u8, 207u8, 222u8, 161u8, 192u8, 51u8, 172u8, 40u8, 239u8,
							126u8, 0u8, 56u8, 130u8, 215u8, 137u8, 234u8, 176u8, 12u8, 98u8, 110u8,
						],
					)
				}
				#[doc = " Expiration block number for each bond fund"]
				pub fn bond_fund_expirations(
					&self,
					_0: impl ::std::borrow::Borrow<types::bond_fund_expirations::Param0>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageKey<
						types::bond_fund_expirations::Param0,
					>,
					types::bond_fund_expirations::BondFundExpirations,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Bond",
						"BondFundExpirations",
						::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							22u8, 162u8, 77u8, 110u8, 141u8, 47u8, 65u8, 85u8, 143u8, 125u8, 17u8,
							130u8, 155u8, 207u8, 222u8, 161u8, 192u8, 51u8, 172u8, 40u8, 239u8,
							126u8, 0u8, 56u8, 130u8, 215u8, 137u8, 234u8, 176u8, 12u8, 98u8, 110u8,
						],
					)
				}
				#[doc = " Bonds by id"]
				pub fn bonds_iter(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::bonds::Bonds,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Bond",
						"Bonds",
						(),
						[
							44u8, 199u8, 6u8, 94u8, 193u8, 79u8, 161u8, 213u8, 219u8, 172u8, 86u8,
							10u8, 38u8, 27u8, 205u8, 190u8, 112u8, 215u8, 75u8, 30u8, 127u8, 147u8,
							74u8, 217u8, 101u8, 84u8, 16u8, 37u8, 46u8, 63u8, 99u8, 225u8,
						],
					)
				}
				#[doc = " Bonds by id"]
				pub fn bonds(
					&self,
					_0: impl ::std::borrow::Borrow<types::bonds::Param0>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageKey<types::bonds::Param0>,
					types::bonds::Bonds,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Bond",
						"Bonds",
						::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							44u8, 199u8, 6u8, 94u8, 193u8, 79u8, 161u8, 213u8, 219u8, 172u8, 86u8,
							10u8, 38u8, 27u8, 205u8, 190u8, 112u8, 215u8, 75u8, 30u8, 127u8, 147u8,
							74u8, 217u8, 101u8, 84u8, 16u8, 37u8, 46u8, 63u8, 99u8, 225u8,
						],
					)
				}
				#[doc = " Completion of each bond, upon which date funds are returned to the bond fund or self-bonder"]
				pub fn bond_completions_iter(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::bond_completions::BondCompletions,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Bond",
						"BondCompletions",
						(),
						[
							90u8, 38u8, 131u8, 102u8, 147u8, 157u8, 59u8, 220u8, 177u8, 233u8,
							31u8, 230u8, 22u8, 137u8, 226u8, 105u8, 41u8, 12u8, 91u8, 40u8, 109u8,
							230u8, 195u8, 27u8, 115u8, 217u8, 17u8, 209u8, 135u8, 113u8, 9u8,
							126u8,
						],
					)
				}
				#[doc = " Completion of each bond, upon which date funds are returned to the bond fund or self-bonder"]
				pub fn bond_completions(
					&self,
					_0: impl ::std::borrow::Borrow<types::bond_completions::Param0>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageKey<types::bond_completions::Param0>,
					types::bond_completions::BondCompletions,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Bond",
						"BondCompletions",
						::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							90u8, 38u8, 131u8, 102u8, 147u8, 157u8, 59u8, 220u8, 177u8, 233u8,
							31u8, 230u8, 22u8, 137u8, 226u8, 105u8, 41u8, 12u8, 91u8, 40u8, 109u8,
							230u8, 195u8, 27u8, 115u8, 217u8, 17u8, 209u8, 135u8, 113u8, 9u8,
							126u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " Minimum amount for a bond"]
				pub fn minimum_bond_amount(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"Bond",
						"MinimumBondAmount",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				#[doc = " Blocks per year used for APR calculations"]
				pub fn blocks_per_year(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Bond",
						"BlocksPerYear",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " Pallet storage requires bounds, so we have to set a maximum number that can expire in a"]
				#[doc = " single block"]
				pub fn max_concurrently_expiring_bond_funds(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Bond",
						"MaxConcurrentlyExpiringBondFunds",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " Pallet storage requires bounds, so we have to set a maximum number that can expire in a"]
				#[doc = " single block"]
				pub fn max_concurrently_expiring_bonds(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Bond",
						"MaxConcurrentlyExpiringBonds",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod notaries {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::pallet_notaries::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_notaries::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Propose {
					pub meta: propose::Meta,
				}
				pub mod propose {
					use super::runtime_types;
					pub type Meta = runtime_types::ulx_primitives::notary::NotaryMeta;
				}
				impl ::subxt::blocks::StaticExtrinsic for Propose {
					const PALLET: &'static str = "Notaries";
					const CALL: &'static str = "propose";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Activate {
					pub operator_account: activate::OperatorAccount,
				}
				pub mod activate {
					use super::runtime_types;
					pub type OperatorAccount = ::subxt::utils::AccountId32;
				}
				impl ::subxt::blocks::StaticExtrinsic for Activate {
					const PALLET: &'static str = "Notaries";
					const CALL: &'static str = "activate";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Update {
					#[codec(compact)]
					pub notary_id: update::NotaryId,
					pub meta: update::Meta,
				}
				pub mod update {
					use super::runtime_types;
					pub type NotaryId = ::core::primitive::u32;
					pub type Meta = runtime_types::ulx_primitives::notary::NotaryMeta;
				}
				impl ::subxt::blocks::StaticExtrinsic for Update {
					const PALLET: &'static str = "Notaries";
					const CALL: &'static str = "update";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn propose(
					&self,
					meta: types::propose::Meta,
				) -> ::subxt::tx::Payload<types::Propose> {
					::subxt::tx::Payload::new_static(
						"Notaries",
						"propose",
						types::Propose { meta },
						[
							44u8, 216u8, 215u8, 181u8, 41u8, 226u8, 208u8, 110u8, 193u8, 208u8,
							231u8, 174u8, 196u8, 194u8, 136u8, 170u8, 222u8, 73u8, 32u8, 199u8,
							57u8, 135u8, 86u8, 140u8, 199u8, 185u8, 84u8, 204u8, 69u8, 69u8, 197u8,
							115u8,
						],
					)
				}
				pub fn activate(
					&self,
					operator_account: types::activate::OperatorAccount,
				) -> ::subxt::tx::Payload<types::Activate> {
					::subxt::tx::Payload::new_static(
						"Notaries",
						"activate",
						types::Activate { operator_account },
						[
							135u8, 95u8, 212u8, 221u8, 190u8, 66u8, 114u8, 221u8, 200u8, 32u8,
							94u8, 119u8, 117u8, 207u8, 216u8, 78u8, 123u8, 114u8, 127u8, 0u8,
							209u8, 4u8, 39u8, 108u8, 188u8, 206u8, 192u8, 165u8, 17u8, 176u8, 0u8,
							203u8,
						],
					)
				}
				pub fn update(
					&self,
					notary_id: types::update::NotaryId,
					meta: types::update::Meta,
				) -> ::subxt::tx::Payload<types::Update> {
					::subxt::tx::Payload::new_static(
						"Notaries",
						"update",
						types::Update { notary_id, meta },
						[
							122u8, 85u8, 68u8, 206u8, 249u8, 188u8, 198u8, 137u8, 101u8, 103u8,
							235u8, 229u8, 92u8, 95u8, 74u8, 205u8, 235u8, 25u8, 69u8, 101u8, 52u8,
							159u8, 79u8, 70u8, 112u8, 84u8, 143u8, 104u8, 135u8, 0u8, 244u8, 98u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_notaries::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A user has proposed operating as a notary"]
			pub struct NotaryProposed {
				pub operator_account: notary_proposed::OperatorAccount,
				pub meta: notary_proposed::Meta,
				pub expires: notary_proposed::Expires,
			}
			pub mod notary_proposed {
				use super::runtime_types;
				pub type OperatorAccount = ::subxt::utils::AccountId32;
				pub type Meta = runtime_types::ulx_primitives::notary::NotaryMeta;
				pub type Expires = ::core::primitive::u32;
			}
			impl ::subxt::events::StaticEvent for NotaryProposed {
				const PALLET: &'static str = "Notaries";
				const EVENT: &'static str = "NotaryProposed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A notary proposal has been accepted"]
			pub struct NotaryActivated {
				pub notary: notary_activated::Notary,
			}
			pub mod notary_activated {
				use super::runtime_types;
				pub type Notary = runtime_types::ulx_primitives::notary::NotaryRecord<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>;
			}
			impl ::subxt::events::StaticEvent for NotaryActivated {
				const PALLET: &'static str = "Notaries";
				const EVENT: &'static str = "NotaryActivated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Notary metadata queued for update"]
			pub struct NotaryMetaUpdateQueued {
				pub notary_id: notary_meta_update_queued::NotaryId,
				pub meta: notary_meta_update_queued::Meta,
				pub effective_block: notary_meta_update_queued::EffectiveBlock,
			}
			pub mod notary_meta_update_queued {
				use super::runtime_types;
				pub type NotaryId = ::core::primitive::u32;
				pub type Meta = runtime_types::ulx_primitives::notary::NotaryMeta;
				pub type EffectiveBlock = ::core::primitive::u32;
			}
			impl ::subxt::events::StaticEvent for NotaryMetaUpdateQueued {
				const PALLET: &'static str = "Notaries";
				const EVENT: &'static str = "NotaryMetaUpdateQueued";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Notary metadata updated"]
			pub struct NotaryMetaUpdated {
				pub notary_id: notary_meta_updated::NotaryId,
				pub meta: notary_meta_updated::Meta,
			}
			pub mod notary_meta_updated {
				use super::runtime_types;
				pub type NotaryId = ::core::primitive::u32;
				pub type Meta = runtime_types::ulx_primitives::notary::NotaryMeta;
			}
			impl ::subxt::events::StaticEvent for NotaryMetaUpdated {
				const PALLET: &'static str = "Notaries";
				const EVENT: &'static str = "NotaryMetaUpdated";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub mod types {
				use super::runtime_types;
				pub mod next_notary_id {
					use super::runtime_types;
					pub type NextNotaryId = ::core::primitive::u32;
				}
				pub mod proposed_notaries {
					use super::runtime_types;
					pub type ProposedNotaries =
						(runtime_types::ulx_primitives::notary::NotaryMeta, ::core::primitive::u32);
					pub type Param0 = ::subxt::utils::AccountId32;
				}
				pub mod expiring_proposals {
					use super::runtime_types;
					pub type ExpiringProposals =
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::subxt::utils::AccountId32,
						>;
					pub type Param0 = ::core::primitive::u32;
				}
				pub mod active_notaries {
					use super::runtime_types;
					pub type ActiveNotaries =
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							runtime_types::ulx_primitives::notary::NotaryRecord<
								::subxt::utils::AccountId32,
								::core::primitive::u32,
							>,
						>;
				}
				pub mod notary_key_history {
					use super::runtime_types;
					pub type NotaryKeyHistory =
						runtime_types::bounded_collections::bounded_vec::BoundedVec<(
							::core::primitive::u32,
							[::core::primitive::u8; 32usize],
						)>;
					pub type Param0 = ::core::primitive::u32;
				}
				pub mod queued_notary_meta_changes {
					use super::runtime_types;
					pub type QueuedNotaryMetaChanges =
						runtime_types::bounded_collections::bounded_btree_map::BoundedBTreeMap<
							::core::primitive::u32,
							runtime_types::ulx_primitives::notary::NotaryMeta,
						>;
					pub type Param0 = ::core::primitive::u32;
				}
			}
			pub struct StorageApi;
			impl StorageApi {
				pub fn next_notary_id(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::next_notary_id::NextNotaryId,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Notaries",
						"NextNotaryId",
						(),
						[
							246u8, 48u8, 149u8, 160u8, 181u8, 5u8, 135u8, 44u8, 164u8, 37u8, 82u8,
							255u8, 240u8, 24u8, 171u8, 176u8, 255u8, 52u8, 54u8, 210u8, 131u8,
							113u8, 102u8, 36u8, 241u8, 251u8, 53u8, 118u8, 13u8, 52u8, 230u8, 7u8,
						],
					)
				}
				pub fn proposed_notaries_iter(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::proposed_notaries::ProposedNotaries,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Notaries",
						"ProposedNotaries",
						(),
						[
							166u8, 255u8, 186u8, 215u8, 39u8, 135u8, 255u8, 9u8, 218u8, 186u8,
							151u8, 170u8, 253u8, 136u8, 125u8, 101u8, 238u8, 157u8, 189u8, 141u8,
							255u8, 95u8, 62u8, 136u8, 122u8, 241u8, 215u8, 59u8, 14u8, 145u8,
							115u8, 164u8,
						],
					)
				}
				pub fn proposed_notaries(
					&self,
					_0: impl ::std::borrow::Borrow<types::proposed_notaries::Param0>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageKey<types::proposed_notaries::Param0>,
					types::proposed_notaries::ProposedNotaries,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Notaries",
						"ProposedNotaries",
						::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							166u8, 255u8, 186u8, 215u8, 39u8, 135u8, 255u8, 9u8, 218u8, 186u8,
							151u8, 170u8, 253u8, 136u8, 125u8, 101u8, 238u8, 157u8, 189u8, 141u8,
							255u8, 95u8, 62u8, 136u8, 122u8, 241u8, 215u8, 59u8, 14u8, 145u8,
							115u8, 164u8,
						],
					)
				}
				pub fn expiring_proposals_iter(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::expiring_proposals::ExpiringProposals,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Notaries",
						"ExpiringProposals",
						(),
						[
							64u8, 68u8, 247u8, 229u8, 147u8, 217u8, 204u8, 231u8, 82u8, 104u8,
							212u8, 163u8, 195u8, 244u8, 63u8, 148u8, 181u8, 120u8, 176u8, 52u8,
							125u8, 39u8, 74u8, 241u8, 126u8, 83u8, 45u8, 96u8, 30u8, 29u8, 155u8,
							108u8,
						],
					)
				}
				pub fn expiring_proposals(
					&self,
					_0: impl ::std::borrow::Borrow<types::expiring_proposals::Param0>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageKey<types::expiring_proposals::Param0>,
					types::expiring_proposals::ExpiringProposals,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Notaries",
						"ExpiringProposals",
						::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							64u8, 68u8, 247u8, 229u8, 147u8, 217u8, 204u8, 231u8, 82u8, 104u8,
							212u8, 163u8, 195u8, 244u8, 63u8, 148u8, 181u8, 120u8, 176u8, 52u8,
							125u8, 39u8, 74u8, 241u8, 126u8, 83u8, 45u8, 96u8, 30u8, 29u8, 155u8,
							108u8,
						],
					)
				}
				pub fn active_notaries(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::active_notaries::ActiveNotaries,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Notaries",
						"ActiveNotaries",
						(),
						[
							104u8, 213u8, 247u8, 16u8, 20u8, 163u8, 47u8, 221u8, 84u8, 68u8, 80u8,
							208u8, 162u8, 121u8, 48u8, 117u8, 7u8, 59u8, 118u8, 120u8, 113u8, 0u8,
							57u8, 234u8, 151u8, 152u8, 21u8, 249u8, 56u8, 222u8, 87u8, 144u8,
						],
					)
				}
				pub fn notary_key_history_iter(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::notary_key_history::NotaryKeyHistory,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Notaries",
						"NotaryKeyHistory",
						(),
						[
							59u8, 52u8, 23u8, 225u8, 223u8, 28u8, 225u8, 2u8, 39u8, 170u8, 155u8,
							214u8, 55u8, 134u8, 180u8, 53u8, 230u8, 255u8, 30u8, 165u8, 102u8,
							81u8, 80u8, 26u8, 213u8, 207u8, 158u8, 183u8, 71u8, 77u8, 191u8, 123u8,
						],
					)
				}
				pub fn notary_key_history(
					&self,
					_0: impl ::std::borrow::Borrow<types::notary_key_history::Param0>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageKey<types::notary_key_history::Param0>,
					types::notary_key_history::NotaryKeyHistory,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Notaries",
						"NotaryKeyHistory",
						::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							59u8, 52u8, 23u8, 225u8, 223u8, 28u8, 225u8, 2u8, 39u8, 170u8, 155u8,
							214u8, 55u8, 134u8, 180u8, 53u8, 230u8, 255u8, 30u8, 165u8, 102u8,
							81u8, 80u8, 26u8, 213u8, 207u8, 158u8, 183u8, 71u8, 77u8, 191u8, 123u8,
						],
					)
				}
				pub fn queued_notary_meta_changes_iter(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::queued_notary_meta_changes::QueuedNotaryMetaChanges,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Notaries",
						"QueuedNotaryMetaChanges",
						(),
						[
							31u8, 5u8, 40u8, 5u8, 210u8, 34u8, 25u8, 238u8, 1u8, 128u8, 173u8,
							92u8, 209u8, 210u8, 102u8, 233u8, 174u8, 54u8, 5u8, 5u8, 186u8, 172u8,
							109u8, 5u8, 138u8, 87u8, 32u8, 58u8, 25u8, 89u8, 26u8, 82u8,
						],
					)
				}
				pub fn queued_notary_meta_changes(
					&self,
					_0: impl ::std::borrow::Borrow<types::queued_notary_meta_changes::Param0>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageKey<
						types::queued_notary_meta_changes::Param0,
					>,
					types::queued_notary_meta_changes::QueuedNotaryMetaChanges,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Notaries",
						"QueuedNotaryMetaChanges",
						::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							31u8, 5u8, 40u8, 5u8, 210u8, 34u8, 25u8, 238u8, 1u8, 128u8, 173u8,
							92u8, 209u8, 210u8, 102u8, 233u8, 174u8, 54u8, 5u8, 5u8, 186u8, 172u8,
							109u8, 5u8, 138u8, 87u8, 32u8, 58u8, 25u8, 89u8, 26u8, 82u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The maximum active notaries allowed"]
				pub fn max_active_notaries(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Notaries",
						"MaxActiveNotaries",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The maximum blocks a proposal can sit unapproved"]
				pub fn max_proposal_hold_blocks(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Notaries",
						"MaxProposalHoldBlocks",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn max_proposals_per_block(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Notaries",
						"MaxProposalsPerBlock",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " Number of blocks to delay changing a notaries' meta"]
				pub fn meta_changes_block_delay(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Notaries",
						"MetaChangesBlockDelay",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " Number of blocks to maintain key history for each notary"]
				#[doc = " NOTE: only pruned when new keys are added"]
				pub fn max_blocks_for_key_history(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Notaries",
						"MaxBlocksForKeyHistory",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " Maximum hosts a notary can supply"]
				pub fn max_notary_hosts(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Notaries",
						"MaxNotaryHosts",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod notebook {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::pallet_notebook::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_notebook::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Submit {
					pub notebooks: submit::Notebooks,
				}
				pub mod submit {
					use super::runtime_types;
					pub type Notebooks = ::std::vec::Vec<
						runtime_types::ulx_primitives::notebook::SignedNotebookHeader,
					>;
				}
				impl ::subxt::blocks::StaticExtrinsic for Submit {
					const PALLET: &'static str = "Notebook";
					const CALL: &'static str = "submit";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn submit(
					&self,
					notebooks: types::submit::Notebooks,
				) -> ::subxt::tx::Payload<types::Submit> {
					::subxt::tx::Payload::new_static(
						"Notebook",
						"submit",
						types::Submit { notebooks },
						[
							10u8, 16u8, 67u8, 117u8, 66u8, 129u8, 194u8, 16u8, 90u8, 92u8, 175u8,
							89u8, 57u8, 209u8, 1u8, 95u8, 186u8, 32u8, 5u8, 155u8, 105u8, 205u8,
							223u8, 127u8, 101u8, 158u8, 100u8, 39u8, 103u8, 167u8, 60u8, 233u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_notebook::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct NotebookSubmitted {
				pub notary_id: notebook_submitted::NotaryId,
				pub notebook_number: notebook_submitted::NotebookNumber,
			}
			pub mod notebook_submitted {
				use super::runtime_types;
				pub type NotaryId = ::core::primitive::u32;
				pub type NotebookNumber = ::core::primitive::u32;
			}
			impl ::subxt::events::StaticEvent for NotebookSubmitted {
				const PALLET: &'static str = "Notebook";
				const EVENT: &'static str = "NotebookSubmitted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct NotebookAuditFailure {
				pub notary_id: notebook_audit_failure::NotaryId,
				pub notebook_number: notebook_audit_failure::NotebookNumber,
				pub first_failure_reason: notebook_audit_failure::FirstFailureReason,
			}
			pub mod notebook_audit_failure {
				use super::runtime_types;
				pub type NotaryId = ::core::primitive::u32;
				pub type NotebookNumber = ::core::primitive::u32;
				pub type FirstFailureReason = runtime_types::ulx_notary_audit::error::VerifyError;
			}
			impl ::subxt::events::StaticEvent for NotebookAuditFailure {
				const PALLET: &'static str = "Notebook";
				const EVENT: &'static str = "NotebookAuditFailure";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub mod types {
				use super::runtime_types;
				pub mod notebook_changed_accounts_root_by_notary {
					use super::runtime_types;
					pub type NotebookChangedAccountsRootByNotary = ::subxt::utils::H256;
					pub type Param0 = ::core::primitive::u32;
					pub type Param1 = ::core::primitive::u32;
				}
				pub mod account_origin_last_changed_notebook_by_notary {
					use super::runtime_types;
					pub type AccountOriginLastChangedNotebookByNotary = ::core::primitive::u32;
					pub type Param0 = ::core::primitive::u32;
					pub type Param1 = runtime_types::ulx_primitives::balance_change::AccountOrigin;
				}
				pub mod last_notebook_details_by_notary {
					use super::runtime_types;
					pub type LastNotebookDetailsByNotary =
						runtime_types::bounded_collections::bounded_vec::BoundedVec<(
							runtime_types::ulx_primitives::notary::NotaryNotebookKeyDetails,
							::core::primitive::bool,
						)>;
					pub type Param0 = ::core::primitive::u32;
				}
				pub mod block_notebooks {
					use super::runtime_types;
					pub type BlockNotebooks =
						runtime_types::ulx_primitives::digests::NotebookDigest<
							runtime_types::ulx_notary_audit::error::VerifyError,
						>;
				}
				pub mod temp_notebook_digest {
					use super::runtime_types;
					pub type TempNotebookDigest =
						runtime_types::ulx_primitives::digests::NotebookDigest<
							runtime_types::ulx_notary_audit::error::VerifyError,
						>;
				}
				pub mod notaries_locked_for_failed_audit {
					use super::runtime_types;
					pub type NotariesLockedForFailedAudit = (
						::core::primitive::u32,
						::core::primitive::u32,
						runtime_types::ulx_notary_audit::error::VerifyError,
					);
					pub type Param0 = ::core::primitive::u32;
				}
			}
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Double storage map of notary id + notebook # to the change root"]				pub fn notebook_changed_accounts_root_by_notary_iter (& self ,) -> :: subxt :: storage :: address :: Address :: < () , types :: notebook_changed_accounts_root_by_notary :: NotebookChangedAccountsRootByNotary , () , () , :: subxt :: storage :: address :: Yes >{
					::subxt::storage::address::Address::new_static(
						"Notebook",
						"NotebookChangedAccountsRootByNotary",
						(),
						[
							84u8, 136u8, 124u8, 162u8, 187u8, 104u8, 116u8, 80u8, 119u8, 130u8,
							77u8, 8u8, 34u8, 154u8, 63u8, 59u8, 4u8, 169u8, 227u8, 231u8, 95u8,
							16u8, 2u8, 116u8, 193u8, 76u8, 174u8, 109u8, 254u8, 206u8, 159u8,
							109u8,
						],
					)
				}
				#[doc = " Double storage map of notary id + notebook # to the change root"]				pub fn notebook_changed_accounts_root_by_notary_iter1 (& self , _0 : impl :: std :: borrow :: Borrow < types :: notebook_changed_accounts_root_by_notary :: Param0 > ,) -> :: subxt :: storage :: address :: Address :: < :: subxt :: storage :: address :: StaticStorageKey < types :: notebook_changed_accounts_root_by_notary :: Param0 > , types :: notebook_changed_accounts_root_by_notary :: NotebookChangedAccountsRootByNotary , () , () , :: subxt :: storage :: address :: Yes >{
					::subxt::storage::address::Address::new_static(
						"Notebook",
						"NotebookChangedAccountsRootByNotary",
						::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							84u8, 136u8, 124u8, 162u8, 187u8, 104u8, 116u8, 80u8, 119u8, 130u8,
							77u8, 8u8, 34u8, 154u8, 63u8, 59u8, 4u8, 169u8, 227u8, 231u8, 95u8,
							16u8, 2u8, 116u8, 193u8, 76u8, 174u8, 109u8, 254u8, 206u8, 159u8,
							109u8,
						],
					)
				}
				#[doc = " Double storage map of notary id + notebook # to the change root"]				pub fn notebook_changed_accounts_root_by_notary (& self , _0 : impl :: std :: borrow :: Borrow < types :: notebook_changed_accounts_root_by_notary :: Param0 > , _1 : impl :: std :: borrow :: Borrow < types :: notebook_changed_accounts_root_by_notary :: Param1 > ,) -> :: subxt :: storage :: address :: Address :: < (:: subxt :: storage :: address :: StaticStorageKey < types :: notebook_changed_accounts_root_by_notary :: Param0 > , :: subxt :: storage :: address :: StaticStorageKey < types :: notebook_changed_accounts_root_by_notary :: Param1 > ,) , types :: notebook_changed_accounts_root_by_notary :: NotebookChangedAccountsRootByNotary , :: subxt :: storage :: address :: Yes , () , () >{
					::subxt::storage::address::Address::new_static(
						"Notebook",
						"NotebookChangedAccountsRootByNotary",
						(
							::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
							::subxt::storage::address::StaticStorageKey::new(_1.borrow()),
						),
						[
							84u8, 136u8, 124u8, 162u8, 187u8, 104u8, 116u8, 80u8, 119u8, 130u8,
							77u8, 8u8, 34u8, 154u8, 63u8, 59u8, 4u8, 169u8, 227u8, 231u8, 95u8,
							16u8, 2u8, 116u8, 193u8, 76u8, 174u8, 109u8, 254u8, 206u8, 159u8,
							109u8,
						],
					)
				}
				#[doc = " Storage map of account origin (notary_id, notebook, account_uid) to the last"]
				#[doc = " notebook containing this account in the changed accounts merkle root"]
				#[doc = " (NotebookChangedAccountsRootByNotary)"]				pub fn account_origin_last_changed_notebook_by_notary_iter (& self ,) -> :: subxt :: storage :: address :: Address :: < () , types :: account_origin_last_changed_notebook_by_notary :: AccountOriginLastChangedNotebookByNotary , () , () , :: subxt :: storage :: address :: Yes >{
					::subxt::storage::address::Address::new_static(
						"Notebook",
						"AccountOriginLastChangedNotebookByNotary",
						(),
						[
							233u8, 5u8, 227u8, 113u8, 187u8, 168u8, 114u8, 176u8, 38u8, 129u8,
							116u8, 70u8, 109u8, 153u8, 173u8, 216u8, 216u8, 105u8, 245u8, 249u8,
							164u8, 236u8, 233u8, 205u8, 156u8, 134u8, 105u8, 157u8, 196u8, 182u8,
							144u8, 213u8,
						],
					)
				}
				#[doc = " Storage map of account origin (notary_id, notebook, account_uid) to the last"]
				#[doc = " notebook containing this account in the changed accounts merkle root"]
				#[doc = " (NotebookChangedAccountsRootByNotary)"]				pub fn account_origin_last_changed_notebook_by_notary_iter1 (& self , _0 : impl :: std :: borrow :: Borrow < types :: account_origin_last_changed_notebook_by_notary :: Param0 > ,) -> :: subxt :: storage :: address :: Address :: < :: subxt :: storage :: address :: StaticStorageKey < types :: account_origin_last_changed_notebook_by_notary :: Param0 > , types :: account_origin_last_changed_notebook_by_notary :: AccountOriginLastChangedNotebookByNotary , () , () , :: subxt :: storage :: address :: Yes >{
					::subxt::storage::address::Address::new_static(
						"Notebook",
						"AccountOriginLastChangedNotebookByNotary",
						::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							233u8, 5u8, 227u8, 113u8, 187u8, 168u8, 114u8, 176u8, 38u8, 129u8,
							116u8, 70u8, 109u8, 153u8, 173u8, 216u8, 216u8, 105u8, 245u8, 249u8,
							164u8, 236u8, 233u8, 205u8, 156u8, 134u8, 105u8, 157u8, 196u8, 182u8,
							144u8, 213u8,
						],
					)
				}
				#[doc = " Storage map of account origin (notary_id, notebook, account_uid) to the last"]
				#[doc = " notebook containing this account in the changed accounts merkle root"]
				#[doc = " (NotebookChangedAccountsRootByNotary)"]				pub fn account_origin_last_changed_notebook_by_notary (& self , _0 : impl :: std :: borrow :: Borrow < types :: account_origin_last_changed_notebook_by_notary :: Param0 > , _1 : impl :: std :: borrow :: Borrow < types :: account_origin_last_changed_notebook_by_notary :: Param1 > ,) -> :: subxt :: storage :: address :: Address :: < (:: subxt :: storage :: address :: StaticStorageKey < types :: account_origin_last_changed_notebook_by_notary :: Param0 > , :: subxt :: storage :: address :: StaticStorageKey < types :: account_origin_last_changed_notebook_by_notary :: Param1 > ,) , types :: account_origin_last_changed_notebook_by_notary :: AccountOriginLastChangedNotebookByNotary , :: subxt :: storage :: address :: Yes , () , () >{
					::subxt::storage::address::Address::new_static(
						"Notebook",
						"AccountOriginLastChangedNotebookByNotary",
						(
							::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
							::subxt::storage::address::StaticStorageKey::new(_1.borrow()),
						),
						[
							233u8, 5u8, 227u8, 113u8, 187u8, 168u8, 114u8, 176u8, 38u8, 129u8,
							116u8, 70u8, 109u8, 153u8, 173u8, 216u8, 216u8, 105u8, 245u8, 249u8,
							164u8, 236u8, 233u8, 205u8, 156u8, 134u8, 105u8, 157u8, 196u8, 182u8,
							144u8, 213u8,
						],
					)
				}
				#[doc = " List of last few notebook details by notary. The bool is whether the notebook is eligible"]
				#[doc = " for votes (received at correct tick and audit passed)"]
				pub fn last_notebook_details_by_notary_iter(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::last_notebook_details_by_notary::LastNotebookDetailsByNotary,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Notebook",
						"LastNotebookDetailsByNotary",
						(),
						[
							64u8, 129u8, 238u8, 122u8, 17u8, 221u8, 69u8, 225u8, 72u8, 184u8,
							105u8, 250u8, 99u8, 151u8, 43u8, 252u8, 57u8, 109u8, 163u8, 1u8, 135u8,
							215u8, 78u8, 62u8, 248u8, 161u8, 207u8, 89u8, 136u8, 227u8, 59u8, 78u8,
						],
					)
				}
				#[doc = " List of last few notebook details by notary. The bool is whether the notebook is eligible"]
				#[doc = " for votes (received at correct tick and audit passed)"]
				pub fn last_notebook_details_by_notary(
					&self,
					_0: impl ::std::borrow::Borrow<types::last_notebook_details_by_notary::Param0>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageKey<
						types::last_notebook_details_by_notary::Param0,
					>,
					types::last_notebook_details_by_notary::LastNotebookDetailsByNotary,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Notebook",
						"LastNotebookDetailsByNotary",
						::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							64u8, 129u8, 238u8, 122u8, 17u8, 221u8, 69u8, 225u8, 72u8, 184u8,
							105u8, 250u8, 99u8, 151u8, 43u8, 252u8, 57u8, 109u8, 163u8, 1u8, 135u8,
							215u8, 78u8, 62u8, 248u8, 161u8, 207u8, 89u8, 136u8, 227u8, 59u8, 78u8,
						],
					)
				}
				#[doc = " The notebooks included in this block"]
				pub fn block_notebooks(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::block_notebooks::BlockNotebooks,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Notebook",
						"BlockNotebooks",
						(),
						[
							49u8, 228u8, 149u8, 219u8, 185u8, 33u8, 88u8, 126u8, 221u8, 3u8, 103u8,
							137u8, 211u8, 36u8, 191u8, 71u8, 47u8, 28u8, 37u8, 151u8, 132u8, 152u8,
							211u8, 110u8, 11u8, 164u8, 92u8, 199u8, 227u8, 148u8, 88u8, 101u8,
						],
					)
				}
				#[doc = " Temporary store a copy of the notebook digest in storage"]
				pub fn temp_notebook_digest(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::temp_notebook_digest::TempNotebookDigest,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Notebook",
						"TempNotebookDigest",
						(),
						[
							232u8, 193u8, 240u8, 248u8, 108u8, 120u8, 24u8, 147u8, 255u8, 99u8,
							49u8, 179u8, 58u8, 139u8, 162u8, 234u8, 127u8, 5u8, 70u8, 11u8, 250u8,
							162u8, 229u8, 3u8, 216u8, 29u8, 239u8, 104u8, 135u8, 169u8, 73u8,
							111u8,
						],
					)
				}
				#[doc = " Notaries locked for failing audits"]
				#[doc = " TODO: we need a mechanism to unlock a notary with \"Fixes\""]
				pub fn notaries_locked_for_failed_audit_iter(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::notaries_locked_for_failed_audit::NotariesLockedForFailedAudit,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Notebook",
						"NotariesLockedForFailedAudit",
						(),
						[
							90u8, 59u8, 51u8, 110u8, 190u8, 10u8, 201u8, 252u8, 144u8, 248u8,
							136u8, 115u8, 219u8, 69u8, 32u8, 210u8, 127u8, 135u8, 168u8, 180u8,
							229u8, 2u8, 181u8, 228u8, 22u8, 155u8, 66u8, 218u8, 215u8, 111u8,
							164u8, 224u8,
						],
					)
				}
				#[doc = " Notaries locked for failing audits"]
				#[doc = " TODO: we need a mechanism to unlock a notary with \"Fixes\""]
				pub fn notaries_locked_for_failed_audit(
					&self,
					_0: impl ::std::borrow::Borrow<types::notaries_locked_for_failed_audit::Param0>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageKey<
						types::notaries_locked_for_failed_audit::Param0,
					>,
					types::notaries_locked_for_failed_audit::NotariesLockedForFailedAudit,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Notebook",
						"NotariesLockedForFailedAudit",
						::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							90u8, 59u8, 51u8, 110u8, 190u8, 10u8, 201u8, 252u8, 144u8, 248u8,
							136u8, 115u8, 219u8, 69u8, 32u8, 210u8, 127u8, 135u8, 168u8, 180u8,
							229u8, 2u8, 181u8, 228u8, 22u8, 155u8, 66u8, 218u8, 215u8, 111u8,
							164u8, 224u8,
						],
					)
				}
			}
		}
	}
	pub mod chain_transfer {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::pallet_chain_transfer::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_chain_transfer::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SendToLocalchain {
					#[codec(compact)]
					pub amount: send_to_localchain::Amount,
					pub notary_id: send_to_localchain::NotaryId,
				}
				pub mod send_to_localchain {
					use super::runtime_types;
					pub type Amount = ::core::primitive::u128;
					pub type NotaryId = ::core::primitive::u32;
				}
				impl ::subxt::blocks::StaticExtrinsic for SendToLocalchain {
					const PALLET: &'static str = "ChainTransfer";
					const CALL: &'static str = "send_to_localchain";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn send_to_localchain(
					&self,
					amount: types::send_to_localchain::Amount,
					notary_id: types::send_to_localchain::NotaryId,
				) -> ::subxt::tx::Payload<types::SendToLocalchain> {
					::subxt::tx::Payload::new_static(
						"ChainTransfer",
						"send_to_localchain",
						types::SendToLocalchain { amount, notary_id },
						[
							83u8, 216u8, 66u8, 149u8, 234u8, 85u8, 61u8, 45u8, 152u8, 156u8, 153u8,
							118u8, 179u8, 201u8, 255u8, 21u8, 5u8, 117u8, 53u8, 241u8, 173u8, 66u8,
							32u8, 8u8, 26u8, 176u8, 221u8, 245u8, 212u8, 13u8, 86u8, 171u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_chain_transfer::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct TransferToLocalchain {
				pub account_id: transfer_to_localchain::AccountId,
				pub amount: transfer_to_localchain::Amount,
				pub transfer_id: transfer_to_localchain::TransferId,
				pub notary_id: transfer_to_localchain::NotaryId,
				pub expiration_block: transfer_to_localchain::ExpirationBlock,
			}
			pub mod transfer_to_localchain {
				use super::runtime_types;
				pub type AccountId = ::subxt::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
				pub type TransferId = ::core::primitive::u32;
				pub type NotaryId = ::core::primitive::u32;
				pub type ExpirationBlock = ::core::primitive::u32;
			}
			impl ::subxt::events::StaticEvent for TransferToLocalchain {
				const PALLET: &'static str = "ChainTransfer";
				const EVENT: &'static str = "TransferToLocalchain";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct TransferToLocalchainExpired {
				pub account_id: transfer_to_localchain_expired::AccountId,
				pub transfer_id: transfer_to_localchain_expired::TransferId,
				pub notary_id: transfer_to_localchain_expired::NotaryId,
			}
			pub mod transfer_to_localchain_expired {
				use super::runtime_types;
				pub type AccountId = ::subxt::utils::AccountId32;
				pub type TransferId = ::core::primitive::u32;
				pub type NotaryId = ::core::primitive::u32;
			}
			impl ::subxt::events::StaticEvent for TransferToLocalchainExpired {
				const PALLET: &'static str = "ChainTransfer";
				const EVENT: &'static str = "TransferToLocalchainExpired";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct TransferIn {
				pub account_id: transfer_in::AccountId,
				pub amount: transfer_in::Amount,
				pub notary_id: transfer_in::NotaryId,
			}
			pub mod transfer_in {
				use super::runtime_types;
				pub type AccountId = ::subxt::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
				pub type NotaryId = ::core::primitive::u32;
			}
			impl ::subxt::events::StaticEvent for TransferIn {
				const PALLET: &'static str = "ChainTransfer";
				const EVENT: &'static str = "TransferIn";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub mod types {
				use super::runtime_types;
				pub mod next_transfer_id {
					use super::runtime_types;
					pub type NextTransferId = ::core::primitive::u32;
				}
				pub mod pending_transfers_out {
					use super::runtime_types;
					pub type PendingTransfersOut =
						runtime_types::pallet_chain_transfer::QueuedTransferOut<
							::subxt::utils::AccountId32,
							::core::primitive::u128,
							::core::primitive::u32,
						>;
					pub type Param0 = ::core::primitive::u32;
				}
				pub mod expiring_transfers_out {
					use super::runtime_types;
					pub type ExpiringTransfersOut =
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u32,
						>;
					pub type Param0 = ::core::primitive::u32;
				}
				pub mod transfers_used_in_block_notebooks {
					use super::runtime_types;
					pub type TransfersUsedInBlockNotebooks =
						runtime_types::bounded_collections::bounded_vec::BoundedVec<(
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						)>;
					pub type Param0 = ::core::primitive::u32;
				}
			}
			pub struct StorageApi;
			impl StorageApi {
				pub fn next_transfer_id(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::next_transfer_id::NextTransferId,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ChainTransfer",
						"NextTransferId",
						(),
						[
							24u8, 254u8, 76u8, 107u8, 53u8, 239u8, 9u8, 199u8, 247u8, 44u8, 22u8,
							150u8, 46u8, 130u8, 241u8, 50u8, 36u8, 76u8, 133u8, 78u8, 69u8, 43u8,
							94u8, 241u8, 60u8, 247u8, 91u8, 71u8, 248u8, 43u8, 217u8, 31u8,
						],
					)
				}
				pub fn pending_transfers_out_iter(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::pending_transfers_out::PendingTransfersOut,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"ChainTransfer",
						"PendingTransfersOut",
						(),
						[
							223u8, 189u8, 59u8, 156u8, 136u8, 151u8, 67u8, 225u8, 88u8, 60u8,
							232u8, 104u8, 79u8, 111u8, 193u8, 250u8, 174u8, 81u8, 143u8, 242u8,
							36u8, 44u8, 229u8, 26u8, 9u8, 216u8, 94u8, 175u8, 246u8, 239u8, 233u8,
							61u8,
						],
					)
				}
				pub fn pending_transfers_out(
					&self,
					_0: impl ::std::borrow::Borrow<types::pending_transfers_out::Param0>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageKey<
						types::pending_transfers_out::Param0,
					>,
					types::pending_transfers_out::PendingTransfersOut,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ChainTransfer",
						"PendingTransfersOut",
						::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							223u8, 189u8, 59u8, 156u8, 136u8, 151u8, 67u8, 225u8, 88u8, 60u8,
							232u8, 104u8, 79u8, 111u8, 193u8, 250u8, 174u8, 81u8, 143u8, 242u8,
							36u8, 44u8, 229u8, 26u8, 9u8, 216u8, 94u8, 175u8, 246u8, 239u8, 233u8,
							61u8,
						],
					)
				}
				pub fn expiring_transfers_out_iter(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::expiring_transfers_out::ExpiringTransfersOut,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"ChainTransfer",
						"ExpiringTransfersOut",
						(),
						[
							161u8, 143u8, 145u8, 104u8, 64u8, 15u8, 148u8, 90u8, 103u8, 166u8,
							253u8, 126u8, 219u8, 219u8, 39u8, 75u8, 19u8, 60u8, 73u8, 41u8, 88u8,
							198u8, 7u8, 23u8, 156u8, 239u8, 49u8, 245u8, 173u8, 198u8, 57u8, 223u8,
						],
					)
				}
				pub fn expiring_transfers_out(
					&self,
					_0: impl ::std::borrow::Borrow<types::expiring_transfers_out::Param0>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageKey<
						types::expiring_transfers_out::Param0,
					>,
					types::expiring_transfers_out::ExpiringTransfersOut,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ChainTransfer",
						"ExpiringTransfersOut",
						::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							161u8, 143u8, 145u8, 104u8, 64u8, 15u8, 148u8, 90u8, 103u8, 166u8,
							253u8, 126u8, 219u8, 219u8, 39u8, 75u8, 19u8, 60u8, 73u8, 41u8, 88u8,
							198u8, 7u8, 23u8, 156u8, 239u8, 49u8, 245u8, 173u8, 198u8, 57u8, 223u8,
						],
					)
				}
				pub fn transfers_used_in_block_notebooks_iter(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::transfers_used_in_block_notebooks::TransfersUsedInBlockNotebooks,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"ChainTransfer",
						"TransfersUsedInBlockNotebooks",
						(),
						[
							56u8, 113u8, 70u8, 50u8, 20u8, 191u8, 57u8, 47u8, 98u8, 209u8, 251u8,
							146u8, 233u8, 41u8, 193u8, 196u8, 198u8, 195u8, 231u8, 184u8, 49u8,
							3u8, 16u8, 180u8, 218u8, 7u8, 51u8, 90u8, 220u8, 111u8, 153u8, 219u8,
						],
					)
				}
				pub fn transfers_used_in_block_notebooks(
					&self,
					_0: impl ::std::borrow::Borrow<types::transfers_used_in_block_notebooks::Param0>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageKey<
						types::transfers_used_in_block_notebooks::Param0,
					>,
					types::transfers_used_in_block_notebooks::TransfersUsedInBlockNotebooks,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ChainTransfer",
						"TransfersUsedInBlockNotebooks",
						::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							56u8, 113u8, 70u8, 50u8, 20u8, 191u8, 57u8, 47u8, 98u8, 209u8, 251u8,
							146u8, 233u8, 41u8, 193u8, 196u8, 198u8, 195u8, 231u8, 184u8, 49u8,
							3u8, 16u8, 180u8, 218u8, 7u8, 51u8, 90u8, 220u8, 111u8, 153u8, 219u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				pub fn pallet_id(
					&self,
				) -> ::subxt::constants::Address<runtime_types::frame_support::PalletId> {
					::subxt::constants::Address::new_static(
						"ChainTransfer",
						"PalletId",
						[
							56u8, 243u8, 53u8, 83u8, 154u8, 179u8, 170u8, 80u8, 133u8, 173u8, 61u8,
							161u8, 47u8, 225u8, 146u8, 21u8, 50u8, 229u8, 248u8, 27u8, 104u8, 58u8,
							129u8, 197u8, 102u8, 160u8, 168u8, 205u8, 154u8, 42u8, 217u8, 53u8,
						],
					)
				}
				#[doc = " How long a transfer should remain in storage before returning."]
				pub fn transfer_expiration_blocks(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"ChainTransfer",
						"TransferExpirationBlocks",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " How many transfers out can be queued per block"]
				pub fn max_pending_transfers_out_per_block(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"ChainTransfer",
						"MaxPendingTransfersOutPerBlock",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod block_seal_spec {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::pallet_block_seal_spec::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_block_seal_spec::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Configure {
					pub vote_minimum: configure::VoteMinimum,
					pub compute_difficulty: configure::ComputeDifficulty,
				}
				pub mod configure {
					use super::runtime_types;
					pub type VoteMinimum = ::core::option::Option<::core::primitive::u128>;
					pub type ComputeDifficulty = ::core::option::Option<::core::primitive::u128>;
				}
				impl ::subxt::blocks::StaticExtrinsic for Configure {
					const PALLET: &'static str = "BlockSealSpec";
					const CALL: &'static str = "configure";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn configure(
					&self,
					vote_minimum: types::configure::VoteMinimum,
					compute_difficulty: types::configure::ComputeDifficulty,
				) -> ::subxt::tx::Payload<types::Configure> {
					::subxt::tx::Payload::new_static(
						"BlockSealSpec",
						"configure",
						types::Configure { vote_minimum, compute_difficulty },
						[
							211u8, 110u8, 104u8, 239u8, 141u8, 253u8, 92u8, 219u8, 37u8, 63u8,
							59u8, 202u8, 96u8, 3u8, 132u8, 207u8, 219u8, 42u8, 253u8, 70u8, 152u8,
							29u8, 72u8, 104u8, 182u8, 58u8, 23u8, 133u8, 41u8, 223u8, 62u8, 28u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_block_seal_spec::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct VoteMinimumAdjusted {
				pub expected_block_votes: vote_minimum_adjusted::ExpectedBlockVotes,
				pub actual_block_votes: vote_minimum_adjusted::ActualBlockVotes,
				pub start_vote_minimum: vote_minimum_adjusted::StartVoteMinimum,
				pub new_vote_minimum: vote_minimum_adjusted::NewVoteMinimum,
			}
			pub mod vote_minimum_adjusted {
				use super::runtime_types;
				pub type ExpectedBlockVotes = ::core::primitive::u128;
				pub type ActualBlockVotes = ::core::primitive::u128;
				pub type StartVoteMinimum = ::core::primitive::u128;
				pub type NewVoteMinimum = ::core::primitive::u128;
			}
			impl ::subxt::events::StaticEvent for VoteMinimumAdjusted {
				const PALLET: &'static str = "BlockSealSpec";
				const EVENT: &'static str = "VoteMinimumAdjusted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ComputeDifficultyAdjusted {
				pub expected_block_time: compute_difficulty_adjusted::ExpectedBlockTime,
				pub actual_block_time: compute_difficulty_adjusted::ActualBlockTime,
				pub start_difficulty: compute_difficulty_adjusted::StartDifficulty,
				pub new_difficulty: compute_difficulty_adjusted::NewDifficulty,
			}
			pub mod compute_difficulty_adjusted {
				use super::runtime_types;
				pub type ExpectedBlockTime = ::core::primitive::u64;
				pub type ActualBlockTime = ::core::primitive::u64;
				pub type StartDifficulty = ::core::primitive::u128;
				pub type NewDifficulty = ::core::primitive::u128;
			}
			impl ::subxt::events::StaticEvent for ComputeDifficultyAdjusted {
				const PALLET: &'static str = "BlockSealSpec";
				const EVENT: &'static str = "ComputeDifficultyAdjusted";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub mod types {
				use super::runtime_types;
				pub mod current_vote_minimum {
					use super::runtime_types;
					pub type CurrentVoteMinimum = ::core::primitive::u128;
				}
				pub mod current_compute_difficulty {
					use super::runtime_types;
					pub type CurrentComputeDifficulty = ::core::primitive::u128;
				}
				pub mod past_compute_block_times {
					use super::runtime_types;
					pub type PastComputeBlockTimes =
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u64,
						>;
				}
				pub mod previous_block_timestamp {
					use super::runtime_types;
					pub type PreviousBlockTimestamp = ::core::primitive::u64;
				}
				pub mod temp_block_timestamp {
					use super::runtime_types;
					pub type TempBlockTimestamp = ::core::primitive::u64;
				}
				pub mod vote_minimum_history {
					use super::runtime_types;
					pub type VoteMinimumHistory =
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u128,
						>;
				}
				pub mod temp_current_tick_notebooks_in_block {
					use super::runtime_types;
					pub type TempCurrentTickNotebooksInBlock =
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							runtime_types::ulx_primitives::notary::NotaryNotebookVoteDigestDetails,
						>;
				}
				pub mod temp_block_vote_digest {
					use super::runtime_types;
					pub type TempBlockVoteDigest =
						runtime_types::ulx_primitives::digests::BlockVoteDigest;
				}
				pub mod past_block_votes {
					use super::runtime_types;
					pub type PastBlockVotes =
						runtime_types::bounded_collections::bounded_vec::BoundedVec<(
							::core::primitive::u32,
							::core::primitive::u32,
							::core::primitive::u128,
						)>;
				}
			}
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The current vote minimum of the chain. Block votes use this minimum to determine the"]
				#[doc = " minimum amount of tax or compute needed to create a vote. It is adjusted up or down to"]
				#[doc = " target a max number of votes"]
				pub fn current_vote_minimum(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::current_vote_minimum::CurrentVoteMinimum,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"BlockSealSpec",
						"CurrentVoteMinimum",
						(),
						[
							12u8, 224u8, 174u8, 92u8, 253u8, 174u8, 51u8, 35u8, 165u8, 155u8,
							173u8, 118u8, 154u8, 150u8, 251u8, 57u8, 233u8, 6u8, 228u8, 92u8,
							186u8, 127u8, 187u8, 158u8, 160u8, 60u8, 117u8, 155u8, 93u8, 1u8,
							160u8, 27u8,
						],
					)
				}
				#[doc = " The current vote minimum of the chain. Block votes use this minimum to determine the"]
				#[doc = " minimum amount of tax or compute needed to create a vote. It is adjusted up or down to"]
				#[doc = " target a max number of votes"]
				pub fn current_compute_difficulty(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::current_compute_difficulty::CurrentComputeDifficulty,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"BlockSealSpec",
						"CurrentComputeDifficulty",
						(),
						[
							65u8, 189u8, 189u8, 218u8, 13u8, 81u8, 240u8, 153u8, 77u8, 3u8, 71u8,
							26u8, 76u8, 244u8, 180u8, 15u8, 215u8, 66u8, 20u8, 70u8, 23u8, 133u8,
							136u8, 235u8, 193u8, 90u8, 222u8, 97u8, 139u8, 166u8, 94u8, 0u8,
						],
					)
				}
				pub fn past_compute_block_times(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::past_compute_block_times::PastComputeBlockTimes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"BlockSealSpec",
						"PastComputeBlockTimes",
						(),
						[
							210u8, 23u8, 204u8, 23u8, 189u8, 60u8, 128u8, 197u8, 199u8, 181u8,
							117u8, 11u8, 70u8, 235u8, 69u8, 110u8, 215u8, 114u8, 95u8, 198u8,
							131u8, 156u8, 166u8, 24u8, 128u8, 145u8, 205u8, 220u8, 107u8, 28u8,
							134u8, 72u8,
						],
					)
				}
				pub fn previous_block_timestamp(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::previous_block_timestamp::PreviousBlockTimestamp,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"BlockSealSpec",
						"PreviousBlockTimestamp",
						(),
						[
							47u8, 107u8, 36u8, 17u8, 213u8, 89u8, 21u8, 145u8, 60u8, 224u8, 86u8,
							101u8, 51u8, 209u8, 56u8, 127u8, 186u8, 80u8, 27u8, 41u8, 23u8, 207u8,
							198u8, 42u8, 58u8, 103u8, 8u8, 226u8, 82u8, 191u8, 89u8, 193u8,
						],
					)
				}
				pub fn temp_block_timestamp(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::temp_block_timestamp::TempBlockTimestamp,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"BlockSealSpec",
						"TempBlockTimestamp",
						(),
						[
							167u8, 201u8, 179u8, 72u8, 25u8, 20u8, 159u8, 162u8, 18u8, 154u8,
							169u8, 53u8, 137u8, 227u8, 96u8, 187u8, 3u8, 133u8, 155u8, 31u8, 92u8,
							145u8, 254u8, 239u8, 86u8, 215u8, 65u8, 223u8, 91u8, 120u8, 79u8, 34u8,
						],
					)
				}
				#[doc = " Keeps the last 3 vote minimums. The first one applies to the current block."]
				pub fn vote_minimum_history(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::vote_minimum_history::VoteMinimumHistory,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"BlockSealSpec",
						"VoteMinimumHistory",
						(),
						[
							197u8, 183u8, 228u8, 59u8, 233u8, 183u8, 83u8, 132u8, 64u8, 76u8,
							112u8, 118u8, 156u8, 127u8, 114u8, 2u8, 189u8, 14u8, 255u8, 83u8,
							185u8, 11u8, 100u8, 71u8, 52u8, 7u8, 102u8, 205u8, 208u8, 103u8, 12u8,
							206u8,
						],
					)
				}
				#[doc = " Temporary store of any current tick notebooks included in this block (vs tick)"]
				pub fn temp_current_tick_notebooks_in_block(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::temp_current_tick_notebooks_in_block::TempCurrentTickNotebooksInBlock,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"BlockSealSpec",
						"TempCurrentTickNotebooksInBlock",
						(),
						[
							44u8, 17u8, 131u8, 64u8, 117u8, 10u8, 84u8, 129u8, 184u8, 227u8, 180u8,
							61u8, 162u8, 160u8, 189u8, 249u8, 202u8, 103u8, 51u8, 254u8, 97u8,
							218u8, 234u8, 192u8, 64u8, 146u8, 10u8, 174u8, 101u8, 110u8, 234u8,
							142u8,
						],
					)
				}
				#[doc = " Temporary store the vote digest"]
				pub fn temp_block_vote_digest(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::temp_block_vote_digest::TempBlockVoteDigest,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"BlockSealSpec",
						"TempBlockVoteDigest",
						(),
						[
							148u8, 195u8, 42u8, 184u8, 217u8, 89u8, 137u8, 65u8, 230u8, 188u8,
							58u8, 194u8, 7u8, 147u8, 16u8, 160u8, 78u8, 186u8, 242u8, 66u8, 159u8,
							43u8, 61u8, 250u8, 181u8, 78u8, 188u8, 53u8, 159u8, 38u8, 210u8, 38u8,
						],
					)
				}
				pub fn past_block_votes(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::past_block_votes::PastBlockVotes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"BlockSealSpec",
						"PastBlockVotes",
						(),
						[
							96u8, 31u8, 172u8, 50u8, 227u8, 32u8, 171u8, 95u8, 14u8, 206u8, 31u8,
							192u8, 30u8, 75u8, 199u8, 111u8, 243u8, 142u8, 194u8, 59u8, 101u8, 4u8,
							207u8, 52u8, 6u8, 131u8, 130u8, 83u8, 227u8, 80u8, 149u8, 168u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The maximum active notaries allowed"]
				pub fn max_active_notaries(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"BlockSealSpec",
						"MaxActiveNotaries",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The desired votes per block"]
				pub fn target_block_votes(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"BlockSealSpec",
						"TargetBlockVotes",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				#[doc = " The frequency for changing the minimum"]
				pub fn change_period(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"BlockSealSpec",
						"ChangePeriod",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod data_domain {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::pallet_data_domain::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_data_domain::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetZoneRecord {
					pub domain_hash: set_zone_record::DomainHash,
					pub zone_record: set_zone_record::ZoneRecord,
				}
				pub mod set_zone_record {
					use super::runtime_types;
					pub type DomainHash = ::subxt::utils::H256;
					pub type ZoneRecord = runtime_types::ulx_primitives::data_domain::ZoneRecord<
						::subxt::utils::AccountId32,
					>;
				}
				impl ::subxt::blocks::StaticExtrinsic for SetZoneRecord {
					const PALLET: &'static str = "DataDomain";
					const CALL: &'static str = "set_zone_record";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn set_zone_record(
					&self,
					domain_hash: types::set_zone_record::DomainHash,
					zone_record: types::set_zone_record::ZoneRecord,
				) -> ::subxt::tx::Payload<types::SetZoneRecord> {
					::subxt::tx::Payload::new_static(
						"DataDomain",
						"set_zone_record",
						types::SetZoneRecord { domain_hash, zone_record },
						[
							50u8, 248u8, 228u8, 101u8, 86u8, 211u8, 24u8, 101u8, 200u8, 6u8, 34u8,
							171u8, 25u8, 68u8, 62u8, 191u8, 115u8, 156u8, 137u8, 190u8, 64u8,
							140u8, 151u8, 39u8, 251u8, 54u8, 169u8, 124u8, 49u8, 255u8, 131u8,
							81u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_data_domain::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A data domain zone record was updated"]
			pub struct ZoneRecordUpdated {
				pub domain_hash: zone_record_updated::DomainHash,
				pub zone_record: zone_record_updated::ZoneRecord,
			}
			pub mod zone_record_updated {
				use super::runtime_types;
				pub type DomainHash = ::subxt::utils::H256;
				pub type ZoneRecord = runtime_types::ulx_primitives::data_domain::ZoneRecord<
					::subxt::utils::AccountId32,
				>;
			}
			impl ::subxt::events::StaticEvent for ZoneRecordUpdated {
				const PALLET: &'static str = "DataDomain";
				const EVENT: &'static str = "ZoneRecordUpdated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A data domain was registered"]
			pub struct DataDomainRegistered {
				pub domain_hash: data_domain_registered::DomainHash,
				pub registration: data_domain_registered::Registration,
			}
			pub mod data_domain_registered {
				use super::runtime_types;
				pub type DomainHash = ::subxt::utils::H256;
				pub type Registration = runtime_types::pallet_data_domain::DataDomainRegistration<
					::subxt::utils::AccountId32,
				>;
			}
			impl ::subxt::events::StaticEvent for DataDomainRegistered {
				const PALLET: &'static str = "DataDomain";
				const EVENT: &'static str = "DataDomainRegistered";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A data domain was registered"]
			pub struct DataDomainRenewed {
				pub domain_hash: data_domain_renewed::DomainHash,
			}
			pub mod data_domain_renewed {
				use super::runtime_types;
				pub type DomainHash = ::subxt::utils::H256;
			}
			impl ::subxt::events::StaticEvent for DataDomainRenewed {
				const PALLET: &'static str = "DataDomain";
				const EVENT: &'static str = "DataDomainRenewed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A data domain was expired"]
			pub struct DataDomainExpired {
				pub domain_hash: data_domain_expired::DomainHash,
			}
			pub mod data_domain_expired {
				use super::runtime_types;
				pub type DomainHash = ::subxt::utils::H256;
			}
			impl ::subxt::events::StaticEvent for DataDomainExpired {
				const PALLET: &'static str = "DataDomain";
				const EVENT: &'static str = "DataDomainExpired";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A data domain registration was canceled due to a conflicting registration in the same"]
			#[doc = "tick"]
			pub struct DataDomainRegistrationCanceled {
				pub domain_hash: data_domain_registration_canceled::DomainHash,
				pub registration: data_domain_registration_canceled::Registration,
			}
			pub mod data_domain_registration_canceled {
				use super::runtime_types;
				pub type DomainHash = ::subxt::utils::H256;
				pub type Registration = runtime_types::pallet_data_domain::DataDomainRegistration<
					::subxt::utils::AccountId32,
				>;
			}
			impl ::subxt::events::StaticEvent for DataDomainRegistrationCanceled {
				const PALLET: &'static str = "DataDomain";
				const EVENT: &'static str = "DataDomainRegistrationCanceled";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub mod types {
				use super::runtime_types;
				pub mod registered_data_domains {
					use super::runtime_types;
					pub type RegisteredDataDomains =
						runtime_types::pallet_data_domain::DataDomainRegistration<
							::subxt::utils::AccountId32,
						>;
					pub type Param0 = ::subxt::utils::H256;
				}
				pub mod zone_records_by_domain {
					use super::runtime_types;
					pub type ZoneRecordsByDomain =
						runtime_types::ulx_primitives::data_domain::ZoneRecord<
							::subxt::utils::AccountId32,
						>;
					pub type Param0 = ::subxt::utils::H256;
				}
				pub mod domain_payment_address_history {
					use super::runtime_types;
					pub type DomainPaymentAddressHistory =
						runtime_types::bounded_collections::bounded_vec::BoundedVec<(
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						)>;
					pub type Param0 = ::subxt::utils::H256;
				}
				pub mod expiring_domains_by_block {
					use super::runtime_types;
					pub type ExpiringDomainsByBlock =
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::subxt::utils::H256,
						>;
					pub type Param0 = ::core::primitive::u32;
				}
			}
			pub struct StorageApi;
			impl StorageApi {
				pub fn registered_data_domains_iter(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::registered_data_domains::RegisteredDataDomains,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"DataDomain",
						"RegisteredDataDomains",
						(),
						[
							170u8, 223u8, 96u8, 217u8, 109u8, 173u8, 60u8, 75u8, 121u8, 255u8,
							183u8, 225u8, 40u8, 141u8, 154u8, 7u8, 236u8, 133u8, 75u8, 191u8,
							127u8, 226u8, 48u8, 207u8, 31u8, 90u8, 194u8, 75u8, 100u8, 165u8,
							146u8, 28u8,
						],
					)
				}
				pub fn registered_data_domains(
					&self,
					_0: impl ::std::borrow::Borrow<types::registered_data_domains::Param0>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageKey<
						types::registered_data_domains::Param0,
					>,
					types::registered_data_domains::RegisteredDataDomains,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"DataDomain",
						"RegisteredDataDomains",
						::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							170u8, 223u8, 96u8, 217u8, 109u8, 173u8, 60u8, 75u8, 121u8, 255u8,
							183u8, 225u8, 40u8, 141u8, 154u8, 7u8, 236u8, 133u8, 75u8, 191u8,
							127u8, 226u8, 48u8, 207u8, 31u8, 90u8, 194u8, 75u8, 100u8, 165u8,
							146u8, 28u8,
						],
					)
				}
				pub fn zone_records_by_domain_iter(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::zone_records_by_domain::ZoneRecordsByDomain,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"DataDomain",
						"ZoneRecordsByDomain",
						(),
						[
							89u8, 9u8, 74u8, 149u8, 114u8, 123u8, 168u8, 120u8, 229u8, 160u8,
							117u8, 192u8, 175u8, 47u8, 173u8, 145u8, 255u8, 91u8, 208u8, 238u8,
							99u8, 100u8, 38u8, 182u8, 219u8, 113u8, 161u8, 196u8, 186u8, 254u8,
							59u8, 213u8,
						],
					)
				}
				pub fn zone_records_by_domain(
					&self,
					_0: impl ::std::borrow::Borrow<types::zone_records_by_domain::Param0>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageKey<
						types::zone_records_by_domain::Param0,
					>,
					types::zone_records_by_domain::ZoneRecordsByDomain,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"DataDomain",
						"ZoneRecordsByDomain",
						::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							89u8, 9u8, 74u8, 149u8, 114u8, 123u8, 168u8, 120u8, 229u8, 160u8,
							117u8, 192u8, 175u8, 47u8, 173u8, 145u8, 255u8, 91u8, 208u8, 238u8,
							99u8, 100u8, 38u8, 182u8, 219u8, 113u8, 161u8, 196u8, 186u8, 254u8,
							59u8, 213u8,
						],
					)
				}
				pub fn domain_payment_address_history_iter(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::domain_payment_address_history::DomainPaymentAddressHistory,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"DataDomain",
						"DomainPaymentAddressHistory",
						(),
						[
							139u8, 10u8, 161u8, 228u8, 6u8, 142u8, 223u8, 232u8, 177u8, 237u8,
							179u8, 75u8, 61u8, 224u8, 169u8, 114u8, 150u8, 245u8, 215u8, 214u8,
							246u8, 127u8, 45u8, 51u8, 246u8, 216u8, 217u8, 59u8, 232u8, 104u8,
							124u8, 82u8,
						],
					)
				}
				pub fn domain_payment_address_history(
					&self,
					_0: impl ::std::borrow::Borrow<types::domain_payment_address_history::Param0>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageKey<
						types::domain_payment_address_history::Param0,
					>,
					types::domain_payment_address_history::DomainPaymentAddressHistory,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"DataDomain",
						"DomainPaymentAddressHistory",
						::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							139u8, 10u8, 161u8, 228u8, 6u8, 142u8, 223u8, 232u8, 177u8, 237u8,
							179u8, 75u8, 61u8, 224u8, 169u8, 114u8, 150u8, 245u8, 215u8, 214u8,
							246u8, 127u8, 45u8, 51u8, 246u8, 216u8, 217u8, 59u8, 232u8, 104u8,
							124u8, 82u8,
						],
					)
				}
				pub fn expiring_domains_by_block_iter(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::expiring_domains_by_block::ExpiringDomainsByBlock,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"DataDomain",
						"ExpiringDomainsByBlock",
						(),
						[
							75u8, 15u8, 133u8, 11u8, 204u8, 248u8, 72u8, 80u8, 4u8, 5u8, 0u8,
							168u8, 130u8, 36u8, 43u8, 246u8, 211u8, 66u8, 249u8, 52u8, 60u8, 67u8,
							113u8, 130u8, 240u8, 148u8, 245u8, 99u8, 98u8, 203u8, 12u8, 116u8,
						],
					)
				}
				pub fn expiring_domains_by_block(
					&self,
					_0: impl ::std::borrow::Borrow<types::expiring_domains_by_block::Param0>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageKey<
						types::expiring_domains_by_block::Param0,
					>,
					types::expiring_domains_by_block::ExpiringDomainsByBlock,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"DataDomain",
						"ExpiringDomainsByBlock",
						::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							75u8, 15u8, 133u8, 11u8, 204u8, 248u8, 72u8, 80u8, 4u8, 5u8, 0u8,
							168u8, 130u8, 36u8, 43u8, 246u8, 211u8, 66u8, 249u8, 52u8, 60u8, 67u8,
							113u8, 130u8, 240u8, 148u8, 245u8, 99u8, 98u8, 203u8, 12u8, 116u8,
						],
					)
				}
			}
		}
	}
	pub mod authorship {
		use super::root_mod;
		use super::runtime_types;
		pub mod storage {
			use super::runtime_types;
			pub mod types {
				use super::runtime_types;
				pub mod author {
					use super::runtime_types;
					pub type Author = ::subxt::utils::AccountId32;
				}
			}
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Author of current block."]
				pub fn author(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::author::Author,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Authorship",
						"Author",
						(),
						[
							247u8, 192u8, 118u8, 227u8, 47u8, 20u8, 203u8, 199u8, 216u8, 87u8,
							220u8, 50u8, 166u8, 61u8, 168u8, 213u8, 253u8, 62u8, 202u8, 199u8,
							61u8, 192u8, 237u8, 53u8, 22u8, 148u8, 164u8, 245u8, 99u8, 24u8, 146u8,
							18u8,
						],
					)
				}
			}
		}
	}
	pub mod historical {
		use super::root_mod;
		use super::runtime_types;
		pub mod storage {
			use super::runtime_types;
			pub mod types {
				use super::runtime_types;
				pub mod historical_sessions {
					use super::runtime_types;
					pub type HistoricalSessions = (::subxt::utils::H256, ::core::primitive::u32);
					pub type Param0 = ::core::primitive::u32;
				}
				pub mod stored_range {
					use super::runtime_types;
					pub type StoredRange = (::core::primitive::u32, ::core::primitive::u32);
				}
			}
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Mapping from historical session indices to session-data root hash and validator count."]
				pub fn historical_sessions_iter(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::historical_sessions::HistoricalSessions,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Historical",
						"HistoricalSessions",
						(),
						[
							9u8, 138u8, 247u8, 141u8, 178u8, 146u8, 124u8, 81u8, 162u8, 211u8,
							205u8, 149u8, 222u8, 254u8, 253u8, 188u8, 170u8, 242u8, 218u8, 41u8,
							124u8, 178u8, 109u8, 209u8, 163u8, 125u8, 225u8, 206u8, 249u8, 175u8,
							117u8, 75u8,
						],
					)
				}
				#[doc = " Mapping from historical session indices to session-data root hash and validator count."]
				pub fn historical_sessions(
					&self,
					_0: impl ::std::borrow::Borrow<types::historical_sessions::Param0>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageKey<types::historical_sessions::Param0>,
					types::historical_sessions::HistoricalSessions,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Historical",
						"HistoricalSessions",
						::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							9u8, 138u8, 247u8, 141u8, 178u8, 146u8, 124u8, 81u8, 162u8, 211u8,
							205u8, 149u8, 222u8, 254u8, 253u8, 188u8, 170u8, 242u8, 218u8, 41u8,
							124u8, 178u8, 109u8, 209u8, 163u8, 125u8, 225u8, 206u8, 249u8, 175u8,
							117u8, 75u8,
						],
					)
				}
				#[doc = " The range of historical sessions we store. [first, last)"]
				pub fn stored_range(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::stored_range::StoredRange,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Historical",
						"StoredRange",
						(),
						[
							134u8, 32u8, 250u8, 13u8, 201u8, 25u8, 54u8, 243u8, 231u8, 81u8, 252u8,
							231u8, 68u8, 217u8, 235u8, 43u8, 22u8, 223u8, 220u8, 133u8, 198u8,
							218u8, 95u8, 152u8, 189u8, 87u8, 6u8, 228u8, 242u8, 59u8, 232u8, 59u8,
						],
					)
				}
			}
		}
	}
	pub mod session {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "Error for the session pallet."]
		pub type Error = runtime_types::pallet_session::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_session::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Sets the session key(s) of the function caller to `keys`."]
				#[doc = "Allows an account to set its session key prior to becoming a validator."]
				#[doc = "This doesn't take effect until the next session."]
				#[doc = ""]
				#[doc = "The dispatch origin of this function must be signed."]
				#[doc = ""]
				#[doc = "## Complexity"]
				#[doc = "- `O(1)`. Actual cost depends on the number of length of `T::Keys::key_ids()` which is"]
				#[doc = "  fixed."]
				pub struct SetKeys {
					pub keys: set_keys::Keys,
					pub proof: set_keys::Proof,
				}
				pub mod set_keys {
					use super::runtime_types;
					pub type Keys = runtime_types::ulx_node_runtime::opaque::SessionKeys;
					pub type Proof = ::std::vec::Vec<::core::primitive::u8>;
				}
				impl ::subxt::blocks::StaticExtrinsic for SetKeys {
					const PALLET: &'static str = "Session";
					const CALL: &'static str = "set_keys";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Removes any session key(s) of the function caller."]
				#[doc = ""]
				#[doc = "This doesn't take effect until the next session."]
				#[doc = ""]
				#[doc = "The dispatch origin of this function must be Signed and the account must be either be"]
				#[doc = "convertible to a validator ID using the chain's typical addressing system (this usually"]
				#[doc = "means being a controller account) or directly convertible into a validator ID (which"]
				#[doc = "usually means being a stash account)."]
				#[doc = ""]
				#[doc = "## Complexity"]
				#[doc = "- `O(1)` in number of key types. Actual cost depends on the number of length of"]
				#[doc = "  `T::Keys::key_ids()` which is fixed."]
				pub struct PurgeKeys;
				impl ::subxt::blocks::StaticExtrinsic for PurgeKeys {
					const PALLET: &'static str = "Session";
					const CALL: &'static str = "purge_keys";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Sets the session key(s) of the function caller to `keys`."]
				#[doc = "Allows an account to set its session key prior to becoming a validator."]
				#[doc = "This doesn't take effect until the next session."]
				#[doc = ""]
				#[doc = "The dispatch origin of this function must be signed."]
				#[doc = ""]
				#[doc = "## Complexity"]
				#[doc = "- `O(1)`. Actual cost depends on the number of length of `T::Keys::key_ids()` which is"]
				#[doc = "  fixed."]
				pub fn set_keys(
					&self,
					keys: types::set_keys::Keys,
					proof: types::set_keys::Proof,
				) -> ::subxt::tx::Payload<types::SetKeys> {
					::subxt::tx::Payload::new_static(
						"Session",
						"set_keys",
						types::SetKeys { keys, proof },
						[
							152u8, 217u8, 176u8, 109u8, 66u8, 122u8, 90u8, 250u8, 1u8, 124u8,
							215u8, 216u8, 118u8, 182u8, 223u8, 236u8, 96u8, 202u8, 80u8, 211u8,
							36u8, 242u8, 64u8, 65u8, 116u8, 9u8, 178u8, 35u8, 202u8, 98u8, 151u8,
							65u8,
						],
					)
				}
				#[doc = "Removes any session key(s) of the function caller."]
				#[doc = ""]
				#[doc = "This doesn't take effect until the next session."]
				#[doc = ""]
				#[doc = "The dispatch origin of this function must be Signed and the account must be either be"]
				#[doc = "convertible to a validator ID using the chain's typical addressing system (this usually"]
				#[doc = "means being a controller account) or directly convertible into a validator ID (which"]
				#[doc = "usually means being a stash account)."]
				#[doc = ""]
				#[doc = "## Complexity"]
				#[doc = "- `O(1)` in number of key types. Actual cost depends on the number of length of"]
				#[doc = "  `T::Keys::key_ids()` which is fixed."]
				pub fn purge_keys(&self) -> ::subxt::tx::Payload<types::PurgeKeys> {
					::subxt::tx::Payload::new_static(
						"Session",
						"purge_keys",
						types::PurgeKeys {},
						[
							215u8, 204u8, 146u8, 236u8, 32u8, 78u8, 198u8, 79u8, 85u8, 214u8, 15u8,
							151u8, 158u8, 31u8, 146u8, 119u8, 119u8, 204u8, 151u8, 169u8, 226u8,
							67u8, 217u8, 39u8, 241u8, 245u8, 203u8, 240u8, 203u8, 172u8, 16u8,
							209u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_session::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "New session has happened. Note that the argument is the session index, not the"]
			#[doc = "block number as the type might suggest."]
			pub struct NewSession {
				pub session_index: new_session::SessionIndex,
			}
			pub mod new_session {
				use super::runtime_types;
				pub type SessionIndex = ::core::primitive::u32;
			}
			impl ::subxt::events::StaticEvent for NewSession {
				const PALLET: &'static str = "Session";
				const EVENT: &'static str = "NewSession";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub mod types {
				use super::runtime_types;
				pub mod validators {
					use super::runtime_types;
					pub type Validators = ::std::vec::Vec<::subxt::utils::AccountId32>;
				}
				pub mod current_index {
					use super::runtime_types;
					pub type CurrentIndex = ::core::primitive::u32;
				}
				pub mod queued_changed {
					use super::runtime_types;
					pub type QueuedChanged = ::core::primitive::bool;
				}
				pub mod queued_keys {
					use super::runtime_types;
					pub type QueuedKeys = ::std::vec::Vec<(
						::subxt::utils::AccountId32,
						runtime_types::ulx_node_runtime::opaque::SessionKeys,
					)>;
				}
				pub mod disabled_validators {
					use super::runtime_types;
					pub type DisabledValidators = ::std::vec::Vec<::core::primitive::u32>;
				}
				pub mod next_keys {
					use super::runtime_types;
					pub type NextKeys = runtime_types::ulx_node_runtime::opaque::SessionKeys;
					pub type Param0 = ::subxt::utils::AccountId32;
				}
				pub mod key_owner {
					use super::runtime_types;
					pub type KeyOwner = ::subxt::utils::AccountId32;
					pub type Param0 = runtime_types::sp_core::crypto::KeyTypeId;
					pub type Param1 = [::core::primitive::u8];
				}
			}
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The current set of validators."]
				pub fn validators(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::validators::Validators,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"Validators",
						(),
						[
							50u8, 86u8, 154u8, 222u8, 249u8, 209u8, 156u8, 22u8, 155u8, 25u8,
							133u8, 194u8, 210u8, 50u8, 38u8, 28u8, 139u8, 201u8, 90u8, 139u8,
							115u8, 12u8, 12u8, 141u8, 4u8, 178u8, 201u8, 241u8, 223u8, 234u8, 6u8,
							86u8,
						],
					)
				}
				#[doc = " Current index of the session."]
				pub fn current_index(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::current_index::CurrentIndex,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"CurrentIndex",
						(),
						[
							167u8, 151u8, 125u8, 150u8, 159u8, 21u8, 78u8, 217u8, 237u8, 183u8,
							135u8, 65u8, 187u8, 114u8, 188u8, 206u8, 16u8, 32u8, 69u8, 208u8,
							134u8, 159u8, 232u8, 224u8, 243u8, 27u8, 31u8, 166u8, 145u8, 44u8,
							221u8, 230u8,
						],
					)
				}
				#[doc = " True if the underlying economic identities or weighting behind the validators"]
				#[doc = " has changed in the queued validator set."]
				pub fn queued_changed(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::queued_changed::QueuedChanged,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"QueuedChanged",
						(),
						[
							184u8, 137u8, 224u8, 137u8, 31u8, 236u8, 95u8, 164u8, 102u8, 225u8,
							198u8, 227u8, 140u8, 37u8, 113u8, 57u8, 59u8, 4u8, 202u8, 102u8, 117u8,
							36u8, 226u8, 64u8, 113u8, 141u8, 199u8, 111u8, 99u8, 144u8, 198u8,
							153u8,
						],
					)
				}
				#[doc = " The queued keys for the next session. When the next session begins, these keys"]
				#[doc = " will be used to determine the validator's session keys."]
				pub fn queued_keys(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::queued_keys::QueuedKeys,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"QueuedKeys",
						(),
						[
							184u8, 60u8, 226u8, 158u8, 118u8, 58u8, 50u8, 111u8, 207u8, 71u8,
							206u8, 234u8, 200u8, 44u8, 199u8, 184u8, 229u8, 70u8, 32u8, 199u8,
							202u8, 46u8, 136u8, 31u8, 181u8, 146u8, 22u8, 226u8, 216u8, 20u8,
							214u8, 253u8,
						],
					)
				}
				#[doc = " Indices of disabled validators."]
				#[doc = ""]
				#[doc = " The vec is always kept sorted so that we can find whether a given validator is"]
				#[doc = " disabled using binary search. It gets cleared when `on_session_ending` returns"]
				#[doc = " a new set of identities."]
				pub fn disabled_validators(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::disabled_validators::DisabledValidators,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"DisabledValidators",
						(),
						[
							213u8, 19u8, 168u8, 234u8, 187u8, 200u8, 180u8, 97u8, 234u8, 189u8,
							36u8, 233u8, 158u8, 184u8, 45u8, 35u8, 129u8, 213u8, 133u8, 8u8, 104u8,
							183u8, 46u8, 68u8, 154u8, 240u8, 132u8, 22u8, 247u8, 11u8, 54u8, 221u8,
						],
					)
				}
				#[doc = " The next session keys for a validator."]
				pub fn next_keys_iter(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::next_keys::NextKeys,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"NextKeys",
						(),
						[
							78u8, 250u8, 134u8, 66u8, 119u8, 121u8, 215u8, 245u8, 120u8, 34u8,
							46u8, 97u8, 251u8, 104u8, 7u8, 233u8, 153u8, 10u8, 6u8, 169u8, 23u8,
							152u8, 62u8, 22u8, 137u8, 3u8, 222u8, 144u8, 201u8, 56u8, 179u8, 87u8,
						],
					)
				}
				#[doc = " The next session keys for a validator."]
				pub fn next_keys(
					&self,
					_0: impl ::std::borrow::Borrow<types::next_keys::Param0>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageKey<types::next_keys::Param0>,
					types::next_keys::NextKeys,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"NextKeys",
						::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							78u8, 250u8, 134u8, 66u8, 119u8, 121u8, 215u8, 245u8, 120u8, 34u8,
							46u8, 97u8, 251u8, 104u8, 7u8, 233u8, 153u8, 10u8, 6u8, 169u8, 23u8,
							152u8, 62u8, 22u8, 137u8, 3u8, 222u8, 144u8, 201u8, 56u8, 179u8, 87u8,
						],
					)
				}
				#[doc = " The owner of a key. The key is the `KeyTypeId` + the encoded key."]
				pub fn key_owner_iter(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::key_owner::KeyOwner,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"KeyOwner",
						(),
						[
							217u8, 204u8, 21u8, 114u8, 247u8, 129u8, 32u8, 242u8, 93u8, 91u8,
							253u8, 253u8, 248u8, 90u8, 12u8, 202u8, 195u8, 25u8, 18u8, 100u8,
							253u8, 109u8, 88u8, 77u8, 217u8, 140u8, 51u8, 40u8, 118u8, 35u8, 107u8,
							206u8,
						],
					)
				}
				#[doc = " The owner of a key. The key is the `KeyTypeId` + the encoded key."]
				pub fn key_owner_iter1(
					&self,
					_0: impl ::std::borrow::Borrow<types::key_owner::Param0>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageKey<types::key_owner::Param0>,
					types::key_owner::KeyOwner,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"KeyOwner",
						::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							217u8, 204u8, 21u8, 114u8, 247u8, 129u8, 32u8, 242u8, 93u8, 91u8,
							253u8, 253u8, 248u8, 90u8, 12u8, 202u8, 195u8, 25u8, 18u8, 100u8,
							253u8, 109u8, 88u8, 77u8, 217u8, 140u8, 51u8, 40u8, 118u8, 35u8, 107u8,
							206u8,
						],
					)
				}
				#[doc = " The owner of a key. The key is the `KeyTypeId` + the encoded key."]
				pub fn key_owner(
					&self,
					_0: impl ::std::borrow::Borrow<types::key_owner::Param0>,
					_1: impl ::std::borrow::Borrow<types::key_owner::Param1>,
				) -> ::subxt::storage::address::Address<
					(
						::subxt::storage::address::StaticStorageKey<types::key_owner::Param0>,
						::subxt::storage::address::StaticStorageKey<types::key_owner::Param1>,
					),
					types::key_owner::KeyOwner,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"KeyOwner",
						(
							::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
							::subxt::storage::address::StaticStorageKey::new(_1.borrow()),
						),
						[
							217u8, 204u8, 21u8, 114u8, 247u8, 129u8, 32u8, 242u8, 93u8, 91u8,
							253u8, 253u8, 248u8, 90u8, 12u8, 202u8, 195u8, 25u8, 18u8, 100u8,
							253u8, 109u8, 88u8, 77u8, 217u8, 140u8, 51u8, 40u8, 118u8, 35u8, 107u8,
							206u8,
						],
					)
				}
			}
		}
	}
	pub mod block_seal {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::pallet_block_seal::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_block_seal::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Apply {
					pub seal: apply::Seal,
				}
				pub mod apply {
					use super::runtime_types;
					pub type Seal = runtime_types::ulx_primitives::inherents::BlockSealInherent;
				}
				impl ::subxt::blocks::StaticExtrinsic for Apply {
					const PALLET: &'static str = "BlockSeal";
					const CALL: &'static str = "apply";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn apply(
					&self,
					seal: types::apply::Seal,
				) -> ::subxt::tx::Payload<types::Apply> {
					::subxt::tx::Payload::new_static(
						"BlockSeal",
						"apply",
						types::Apply { seal },
						[
							219u8, 28u8, 137u8, 83u8, 119u8, 114u8, 20u8, 229u8, 253u8, 233u8,
							140u8, 58u8, 32u8, 228u8, 139u8, 84u8, 9u8, 93u8, 54u8, 8u8, 96u8, 8u8,
							97u8, 25u8, 73u8, 9u8, 55u8, 175u8, 198u8, 182u8, 196u8, 85u8,
						],
					)
				}
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub mod types {
				use super::runtime_types;
				pub mod last_block_sealer_info {
					use super::runtime_types;
					pub type LastBlockSealerInfo =
						runtime_types::ulx_primitives::providers::BlockSealerInfo<
							::subxt::utils::AccountId32,
						>;
				}
				pub mod parent_voting_key {
					use super::runtime_types;
					pub type ParentVotingKey = ::core::option::Option<::subxt::utils::H256>;
				}
				pub mod temp_author {
					use super::runtime_types;
					pub type TempAuthor = ::subxt::utils::AccountId32;
				}
				pub mod temp_seal_inherent {
					use super::runtime_types;
					pub type TempSealInherent =
						runtime_types::ulx_primitives::inherents::BlockSealInherent;
				}
				pub mod temp_voting_key_digest {
					use super::runtime_types;
					pub type TempVotingKeyDigest =
						runtime_types::ulx_primitives::digests::ParentVotingKeyDigest;
				}
			}
			pub struct StorageApi;
			impl StorageApi {
				pub fn last_block_sealer_info(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::last_block_sealer_info::LastBlockSealerInfo,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"BlockSeal",
						"LastBlockSealerInfo",
						(),
						[
							26u8, 56u8, 49u8, 198u8, 82u8, 141u8, 26u8, 125u8, 201u8, 185u8, 196u8,
							156u8, 211u8, 60u8, 115u8, 46u8, 223u8, 209u8, 22u8, 253u8, 136u8,
							203u8, 213u8, 99u8, 14u8, 212u8, 79u8, 204u8, 213u8, 39u8, 16u8, 34u8,
						],
					)
				}
				#[doc = " The calculated parent voting key for a block. Refers to the Notebook BlockVote Revealed"]
				#[doc = " Secret + VotesMerkleRoot of the parent block notebooks."]
				pub fn parent_voting_key(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::parent_voting_key::ParentVotingKey,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"BlockSeal",
						"ParentVotingKey",
						(),
						[
							12u8, 73u8, 52u8, 154u8, 15u8, 127u8, 150u8, 214u8, 178u8, 186u8,
							231u8, 204u8, 104u8, 196u8, 141u8, 55u8, 198u8, 11u8, 23u8, 252u8,
							108u8, 65u8, 42u8, 124u8, 77u8, 77u8, 88u8, 35u8, 154u8, 241u8, 50u8,
							216u8,
						],
					)
				}
				#[doc = " Author of current block (temporary storage)."]
				pub fn temp_author(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::temp_author::TempAuthor,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"BlockSeal",
						"TempAuthor",
						(),
						[
							29u8, 149u8, 234u8, 74u8, 206u8, 138u8, 152u8, 92u8, 28u8, 103u8, 4u8,
							236u8, 161u8, 51u8, 52u8, 196u8, 28u8, 242u8, 250u8, 210u8, 187u8,
							78u8, 217u8, 251u8, 157u8, 143u8, 91u8, 60u8, 246u8, 218u8, 227u8,
							114u8,
						],
					)
				}
				#[doc = " Ensures only a single inherent is applied"]
				pub fn temp_seal_inherent(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::temp_seal_inherent::TempSealInherent,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"BlockSeal",
						"TempSealInherent",
						(),
						[
							155u8, 198u8, 152u8, 193u8, 160u8, 65u8, 79u8, 57u8, 173u8, 165u8,
							225u8, 42u8, 102u8, 64u8, 210u8, 78u8, 243u8, 181u8, 84u8, 156u8,
							139u8, 238u8, 34u8, 129u8, 199u8, 138u8, 84u8, 136u8, 156u8, 115u8,
							10u8, 107u8,
						],
					)
				}
				#[doc = " Temporarily track the parent voting key digest"]
				pub fn temp_voting_key_digest(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::temp_voting_key_digest::TempVotingKeyDigest,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"BlockSeal",
						"TempVotingKeyDigest",
						(),
						[
							62u8, 236u8, 18u8, 31u8, 216u8, 143u8, 7u8, 128u8, 99u8, 129u8, 168u8,
							182u8, 205u8, 207u8, 253u8, 199u8, 82u8, 185u8, 26u8, 190u8, 222u8,
							160u8, 10u8, 186u8, 150u8, 3u8, 192u8, 56u8, 145u8, 106u8, 159u8, 73u8,
						],
					)
				}
			}
		}
	}
	pub mod block_rewards {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::pallet_block_rewards::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_block_rewards::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
			}
			pub struct TransactionApi;
			impl TransactionApi {}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_block_rewards::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct RewardCreated {
				pub maturation_block: reward_created::MaturationBlock,
				pub rewards: reward_created::Rewards,
			}
			pub mod reward_created {
				use super::runtime_types;
				pub type MaturationBlock = ::core::primitive::u32;
				pub type Rewards = ::std::vec::Vec<
					runtime_types::pallet_block_rewards::pallet::BlockPayout<
						::subxt::utils::AccountId32,
						::core::primitive::u128,
					>,
				>;
			}
			impl ::subxt::events::StaticEvent for RewardCreated {
				const PALLET: &'static str = "BlockRewards";
				const EVENT: &'static str = "RewardCreated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct RewardUnlocked {
				pub rewards: reward_unlocked::Rewards,
			}
			pub mod reward_unlocked {
				use super::runtime_types;
				pub type Rewards = ::std::vec::Vec<
					runtime_types::pallet_block_rewards::pallet::BlockPayout<
						::subxt::utils::AccountId32,
						::core::primitive::u128,
					>,
				>;
			}
			impl ::subxt::events::StaticEvent for RewardUnlocked {
				const PALLET: &'static str = "BlockRewards";
				const EVENT: &'static str = "RewardUnlocked";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub mod types {
				use super::runtime_types;
				pub mod payouts_by_block {
					use super::runtime_types;
					pub type PayoutsByBlock =
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							runtime_types::pallet_block_rewards::pallet::BlockPayout<
								::subxt::utils::AccountId32,
								::core::primitive::u128,
							>,
						>;
					pub type Param0 = ::core::primitive::u32;
				}
			}
			pub struct StorageApi;
			impl StorageApi {
				pub fn payouts_by_block_iter(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::payouts_by_block::PayoutsByBlock,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"BlockRewards",
						"PayoutsByBlock",
						(),
						[
							69u8, 232u8, 99u8, 71u8, 94u8, 15u8, 227u8, 177u8, 153u8, 246u8, 17u8,
							248u8, 101u8, 239u8, 203u8, 172u8, 253u8, 130u8, 39u8, 90u8, 48u8,
							233u8, 49u8, 36u8, 118u8, 214u8, 241u8, 47u8, 136u8, 190u8, 36u8, 91u8,
						],
					)
				}
				pub fn payouts_by_block(
					&self,
					_0: impl ::std::borrow::Borrow<types::payouts_by_block::Param0>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageKey<types::payouts_by_block::Param0>,
					types::payouts_by_block::PayoutsByBlock,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"BlockRewards",
						"PayoutsByBlock",
						::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							69u8, 232u8, 99u8, 71u8, 94u8, 15u8, 227u8, 177u8, 153u8, 246u8, 17u8,
							248u8, 101u8, 239u8, 203u8, 172u8, 253u8, 130u8, 39u8, 90u8, 48u8,
							233u8, 49u8, 36u8, 118u8, 214u8, 241u8, 47u8, 136u8, 190u8, 36u8, 91u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " Number of argons minted per block"]
				pub fn argons_per_block(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"BlockRewards",
						"ArgonsPerBlock",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				#[doc = " Number of ulixees minted per block"]
				pub fn starting_ulixees_per_block(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"BlockRewards",
						"StartingUlixeesPerBlock",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				#[doc = " Number of blocks for halving of ulixee rewards"]
				pub fn halving_blocks(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"BlockRewards",
						"HalvingBlocks",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " Percent as a number out of 100 of the block reward that goes to the miner."]
				pub fn miner_payout_percent(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"BlockRewards",
						"MinerPayoutPercent",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " Blocks until a block reward is mature"]
				pub fn maturation_blocks(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"BlockRewards",
						"MaturationBlocks",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod grandpa {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::pallet_grandpa::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_grandpa::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Report voter equivocation/misbehavior. This method will verify the"]
				#[doc = "equivocation proof and validate the given key ownership proof"]
				#[doc = "against the extracted offender. If both are valid, the offence"]
				#[doc = "will be reported."]
				pub struct ReportEquivocation {
					pub equivocation_proof:
						::std::boxed::Box<report_equivocation::EquivocationProof>,
					pub key_owner_proof: report_equivocation::KeyOwnerProof,
				}
				pub mod report_equivocation {
					use super::runtime_types;
					pub type EquivocationProof =
						runtime_types::sp_consensus_grandpa::EquivocationProof<
							::subxt::utils::H256,
							::core::primitive::u32,
						>;
					pub type KeyOwnerProof = runtime_types::sp_session::MembershipProof;
				}
				impl ::subxt::blocks::StaticExtrinsic for ReportEquivocation {
					const PALLET: &'static str = "Grandpa";
					const CALL: &'static str = "report_equivocation";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Report voter equivocation/misbehavior. This method will verify the"]
				#[doc = "equivocation proof and validate the given key ownership proof"]
				#[doc = "against the extracted offender. If both are valid, the offence"]
				#[doc = "will be reported."]
				#[doc = ""]
				#[doc = "This extrinsic must be called unsigned and it is expected that only"]
				#[doc = "block authors will call it (validated in `ValidateUnsigned`), as such"]
				#[doc = "if the block author is defined it will be defined as the equivocation"]
				#[doc = "reporter."]
				pub struct ReportEquivocationUnsigned {
					pub equivocation_proof:
						::std::boxed::Box<report_equivocation_unsigned::EquivocationProof>,
					pub key_owner_proof: report_equivocation_unsigned::KeyOwnerProof,
				}
				pub mod report_equivocation_unsigned {
					use super::runtime_types;
					pub type EquivocationProof =
						runtime_types::sp_consensus_grandpa::EquivocationProof<
							::subxt::utils::H256,
							::core::primitive::u32,
						>;
					pub type KeyOwnerProof = runtime_types::sp_session::MembershipProof;
				}
				impl ::subxt::blocks::StaticExtrinsic for ReportEquivocationUnsigned {
					const PALLET: &'static str = "Grandpa";
					const CALL: &'static str = "report_equivocation_unsigned";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Note that the current authority set of the GRANDPA finality gadget has stalled."]
				#[doc = ""]
				#[doc = "This will trigger a forced authority set change at the beginning of the next session, to"]
				#[doc = "be enacted `delay` blocks after that. The `delay` should be high enough to safely assume"]
				#[doc = "that the block signalling the forced change will not be re-orged e.g. 1000 blocks."]
				#[doc = "The block production rate (which may be slowed down because of finality lagging) should"]
				#[doc = "be taken into account when choosing the `delay`. The GRANDPA voters based on the new"]
				#[doc = "authority will start voting on top of `best_finalized_block_number` for new finalized"]
				#[doc = "blocks. `best_finalized_block_number` should be the highest of the latest finalized"]
				#[doc = "block of all validators of the new authority set."]
				#[doc = ""]
				#[doc = "Only callable by root."]
				pub struct NoteStalled {
					pub delay: note_stalled::Delay,
					pub best_finalized_block_number: note_stalled::BestFinalizedBlockNumber,
				}
				pub mod note_stalled {
					use super::runtime_types;
					pub type Delay = ::core::primitive::u32;
					pub type BestFinalizedBlockNumber = ::core::primitive::u32;
				}
				impl ::subxt::blocks::StaticExtrinsic for NoteStalled {
					const PALLET: &'static str = "Grandpa";
					const CALL: &'static str = "note_stalled";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Report voter equivocation/misbehavior. This method will verify the"]
				#[doc = "equivocation proof and validate the given key ownership proof"]
				#[doc = "against the extracted offender. If both are valid, the offence"]
				#[doc = "will be reported."]
				pub fn report_equivocation(
					&self,
					equivocation_proof: types::report_equivocation::EquivocationProof,
					key_owner_proof: types::report_equivocation::KeyOwnerProof,
				) -> ::subxt::tx::Payload<types::ReportEquivocation> {
					::subxt::tx::Payload::new_static(
						"Grandpa",
						"report_equivocation",
						types::ReportEquivocation {
							equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
							key_owner_proof,
						},
						[
							197u8, 206u8, 246u8, 26u8, 171u8, 25u8, 214u8, 211u8, 138u8, 132u8,
							148u8, 48u8, 66u8, 12u8, 92u8, 17u8, 190u8, 155u8, 121u8, 222u8, 226u8,
							171u8, 208u8, 123u8, 253u8, 247u8, 253u8, 191u8, 90u8, 4u8, 224u8,
							104u8,
						],
					)
				}
				#[doc = "Report voter equivocation/misbehavior. This method will verify the"]
				#[doc = "equivocation proof and validate the given key ownership proof"]
				#[doc = "against the extracted offender. If both are valid, the offence"]
				#[doc = "will be reported."]
				#[doc = ""]
				#[doc = "This extrinsic must be called unsigned and it is expected that only"]
				#[doc = "block authors will call it (validated in `ValidateUnsigned`), as such"]
				#[doc = "if the block author is defined it will be defined as the equivocation"]
				#[doc = "reporter."]
				pub fn report_equivocation_unsigned(
					&self,
					equivocation_proof: types::report_equivocation_unsigned::EquivocationProof,
					key_owner_proof: types::report_equivocation_unsigned::KeyOwnerProof,
				) -> ::subxt::tx::Payload<types::ReportEquivocationUnsigned> {
					::subxt::tx::Payload::new_static(
						"Grandpa",
						"report_equivocation_unsigned",
						types::ReportEquivocationUnsigned {
							equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
							key_owner_proof,
						},
						[
							109u8, 97u8, 251u8, 184u8, 77u8, 61u8, 95u8, 187u8, 132u8, 146u8, 18u8,
							105u8, 109u8, 124u8, 181u8, 74u8, 143u8, 171u8, 248u8, 188u8, 69u8,
							63u8, 65u8, 92u8, 64u8, 42u8, 104u8, 131u8, 67u8, 202u8, 172u8, 73u8,
						],
					)
				}
				#[doc = "Note that the current authority set of the GRANDPA finality gadget has stalled."]
				#[doc = ""]
				#[doc = "This will trigger a forced authority set change at the beginning of the next session, to"]
				#[doc = "be enacted `delay` blocks after that. The `delay` should be high enough to safely assume"]
				#[doc = "that the block signalling the forced change will not be re-orged e.g. 1000 blocks."]
				#[doc = "The block production rate (which may be slowed down because of finality lagging) should"]
				#[doc = "be taken into account when choosing the `delay`. The GRANDPA voters based on the new"]
				#[doc = "authority will start voting on top of `best_finalized_block_number` for new finalized"]
				#[doc = "blocks. `best_finalized_block_number` should be the highest of the latest finalized"]
				#[doc = "block of all validators of the new authority set."]
				#[doc = ""]
				#[doc = "Only callable by root."]
				pub fn note_stalled(
					&self,
					delay: types::note_stalled::Delay,
					best_finalized_block_number: types::note_stalled::BestFinalizedBlockNumber,
				) -> ::subxt::tx::Payload<types::NoteStalled> {
					::subxt::tx::Payload::new_static(
						"Grandpa",
						"note_stalled",
						types::NoteStalled { delay, best_finalized_block_number },
						[
							158u8, 25u8, 64u8, 114u8, 131u8, 139u8, 227u8, 132u8, 42u8, 107u8,
							40u8, 249u8, 18u8, 93u8, 254u8, 86u8, 37u8, 67u8, 250u8, 35u8, 241u8,
							194u8, 209u8, 20u8, 39u8, 75u8, 186u8, 21u8, 48u8, 124u8, 151u8, 31u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_grandpa::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "New authority set has been applied."]
			pub struct NewAuthorities {
				pub authority_set: new_authorities::AuthoritySet,
			}
			pub mod new_authorities {
				use super::runtime_types;
				pub type AuthoritySet = ::std::vec::Vec<(
					runtime_types::sp_consensus_grandpa::app::Public,
					::core::primitive::u64,
				)>;
			}
			impl ::subxt::events::StaticEvent for NewAuthorities {
				const PALLET: &'static str = "Grandpa";
				const EVENT: &'static str = "NewAuthorities";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Current authority set has been paused."]
			pub struct Paused;
			impl ::subxt::events::StaticEvent for Paused {
				const PALLET: &'static str = "Grandpa";
				const EVENT: &'static str = "Paused";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Current authority set has been resumed."]
			pub struct Resumed;
			impl ::subxt::events::StaticEvent for Resumed {
				const PALLET: &'static str = "Grandpa";
				const EVENT: &'static str = "Resumed";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub mod types {
				use super::runtime_types;
				pub mod state {
					use super::runtime_types;
					pub type State =
						runtime_types::pallet_grandpa::StoredState<::core::primitive::u32>;
				}
				pub mod pending_change {
					use super::runtime_types;
					pub type PendingChange =
						runtime_types::pallet_grandpa::StoredPendingChange<::core::primitive::u32>;
				}
				pub mod next_forced {
					use super::runtime_types;
					pub type NextForced = ::core::primitive::u32;
				}
				pub mod stalled {
					use super::runtime_types;
					pub type Stalled = (::core::primitive::u32, ::core::primitive::u32);
				}
				pub mod current_set_id {
					use super::runtime_types;
					pub type CurrentSetId = ::core::primitive::u64;
				}
				pub mod set_id_session {
					use super::runtime_types;
					pub type SetIdSession = ::core::primitive::u32;
					pub type Param0 = ::core::primitive::u64;
				}
				pub mod authorities {
					use super::runtime_types;
					pub type Authorities =
						runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<(
							runtime_types::sp_consensus_grandpa::app::Public,
							::core::primitive::u64,
						)>;
				}
			}
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " State of the current authority set."]
				pub fn state(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::state::State,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Grandpa",
						"State",
						(),
						[
							73u8, 71u8, 112u8, 83u8, 238u8, 75u8, 44u8, 9u8, 180u8, 33u8, 30u8,
							121u8, 98u8, 96u8, 61u8, 133u8, 16u8, 70u8, 30u8, 249u8, 34u8, 148u8,
							15u8, 239u8, 164u8, 157u8, 52u8, 27u8, 144u8, 52u8, 223u8, 109u8,
						],
					)
				}
				#[doc = " Pending change: (signaled at, scheduled change)."]
				pub fn pending_change(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::pending_change::PendingChange,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Grandpa",
						"PendingChange",
						(),
						[
							32u8, 165u8, 141u8, 100u8, 109u8, 66u8, 58u8, 22u8, 118u8, 84u8, 92u8,
							164u8, 119u8, 130u8, 104u8, 25u8, 244u8, 111u8, 223u8, 54u8, 184u8,
							95u8, 196u8, 30u8, 244u8, 129u8, 110u8, 127u8, 200u8, 66u8, 226u8,
							26u8,
						],
					)
				}
				#[doc = " next block number where we can force a change."]
				pub fn next_forced(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::next_forced::NextForced,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Grandpa",
						"NextForced",
						(),
						[
							3u8, 231u8, 56u8, 18u8, 87u8, 112u8, 227u8, 126u8, 180u8, 131u8, 255u8,
							141u8, 82u8, 34u8, 61u8, 47u8, 234u8, 37u8, 95u8, 62u8, 33u8, 235u8,
							231u8, 122u8, 125u8, 8u8, 223u8, 95u8, 255u8, 204u8, 40u8, 97u8,
						],
					)
				}
				#[doc = " `true` if we are currently stalled."]
				pub fn stalled(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::stalled::Stalled,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Grandpa",
						"Stalled",
						(),
						[
							6u8, 81u8, 205u8, 142u8, 195u8, 48u8, 0u8, 247u8, 108u8, 170u8, 10u8,
							249u8, 72u8, 206u8, 32u8, 103u8, 109u8, 57u8, 51u8, 21u8, 144u8, 204u8,
							79u8, 8u8, 191u8, 185u8, 38u8, 34u8, 118u8, 223u8, 75u8, 241u8,
						],
					)
				}
				#[doc = " The number of changes (both in terms of keys and underlying economic responsibilities)"]
				#[doc = " in the \"set\" of Grandpa validators from genesis."]
				pub fn current_set_id(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::current_set_id::CurrentSetId,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Grandpa",
						"CurrentSetId",
						(),
						[
							234u8, 215u8, 218u8, 42u8, 30u8, 76u8, 129u8, 40u8, 125u8, 137u8,
							207u8, 47u8, 46u8, 213u8, 159u8, 50u8, 175u8, 81u8, 155u8, 123u8,
							246u8, 175u8, 156u8, 68u8, 22u8, 113u8, 135u8, 137u8, 163u8, 18u8,
							115u8, 73u8,
						],
					)
				}
				#[doc = " A mapping from grandpa set ID to the index of the *most recent* session for which its"]
				#[doc = " members were responsible."]
				#[doc = ""]
				#[doc = " This is only used for validating equivocation proofs. An equivocation proof must"]
				#[doc = " contains a key-ownership proof for a given session, therefore we need a way to tie"]
				#[doc = " together sessions and GRANDPA set ids, i.e. we need to validate that a validator"]
				#[doc = " was the owner of a given key on a given session, and what the active set ID was"]
				#[doc = " during that session."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: `SetId` is not under user control."]
				pub fn set_id_session_iter(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::set_id_session::SetIdSession,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Grandpa",
						"SetIdSession",
						(),
						[
							47u8, 0u8, 239u8, 121u8, 187u8, 213u8, 254u8, 50u8, 238u8, 10u8, 162u8,
							65u8, 189u8, 166u8, 37u8, 74u8, 82u8, 81u8, 160u8, 20u8, 180u8, 253u8,
							238u8, 18u8, 209u8, 203u8, 38u8, 148u8, 16u8, 105u8, 72u8, 169u8,
						],
					)
				}
				#[doc = " A mapping from grandpa set ID to the index of the *most recent* session for which its"]
				#[doc = " members were responsible."]
				#[doc = ""]
				#[doc = " This is only used for validating equivocation proofs. An equivocation proof must"]
				#[doc = " contains a key-ownership proof for a given session, therefore we need a way to tie"]
				#[doc = " together sessions and GRANDPA set ids, i.e. we need to validate that a validator"]
				#[doc = " was the owner of a given key on a given session, and what the active set ID was"]
				#[doc = " during that session."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: `SetId` is not under user control."]
				pub fn set_id_session(
					&self,
					_0: impl ::std::borrow::Borrow<types::set_id_session::Param0>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageKey<types::set_id_session::Param0>,
					types::set_id_session::SetIdSession,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Grandpa",
						"SetIdSession",
						::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							47u8, 0u8, 239u8, 121u8, 187u8, 213u8, 254u8, 50u8, 238u8, 10u8, 162u8,
							65u8, 189u8, 166u8, 37u8, 74u8, 82u8, 81u8, 160u8, 20u8, 180u8, 253u8,
							238u8, 18u8, 209u8, 203u8, 38u8, 148u8, 16u8, 105u8, 72u8, 169u8,
						],
					)
				}
				#[doc = " The current list of authorities."]
				pub fn authorities(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::authorities::Authorities,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Grandpa",
						"Authorities",
						(),
						[
							192u8, 157u8, 98u8, 244u8, 104u8, 38u8, 195u8, 114u8, 183u8, 62u8,
							247u8, 18u8, 31u8, 152u8, 246u8, 206u8, 97u8, 13u8, 118u8, 211u8,
							104u8, 54u8, 150u8, 152u8, 126u8, 170u8, 228u8, 158u8, 108u8, 129u8,
							134u8, 44u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " Max Authorities in use"]
				pub fn max_authorities(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Grandpa",
						"MaxAuthorities",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The maximum number of nominators for each validator."]
				pub fn max_nominators(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Grandpa",
						"MaxNominators",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The maximum number of entries to keep in the set id to session index mapping."]
				#[doc = ""]
				#[doc = " Since the `SetIdSession` map is only used for validating equivocations this"]
				#[doc = " value should relate to the bonding duration of whatever staking system is"]
				#[doc = " being used (if any). If equivocation handling is not enabled then this value"]
				#[doc = " can be zero."]
				pub fn max_set_id_session_entries(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u64> {
					::subxt::constants::Address::new_static(
						"Grandpa",
						"MaxSetIdSessionEntries",
						[
							128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
							59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
							103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
							246u8,
						],
					)
				}
			}
		}
	}
	pub mod offences {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "Events type."]
		pub type Event = runtime_types::pallet_offences::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "There is an offence reported of the given `kind` happened at the `session_index` and"]
			#[doc = "(kind-specific) time slot. This event is not deposited for duplicate slashes."]
			#[doc = "\\[kind, timeslot\\]."]
			pub struct Offence {
				pub kind: offence::Kind,
				pub timeslot: offence::Timeslot,
			}
			pub mod offence {
				use super::runtime_types;
				pub type Kind = [::core::primitive::u8; 16usize];
				pub type Timeslot = ::std::vec::Vec<::core::primitive::u8>;
			}
			impl ::subxt::events::StaticEvent for Offence {
				const PALLET: &'static str = "Offences";
				const EVENT: &'static str = "Offence";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub mod types {
				use super::runtime_types;
				pub mod reports {
					use super::runtime_types;
					pub type Reports = runtime_types::sp_staking::offence::OffenceDetails<
						::subxt::utils::AccountId32,
						(
							::subxt::utils::AccountId32,
							runtime_types::pallet_mining_slot::MinerHistory,
						),
					>;
					pub type Param0 = ::subxt::utils::H256;
				}
				pub mod concurrent_reports_index {
					use super::runtime_types;
					pub type ConcurrentReportsIndex = ::std::vec::Vec<::subxt::utils::H256>;
					pub type Param0 = [::core::primitive::u8; 16usize];
					pub type Param1 = [::core::primitive::u8];
				}
			}
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The primary structure that holds all offence records keyed by report identifiers."]
				pub fn reports_iter(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::reports::Reports,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Offences",
						"Reports",
						(),
						[
							205u8, 231u8, 221u8, 1u8, 157u8, 93u8, 122u8, 97u8, 61u8, 216u8, 201u8,
							203u8, 114u8, 249u8, 113u8, 235u8, 82u8, 159u8, 25u8, 19u8, 207u8,
							108u8, 214u8, 122u8, 8u8, 1u8, 110u8, 191u8, 218u8, 248u8, 56u8, 36u8,
						],
					)
				}
				#[doc = " The primary structure that holds all offence records keyed by report identifiers."]
				pub fn reports(
					&self,
					_0: impl ::std::borrow::Borrow<types::reports::Param0>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageKey<types::reports::Param0>,
					types::reports::Reports,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Offences",
						"Reports",
						::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							205u8, 231u8, 221u8, 1u8, 157u8, 93u8, 122u8, 97u8, 61u8, 216u8, 201u8,
							203u8, 114u8, 249u8, 113u8, 235u8, 82u8, 159u8, 25u8, 19u8, 207u8,
							108u8, 214u8, 122u8, 8u8, 1u8, 110u8, 191u8, 218u8, 248u8, 56u8, 36u8,
						],
					)
				}
				#[doc = " A vector of reports of the same kind that happened at the same time slot."]
				pub fn concurrent_reports_index_iter(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::concurrent_reports_index::ConcurrentReportsIndex,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Offences",
						"ConcurrentReportsIndex",
						(),
						[
							170u8, 186u8, 72u8, 29u8, 251u8, 38u8, 193u8, 195u8, 109u8, 86u8, 0u8,
							241u8, 20u8, 235u8, 108u8, 126u8, 215u8, 82u8, 73u8, 113u8, 199u8,
							138u8, 24u8, 58u8, 216u8, 72u8, 221u8, 232u8, 252u8, 244u8, 96u8,
							247u8,
						],
					)
				}
				#[doc = " A vector of reports of the same kind that happened at the same time slot."]
				pub fn concurrent_reports_index_iter1(
					&self,
					_0: impl ::std::borrow::Borrow<types::concurrent_reports_index::Param0>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageKey<
						types::concurrent_reports_index::Param0,
					>,
					types::concurrent_reports_index::ConcurrentReportsIndex,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Offences",
						"ConcurrentReportsIndex",
						::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							170u8, 186u8, 72u8, 29u8, 251u8, 38u8, 193u8, 195u8, 109u8, 86u8, 0u8,
							241u8, 20u8, 235u8, 108u8, 126u8, 215u8, 82u8, 73u8, 113u8, 199u8,
							138u8, 24u8, 58u8, 216u8, 72u8, 221u8, 232u8, 252u8, 244u8, 96u8,
							247u8,
						],
					)
				}
				#[doc = " A vector of reports of the same kind that happened at the same time slot."]
				pub fn concurrent_reports_index(
					&self,
					_0: impl ::std::borrow::Borrow<types::concurrent_reports_index::Param0>,
					_1: impl ::std::borrow::Borrow<types::concurrent_reports_index::Param1>,
				) -> ::subxt::storage::address::Address<
					(
						::subxt::storage::address::StaticStorageKey<
							types::concurrent_reports_index::Param0,
						>,
						::subxt::storage::address::StaticStorageKey<
							types::concurrent_reports_index::Param1,
						>,
					),
					types::concurrent_reports_index::ConcurrentReportsIndex,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Offences",
						"ConcurrentReportsIndex",
						(
							::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
							::subxt::storage::address::StaticStorageKey::new(_1.borrow()),
						),
						[
							170u8, 186u8, 72u8, 29u8, 251u8, 38u8, 193u8, 195u8, 109u8, 86u8, 0u8,
							241u8, 20u8, 235u8, 108u8, 126u8, 215u8, 82u8, 73u8, 113u8, 199u8,
							138u8, 24u8, 58u8, 216u8, 72u8, 221u8, 232u8, 252u8, 244u8, 96u8,
							247u8,
						],
					)
				}
			}
		}
	}
	pub mod argon_balances {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::pallet_balances::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_balances::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Transfer some liquid free balance to another account."]
				#[doc = ""]
				#[doc = "`transfer_allow_death` will set the `FreeBalance` of the sender and receiver."]
				#[doc = "If the sender's account is below the existential deposit as a result"]
				#[doc = "of the transfer, the account will be reaped."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
				pub struct TransferAllowDeath {
					pub dest: transfer_allow_death::Dest,
					#[codec(compact)]
					pub value: transfer_allow_death::Value,
				}
				pub mod transfer_allow_death {
					use super::runtime_types;
					pub type Dest = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
					pub type Value = ::core::primitive::u128;
				}
				impl ::subxt::blocks::StaticExtrinsic for TransferAllowDeath {
					const PALLET: &'static str = "ArgonBalances";
					const CALL: &'static str = "transfer_allow_death";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Exactly as `transfer_allow_death`, except the origin must be root and the source account"]
				#[doc = "may be specified."]
				pub struct ForceTransfer {
					pub source: force_transfer::Source,
					pub dest: force_transfer::Dest,
					#[codec(compact)]
					pub value: force_transfer::Value,
				}
				pub mod force_transfer {
					use super::runtime_types;
					pub type Source = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
					pub type Dest = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
					pub type Value = ::core::primitive::u128;
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceTransfer {
					const PALLET: &'static str = "ArgonBalances";
					const CALL: &'static str = "force_transfer";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Same as the [`transfer_allow_death`] call, but with a check that the transfer will not"]
				#[doc = "kill the origin account."]
				#[doc = ""]
				#[doc = "99% of the time you want [`transfer_allow_death`] instead."]
				#[doc = ""]
				#[doc = "[`transfer_allow_death`]: struct.Pallet.html#method.transfer"]
				pub struct TransferKeepAlive {
					pub dest: transfer_keep_alive::Dest,
					#[codec(compact)]
					pub value: transfer_keep_alive::Value,
				}
				pub mod transfer_keep_alive {
					use super::runtime_types;
					pub type Dest = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
					pub type Value = ::core::primitive::u128;
				}
				impl ::subxt::blocks::StaticExtrinsic for TransferKeepAlive {
					const PALLET: &'static str = "ArgonBalances";
					const CALL: &'static str = "transfer_keep_alive";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Transfer the entire transferable balance from the caller account."]
				#[doc = ""]
				#[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
				#[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
				#[doc = "transferred by this function. To ensure that this function results in a killed account,"]
				#[doc = "you might need to prepare the account by removing any reference counters, storage"]
				#[doc = "deposits, etc..."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be Signed."]
				#[doc = ""]
				#[doc = "- `dest`: The recipient of the transfer."]
				#[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
				#[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
				#[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
				#[doc = "  keep the sender account alive (true)."]
				pub struct TransferAll {
					pub dest: transfer_all::Dest,
					pub keep_alive: transfer_all::KeepAlive,
				}
				pub mod transfer_all {
					use super::runtime_types;
					pub type Dest = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
					pub type KeepAlive = ::core::primitive::bool;
				}
				impl ::subxt::blocks::StaticExtrinsic for TransferAll {
					const PALLET: &'static str = "ArgonBalances";
					const CALL: &'static str = "transfer_all";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Unreserve some balance from a user by force."]
				#[doc = ""]
				#[doc = "Can only be called by ROOT."]
				pub struct ForceUnreserve {
					pub who: force_unreserve::Who,
					pub amount: force_unreserve::Amount,
				}
				pub mod force_unreserve {
					use super::runtime_types;
					pub type Who = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
					pub type Amount = ::core::primitive::u128;
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceUnreserve {
					const PALLET: &'static str = "ArgonBalances";
					const CALL: &'static str = "force_unreserve";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Upgrade a specified account."]
				#[doc = ""]
				#[doc = "- `origin`: Must be `Signed`."]
				#[doc = "- `who`: The account to be upgraded."]
				#[doc = ""]
				#[doc = "This will waive the transaction fee if at least all but 10% of the accounts needed to"]
				#[doc = "be upgraded. (We let some not have to be upgraded just in order to allow for the"]
				#[doc = "possibility of churn)."]
				pub struct UpgradeAccounts {
					pub who: upgrade_accounts::Who,
				}
				pub mod upgrade_accounts {
					use super::runtime_types;
					pub type Who = ::std::vec::Vec<::subxt::utils::AccountId32>;
				}
				impl ::subxt::blocks::StaticExtrinsic for UpgradeAccounts {
					const PALLET: &'static str = "ArgonBalances";
					const CALL: &'static str = "upgrade_accounts";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Set the regular balance of a given account."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call is `root`."]
				pub struct ForceSetBalance {
					pub who: force_set_balance::Who,
					#[codec(compact)]
					pub new_free: force_set_balance::NewFree,
				}
				pub mod force_set_balance {
					use super::runtime_types;
					pub type Who = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
					pub type NewFree = ::core::primitive::u128;
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceSetBalance {
					const PALLET: &'static str = "ArgonBalances";
					const CALL: &'static str = "force_set_balance";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Adjust the total issuance in a saturating way."]
				#[doc = ""]
				#[doc = "Can only be called by root and always needs a positive `delta`."]
				#[doc = ""]
				#[doc = "# Example"]
				pub struct ForceAdjustTotalIssuance {
					pub direction: force_adjust_total_issuance::Direction,
					#[codec(compact)]
					pub delta: force_adjust_total_issuance::Delta,
				}
				pub mod force_adjust_total_issuance {
					use super::runtime_types;
					pub type Direction = runtime_types::pallet_balances::types::AdjustmentDirection;
					pub type Delta = ::core::primitive::u128;
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceAdjustTotalIssuance {
					const PALLET: &'static str = "ArgonBalances";
					const CALL: &'static str = "force_adjust_total_issuance";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Transfer some liquid free balance to another account."]
				#[doc = ""]
				#[doc = "`transfer_allow_death` will set the `FreeBalance` of the sender and receiver."]
				#[doc = "If the sender's account is below the existential deposit as a result"]
				#[doc = "of the transfer, the account will be reaped."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
				pub fn transfer_allow_death(
					&self,
					dest: types::transfer_allow_death::Dest,
					value: types::transfer_allow_death::Value,
				) -> ::subxt::tx::Payload<types::TransferAllowDeath> {
					::subxt::tx::Payload::new_static(
						"ArgonBalances",
						"transfer_allow_death",
						types::TransferAllowDeath { dest, value },
						[
							51u8, 166u8, 195u8, 10u8, 139u8, 218u8, 55u8, 130u8, 6u8, 194u8, 35u8,
							140u8, 27u8, 205u8, 214u8, 222u8, 102u8, 43u8, 143u8, 145u8, 86u8,
							219u8, 210u8, 147u8, 13u8, 39u8, 51u8, 21u8, 237u8, 179u8, 132u8,
							130u8,
						],
					)
				}
				#[doc = "Exactly as `transfer_allow_death`, except the origin must be root and the source account"]
				#[doc = "may be specified."]
				pub fn force_transfer(
					&self,
					source: types::force_transfer::Source,
					dest: types::force_transfer::Dest,
					value: types::force_transfer::Value,
				) -> ::subxt::tx::Payload<types::ForceTransfer> {
					::subxt::tx::Payload::new_static(
						"ArgonBalances",
						"force_transfer",
						types::ForceTransfer { source, dest, value },
						[
							154u8, 93u8, 222u8, 27u8, 12u8, 248u8, 63u8, 213u8, 224u8, 86u8, 250u8,
							153u8, 249u8, 102u8, 83u8, 160u8, 79u8, 125u8, 105u8, 222u8, 77u8,
							180u8, 90u8, 105u8, 81u8, 217u8, 60u8, 25u8, 213u8, 51u8, 185u8, 96u8,
						],
					)
				}
				#[doc = "Same as the [`transfer_allow_death`] call, but with a check that the transfer will not"]
				#[doc = "kill the origin account."]
				#[doc = ""]
				#[doc = "99% of the time you want [`transfer_allow_death`] instead."]
				#[doc = ""]
				#[doc = "[`transfer_allow_death`]: struct.Pallet.html#method.transfer"]
				pub fn transfer_keep_alive(
					&self,
					dest: types::transfer_keep_alive::Dest,
					value: types::transfer_keep_alive::Value,
				) -> ::subxt::tx::Payload<types::TransferKeepAlive> {
					::subxt::tx::Payload::new_static(
						"ArgonBalances",
						"transfer_keep_alive",
						types::TransferKeepAlive { dest, value },
						[
							245u8, 14u8, 190u8, 193u8, 32u8, 210u8, 74u8, 92u8, 25u8, 182u8, 76u8,
							55u8, 247u8, 83u8, 114u8, 75u8, 143u8, 236u8, 117u8, 25u8, 54u8, 157u8,
							208u8, 207u8, 233u8, 89u8, 70u8, 161u8, 235u8, 242u8, 222u8, 59u8,
						],
					)
				}
				#[doc = "Transfer the entire transferable balance from the caller account."]
				#[doc = ""]
				#[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
				#[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
				#[doc = "transferred by this function. To ensure that this function results in a killed account,"]
				#[doc = "you might need to prepare the account by removing any reference counters, storage"]
				#[doc = "deposits, etc..."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be Signed."]
				#[doc = ""]
				#[doc = "- `dest`: The recipient of the transfer."]
				#[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
				#[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
				#[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
				#[doc = "  keep the sender account alive (true)."]
				pub fn transfer_all(
					&self,
					dest: types::transfer_all::Dest,
					keep_alive: types::transfer_all::KeepAlive,
				) -> ::subxt::tx::Payload<types::TransferAll> {
					::subxt::tx::Payload::new_static(
						"ArgonBalances",
						"transfer_all",
						types::TransferAll { dest, keep_alive },
						[
							105u8, 132u8, 49u8, 144u8, 195u8, 250u8, 34u8, 46u8, 213u8, 248u8,
							112u8, 188u8, 81u8, 228u8, 136u8, 18u8, 67u8, 172u8, 37u8, 38u8, 238u8,
							9u8, 34u8, 15u8, 67u8, 34u8, 148u8, 195u8, 223u8, 29u8, 154u8, 6u8,
						],
					)
				}
				#[doc = "Unreserve some balance from a user by force."]
				#[doc = ""]
				#[doc = "Can only be called by ROOT."]
				pub fn force_unreserve(
					&self,
					who: types::force_unreserve::Who,
					amount: types::force_unreserve::Amount,
				) -> ::subxt::tx::Payload<types::ForceUnreserve> {
					::subxt::tx::Payload::new_static(
						"ArgonBalances",
						"force_unreserve",
						types::ForceUnreserve { who, amount },
						[
							142u8, 151u8, 64u8, 205u8, 46u8, 64u8, 62u8, 122u8, 108u8, 49u8, 223u8,
							140u8, 120u8, 153u8, 35u8, 165u8, 187u8, 38u8, 157u8, 200u8, 123u8,
							199u8, 198u8, 168u8, 208u8, 159u8, 39u8, 134u8, 92u8, 103u8, 84u8,
							171u8,
						],
					)
				}
				#[doc = "Upgrade a specified account."]
				#[doc = ""]
				#[doc = "- `origin`: Must be `Signed`."]
				#[doc = "- `who`: The account to be upgraded."]
				#[doc = ""]
				#[doc = "This will waive the transaction fee if at least all but 10% of the accounts needed to"]
				#[doc = "be upgraded. (We let some not have to be upgraded just in order to allow for the"]
				#[doc = "possibility of churn)."]
				pub fn upgrade_accounts(
					&self,
					who: types::upgrade_accounts::Who,
				) -> ::subxt::tx::Payload<types::UpgradeAccounts> {
					::subxt::tx::Payload::new_static(
						"ArgonBalances",
						"upgrade_accounts",
						types::UpgradeAccounts { who },
						[
							66u8, 200u8, 179u8, 104u8, 65u8, 2u8, 101u8, 56u8, 130u8, 161u8, 224u8,
							233u8, 255u8, 124u8, 70u8, 122u8, 8u8, 49u8, 103u8, 178u8, 68u8, 47u8,
							214u8, 166u8, 217u8, 116u8, 178u8, 50u8, 212u8, 164u8, 98u8, 226u8,
						],
					)
				}
				#[doc = "Set the regular balance of a given account."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call is `root`."]
				pub fn force_set_balance(
					&self,
					who: types::force_set_balance::Who,
					new_free: types::force_set_balance::NewFree,
				) -> ::subxt::tx::Payload<types::ForceSetBalance> {
					::subxt::tx::Payload::new_static(
						"ArgonBalances",
						"force_set_balance",
						types::ForceSetBalance { who, new_free },
						[
							114u8, 229u8, 59u8, 204u8, 180u8, 83u8, 17u8, 4u8, 59u8, 4u8, 55u8,
							39u8, 151u8, 196u8, 124u8, 60u8, 209u8, 65u8, 193u8, 11u8, 44u8, 164u8,
							116u8, 93u8, 169u8, 30u8, 199u8, 165u8, 55u8, 231u8, 223u8, 43u8,
						],
					)
				}
				#[doc = "Adjust the total issuance in a saturating way."]
				#[doc = ""]
				#[doc = "Can only be called by root and always needs a positive `delta`."]
				#[doc = ""]
				#[doc = "# Example"]
				pub fn force_adjust_total_issuance(
					&self,
					direction: types::force_adjust_total_issuance::Direction,
					delta: types::force_adjust_total_issuance::Delta,
				) -> ::subxt::tx::Payload<types::ForceAdjustTotalIssuance> {
					::subxt::tx::Payload::new_static(
						"ArgonBalances",
						"force_adjust_total_issuance",
						types::ForceAdjustTotalIssuance { direction, delta },
						[
							208u8, 134u8, 56u8, 133u8, 232u8, 164u8, 10u8, 213u8, 53u8, 193u8,
							190u8, 63u8, 236u8, 186u8, 96u8, 122u8, 104u8, 87u8, 173u8, 38u8, 58u8,
							176u8, 21u8, 78u8, 42u8, 106u8, 46u8, 248u8, 251u8, 190u8, 150u8,
							202u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_balances::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "An account was created with some free balance."]
			pub struct Endowed {
				pub account: endowed::Account,
				pub free_balance: endowed::FreeBalance,
			}
			pub mod endowed {
				use super::runtime_types;
				pub type Account = ::subxt::utils::AccountId32;
				pub type FreeBalance = ::core::primitive::u128;
			}
			impl ::subxt::events::StaticEvent for Endowed {
				const PALLET: &'static str = "ArgonBalances";
				const EVENT: &'static str = "Endowed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
			#[doc = "resulting in an outright loss."]
			pub struct DustLost {
				pub account: dust_lost::Account,
				pub amount: dust_lost::Amount,
			}
			pub mod dust_lost {
				use super::runtime_types;
				pub type Account = ::subxt::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::events::StaticEvent for DustLost {
				const PALLET: &'static str = "ArgonBalances";
				const EVENT: &'static str = "DustLost";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Transfer succeeded."]
			pub struct Transfer {
				pub from: transfer::From,
				pub to: transfer::To,
				pub amount: transfer::Amount,
			}
			pub mod transfer {
				use super::runtime_types;
				pub type From = ::subxt::utils::AccountId32;
				pub type To = ::subxt::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::events::StaticEvent for Transfer {
				const PALLET: &'static str = "ArgonBalances";
				const EVENT: &'static str = "Transfer";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A balance was set by root."]
			pub struct BalanceSet {
				pub who: balance_set::Who,
				pub free: balance_set::Free,
			}
			pub mod balance_set {
				use super::runtime_types;
				pub type Who = ::subxt::utils::AccountId32;
				pub type Free = ::core::primitive::u128;
			}
			impl ::subxt::events::StaticEvent for BalanceSet {
				const PALLET: &'static str = "ArgonBalances";
				const EVENT: &'static str = "BalanceSet";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some balance was reserved (moved from free to reserved)."]
			pub struct Reserved {
				pub who: reserved::Who,
				pub amount: reserved::Amount,
			}
			pub mod reserved {
				use super::runtime_types;
				pub type Who = ::subxt::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::events::StaticEvent for Reserved {
				const PALLET: &'static str = "ArgonBalances";
				const EVENT: &'static str = "Reserved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some balance was unreserved (moved from reserved to free)."]
			pub struct Unreserved {
				pub who: unreserved::Who,
				pub amount: unreserved::Amount,
			}
			pub mod unreserved {
				use super::runtime_types;
				pub type Who = ::subxt::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::events::StaticEvent for Unreserved {
				const PALLET: &'static str = "ArgonBalances";
				const EVENT: &'static str = "Unreserved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some balance was moved from the reserve of the first account to the second account."]
			#[doc = "Final argument indicates the destination balance type."]
			pub struct ReserveRepatriated {
				pub from: reserve_repatriated::From,
				pub to: reserve_repatriated::To,
				pub amount: reserve_repatriated::Amount,
				pub destination_status: reserve_repatriated::DestinationStatus,
			}
			pub mod reserve_repatriated {
				use super::runtime_types;
				pub type From = ::subxt::utils::AccountId32;
				pub type To = ::subxt::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
				pub type DestinationStatus =
					runtime_types::frame_support::traits::tokens::misc::BalanceStatus;
			}
			impl ::subxt::events::StaticEvent for ReserveRepatriated {
				const PALLET: &'static str = "ArgonBalances";
				const EVENT: &'static str = "ReserveRepatriated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some amount was deposited (e.g. for transaction fees)."]
			pub struct Deposit {
				pub who: deposit::Who,
				pub amount: deposit::Amount,
			}
			pub mod deposit {
				use super::runtime_types;
				pub type Who = ::subxt::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::events::StaticEvent for Deposit {
				const PALLET: &'static str = "ArgonBalances";
				const EVENT: &'static str = "Deposit";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
			pub struct Withdraw {
				pub who: withdraw::Who,
				pub amount: withdraw::Amount,
			}
			pub mod withdraw {
				use super::runtime_types;
				pub type Who = ::subxt::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::events::StaticEvent for Withdraw {
				const PALLET: &'static str = "ArgonBalances";
				const EVENT: &'static str = "Withdraw";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
			pub struct Slashed {
				pub who: slashed::Who,
				pub amount: slashed::Amount,
			}
			pub mod slashed {
				use super::runtime_types;
				pub type Who = ::subxt::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::events::StaticEvent for Slashed {
				const PALLET: &'static str = "ArgonBalances";
				const EVENT: &'static str = "Slashed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some amount was minted into an account."]
			pub struct Minted {
				pub who: minted::Who,
				pub amount: minted::Amount,
			}
			pub mod minted {
				use super::runtime_types;
				pub type Who = ::subxt::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::events::StaticEvent for Minted {
				const PALLET: &'static str = "ArgonBalances";
				const EVENT: &'static str = "Minted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some amount was burned from an account."]
			pub struct Burned {
				pub who: burned::Who,
				pub amount: burned::Amount,
			}
			pub mod burned {
				use super::runtime_types;
				pub type Who = ::subxt::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::events::StaticEvent for Burned {
				const PALLET: &'static str = "ArgonBalances";
				const EVENT: &'static str = "Burned";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some amount was suspended from an account (it can be restored later)."]
			pub struct Suspended {
				pub who: suspended::Who,
				pub amount: suspended::Amount,
			}
			pub mod suspended {
				use super::runtime_types;
				pub type Who = ::subxt::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::events::StaticEvent for Suspended {
				const PALLET: &'static str = "ArgonBalances";
				const EVENT: &'static str = "Suspended";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some amount was restored into an account."]
			pub struct Restored {
				pub who: restored::Who,
				pub amount: restored::Amount,
			}
			pub mod restored {
				use super::runtime_types;
				pub type Who = ::subxt::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::events::StaticEvent for Restored {
				const PALLET: &'static str = "ArgonBalances";
				const EVENT: &'static str = "Restored";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "An account was upgraded."]
			pub struct Upgraded {
				pub who: upgraded::Who,
			}
			pub mod upgraded {
				use super::runtime_types;
				pub type Who = ::subxt::utils::AccountId32;
			}
			impl ::subxt::events::StaticEvent for Upgraded {
				const PALLET: &'static str = "ArgonBalances";
				const EVENT: &'static str = "Upgraded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Total issuance was increased by `amount`, creating a credit to be balanced."]
			pub struct Issued {
				pub amount: issued::Amount,
			}
			pub mod issued {
				use super::runtime_types;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::events::StaticEvent for Issued {
				const PALLET: &'static str = "ArgonBalances";
				const EVENT: &'static str = "Issued";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Total issuance was decreased by `amount`, creating a debt to be balanced."]
			pub struct Rescinded {
				pub amount: rescinded::Amount,
			}
			pub mod rescinded {
				use super::runtime_types;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::events::StaticEvent for Rescinded {
				const PALLET: &'static str = "ArgonBalances";
				const EVENT: &'static str = "Rescinded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some balance was locked."]
			pub struct Locked {
				pub who: locked::Who,
				pub amount: locked::Amount,
			}
			pub mod locked {
				use super::runtime_types;
				pub type Who = ::subxt::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::events::StaticEvent for Locked {
				const PALLET: &'static str = "ArgonBalances";
				const EVENT: &'static str = "Locked";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some balance was unlocked."]
			pub struct Unlocked {
				pub who: unlocked::Who,
				pub amount: unlocked::Amount,
			}
			pub mod unlocked {
				use super::runtime_types;
				pub type Who = ::subxt::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::events::StaticEvent for Unlocked {
				const PALLET: &'static str = "ArgonBalances";
				const EVENT: &'static str = "Unlocked";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some balance was frozen."]
			pub struct Frozen {
				pub who: frozen::Who,
				pub amount: frozen::Amount,
			}
			pub mod frozen {
				use super::runtime_types;
				pub type Who = ::subxt::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::events::StaticEvent for Frozen {
				const PALLET: &'static str = "ArgonBalances";
				const EVENT: &'static str = "Frozen";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some balance was thawed."]
			pub struct Thawed {
				pub who: thawed::Who,
				pub amount: thawed::Amount,
			}
			pub mod thawed {
				use super::runtime_types;
				pub type Who = ::subxt::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::events::StaticEvent for Thawed {
				const PALLET: &'static str = "ArgonBalances";
				const EVENT: &'static str = "Thawed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "The `TotalIssuance` was forcefully changed."]
			pub struct TotalIssuanceForced {
				pub old: total_issuance_forced::Old,
				pub new: total_issuance_forced::New,
			}
			pub mod total_issuance_forced {
				use super::runtime_types;
				pub type Old = ::core::primitive::u128;
				pub type New = ::core::primitive::u128;
			}
			impl ::subxt::events::StaticEvent for TotalIssuanceForced {
				const PALLET: &'static str = "ArgonBalances";
				const EVENT: &'static str = "TotalIssuanceForced";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub mod types {
				use super::runtime_types;
				pub mod total_issuance {
					use super::runtime_types;
					pub type TotalIssuance = ::core::primitive::u128;
				}
				pub mod inactive_issuance {
					use super::runtime_types;
					pub type InactiveIssuance = ::core::primitive::u128;
				}
				pub mod account {
					use super::runtime_types;
					pub type Account =
						runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>;
					pub type Param0 = ::subxt::utils::AccountId32;
				}
				pub mod locks {
					use super::runtime_types;
					pub type Locks =
						runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
							runtime_types::pallet_balances::types::BalanceLock<
								::core::primitive::u128,
							>,
						>;
					pub type Param0 = ::subxt::utils::AccountId32;
				}
				pub mod reserves {
					use super::runtime_types;
					pub type Reserves = runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::pallet_balances::types::ReserveData<
							[::core::primitive::u8; 8usize],
							::core::primitive::u128,
						>,
					>;
					pub type Param0 = ::subxt::utils::AccountId32;
				}
				pub mod holds {
					use super::runtime_types;
					pub type Holds = runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::pallet_balances::types::IdAmount<
							runtime_types::ulx_node_runtime::RuntimeHoldReason,
							::core::primitive::u128,
						>,
					>;
					pub type Param0 = ::subxt::utils::AccountId32;
				}
				pub mod freezes {
					use super::runtime_types;
					pub type Freezes = runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::pallet_balances::types::IdAmount<
							runtime_types::ulx_node_runtime::RuntimeFreezeReason,
							::core::primitive::u128,
						>,
					>;
					pub type Param0 = ::subxt::utils::AccountId32;
				}
			}
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The total units issued in the system."]
				pub fn total_issuance(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::total_issuance::TotalIssuance,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ArgonBalances",
						"TotalIssuance",
						(),
						[
							116u8, 70u8, 119u8, 194u8, 69u8, 37u8, 116u8, 206u8, 171u8, 70u8,
							171u8, 210u8, 226u8, 111u8, 184u8, 204u8, 206u8, 11u8, 68u8, 72u8,
							255u8, 19u8, 194u8, 11u8, 27u8, 194u8, 81u8, 204u8, 59u8, 224u8, 202u8,
							185u8,
						],
					)
				}
				#[doc = " The total units of outstanding deactivated balance in the system."]
				pub fn inactive_issuance(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::inactive_issuance::InactiveIssuance,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ArgonBalances",
						"InactiveIssuance",
						(),
						[
							212u8, 185u8, 19u8, 50u8, 250u8, 72u8, 173u8, 50u8, 4u8, 104u8, 161u8,
							249u8, 77u8, 247u8, 204u8, 248u8, 11u8, 18u8, 57u8, 4u8, 82u8, 110u8,
							30u8, 216u8, 16u8, 37u8, 87u8, 67u8, 189u8, 235u8, 214u8, 155u8,
						],
					)
				}
				#[doc = " The Balances pallet example of storing the balance of an account."]
				#[doc = ""]
				#[doc = " # Example"]
				#[doc = ""]
				#[doc = " ```nocompile"]
				#[doc = "  impl pallet_balances::Config for Runtime {"]
				#[doc = "    type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>"]
				#[doc = "  }"]
				#[doc = " ```"]
				#[doc = ""]
				#[doc = " You can also store the balance of an account in the `System` pallet."]
				#[doc = ""]
				#[doc = " # Example"]
				#[doc = ""]
				#[doc = " ```nocompile"]
				#[doc = "  impl pallet_balances::Config for Runtime {"]
				#[doc = "   type AccountStore = System"]
				#[doc = "  }"]
				#[doc = " ```"]
				#[doc = ""]
				#[doc = " But this comes with tradeoffs, storing account balances in the system pallet stores"]
				#[doc = " `frame_system` data alongside the account data contrary to storing account balances in the"]
				#[doc = " `Balances` pallet, which uses a `StorageMap` to store balances data only."]
				#[doc = " NOTE: This is only used in the case that this pallet is used to store balances."]
				pub fn account_iter(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::account::Account,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"ArgonBalances",
						"Account",
						(),
						[
							213u8, 38u8, 200u8, 69u8, 218u8, 0u8, 112u8, 181u8, 160u8, 23u8, 96u8,
							90u8, 3u8, 88u8, 126u8, 22u8, 103u8, 74u8, 64u8, 69u8, 29u8, 247u8,
							18u8, 17u8, 234u8, 143u8, 189u8, 22u8, 247u8, 194u8, 154u8, 249u8,
						],
					)
				}
				#[doc = " The Balances pallet example of storing the balance of an account."]
				#[doc = ""]
				#[doc = " # Example"]
				#[doc = ""]
				#[doc = " ```nocompile"]
				#[doc = "  impl pallet_balances::Config for Runtime {"]
				#[doc = "    type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>"]
				#[doc = "  }"]
				#[doc = " ```"]
				#[doc = ""]
				#[doc = " You can also store the balance of an account in the `System` pallet."]
				#[doc = ""]
				#[doc = " # Example"]
				#[doc = ""]
				#[doc = " ```nocompile"]
				#[doc = "  impl pallet_balances::Config for Runtime {"]
				#[doc = "   type AccountStore = System"]
				#[doc = "  }"]
				#[doc = " ```"]
				#[doc = ""]
				#[doc = " But this comes with tradeoffs, storing account balances in the system pallet stores"]
				#[doc = " `frame_system` data alongside the account data contrary to storing account balances in the"]
				#[doc = " `Balances` pallet, which uses a `StorageMap` to store balances data only."]
				#[doc = " NOTE: This is only used in the case that this pallet is used to store balances."]
				pub fn account(
					&self,
					_0: impl ::std::borrow::Borrow<types::account::Param0>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageKey<types::account::Param0>,
					types::account::Account,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ArgonBalances",
						"Account",
						::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							213u8, 38u8, 200u8, 69u8, 218u8, 0u8, 112u8, 181u8, 160u8, 23u8, 96u8,
							90u8, 3u8, 88u8, 126u8, 22u8, 103u8, 74u8, 64u8, 69u8, 29u8, 247u8,
							18u8, 17u8, 234u8, 143u8, 189u8, 22u8, 247u8, 194u8, 154u8, 249u8,
						],
					)
				}
				#[doc = " Any liquidity locks on some account balances."]
				#[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
				#[doc = ""]
				#[doc = " Use of locks is deprecated in favour of freezes. See `https://github.com/paritytech/substrate/pull/12951/`"]
				pub fn locks_iter(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::locks::Locks,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"ArgonBalances",
						"Locks",
						(),
						[
							10u8, 223u8, 55u8, 0u8, 249u8, 69u8, 168u8, 41u8, 75u8, 35u8, 120u8,
							167u8, 18u8, 132u8, 9u8, 20u8, 91u8, 51u8, 27u8, 69u8, 136u8, 187u8,
							13u8, 220u8, 163u8, 122u8, 26u8, 141u8, 174u8, 249u8, 85u8, 37u8,
						],
					)
				}
				#[doc = " Any liquidity locks on some account balances."]
				#[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
				#[doc = ""]
				#[doc = " Use of locks is deprecated in favour of freezes. See `https://github.com/paritytech/substrate/pull/12951/`"]
				pub fn locks(
					&self,
					_0: impl ::std::borrow::Borrow<types::locks::Param0>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageKey<types::locks::Param0>,
					types::locks::Locks,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ArgonBalances",
						"Locks",
						::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							10u8, 223u8, 55u8, 0u8, 249u8, 69u8, 168u8, 41u8, 75u8, 35u8, 120u8,
							167u8, 18u8, 132u8, 9u8, 20u8, 91u8, 51u8, 27u8, 69u8, 136u8, 187u8,
							13u8, 220u8, 163u8, 122u8, 26u8, 141u8, 174u8, 249u8, 85u8, 37u8,
						],
					)
				}
				#[doc = " Named reserves on some account balances."]
				#[doc = ""]
				#[doc = " Use of reserves is deprecated in favour of holds. See `https://github.com/paritytech/substrate/pull/12951/`"]
				pub fn reserves_iter(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::reserves::Reserves,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"ArgonBalances",
						"Reserves",
						(),
						[
							112u8, 10u8, 241u8, 77u8, 64u8, 187u8, 106u8, 159u8, 13u8, 153u8,
							140u8, 178u8, 182u8, 50u8, 1u8, 55u8, 149u8, 92u8, 196u8, 229u8, 170u8,
							106u8, 193u8, 88u8, 255u8, 244u8, 2u8, 193u8, 62u8, 235u8, 204u8, 91u8,
						],
					)
				}
				#[doc = " Named reserves on some account balances."]
				#[doc = ""]
				#[doc = " Use of reserves is deprecated in favour of holds. See `https://github.com/paritytech/substrate/pull/12951/`"]
				pub fn reserves(
					&self,
					_0: impl ::std::borrow::Borrow<types::reserves::Param0>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageKey<types::reserves::Param0>,
					types::reserves::Reserves,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ArgonBalances",
						"Reserves",
						::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							112u8, 10u8, 241u8, 77u8, 64u8, 187u8, 106u8, 159u8, 13u8, 153u8,
							140u8, 178u8, 182u8, 50u8, 1u8, 55u8, 149u8, 92u8, 196u8, 229u8, 170u8,
							106u8, 193u8, 88u8, 255u8, 244u8, 2u8, 193u8, 62u8, 235u8, 204u8, 91u8,
						],
					)
				}
				#[doc = " Holds on account balances."]
				pub fn holds_iter(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::holds::Holds,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"ArgonBalances",
						"Holds",
						(),
						[
							85u8, 24u8, 200u8, 7u8, 154u8, 94u8, 116u8, 110u8, 33u8, 50u8, 143u8,
							62u8, 93u8, 155u8, 53u8, 121u8, 132u8, 232u8, 173u8, 102u8, 117u8,
							201u8, 165u8, 121u8, 184u8, 147u8, 237u8, 67u8, 74u8, 66u8, 206u8,
							55u8,
						],
					)
				}
				#[doc = " Holds on account balances."]
				pub fn holds(
					&self,
					_0: impl ::std::borrow::Borrow<types::holds::Param0>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageKey<types::holds::Param0>,
					types::holds::Holds,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ArgonBalances",
						"Holds",
						::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							85u8, 24u8, 200u8, 7u8, 154u8, 94u8, 116u8, 110u8, 33u8, 50u8, 143u8,
							62u8, 93u8, 155u8, 53u8, 121u8, 132u8, 232u8, 173u8, 102u8, 117u8,
							201u8, 165u8, 121u8, 184u8, 147u8, 237u8, 67u8, 74u8, 66u8, 206u8,
							55u8,
						],
					)
				}
				#[doc = " Freeze locks on account balances."]
				pub fn freezes_iter(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::freezes::Freezes,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"ArgonBalances",
						"Freezes",
						(),
						[
							137u8, 54u8, 103u8, 63u8, 166u8, 153u8, 14u8, 79u8, 7u8, 65u8, 178u8,
							80u8, 204u8, 36u8, 206u8, 69u8, 194u8, 200u8, 174u8, 172u8, 20u8,
							157u8, 156u8, 101u8, 214u8, 98u8, 160u8, 16u8, 102u8, 198u8, 126u8,
							198u8,
						],
					)
				}
				#[doc = " Freeze locks on account balances."]
				pub fn freezes(
					&self,
					_0: impl ::std::borrow::Borrow<types::freezes::Param0>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageKey<types::freezes::Param0>,
					types::freezes::Freezes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ArgonBalances",
						"Freezes",
						::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							137u8, 54u8, 103u8, 63u8, 166u8, 153u8, 14u8, 79u8, 7u8, 65u8, 178u8,
							80u8, 204u8, 36u8, 206u8, 69u8, 194u8, 200u8, 174u8, 172u8, 20u8,
							157u8, 156u8, 101u8, 214u8, 98u8, 160u8, 16u8, 102u8, 198u8, 126u8,
							198u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The minimum amount required to keep an account open. MUST BE GREATER THAN ZERO!"]
				#[doc = ""]
				#[doc = " If you *really* need it to be zero, you can enable the feature `insecure_zero_ed` for"]
				#[doc = " this pallet. However, you do so at your own risk: this will open up a major DoS vector."]
				#[doc = " In case you have multiple sources of provider references, you may also get unexpected"]
				#[doc = " behaviour if you set this to zero."]
				#[doc = ""]
				#[doc = " Bottom line: Do yourself a favour and make it at least one!"]
				pub fn existential_deposit(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"ArgonBalances",
						"ExistentialDeposit",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				#[doc = " The maximum number of locks that should exist on an account."]
				#[doc = " Not strictly enforced, but used for weight estimation."]
				#[doc = ""]
				#[doc = " Use of locks is deprecated in favour of freezes. See `https://github.com/paritytech/substrate/pull/12951/`"]
				pub fn max_locks(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"ArgonBalances",
						"MaxLocks",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The maximum number of named reserves that can exist on an account."]
				#[doc = ""]
				#[doc = " Use of reserves is deprecated in favour of holds. See `https://github.com/paritytech/substrate/pull/12951/`"]
				pub fn max_reserves(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"ArgonBalances",
						"MaxReserves",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The maximum number of individual freeze locks that can exist on an account at any time."]
				pub fn max_freezes(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"ArgonBalances",
						"MaxFreezes",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod mint {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::pallet_mint::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_mint::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
			}
			pub struct TransactionApi;
			impl TransactionApi {}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_mint::pallet::Event;
		pub mod events {
			use super::runtime_types;
		}
		pub mod storage {
			use super::runtime_types;
			pub mod types {
				use super::runtime_types;
				pub mod ulixee_account_last_transfer_block {
					use super::runtime_types;
					pub type UlixeeAccountLastTransferBlock = ::core::primitive::u32;
					pub type Param0 = ::subxt::utils::AccountId32;
				}
			}
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Last moved block of ulixee tokens"]
				pub fn ulixee_account_last_transfer_block_iter(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::ulixee_account_last_transfer_block::UlixeeAccountLastTransferBlock,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Mint",
						"UlixeeAccountLastTransferBlock",
						(),
						[
							232u8, 20u8, 173u8, 109u8, 248u8, 121u8, 166u8, 86u8, 210u8, 179u8,
							221u8, 52u8, 245u8, 209u8, 218u8, 193u8, 1u8, 95u8, 84u8, 124u8, 97u8,
							252u8, 90u8, 88u8, 125u8, 51u8, 91u8, 134u8, 239u8, 21u8, 124u8, 41u8,
						],
					)
				}
				#[doc = " Last moved block of ulixee tokens"]
				pub fn ulixee_account_last_transfer_block(
					&self,
					_0: impl ::std::borrow::Borrow<types::ulixee_account_last_transfer_block::Param0>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageKey<
						types::ulixee_account_last_transfer_block::Param0,
					>,
					types::ulixee_account_last_transfer_block::UlixeeAccountLastTransferBlock,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Mint",
						"UlixeeAccountLastTransferBlock",
						::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							232u8, 20u8, 173u8, 109u8, 248u8, 121u8, 166u8, 86u8, 210u8, 179u8,
							221u8, 52u8, 245u8, 209u8, 218u8, 193u8, 1u8, 95u8, 84u8, 124u8, 97u8,
							252u8, 90u8, 88u8, 125u8, 51u8, 91u8, 134u8, 239u8, 21u8, 124u8, 41u8,
						],
					)
				}
			}
		}
	}
	pub mod ulixee_balances {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::pallet_balances::pallet::Error2;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_balances::pallet::Call2;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Transfer some liquid free balance to another account."]
				#[doc = ""]
				#[doc = "`transfer_allow_death` will set the `FreeBalance` of the sender and receiver."]
				#[doc = "If the sender's account is below the existential deposit as a result"]
				#[doc = "of the transfer, the account will be reaped."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
				pub struct TransferAllowDeath {
					pub dest: transfer_allow_death::Dest,
					#[codec(compact)]
					pub value: transfer_allow_death::Value,
				}
				pub mod transfer_allow_death {
					use super::runtime_types;
					pub type Dest = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
					pub type Value = ::core::primitive::u128;
				}
				impl ::subxt::blocks::StaticExtrinsic for TransferAllowDeath {
					const PALLET: &'static str = "UlixeeBalances";
					const CALL: &'static str = "transfer_allow_death";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Exactly as `transfer_allow_death`, except the origin must be root and the source account"]
				#[doc = "may be specified."]
				pub struct ForceTransfer {
					pub source: force_transfer::Source,
					pub dest: force_transfer::Dest,
					#[codec(compact)]
					pub value: force_transfer::Value,
				}
				pub mod force_transfer {
					use super::runtime_types;
					pub type Source = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
					pub type Dest = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
					pub type Value = ::core::primitive::u128;
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceTransfer {
					const PALLET: &'static str = "UlixeeBalances";
					const CALL: &'static str = "force_transfer";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Same as the [`transfer_allow_death`] call, but with a check that the transfer will not"]
				#[doc = "kill the origin account."]
				#[doc = ""]
				#[doc = "99% of the time you want [`transfer_allow_death`] instead."]
				#[doc = ""]
				#[doc = "[`transfer_allow_death`]: struct.Pallet.html#method.transfer"]
				pub struct TransferKeepAlive {
					pub dest: transfer_keep_alive::Dest,
					#[codec(compact)]
					pub value: transfer_keep_alive::Value,
				}
				pub mod transfer_keep_alive {
					use super::runtime_types;
					pub type Dest = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
					pub type Value = ::core::primitive::u128;
				}
				impl ::subxt::blocks::StaticExtrinsic for TransferKeepAlive {
					const PALLET: &'static str = "UlixeeBalances";
					const CALL: &'static str = "transfer_keep_alive";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Transfer the entire transferable balance from the caller account."]
				#[doc = ""]
				#[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
				#[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
				#[doc = "transferred by this function. To ensure that this function results in a killed account,"]
				#[doc = "you might need to prepare the account by removing any reference counters, storage"]
				#[doc = "deposits, etc..."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be Signed."]
				#[doc = ""]
				#[doc = "- `dest`: The recipient of the transfer."]
				#[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
				#[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
				#[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
				#[doc = "  keep the sender account alive (true)."]
				pub struct TransferAll {
					pub dest: transfer_all::Dest,
					pub keep_alive: transfer_all::KeepAlive,
				}
				pub mod transfer_all {
					use super::runtime_types;
					pub type Dest = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
					pub type KeepAlive = ::core::primitive::bool;
				}
				impl ::subxt::blocks::StaticExtrinsic for TransferAll {
					const PALLET: &'static str = "UlixeeBalances";
					const CALL: &'static str = "transfer_all";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Unreserve some balance from a user by force."]
				#[doc = ""]
				#[doc = "Can only be called by ROOT."]
				pub struct ForceUnreserve {
					pub who: force_unreserve::Who,
					pub amount: force_unreserve::Amount,
				}
				pub mod force_unreserve {
					use super::runtime_types;
					pub type Who = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
					pub type Amount = ::core::primitive::u128;
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceUnreserve {
					const PALLET: &'static str = "UlixeeBalances";
					const CALL: &'static str = "force_unreserve";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Upgrade a specified account."]
				#[doc = ""]
				#[doc = "- `origin`: Must be `Signed`."]
				#[doc = "- `who`: The account to be upgraded."]
				#[doc = ""]
				#[doc = "This will waive the transaction fee if at least all but 10% of the accounts needed to"]
				#[doc = "be upgraded. (We let some not have to be upgraded just in order to allow for the"]
				#[doc = "possibility of churn)."]
				pub struct UpgradeAccounts {
					pub who: upgrade_accounts::Who,
				}
				pub mod upgrade_accounts {
					use super::runtime_types;
					pub type Who = ::std::vec::Vec<::subxt::utils::AccountId32>;
				}
				impl ::subxt::blocks::StaticExtrinsic for UpgradeAccounts {
					const PALLET: &'static str = "UlixeeBalances";
					const CALL: &'static str = "upgrade_accounts";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Set the regular balance of a given account."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call is `root`."]
				pub struct ForceSetBalance {
					pub who: force_set_balance::Who,
					#[codec(compact)]
					pub new_free: force_set_balance::NewFree,
				}
				pub mod force_set_balance {
					use super::runtime_types;
					pub type Who = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
					pub type NewFree = ::core::primitive::u128;
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceSetBalance {
					const PALLET: &'static str = "UlixeeBalances";
					const CALL: &'static str = "force_set_balance";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Adjust the total issuance in a saturating way."]
				#[doc = ""]
				#[doc = "Can only be called by root and always needs a positive `delta`."]
				#[doc = ""]
				#[doc = "# Example"]
				pub struct ForceAdjustTotalIssuance {
					pub direction: force_adjust_total_issuance::Direction,
					#[codec(compact)]
					pub delta: force_adjust_total_issuance::Delta,
				}
				pub mod force_adjust_total_issuance {
					use super::runtime_types;
					pub type Direction = runtime_types::pallet_balances::types::AdjustmentDirection;
					pub type Delta = ::core::primitive::u128;
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceAdjustTotalIssuance {
					const PALLET: &'static str = "UlixeeBalances";
					const CALL: &'static str = "force_adjust_total_issuance";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Transfer some liquid free balance to another account."]
				#[doc = ""]
				#[doc = "`transfer_allow_death` will set the `FreeBalance` of the sender and receiver."]
				#[doc = "If the sender's account is below the existential deposit as a result"]
				#[doc = "of the transfer, the account will be reaped."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
				pub fn transfer_allow_death(
					&self,
					dest: types::transfer_allow_death::Dest,
					value: types::transfer_allow_death::Value,
				) -> ::subxt::tx::Payload<types::TransferAllowDeath> {
					::subxt::tx::Payload::new_static(
						"UlixeeBalances",
						"transfer_allow_death",
						types::TransferAllowDeath { dest, value },
						[
							51u8, 166u8, 195u8, 10u8, 139u8, 218u8, 55u8, 130u8, 6u8, 194u8, 35u8,
							140u8, 27u8, 205u8, 214u8, 222u8, 102u8, 43u8, 143u8, 145u8, 86u8,
							219u8, 210u8, 147u8, 13u8, 39u8, 51u8, 21u8, 237u8, 179u8, 132u8,
							130u8,
						],
					)
				}
				#[doc = "Exactly as `transfer_allow_death`, except the origin must be root and the source account"]
				#[doc = "may be specified."]
				pub fn force_transfer(
					&self,
					source: types::force_transfer::Source,
					dest: types::force_transfer::Dest,
					value: types::force_transfer::Value,
				) -> ::subxt::tx::Payload<types::ForceTransfer> {
					::subxt::tx::Payload::new_static(
						"UlixeeBalances",
						"force_transfer",
						types::ForceTransfer { source, dest, value },
						[
							154u8, 93u8, 222u8, 27u8, 12u8, 248u8, 63u8, 213u8, 224u8, 86u8, 250u8,
							153u8, 249u8, 102u8, 83u8, 160u8, 79u8, 125u8, 105u8, 222u8, 77u8,
							180u8, 90u8, 105u8, 81u8, 217u8, 60u8, 25u8, 213u8, 51u8, 185u8, 96u8,
						],
					)
				}
				#[doc = "Same as the [`transfer_allow_death`] call, but with a check that the transfer will not"]
				#[doc = "kill the origin account."]
				#[doc = ""]
				#[doc = "99% of the time you want [`transfer_allow_death`] instead."]
				#[doc = ""]
				#[doc = "[`transfer_allow_death`]: struct.Pallet.html#method.transfer"]
				pub fn transfer_keep_alive(
					&self,
					dest: types::transfer_keep_alive::Dest,
					value: types::transfer_keep_alive::Value,
				) -> ::subxt::tx::Payload<types::TransferKeepAlive> {
					::subxt::tx::Payload::new_static(
						"UlixeeBalances",
						"transfer_keep_alive",
						types::TransferKeepAlive { dest, value },
						[
							245u8, 14u8, 190u8, 193u8, 32u8, 210u8, 74u8, 92u8, 25u8, 182u8, 76u8,
							55u8, 247u8, 83u8, 114u8, 75u8, 143u8, 236u8, 117u8, 25u8, 54u8, 157u8,
							208u8, 207u8, 233u8, 89u8, 70u8, 161u8, 235u8, 242u8, 222u8, 59u8,
						],
					)
				}
				#[doc = "Transfer the entire transferable balance from the caller account."]
				#[doc = ""]
				#[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
				#[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
				#[doc = "transferred by this function. To ensure that this function results in a killed account,"]
				#[doc = "you might need to prepare the account by removing any reference counters, storage"]
				#[doc = "deposits, etc..."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be Signed."]
				#[doc = ""]
				#[doc = "- `dest`: The recipient of the transfer."]
				#[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
				#[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
				#[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
				#[doc = "  keep the sender account alive (true)."]
				pub fn transfer_all(
					&self,
					dest: types::transfer_all::Dest,
					keep_alive: types::transfer_all::KeepAlive,
				) -> ::subxt::tx::Payload<types::TransferAll> {
					::subxt::tx::Payload::new_static(
						"UlixeeBalances",
						"transfer_all",
						types::TransferAll { dest, keep_alive },
						[
							105u8, 132u8, 49u8, 144u8, 195u8, 250u8, 34u8, 46u8, 213u8, 248u8,
							112u8, 188u8, 81u8, 228u8, 136u8, 18u8, 67u8, 172u8, 37u8, 38u8, 238u8,
							9u8, 34u8, 15u8, 67u8, 34u8, 148u8, 195u8, 223u8, 29u8, 154u8, 6u8,
						],
					)
				}
				#[doc = "Unreserve some balance from a user by force."]
				#[doc = ""]
				#[doc = "Can only be called by ROOT."]
				pub fn force_unreserve(
					&self,
					who: types::force_unreserve::Who,
					amount: types::force_unreserve::Amount,
				) -> ::subxt::tx::Payload<types::ForceUnreserve> {
					::subxt::tx::Payload::new_static(
						"UlixeeBalances",
						"force_unreserve",
						types::ForceUnreserve { who, amount },
						[
							142u8, 151u8, 64u8, 205u8, 46u8, 64u8, 62u8, 122u8, 108u8, 49u8, 223u8,
							140u8, 120u8, 153u8, 35u8, 165u8, 187u8, 38u8, 157u8, 200u8, 123u8,
							199u8, 198u8, 168u8, 208u8, 159u8, 39u8, 134u8, 92u8, 103u8, 84u8,
							171u8,
						],
					)
				}
				#[doc = "Upgrade a specified account."]
				#[doc = ""]
				#[doc = "- `origin`: Must be `Signed`."]
				#[doc = "- `who`: The account to be upgraded."]
				#[doc = ""]
				#[doc = "This will waive the transaction fee if at least all but 10% of the accounts needed to"]
				#[doc = "be upgraded. (We let some not have to be upgraded just in order to allow for the"]
				#[doc = "possibility of churn)."]
				pub fn upgrade_accounts(
					&self,
					who: types::upgrade_accounts::Who,
				) -> ::subxt::tx::Payload<types::UpgradeAccounts> {
					::subxt::tx::Payload::new_static(
						"UlixeeBalances",
						"upgrade_accounts",
						types::UpgradeAccounts { who },
						[
							66u8, 200u8, 179u8, 104u8, 65u8, 2u8, 101u8, 56u8, 130u8, 161u8, 224u8,
							233u8, 255u8, 124u8, 70u8, 122u8, 8u8, 49u8, 103u8, 178u8, 68u8, 47u8,
							214u8, 166u8, 217u8, 116u8, 178u8, 50u8, 212u8, 164u8, 98u8, 226u8,
						],
					)
				}
				#[doc = "Set the regular balance of a given account."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call is `root`."]
				pub fn force_set_balance(
					&self,
					who: types::force_set_balance::Who,
					new_free: types::force_set_balance::NewFree,
				) -> ::subxt::tx::Payload<types::ForceSetBalance> {
					::subxt::tx::Payload::new_static(
						"UlixeeBalances",
						"force_set_balance",
						types::ForceSetBalance { who, new_free },
						[
							114u8, 229u8, 59u8, 204u8, 180u8, 83u8, 17u8, 4u8, 59u8, 4u8, 55u8,
							39u8, 151u8, 196u8, 124u8, 60u8, 209u8, 65u8, 193u8, 11u8, 44u8, 164u8,
							116u8, 93u8, 169u8, 30u8, 199u8, 165u8, 55u8, 231u8, 223u8, 43u8,
						],
					)
				}
				#[doc = "Adjust the total issuance in a saturating way."]
				#[doc = ""]
				#[doc = "Can only be called by root and always needs a positive `delta`."]
				#[doc = ""]
				#[doc = "# Example"]
				pub fn force_adjust_total_issuance(
					&self,
					direction: types::force_adjust_total_issuance::Direction,
					delta: types::force_adjust_total_issuance::Delta,
				) -> ::subxt::tx::Payload<types::ForceAdjustTotalIssuance> {
					::subxt::tx::Payload::new_static(
						"UlixeeBalances",
						"force_adjust_total_issuance",
						types::ForceAdjustTotalIssuance { direction, delta },
						[
							208u8, 134u8, 56u8, 133u8, 232u8, 164u8, 10u8, 213u8, 53u8, 193u8,
							190u8, 63u8, 236u8, 186u8, 96u8, 122u8, 104u8, 87u8, 173u8, 38u8, 58u8,
							176u8, 21u8, 78u8, 42u8, 106u8, 46u8, 248u8, 251u8, 190u8, 150u8,
							202u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_balances::pallet::Event2;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "An account was created with some free balance."]
			pub struct Endowed {
				pub account: endowed::Account,
				pub free_balance: endowed::FreeBalance,
			}
			pub mod endowed {
				use super::runtime_types;
				pub type Account = ::subxt::utils::AccountId32;
				pub type FreeBalance = ::core::primitive::u128;
			}
			impl ::subxt::events::StaticEvent for Endowed {
				const PALLET: &'static str = "UlixeeBalances";
				const EVENT: &'static str = "Endowed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
			#[doc = "resulting in an outright loss."]
			pub struct DustLost {
				pub account: dust_lost::Account,
				pub amount: dust_lost::Amount,
			}
			pub mod dust_lost {
				use super::runtime_types;
				pub type Account = ::subxt::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::events::StaticEvent for DustLost {
				const PALLET: &'static str = "UlixeeBalances";
				const EVENT: &'static str = "DustLost";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Transfer succeeded."]
			pub struct Transfer {
				pub from: transfer::From,
				pub to: transfer::To,
				pub amount: transfer::Amount,
			}
			pub mod transfer {
				use super::runtime_types;
				pub type From = ::subxt::utils::AccountId32;
				pub type To = ::subxt::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::events::StaticEvent for Transfer {
				const PALLET: &'static str = "UlixeeBalances";
				const EVENT: &'static str = "Transfer";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A balance was set by root."]
			pub struct BalanceSet {
				pub who: balance_set::Who,
				pub free: balance_set::Free,
			}
			pub mod balance_set {
				use super::runtime_types;
				pub type Who = ::subxt::utils::AccountId32;
				pub type Free = ::core::primitive::u128;
			}
			impl ::subxt::events::StaticEvent for BalanceSet {
				const PALLET: &'static str = "UlixeeBalances";
				const EVENT: &'static str = "BalanceSet";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some balance was reserved (moved from free to reserved)."]
			pub struct Reserved {
				pub who: reserved::Who,
				pub amount: reserved::Amount,
			}
			pub mod reserved {
				use super::runtime_types;
				pub type Who = ::subxt::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::events::StaticEvent for Reserved {
				const PALLET: &'static str = "UlixeeBalances";
				const EVENT: &'static str = "Reserved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some balance was unreserved (moved from reserved to free)."]
			pub struct Unreserved {
				pub who: unreserved::Who,
				pub amount: unreserved::Amount,
			}
			pub mod unreserved {
				use super::runtime_types;
				pub type Who = ::subxt::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::events::StaticEvent for Unreserved {
				const PALLET: &'static str = "UlixeeBalances";
				const EVENT: &'static str = "Unreserved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some balance was moved from the reserve of the first account to the second account."]
			#[doc = "Final argument indicates the destination balance type."]
			pub struct ReserveRepatriated {
				pub from: reserve_repatriated::From,
				pub to: reserve_repatriated::To,
				pub amount: reserve_repatriated::Amount,
				pub destination_status: reserve_repatriated::DestinationStatus,
			}
			pub mod reserve_repatriated {
				use super::runtime_types;
				pub type From = ::subxt::utils::AccountId32;
				pub type To = ::subxt::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
				pub type DestinationStatus =
					runtime_types::frame_support::traits::tokens::misc::BalanceStatus;
			}
			impl ::subxt::events::StaticEvent for ReserveRepatriated {
				const PALLET: &'static str = "UlixeeBalances";
				const EVENT: &'static str = "ReserveRepatriated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some amount was deposited (e.g. for transaction fees)."]
			pub struct Deposit {
				pub who: deposit::Who,
				pub amount: deposit::Amount,
			}
			pub mod deposit {
				use super::runtime_types;
				pub type Who = ::subxt::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::events::StaticEvent for Deposit {
				const PALLET: &'static str = "UlixeeBalances";
				const EVENT: &'static str = "Deposit";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
			pub struct Withdraw {
				pub who: withdraw::Who,
				pub amount: withdraw::Amount,
			}
			pub mod withdraw {
				use super::runtime_types;
				pub type Who = ::subxt::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::events::StaticEvent for Withdraw {
				const PALLET: &'static str = "UlixeeBalances";
				const EVENT: &'static str = "Withdraw";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
			pub struct Slashed {
				pub who: slashed::Who,
				pub amount: slashed::Amount,
			}
			pub mod slashed {
				use super::runtime_types;
				pub type Who = ::subxt::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::events::StaticEvent for Slashed {
				const PALLET: &'static str = "UlixeeBalances";
				const EVENT: &'static str = "Slashed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some amount was minted into an account."]
			pub struct Minted {
				pub who: minted::Who,
				pub amount: minted::Amount,
			}
			pub mod minted {
				use super::runtime_types;
				pub type Who = ::subxt::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::events::StaticEvent for Minted {
				const PALLET: &'static str = "UlixeeBalances";
				const EVENT: &'static str = "Minted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some amount was burned from an account."]
			pub struct Burned {
				pub who: burned::Who,
				pub amount: burned::Amount,
			}
			pub mod burned {
				use super::runtime_types;
				pub type Who = ::subxt::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::events::StaticEvent for Burned {
				const PALLET: &'static str = "UlixeeBalances";
				const EVENT: &'static str = "Burned";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some amount was suspended from an account (it can be restored later)."]
			pub struct Suspended {
				pub who: suspended::Who,
				pub amount: suspended::Amount,
			}
			pub mod suspended {
				use super::runtime_types;
				pub type Who = ::subxt::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::events::StaticEvent for Suspended {
				const PALLET: &'static str = "UlixeeBalances";
				const EVENT: &'static str = "Suspended";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some amount was restored into an account."]
			pub struct Restored {
				pub who: restored::Who,
				pub amount: restored::Amount,
			}
			pub mod restored {
				use super::runtime_types;
				pub type Who = ::subxt::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::events::StaticEvent for Restored {
				const PALLET: &'static str = "UlixeeBalances";
				const EVENT: &'static str = "Restored";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "An account was upgraded."]
			pub struct Upgraded {
				pub who: upgraded::Who,
			}
			pub mod upgraded {
				use super::runtime_types;
				pub type Who = ::subxt::utils::AccountId32;
			}
			impl ::subxt::events::StaticEvent for Upgraded {
				const PALLET: &'static str = "UlixeeBalances";
				const EVENT: &'static str = "Upgraded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Total issuance was increased by `amount`, creating a credit to be balanced."]
			pub struct Issued {
				pub amount: issued::Amount,
			}
			pub mod issued {
				use super::runtime_types;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::events::StaticEvent for Issued {
				const PALLET: &'static str = "UlixeeBalances";
				const EVENT: &'static str = "Issued";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Total issuance was decreased by `amount`, creating a debt to be balanced."]
			pub struct Rescinded {
				pub amount: rescinded::Amount,
			}
			pub mod rescinded {
				use super::runtime_types;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::events::StaticEvent for Rescinded {
				const PALLET: &'static str = "UlixeeBalances";
				const EVENT: &'static str = "Rescinded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some balance was locked."]
			pub struct Locked {
				pub who: locked::Who,
				pub amount: locked::Amount,
			}
			pub mod locked {
				use super::runtime_types;
				pub type Who = ::subxt::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::events::StaticEvent for Locked {
				const PALLET: &'static str = "UlixeeBalances";
				const EVENT: &'static str = "Locked";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some balance was unlocked."]
			pub struct Unlocked {
				pub who: unlocked::Who,
				pub amount: unlocked::Amount,
			}
			pub mod unlocked {
				use super::runtime_types;
				pub type Who = ::subxt::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::events::StaticEvent for Unlocked {
				const PALLET: &'static str = "UlixeeBalances";
				const EVENT: &'static str = "Unlocked";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some balance was frozen."]
			pub struct Frozen {
				pub who: frozen::Who,
				pub amount: frozen::Amount,
			}
			pub mod frozen {
				use super::runtime_types;
				pub type Who = ::subxt::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::events::StaticEvent for Frozen {
				const PALLET: &'static str = "UlixeeBalances";
				const EVENT: &'static str = "Frozen";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some balance was thawed."]
			pub struct Thawed {
				pub who: thawed::Who,
				pub amount: thawed::Amount,
			}
			pub mod thawed {
				use super::runtime_types;
				pub type Who = ::subxt::utils::AccountId32;
				pub type Amount = ::core::primitive::u128;
			}
			impl ::subxt::events::StaticEvent for Thawed {
				const PALLET: &'static str = "UlixeeBalances";
				const EVENT: &'static str = "Thawed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "The `TotalIssuance` was forcefully changed."]
			pub struct TotalIssuanceForced {
				pub old: total_issuance_forced::Old,
				pub new: total_issuance_forced::New,
			}
			pub mod total_issuance_forced {
				use super::runtime_types;
				pub type Old = ::core::primitive::u128;
				pub type New = ::core::primitive::u128;
			}
			impl ::subxt::events::StaticEvent for TotalIssuanceForced {
				const PALLET: &'static str = "UlixeeBalances";
				const EVENT: &'static str = "TotalIssuanceForced";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub mod types {
				use super::runtime_types;
				pub mod total_issuance {
					use super::runtime_types;
					pub type TotalIssuance = ::core::primitive::u128;
				}
				pub mod inactive_issuance {
					use super::runtime_types;
					pub type InactiveIssuance = ::core::primitive::u128;
				}
				pub mod account {
					use super::runtime_types;
					pub type Account =
						runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>;
					pub type Param0 = ::subxt::utils::AccountId32;
				}
				pub mod locks {
					use super::runtime_types;
					pub type Locks =
						runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
							runtime_types::pallet_balances::types::BalanceLock<
								::core::primitive::u128,
							>,
						>;
					pub type Param0 = ::subxt::utils::AccountId32;
				}
				pub mod reserves {
					use super::runtime_types;
					pub type Reserves = runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::pallet_balances::types::ReserveData<
							[::core::primitive::u8; 8usize],
							::core::primitive::u128,
						>,
					>;
					pub type Param0 = ::subxt::utils::AccountId32;
				}
				pub mod holds {
					use super::runtime_types;
					pub type Holds = runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::pallet_balances::types::IdAmount<
							runtime_types::ulx_node_runtime::RuntimeHoldReason,
							::core::primitive::u128,
						>,
					>;
					pub type Param0 = ::subxt::utils::AccountId32;
				}
				pub mod freezes {
					use super::runtime_types;
					pub type Freezes = runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::pallet_balances::types::IdAmount<
							runtime_types::ulx_node_runtime::RuntimeFreezeReason,
							::core::primitive::u128,
						>,
					>;
					pub type Param0 = ::subxt::utils::AccountId32;
				}
			}
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The total units issued in the system."]
				pub fn total_issuance(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::total_issuance::TotalIssuance,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"UlixeeBalances",
						"TotalIssuance",
						(),
						[
							116u8, 70u8, 119u8, 194u8, 69u8, 37u8, 116u8, 206u8, 171u8, 70u8,
							171u8, 210u8, 226u8, 111u8, 184u8, 204u8, 206u8, 11u8, 68u8, 72u8,
							255u8, 19u8, 194u8, 11u8, 27u8, 194u8, 81u8, 204u8, 59u8, 224u8, 202u8,
							185u8,
						],
					)
				}
				#[doc = " The total units of outstanding deactivated balance in the system."]
				pub fn inactive_issuance(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::inactive_issuance::InactiveIssuance,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"UlixeeBalances",
						"InactiveIssuance",
						(),
						[
							212u8, 185u8, 19u8, 50u8, 250u8, 72u8, 173u8, 50u8, 4u8, 104u8, 161u8,
							249u8, 77u8, 247u8, 204u8, 248u8, 11u8, 18u8, 57u8, 4u8, 82u8, 110u8,
							30u8, 216u8, 16u8, 37u8, 87u8, 67u8, 189u8, 235u8, 214u8, 155u8,
						],
					)
				}
				#[doc = " The Balances pallet example of storing the balance of an account."]
				#[doc = ""]
				#[doc = " # Example"]
				#[doc = ""]
				#[doc = " ```nocompile"]
				#[doc = "  impl pallet_balances::Config for Runtime {"]
				#[doc = "    type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>"]
				#[doc = "  }"]
				#[doc = " ```"]
				#[doc = ""]
				#[doc = " You can also store the balance of an account in the `System` pallet."]
				#[doc = ""]
				#[doc = " # Example"]
				#[doc = ""]
				#[doc = " ```nocompile"]
				#[doc = "  impl pallet_balances::Config for Runtime {"]
				#[doc = "   type AccountStore = System"]
				#[doc = "  }"]
				#[doc = " ```"]
				#[doc = ""]
				#[doc = " But this comes with tradeoffs, storing account balances in the system pallet stores"]
				#[doc = " `frame_system` data alongside the account data contrary to storing account balances in the"]
				#[doc = " `Balances` pallet, which uses a `StorageMap` to store balances data only."]
				#[doc = " NOTE: This is only used in the case that this pallet is used to store balances."]
				pub fn account_iter(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::account::Account,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"UlixeeBalances",
						"Account",
						(),
						[
							213u8, 38u8, 200u8, 69u8, 218u8, 0u8, 112u8, 181u8, 160u8, 23u8, 96u8,
							90u8, 3u8, 88u8, 126u8, 22u8, 103u8, 74u8, 64u8, 69u8, 29u8, 247u8,
							18u8, 17u8, 234u8, 143u8, 189u8, 22u8, 247u8, 194u8, 154u8, 249u8,
						],
					)
				}
				#[doc = " The Balances pallet example of storing the balance of an account."]
				#[doc = ""]
				#[doc = " # Example"]
				#[doc = ""]
				#[doc = " ```nocompile"]
				#[doc = "  impl pallet_balances::Config for Runtime {"]
				#[doc = "    type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>"]
				#[doc = "  }"]
				#[doc = " ```"]
				#[doc = ""]
				#[doc = " You can also store the balance of an account in the `System` pallet."]
				#[doc = ""]
				#[doc = " # Example"]
				#[doc = ""]
				#[doc = " ```nocompile"]
				#[doc = "  impl pallet_balances::Config for Runtime {"]
				#[doc = "   type AccountStore = System"]
				#[doc = "  }"]
				#[doc = " ```"]
				#[doc = ""]
				#[doc = " But this comes with tradeoffs, storing account balances in the system pallet stores"]
				#[doc = " `frame_system` data alongside the account data contrary to storing account balances in the"]
				#[doc = " `Balances` pallet, which uses a `StorageMap` to store balances data only."]
				#[doc = " NOTE: This is only used in the case that this pallet is used to store balances."]
				pub fn account(
					&self,
					_0: impl ::std::borrow::Borrow<types::account::Param0>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageKey<types::account::Param0>,
					types::account::Account,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"UlixeeBalances",
						"Account",
						::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							213u8, 38u8, 200u8, 69u8, 218u8, 0u8, 112u8, 181u8, 160u8, 23u8, 96u8,
							90u8, 3u8, 88u8, 126u8, 22u8, 103u8, 74u8, 64u8, 69u8, 29u8, 247u8,
							18u8, 17u8, 234u8, 143u8, 189u8, 22u8, 247u8, 194u8, 154u8, 249u8,
						],
					)
				}
				#[doc = " Any liquidity locks on some account balances."]
				#[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
				#[doc = ""]
				#[doc = " Use of locks is deprecated in favour of freezes. See `https://github.com/paritytech/substrate/pull/12951/`"]
				pub fn locks_iter(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::locks::Locks,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"UlixeeBalances",
						"Locks",
						(),
						[
							10u8, 223u8, 55u8, 0u8, 249u8, 69u8, 168u8, 41u8, 75u8, 35u8, 120u8,
							167u8, 18u8, 132u8, 9u8, 20u8, 91u8, 51u8, 27u8, 69u8, 136u8, 187u8,
							13u8, 220u8, 163u8, 122u8, 26u8, 141u8, 174u8, 249u8, 85u8, 37u8,
						],
					)
				}
				#[doc = " Any liquidity locks on some account balances."]
				#[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
				#[doc = ""]
				#[doc = " Use of locks is deprecated in favour of freezes. See `https://github.com/paritytech/substrate/pull/12951/`"]
				pub fn locks(
					&self,
					_0: impl ::std::borrow::Borrow<types::locks::Param0>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageKey<types::locks::Param0>,
					types::locks::Locks,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"UlixeeBalances",
						"Locks",
						::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							10u8, 223u8, 55u8, 0u8, 249u8, 69u8, 168u8, 41u8, 75u8, 35u8, 120u8,
							167u8, 18u8, 132u8, 9u8, 20u8, 91u8, 51u8, 27u8, 69u8, 136u8, 187u8,
							13u8, 220u8, 163u8, 122u8, 26u8, 141u8, 174u8, 249u8, 85u8, 37u8,
						],
					)
				}
				#[doc = " Named reserves on some account balances."]
				#[doc = ""]
				#[doc = " Use of reserves is deprecated in favour of holds. See `https://github.com/paritytech/substrate/pull/12951/`"]
				pub fn reserves_iter(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::reserves::Reserves,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"UlixeeBalances",
						"Reserves",
						(),
						[
							112u8, 10u8, 241u8, 77u8, 64u8, 187u8, 106u8, 159u8, 13u8, 153u8,
							140u8, 178u8, 182u8, 50u8, 1u8, 55u8, 149u8, 92u8, 196u8, 229u8, 170u8,
							106u8, 193u8, 88u8, 255u8, 244u8, 2u8, 193u8, 62u8, 235u8, 204u8, 91u8,
						],
					)
				}
				#[doc = " Named reserves on some account balances."]
				#[doc = ""]
				#[doc = " Use of reserves is deprecated in favour of holds. See `https://github.com/paritytech/substrate/pull/12951/`"]
				pub fn reserves(
					&self,
					_0: impl ::std::borrow::Borrow<types::reserves::Param0>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageKey<types::reserves::Param0>,
					types::reserves::Reserves,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"UlixeeBalances",
						"Reserves",
						::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							112u8, 10u8, 241u8, 77u8, 64u8, 187u8, 106u8, 159u8, 13u8, 153u8,
							140u8, 178u8, 182u8, 50u8, 1u8, 55u8, 149u8, 92u8, 196u8, 229u8, 170u8,
							106u8, 193u8, 88u8, 255u8, 244u8, 2u8, 193u8, 62u8, 235u8, 204u8, 91u8,
						],
					)
				}
				#[doc = " Holds on account balances."]
				pub fn holds_iter(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::holds::Holds,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"UlixeeBalances",
						"Holds",
						(),
						[
							85u8, 24u8, 200u8, 7u8, 154u8, 94u8, 116u8, 110u8, 33u8, 50u8, 143u8,
							62u8, 93u8, 155u8, 53u8, 121u8, 132u8, 232u8, 173u8, 102u8, 117u8,
							201u8, 165u8, 121u8, 184u8, 147u8, 237u8, 67u8, 74u8, 66u8, 206u8,
							55u8,
						],
					)
				}
				#[doc = " Holds on account balances."]
				pub fn holds(
					&self,
					_0: impl ::std::borrow::Borrow<types::holds::Param0>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageKey<types::holds::Param0>,
					types::holds::Holds,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"UlixeeBalances",
						"Holds",
						::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							85u8, 24u8, 200u8, 7u8, 154u8, 94u8, 116u8, 110u8, 33u8, 50u8, 143u8,
							62u8, 93u8, 155u8, 53u8, 121u8, 132u8, 232u8, 173u8, 102u8, 117u8,
							201u8, 165u8, 121u8, 184u8, 147u8, 237u8, 67u8, 74u8, 66u8, 206u8,
							55u8,
						],
					)
				}
				#[doc = " Freeze locks on account balances."]
				pub fn freezes_iter(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::freezes::Freezes,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"UlixeeBalances",
						"Freezes",
						(),
						[
							137u8, 54u8, 103u8, 63u8, 166u8, 153u8, 14u8, 79u8, 7u8, 65u8, 178u8,
							80u8, 204u8, 36u8, 206u8, 69u8, 194u8, 200u8, 174u8, 172u8, 20u8,
							157u8, 156u8, 101u8, 214u8, 98u8, 160u8, 16u8, 102u8, 198u8, 126u8,
							198u8,
						],
					)
				}
				#[doc = " Freeze locks on account balances."]
				pub fn freezes(
					&self,
					_0: impl ::std::borrow::Borrow<types::freezes::Param0>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageKey<types::freezes::Param0>,
					types::freezes::Freezes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"UlixeeBalances",
						"Freezes",
						::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							137u8, 54u8, 103u8, 63u8, 166u8, 153u8, 14u8, 79u8, 7u8, 65u8, 178u8,
							80u8, 204u8, 36u8, 206u8, 69u8, 194u8, 200u8, 174u8, 172u8, 20u8,
							157u8, 156u8, 101u8, 214u8, 98u8, 160u8, 16u8, 102u8, 198u8, 126u8,
							198u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The minimum amount required to keep an account open. MUST BE GREATER THAN ZERO!"]
				#[doc = ""]
				#[doc = " If you *really* need it to be zero, you can enable the feature `insecure_zero_ed` for"]
				#[doc = " this pallet. However, you do so at your own risk: this will open up a major DoS vector."]
				#[doc = " In case you have multiple sources of provider references, you may also get unexpected"]
				#[doc = " behaviour if you set this to zero."]
				#[doc = ""]
				#[doc = " Bottom line: Do yourself a favour and make it at least one!"]
				pub fn existential_deposit(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"UlixeeBalances",
						"ExistentialDeposit",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				#[doc = " The maximum number of locks that should exist on an account."]
				#[doc = " Not strictly enforced, but used for weight estimation."]
				#[doc = ""]
				#[doc = " Use of locks is deprecated in favour of freezes. See `https://github.com/paritytech/substrate/pull/12951/`"]
				pub fn max_locks(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"UlixeeBalances",
						"MaxLocks",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The maximum number of named reserves that can exist on an account."]
				#[doc = ""]
				#[doc = " Use of reserves is deprecated in favour of holds. See `https://github.com/paritytech/substrate/pull/12951/`"]
				pub fn max_reserves(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"UlixeeBalances",
						"MaxReserves",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The maximum number of individual freeze locks that can exist on an account at any time."]
				pub fn max_freezes(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"UlixeeBalances",
						"MaxFreezes",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod tx_pause {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::pallet_tx_pause::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_tx_pause::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Pause a call."]
				#[doc = ""]
				#[doc = "Can only be called by [`Config::PauseOrigin`]."]
				#[doc = "Emits an [`Event::CallPaused`] event on success."]
				pub struct Pause {
					pub full_name: pause::FullName,
				}
				pub mod pause {
					use super::runtime_types;
					pub type FullName = (
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
					);
				}
				impl ::subxt::blocks::StaticExtrinsic for Pause {
					const PALLET: &'static str = "TxPause";
					const CALL: &'static str = "pause";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Un-pause a call."]
				#[doc = ""]
				#[doc = "Can only be called by [`Config::UnpauseOrigin`]."]
				#[doc = "Emits an [`Event::CallUnpaused`] event on success."]
				pub struct Unpause {
					pub ident: unpause::Ident,
				}
				pub mod unpause {
					use super::runtime_types;
					pub type Ident = (
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
					);
				}
				impl ::subxt::blocks::StaticExtrinsic for Unpause {
					const PALLET: &'static str = "TxPause";
					const CALL: &'static str = "unpause";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Pause a call."]
				#[doc = ""]
				#[doc = "Can only be called by [`Config::PauseOrigin`]."]
				#[doc = "Emits an [`Event::CallPaused`] event on success."]
				pub fn pause(
					&self,
					full_name: types::pause::FullName,
				) -> ::subxt::tx::Payload<types::Pause> {
					::subxt::tx::Payload::new_static(
						"TxPause",
						"pause",
						types::Pause { full_name },
						[
							244u8, 112u8, 104u8, 148u8, 17u8, 164u8, 228u8, 229u8, 103u8, 212u8,
							137u8, 16u8, 194u8, 167u8, 150u8, 148u8, 151u8, 233u8, 15u8, 2u8, 54u8,
							96u8, 158u8, 43u8, 222u8, 128u8, 199u8, 87u8, 74u8, 38u8, 6u8, 215u8,
						],
					)
				}
				#[doc = "Un-pause a call."]
				#[doc = ""]
				#[doc = "Can only be called by [`Config::UnpauseOrigin`]."]
				#[doc = "Emits an [`Event::CallUnpaused`] event on success."]
				pub fn unpause(
					&self,
					ident: types::unpause::Ident,
				) -> ::subxt::tx::Payload<types::Unpause> {
					::subxt::tx::Payload::new_static(
						"TxPause",
						"unpause",
						types::Unpause { ident },
						[
							213u8, 245u8, 75u8, 131u8, 24u8, 188u8, 101u8, 168u8, 39u8, 246u8,
							228u8, 155u8, 255u8, 146u8, 245u8, 218u8, 68u8, 102u8, 75u8, 133u8,
							54u8, 142u8, 191u8, 87u8, 148u8, 59u8, 99u8, 11u8, 33u8, 184u8, 24u8,
							179u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_tx_pause::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "This pallet, or a specific call is now paused."]
			pub struct CallPaused {
				pub full_name: call_paused::FullName,
			}
			pub mod call_paused {
				use super::runtime_types;
				pub type FullName = (
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
				);
			}
			impl ::subxt::events::StaticEvent for CallPaused {
				const PALLET: &'static str = "TxPause";
				const EVENT: &'static str = "CallPaused";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "This pallet, or a specific call is now unpaused."]
			pub struct CallUnpaused {
				pub full_name: call_unpaused::FullName,
			}
			pub mod call_unpaused {
				use super::runtime_types;
				pub type FullName = (
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
				);
			}
			impl ::subxt::events::StaticEvent for CallUnpaused {
				const PALLET: &'static str = "TxPause";
				const EVENT: &'static str = "CallUnpaused";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub mod types {
				use super::runtime_types;
				pub mod paused_calls {
					use super::runtime_types;
					pub type PausedCalls = ();
					pub type Param0 = runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>;
					pub type Param1 = runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>;
				}
			}
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The set of calls that are explicitly paused."]
				pub fn paused_calls_iter(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::paused_calls::PausedCalls,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"TxPause",
						"PausedCalls",
						(),
						[
							36u8, 9u8, 29u8, 154u8, 39u8, 47u8, 237u8, 97u8, 176u8, 241u8, 153u8,
							131u8, 20u8, 16u8, 73u8, 63u8, 27u8, 21u8, 107u8, 5u8, 147u8, 198u8,
							82u8, 212u8, 38u8, 162u8, 1u8, 203u8, 57u8, 187u8, 53u8, 132u8,
						],
					)
				}
				#[doc = " The set of calls that are explicitly paused."]
				pub fn paused_calls_iter1(
					&self,
					_0: impl ::std::borrow::Borrow<types::paused_calls::Param0>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageKey<types::paused_calls::Param0>,
					types::paused_calls::PausedCalls,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"TxPause",
						"PausedCalls",
						::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
						[
							36u8, 9u8, 29u8, 154u8, 39u8, 47u8, 237u8, 97u8, 176u8, 241u8, 153u8,
							131u8, 20u8, 16u8, 73u8, 63u8, 27u8, 21u8, 107u8, 5u8, 147u8, 198u8,
							82u8, 212u8, 38u8, 162u8, 1u8, 203u8, 57u8, 187u8, 53u8, 132u8,
						],
					)
				}
				#[doc = " The set of calls that are explicitly paused."]
				pub fn paused_calls(
					&self,
					_0: impl ::std::borrow::Borrow<types::paused_calls::Param0>,
					_1: impl ::std::borrow::Borrow<types::paused_calls::Param1>,
				) -> ::subxt::storage::address::Address<
					(
						::subxt::storage::address::StaticStorageKey<types::paused_calls::Param0>,
						::subxt::storage::address::StaticStorageKey<types::paused_calls::Param1>,
					),
					types::paused_calls::PausedCalls,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"TxPause",
						"PausedCalls",
						(
							::subxt::storage::address::StaticStorageKey::new(_0.borrow()),
							::subxt::storage::address::StaticStorageKey::new(_1.borrow()),
						),
						[
							36u8, 9u8, 29u8, 154u8, 39u8, 47u8, 237u8, 97u8, 176u8, 241u8, 153u8,
							131u8, 20u8, 16u8, 73u8, 63u8, 27u8, 21u8, 107u8, 5u8, 147u8, 198u8,
							82u8, 212u8, 38u8, 162u8, 1u8, 203u8, 57u8, 187u8, 53u8, 132u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " Maximum length for pallet name and call name SCALE encoded string names."]
				#[doc = ""]
				#[doc = " TOO LONG NAMES WILL BE TREATED AS PAUSED."]
				pub fn max_name_len(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"TxPause",
						"MaxNameLen",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod transaction_payment {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_transaction_payment::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,"]
			#[doc = "has been paid by `who`."]
			pub struct TransactionFeePaid {
				pub who: transaction_fee_paid::Who,
				pub actual_fee: transaction_fee_paid::ActualFee,
				pub tip: transaction_fee_paid::Tip,
			}
			pub mod transaction_fee_paid {
				use super::runtime_types;
				pub type Who = ::subxt::utils::AccountId32;
				pub type ActualFee = ::core::primitive::u128;
				pub type Tip = ::core::primitive::u128;
			}
			impl ::subxt::events::StaticEvent for TransactionFeePaid {
				const PALLET: &'static str = "TransactionPayment";
				const EVENT: &'static str = "TransactionFeePaid";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub mod types {
				use super::runtime_types;
				pub mod next_fee_multiplier {
					use super::runtime_types;
					pub type NextFeeMultiplier =
						runtime_types::sp_arithmetic::fixed_point::FixedU128;
				}
				pub mod storage_version {
					use super::runtime_types;
					pub type StorageVersion = runtime_types::pallet_transaction_payment::Releases;
				}
			}
			pub struct StorageApi;
			impl StorageApi {
				pub fn next_fee_multiplier(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::next_fee_multiplier::NextFeeMultiplier,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"TransactionPayment",
						"NextFeeMultiplier",
						(),
						[
							247u8, 39u8, 81u8, 170u8, 225u8, 226u8, 82u8, 147u8, 34u8, 113u8,
							147u8, 213u8, 59u8, 80u8, 139u8, 35u8, 36u8, 196u8, 152u8, 19u8, 9u8,
							159u8, 176u8, 79u8, 249u8, 201u8, 170u8, 1u8, 129u8, 79u8, 146u8,
							197u8,
						],
					)
				}
				pub fn storage_version(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::storage_version::StorageVersion,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"TransactionPayment",
						"StorageVersion",
						(),
						[
							105u8, 243u8, 158u8, 241u8, 159u8, 231u8, 253u8, 6u8, 4u8, 32u8, 85u8,
							178u8, 126u8, 31u8, 203u8, 134u8, 154u8, 38u8, 122u8, 155u8, 150u8,
							251u8, 174u8, 15u8, 74u8, 134u8, 216u8, 244u8, 168u8, 175u8, 158u8,
							144u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " A fee multiplier for `Operational` extrinsics to compute \"virtual tip\" to boost their"]
				#[doc = " `priority`"]
				#[doc = ""]
				#[doc = " This value is multiplied by the `final_fee` to obtain a \"virtual tip\" that is later"]
				#[doc = " added to a tip component in regular `priority` calculations."]
				#[doc = " It means that a `Normal` transaction can front-run a similarly-sized `Operational`"]
				#[doc = " extrinsic (with no tip), by including a tip value greater than the virtual tip."]
				#[doc = ""]
				#[doc = " ```rust,ignore"]
				#[doc = " // For `Normal`"]
				#[doc = " let priority = priority_calc(tip);"]
				#[doc = ""]
				#[doc = " // For `Operational`"]
				#[doc = " let virtual_tip = (inclusion_fee + tip) * OperationalFeeMultiplier;"]
				#[doc = " let priority = priority_calc(tip + virtual_tip);"]
				#[doc = " ```"]
				#[doc = ""]
				#[doc = " Note that since we use `final_fee` the multiplier applies also to the regular `tip`"]
				#[doc = " sent with the transaction. So, not only does the transaction get a priority bump based"]
				#[doc = " on the `inclusion_fee`, but we also amplify the impact of tips applied to `Operational`"]
				#[doc = " transactions."]
				pub fn operational_fee_multiplier(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u8> {
					::subxt::constants::Address::new_static(
						"TransactionPayment",
						"OperationalFeeMultiplier",
						[
							141u8, 130u8, 11u8, 35u8, 226u8, 114u8, 92u8, 179u8, 168u8, 110u8,
							28u8, 91u8, 221u8, 64u8, 4u8, 148u8, 201u8, 193u8, 185u8, 66u8, 226u8,
							114u8, 97u8, 79u8, 62u8, 212u8, 202u8, 114u8, 237u8, 228u8, 183u8,
							165u8,
						],
					)
				}
			}
		}
	}
	pub mod sudo {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "Error for the Sudo pallet."]
		pub type Error = runtime_types::pallet_sudo::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_sudo::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
				pub struct Sudo {
					pub call: ::std::boxed::Box<sudo::Call>,
				}
				pub mod sudo {
					use super::runtime_types;
					pub type Call = runtime_types::ulx_node_runtime::RuntimeCall;
				}
				impl ::subxt::blocks::StaticExtrinsic for Sudo {
					const PALLET: &'static str = "Sudo";
					const CALL: &'static str = "sudo";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
				#[doc = "This function does not check the weight of the call, and instead allows the"]
				#[doc = "Sudo user to specify the weight of the call."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				pub struct SudoUncheckedWeight {
					pub call: ::std::boxed::Box<sudo_unchecked_weight::Call>,
					pub weight: sudo_unchecked_weight::Weight,
				}
				pub mod sudo_unchecked_weight {
					use super::runtime_types;
					pub type Call = runtime_types::ulx_node_runtime::RuntimeCall;
					pub type Weight = runtime_types::sp_weights::weight_v2::Weight;
				}
				impl ::subxt::blocks::StaticExtrinsic for SudoUncheckedWeight {
					const PALLET: &'static str = "Sudo";
					const CALL: &'static str = "sudo_unchecked_weight";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo"]
				#[doc = "key."]
				pub struct SetKey {
					pub new: set_key::New,
				}
				pub mod set_key {
					use super::runtime_types;
					pub type New = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
				}
				impl ::subxt::blocks::StaticExtrinsic for SetKey {
					const PALLET: &'static str = "Sudo";
					const CALL: &'static str = "set_key";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Authenticates the sudo key and dispatches a function call with `Signed` origin from"]
				#[doc = "a given account."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				pub struct SudoAs {
					pub who: sudo_as::Who,
					pub call: ::std::boxed::Box<sudo_as::Call>,
				}
				pub mod sudo_as {
					use super::runtime_types;
					pub type Who = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
					pub type Call = runtime_types::ulx_node_runtime::RuntimeCall;
				}
				impl ::subxt::blocks::StaticExtrinsic for SudoAs {
					const PALLET: &'static str = "Sudo";
					const CALL: &'static str = "sudo_as";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Permanently removes the sudo key."]
				#[doc = ""]
				#[doc = "**This cannot be un-done.**"]
				pub struct RemoveKey;
				impl ::subxt::blocks::StaticExtrinsic for RemoveKey {
					const PALLET: &'static str = "Sudo";
					const CALL: &'static str = "remove_key";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
				pub fn sudo(&self, call: types::sudo::Call) -> ::subxt::tx::Payload<types::Sudo> {
					::subxt::tx::Payload::new_static(
						"Sudo",
						"sudo",
						types::Sudo { call: ::std::boxed::Box::new(call) },
						[
							9u8, 123u8, 116u8, 167u8, 80u8, 41u8, 40u8, 18u8, 198u8, 158u8, 6u8,
							245u8, 195u8, 163u8, 1u8, 14u8, 95u8, 39u8, 216u8, 53u8, 176u8, 10u8,
							158u8, 57u8, 110u8, 233u8, 182u8, 222u8, 130u8, 158u8, 232u8, 230u8,
						],
					)
				}
				#[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
				#[doc = "This function does not check the weight of the call, and instead allows the"]
				#[doc = "Sudo user to specify the weight of the call."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				pub fn sudo_unchecked_weight(
					&self,
					call: types::sudo_unchecked_weight::Call,
					weight: types::sudo_unchecked_weight::Weight,
				) -> ::subxt::tx::Payload<types::SudoUncheckedWeight> {
					::subxt::tx::Payload::new_static(
						"Sudo",
						"sudo_unchecked_weight",
						types::SudoUncheckedWeight { call: ::std::boxed::Box::new(call), weight },
						[
							153u8, 235u8, 175u8, 85u8, 181u8, 3u8, 34u8, 99u8, 200u8, 185u8, 226u8,
							254u8, 156u8, 24u8, 131u8, 192u8, 191u8, 130u8, 68u8, 235u8, 245u8,
							32u8, 91u8, 226u8, 32u8, 106u8, 171u8, 175u8, 73u8, 146u8, 130u8,
							184u8,
						],
					)
				}
				#[doc = "Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo"]
				#[doc = "key."]
				pub fn set_key(
					&self,
					new: types::set_key::New,
				) -> ::subxt::tx::Payload<types::SetKey> {
					::subxt::tx::Payload::new_static(
						"Sudo",
						"set_key",
						types::SetKey { new },
						[
							9u8, 73u8, 39u8, 205u8, 188u8, 127u8, 143u8, 54u8, 128u8, 94u8, 8u8,
							227u8, 197u8, 44u8, 70u8, 93u8, 228u8, 196u8, 64u8, 165u8, 226u8,
							158u8, 101u8, 192u8, 22u8, 193u8, 102u8, 84u8, 21u8, 35u8, 92u8, 198u8,
						],
					)
				}
				#[doc = "Authenticates the sudo key and dispatches a function call with `Signed` origin from"]
				#[doc = "a given account."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				pub fn sudo_as(
					&self,
					who: types::sudo_as::Who,
					call: types::sudo_as::Call,
				) -> ::subxt::tx::Payload<types::SudoAs> {
					::subxt::tx::Payload::new_static(
						"Sudo",
						"sudo_as",
						types::SudoAs { who, call: ::std::boxed::Box::new(call) },
						[
							175u8, 109u8, 231u8, 126u8, 67u8, 69u8, 212u8, 240u8, 108u8, 124u8,
							205u8, 216u8, 101u8, 242u8, 104u8, 164u8, 250u8, 137u8, 153u8, 72u8,
							101u8, 80u8, 186u8, 230u8, 73u8, 227u8, 228u8, 37u8, 113u8, 4u8, 48u8,
							72u8,
						],
					)
				}
				#[doc = "Permanently removes the sudo key."]
				#[doc = ""]
				#[doc = "**This cannot be un-done.**"]
				pub fn remove_key(&self) -> ::subxt::tx::Payload<types::RemoveKey> {
					::subxt::tx::Payload::new_static(
						"Sudo",
						"remove_key",
						types::RemoveKey {},
						[
							133u8, 253u8, 54u8, 175u8, 202u8, 239u8, 5u8, 198u8, 180u8, 138u8,
							25u8, 28u8, 109u8, 40u8, 30u8, 56u8, 126u8, 100u8, 52u8, 205u8, 250u8,
							191u8, 61u8, 195u8, 172u8, 142u8, 184u8, 239u8, 247u8, 10u8, 211u8,
							79u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_sudo::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A sudo call just took place."]
			pub struct Sudid {
				pub sudo_result: sudid::SudoResult,
			}
			pub mod sudid {
				use super::runtime_types;
				pub type SudoResult =
					::core::result::Result<(), runtime_types::sp_runtime::DispatchError>;
			}
			impl ::subxt::events::StaticEvent for Sudid {
				const PALLET: &'static str = "Sudo";
				const EVENT: &'static str = "Sudid";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "The sudo key has been updated."]
			pub struct KeyChanged {
				pub old: key_changed::Old,
				pub new: key_changed::New,
			}
			pub mod key_changed {
				use super::runtime_types;
				pub type Old = ::core::option::Option<::subxt::utils::AccountId32>;
				pub type New = ::subxt::utils::AccountId32;
			}
			impl ::subxt::events::StaticEvent for KeyChanged {
				const PALLET: &'static str = "Sudo";
				const EVENT: &'static str = "KeyChanged";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "The key was permanently removed."]
			pub struct KeyRemoved;
			impl ::subxt::events::StaticEvent for KeyRemoved {
				const PALLET: &'static str = "Sudo";
				const EVENT: &'static str = "KeyRemoved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A [sudo_as](Pallet::sudo_as) call just took place."]
			pub struct SudoAsDone {
				pub sudo_result: sudo_as_done::SudoResult,
			}
			pub mod sudo_as_done {
				use super::runtime_types;
				pub type SudoResult =
					::core::result::Result<(), runtime_types::sp_runtime::DispatchError>;
			}
			impl ::subxt::events::StaticEvent for SudoAsDone {
				const PALLET: &'static str = "Sudo";
				const EVENT: &'static str = "SudoAsDone";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub mod types {
				use super::runtime_types;
				pub mod key {
					use super::runtime_types;
					pub type Key = ::subxt::utils::AccountId32;
				}
			}
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The `AccountId` of the sudo key."]
				pub fn key(
					&self,
				) -> ::subxt::storage::address::Address<
					(),
					types::key::Key,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Sudo",
						"Key",
						(),
						[
							72u8, 14u8, 225u8, 162u8, 205u8, 247u8, 227u8, 105u8, 116u8, 57u8, 4u8,
							31u8, 84u8, 137u8, 227u8, 228u8, 133u8, 245u8, 206u8, 227u8, 117u8,
							36u8, 252u8, 151u8, 107u8, 15u8, 180u8, 4u8, 4u8, 152u8, 195u8, 144u8,
						],
					)
				}
			}
		}
	}
	pub mod runtime_types {
		use super::runtime_types;
		pub mod bounded_collections {
			use super::runtime_types;
			pub mod bounded_btree_map {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct BoundedBTreeMap<_0, _1>(pub ::subxt::utils::KeyedVec<_0, _1>);
			}
			pub mod bounded_vec {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					serde :: Serialize,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[serde(transparent)]
				pub struct BoundedVec<_0>(pub ::std::vec::Vec<_0>);
			}
			pub mod weak_bounded_vec {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct WeakBoundedVec<_0>(pub ::std::vec::Vec<_0>);
			}
		}
		pub mod finality_grandpa {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Equivocation<_0, _1, _2> {
				pub round_number: ::core::primitive::u64,
				pub identity: _0,
				pub first: (_1, _2),
				pub second: (_1, _2),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Precommit<_0, _1> {
				pub target_hash: _0,
				pub target_number: _1,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Prevote<_0, _1> {
				pub target_hash: _0,
				pub target_number: _1,
			}
		}
		pub mod frame_support {
			use super::runtime_types;
			pub mod dispatch {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum DispatchClass {
					#[codec(index = 0)]
					Normal,
					#[codec(index = 1)]
					Operational,
					#[codec(index = 2)]
					Mandatory,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct DispatchInfo {
					pub weight: runtime_types::sp_weights::weight_v2::Weight,
					pub class: runtime_types::frame_support::dispatch::DispatchClass,
					pub pays_fee: runtime_types::frame_support::dispatch::Pays,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Pays {
					#[codec(index = 0)]
					Yes,
					#[codec(index = 1)]
					No,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct PerDispatchClass<_0> {
					pub normal: _0,
					pub operational: _0,
					pub mandatory: _0,
				}
			}
			pub mod traits {
				use super::runtime_types;
				pub mod tokens {
					use super::runtime_types;
					pub mod misc {
						use super::runtime_types;
						#[derive(
							:: subxt :: ext :: codec :: Decode,
							:: subxt :: ext :: codec :: Encode,
							:: subxt :: ext :: scale_decode :: DecodeAsType,
							:: subxt :: ext :: scale_encode :: EncodeAsType,
							Clone,
							Debug,
						)]
						# [codec (crate = :: subxt :: ext :: codec)]
						#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
						#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
						pub enum BalanceStatus {
							#[codec(index = 0)]
							Free,
							#[codec(index = 1)]
							Reserved,
						}
					}
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct PalletId(pub [::core::primitive::u8; 8usize]);
		}
		pub mod frame_system {
			use super::runtime_types;
			pub mod extensions {
				use super::runtime_types;
				pub mod check_genesis {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Clone,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct CheckGenesis;
				}
				pub mod check_mortality {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Clone,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct CheckMortality(pub runtime_types::sp_runtime::generic::era::Era);
				}
				pub mod check_non_zero_sender {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Clone,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct CheckNonZeroSender;
				}
				pub mod check_nonce {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Clone,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct CheckNonce(#[codec(compact)] pub ::core::primitive::u32);
				}
				pub mod check_spec_version {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Clone,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct CheckSpecVersion;
				}
				pub mod check_tx_version {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Clone,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct CheckTxVersion;
				}
				pub mod check_weight {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Clone,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct CheckWeight;
				}
			}
			pub mod limits {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct BlockLength {
					pub max: runtime_types::frame_support::dispatch::PerDispatchClass<
						::core::primitive::u32,
					>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct BlockWeights {
					pub base_block: runtime_types::sp_weights::weight_v2::Weight,
					pub max_block: runtime_types::sp_weights::weight_v2::Weight,
					pub per_class: runtime_types::frame_support::dispatch::PerDispatchClass<
						runtime_types::frame_system::limits::WeightsPerClass,
					>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct WeightsPerClass {
					pub base_extrinsic: runtime_types::sp_weights::weight_v2::Weight,
					pub max_extrinsic:
						::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
					pub max_total:
						::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
					pub reserved:
						::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
				}
			}
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Make some on-chain remark."]
					#[doc = ""]
					#[doc = "Can be executed by every `origin`."]
					remark { remark: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 1)]
					#[doc = "Set the number of pages in the WebAssembly environment's heap."]
					set_heap_pages { pages: ::core::primitive::u64 },
					#[codec(index = 2)]
					#[doc = "Set the new runtime code."]
					set_code { code: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 3)]
					#[doc = "Set the new runtime code without doing any checks of the given `code`."]
					#[doc = ""]
					#[doc = "Note that runtime upgrades will not run if this is called with a not-increasing spec"]
					#[doc = "version!"]
					set_code_without_checks { code: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 4)]
					#[doc = "Set some items of storage."]
					set_storage {
						items: ::std::vec::Vec<(
							::std::vec::Vec<::core::primitive::u8>,
							::std::vec::Vec<::core::primitive::u8>,
						)>,
					},
					#[codec(index = 5)]
					#[doc = "Kill some items from storage."]
					kill_storage { keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>> },
					#[codec(index = 6)]
					#[doc = "Kill all storage items with a key that starts with the given prefix."]
					#[doc = ""]
					#[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
					#[doc = "the prefix we are removing to accurately calculate the weight of this function."]
					kill_prefix {
						prefix: ::std::vec::Vec<::core::primitive::u8>,
						subkeys: ::core::primitive::u32,
					},
					#[codec(index = 7)]
					#[doc = "Make some on-chain remark and emit event."]
					remark_with_event { remark: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 9)]
					#[doc = "Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied"]
					#[doc = "later."]
					#[doc = ""]
					#[doc = "This call requires Root origin."]
					authorize_upgrade { code_hash: ::subxt::utils::H256 },
					#[codec(index = 10)]
					#[doc = "Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied"]
					#[doc = "later."]
					#[doc = ""]
					#[doc = "WARNING: This authorizes an upgrade that will take place without any safety checks, for"]
					#[doc = "example that the spec name remains the same and that the version number increases. Not"]
					#[doc = "recommended for normal use. Use `authorize_upgrade` instead."]
					#[doc = ""]
					#[doc = "This call requires Root origin."]
					authorize_upgrade_without_checks { code_hash: ::subxt::utils::H256 },
					#[codec(index = 11)]
					#[doc = "Provide the preimage (runtime binary) `code` for an upgrade that has been authorized."]
					#[doc = ""]
					#[doc = "If the authorization required a version check, this call will ensure the spec name"]
					#[doc = "remains unchanged and that the spec version has increased."]
					#[doc = ""]
					#[doc = "Depending on the runtime's `OnSetCode` configuration, this function may directly apply"]
					#[doc = "the new `code` in the same block or attempt to schedule the upgrade."]
					#[doc = ""]
					#[doc = "All origins are allowed."]
					apply_authorized_upgrade { code: ::std::vec::Vec<::core::primitive::u8> },
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Error for the System pallet"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "The name of specification does not match between the current runtime"]
					#[doc = "and the new runtime."]
					InvalidSpecName,
					#[codec(index = 1)]
					#[doc = "The specification version is not allowed to decrease between the current runtime"]
					#[doc = "and the new runtime."]
					SpecVersionNeedsToIncrease,
					#[codec(index = 2)]
					#[doc = "Failed to extract the runtime version from the new runtime."]
					#[doc = ""]
					#[doc = "Either calling `Core_version` or decoding `RuntimeVersion` failed."]
					FailedToExtractRuntimeVersion,
					#[codec(index = 3)]
					#[doc = "Suicide called when the account has non-default composite data."]
					NonDefaultComposite,
					#[codec(index = 4)]
					#[doc = "There is a non-zero reference count preventing the account from being purged."]
					NonZeroRefCount,
					#[codec(index = 5)]
					#[doc = "The origin filter prevent the call to be dispatched."]
					CallFiltered,
					#[codec(index = 6)]
					#[doc = "A multi-block migration is ongoing and prevents the current code from being replaced."]
					MultiBlockMigrationsOngoing,
					#[codec(index = 7)]
					#[doc = "No upgrade authorized."]
					NothingAuthorized,
					#[codec(index = 8)]
					#[doc = "The submitted code is not authorized."]
					Unauthorized,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Event for the System pallet."]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "An extrinsic completed successfully."]
					ExtrinsicSuccess {
						dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
					},
					#[codec(index = 1)]
					#[doc = "An extrinsic failed."]
					ExtrinsicFailed {
						dispatch_error: runtime_types::sp_runtime::DispatchError,
						dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
					},
					#[codec(index = 2)]
					#[doc = "`:code` was updated."]
					CodeUpdated,
					#[codec(index = 3)]
					#[doc = "A new account was created."]
					NewAccount { account: ::subxt::utils::AccountId32 },
					#[codec(index = 4)]
					#[doc = "An account was reaped."]
					KilledAccount { account: ::subxt::utils::AccountId32 },
					#[codec(index = 5)]
					#[doc = "On on-chain remark happened."]
					Remarked { sender: ::subxt::utils::AccountId32, hash: ::subxt::utils::H256 },
					#[codec(index = 6)]
					#[doc = "An upgrade was authorized."]
					UpgradeAuthorized {
						code_hash: ::subxt::utils::H256,
						check_version: ::core::primitive::bool,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct AccountInfo<_0, _1> {
				pub nonce: _0,
				pub consumers: ::core::primitive::u32,
				pub providers: ::core::primitive::u32,
				pub sufficients: ::core::primitive::u32,
				pub data: _1,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct CodeUpgradeAuthorization {
				pub code_hash: ::subxt::utils::H256,
				pub check_version: ::core::primitive::bool,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct EventRecord<_0, _1> {
				pub phase: runtime_types::frame_system::Phase,
				pub event: _0,
				pub topics: ::std::vec::Vec<_1>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct LastRuntimeUpgradeInfo {
				#[codec(compact)]
				pub spec_version: ::core::primitive::u32,
				pub spec_name: ::std::string::String,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum Phase {
				#[codec(index = 0)]
				ApplyExtrinsic(::core::primitive::u32),
				#[codec(index = 1)]
				Finalization,
				#[codec(index = 2)]
				Initialization,
			}
		}
		pub mod pallet_balances {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Transfer some liquid free balance to another account."]
					#[doc = ""]
					#[doc = "`transfer_allow_death` will set the `FreeBalance` of the sender and receiver."]
					#[doc = "If the sender's account is below the existential deposit as a result"]
					#[doc = "of the transfer, the account will be reaped."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
					transfer_allow_death {
						dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					#[doc = "Exactly as `transfer_allow_death`, except the origin must be root and the source account"]
					#[doc = "may be specified."]
					force_transfer {
						source: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					#[doc = "Same as the [`transfer_allow_death`] call, but with a check that the transfer will not"]
					#[doc = "kill the origin account."]
					#[doc = ""]
					#[doc = "99% of the time you want [`transfer_allow_death`] instead."]
					#[doc = ""]
					#[doc = "[`transfer_allow_death`]: struct.Pallet.html#method.transfer"]
					transfer_keep_alive {
						dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					#[doc = "Transfer the entire transferable balance from the caller account."]
					#[doc = ""]
					#[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
					#[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
					#[doc = "transferred by this function. To ensure that this function results in a killed account,"]
					#[doc = "you might need to prepare the account by removing any reference counters, storage"]
					#[doc = "deposits, etc..."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be Signed."]
					#[doc = ""]
					#[doc = "- `dest`: The recipient of the transfer."]
					#[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
					#[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
					#[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
					#[doc = "  keep the sender account alive (true)."]
					transfer_all {
						dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						keep_alive: ::core::primitive::bool,
					},
					#[codec(index = 5)]
					#[doc = "Unreserve some balance from a user by force."]
					#[doc = ""]
					#[doc = "Can only be called by ROOT."]
					force_unreserve {
						who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 6)]
					#[doc = "Upgrade a specified account."]
					#[doc = ""]
					#[doc = "- `origin`: Must be `Signed`."]
					#[doc = "- `who`: The account to be upgraded."]
					#[doc = ""]
					#[doc = "This will waive the transaction fee if at least all but 10% of the accounts needed to"]
					#[doc = "be upgraded. (We let some not have to be upgraded just in order to allow for the"]
					#[doc = "possibility of churn)."]
					upgrade_accounts { who: ::std::vec::Vec<::subxt::utils::AccountId32> },
					#[codec(index = 8)]
					#[doc = "Set the regular balance of a given account."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call is `root`."]
					force_set_balance {
						who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						new_free: ::core::primitive::u128,
					},
					#[codec(index = 9)]
					#[doc = "Adjust the total issuance in a saturating way."]
					#[doc = ""]
					#[doc = "Can only be called by root and always needs a positive `delta`."]
					#[doc = ""]
					#[doc = "# Example"]
					force_adjust_total_issuance {
						direction: runtime_types::pallet_balances::types::AdjustmentDirection,
						#[codec(compact)]
						delta: ::core::primitive::u128,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call2 {
					#[codec(index = 0)]
					#[doc = "Transfer some liquid free balance to another account."]
					#[doc = ""]
					#[doc = "`transfer_allow_death` will set the `FreeBalance` of the sender and receiver."]
					#[doc = "If the sender's account is below the existential deposit as a result"]
					#[doc = "of the transfer, the account will be reaped."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
					transfer_allow_death {
						dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					#[doc = "Exactly as `transfer_allow_death`, except the origin must be root and the source account"]
					#[doc = "may be specified."]
					force_transfer {
						source: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					#[doc = "Same as the [`transfer_allow_death`] call, but with a check that the transfer will not"]
					#[doc = "kill the origin account."]
					#[doc = ""]
					#[doc = "99% of the time you want [`transfer_allow_death`] instead."]
					#[doc = ""]
					#[doc = "[`transfer_allow_death`]: struct.Pallet.html#method.transfer"]
					transfer_keep_alive {
						dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					#[doc = "Transfer the entire transferable balance from the caller account."]
					#[doc = ""]
					#[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
					#[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
					#[doc = "transferred by this function. To ensure that this function results in a killed account,"]
					#[doc = "you might need to prepare the account by removing any reference counters, storage"]
					#[doc = "deposits, etc..."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be Signed."]
					#[doc = ""]
					#[doc = "- `dest`: The recipient of the transfer."]
					#[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
					#[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
					#[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
					#[doc = "  keep the sender account alive (true)."]
					transfer_all {
						dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						keep_alive: ::core::primitive::bool,
					},
					#[codec(index = 5)]
					#[doc = "Unreserve some balance from a user by force."]
					#[doc = ""]
					#[doc = "Can only be called by ROOT."]
					force_unreserve {
						who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 6)]
					#[doc = "Upgrade a specified account."]
					#[doc = ""]
					#[doc = "- `origin`: Must be `Signed`."]
					#[doc = "- `who`: The account to be upgraded."]
					#[doc = ""]
					#[doc = "This will waive the transaction fee if at least all but 10% of the accounts needed to"]
					#[doc = "be upgraded. (We let some not have to be upgraded just in order to allow for the"]
					#[doc = "possibility of churn)."]
					upgrade_accounts { who: ::std::vec::Vec<::subxt::utils::AccountId32> },
					#[codec(index = 8)]
					#[doc = "Set the regular balance of a given account."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call is `root`."]
					force_set_balance {
						who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						new_free: ::core::primitive::u128,
					},
					#[codec(index = 9)]
					#[doc = "Adjust the total issuance in a saturating way."]
					#[doc = ""]
					#[doc = "Can only be called by root and always needs a positive `delta`."]
					#[doc = ""]
					#[doc = "# Example"]
					force_adjust_total_issuance {
						direction: runtime_types::pallet_balances::types::AdjustmentDirection,
						#[codec(compact)]
						delta: ::core::primitive::u128,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Vesting balance too high to send value."]
					VestingBalance,
					#[codec(index = 1)]
					#[doc = "Account liquidity restrictions prevent withdrawal."]
					LiquidityRestrictions,
					#[codec(index = 2)]
					#[doc = "Balance too low to send value."]
					InsufficientBalance,
					#[codec(index = 3)]
					#[doc = "Value too low to create account due to existential deposit."]
					ExistentialDeposit,
					#[codec(index = 4)]
					#[doc = "Transfer/payment would kill account."]
					Expendability,
					#[codec(index = 5)]
					#[doc = "A vesting schedule already exists for this account."]
					ExistingVestingSchedule,
					#[codec(index = 6)]
					#[doc = "Beneficiary account must pre-exist."]
					DeadAccount,
					#[codec(index = 7)]
					#[doc = "Number of named reserves exceed `MaxReserves`."]
					TooManyReserves,
					#[codec(index = 8)]
					#[doc = "Number of holds exceed `VariantCountOf<T::RuntimeHoldReason>`."]
					TooManyHolds,
					#[codec(index = 9)]
					#[doc = "Number of freezes exceed `MaxFreezes`."]
					TooManyFreezes,
					#[codec(index = 10)]
					#[doc = "The issuance cannot be modified since it is already deactivated."]
					IssuanceDeactivated,
					#[codec(index = 11)]
					#[doc = "The delta cannot be zero."]
					DeltaZero,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error2 {
					#[codec(index = 0)]
					#[doc = "Vesting balance too high to send value."]
					VestingBalance,
					#[codec(index = 1)]
					#[doc = "Account liquidity restrictions prevent withdrawal."]
					LiquidityRestrictions,
					#[codec(index = 2)]
					#[doc = "Balance too low to send value."]
					InsufficientBalance,
					#[codec(index = 3)]
					#[doc = "Value too low to create account due to existential deposit."]
					ExistentialDeposit,
					#[codec(index = 4)]
					#[doc = "Transfer/payment would kill account."]
					Expendability,
					#[codec(index = 5)]
					#[doc = "A vesting schedule already exists for this account."]
					ExistingVestingSchedule,
					#[codec(index = 6)]
					#[doc = "Beneficiary account must pre-exist."]
					DeadAccount,
					#[codec(index = 7)]
					#[doc = "Number of named reserves exceed `MaxReserves`."]
					TooManyReserves,
					#[codec(index = 8)]
					#[doc = "Number of holds exceed `VariantCountOf<T::RuntimeHoldReason>`."]
					TooManyHolds,
					#[codec(index = 9)]
					#[doc = "Number of freezes exceed `MaxFreezes`."]
					TooManyFreezes,
					#[codec(index = 10)]
					#[doc = "The issuance cannot be modified since it is already deactivated."]
					IssuanceDeactivated,
					#[codec(index = 11)]
					#[doc = "The delta cannot be zero."]
					DeltaZero,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "An account was created with some free balance."]
					Endowed {
						account: ::subxt::utils::AccountId32,
						free_balance: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					#[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
					#[doc = "resulting in an outright loss."]
					DustLost {
						account: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					#[doc = "Transfer succeeded."]
					Transfer {
						from: ::subxt::utils::AccountId32,
						to: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					#[doc = "A balance was set by root."]
					BalanceSet { who: ::subxt::utils::AccountId32, free: ::core::primitive::u128 },
					#[codec(index = 4)]
					#[doc = "Some balance was reserved (moved from free to reserved)."]
					Reserved { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 5)]
					#[doc = "Some balance was unreserved (moved from reserved to free)."]
					Unreserved { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 6)]
					#[doc = "Some balance was moved from the reserve of the first account to the second account."]
					#[doc = "Final argument indicates the destination balance type."]
					ReserveRepatriated {
						from: ::subxt::utils::AccountId32,
						to: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
						destination_status:
							runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
					},
					#[codec(index = 7)]
					#[doc = "Some amount was deposited (e.g. for transaction fees)."]
					Deposit { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 8)]
					#[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
					Withdraw { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 9)]
					#[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
					Slashed { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 10)]
					#[doc = "Some amount was minted into an account."]
					Minted { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 11)]
					#[doc = "Some amount was burned from an account."]
					Burned { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 12)]
					#[doc = "Some amount was suspended from an account (it can be restored later)."]
					Suspended { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 13)]
					#[doc = "Some amount was restored into an account."]
					Restored { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 14)]
					#[doc = "An account was upgraded."]
					Upgraded { who: ::subxt::utils::AccountId32 },
					#[codec(index = 15)]
					#[doc = "Total issuance was increased by `amount`, creating a credit to be balanced."]
					Issued { amount: ::core::primitive::u128 },
					#[codec(index = 16)]
					#[doc = "Total issuance was decreased by `amount`, creating a debt to be balanced."]
					Rescinded { amount: ::core::primitive::u128 },
					#[codec(index = 17)]
					#[doc = "Some balance was locked."]
					Locked { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 18)]
					#[doc = "Some balance was unlocked."]
					Unlocked { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 19)]
					#[doc = "Some balance was frozen."]
					Frozen { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 20)]
					#[doc = "Some balance was thawed."]
					Thawed { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 21)]
					#[doc = "The `TotalIssuance` was forcefully changed."]
					TotalIssuanceForced {
						old: ::core::primitive::u128,
						new: ::core::primitive::u128,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event2 {
					#[codec(index = 0)]
					#[doc = "An account was created with some free balance."]
					Endowed {
						account: ::subxt::utils::AccountId32,
						free_balance: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					#[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
					#[doc = "resulting in an outright loss."]
					DustLost {
						account: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					#[doc = "Transfer succeeded."]
					Transfer {
						from: ::subxt::utils::AccountId32,
						to: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					#[doc = "A balance was set by root."]
					BalanceSet { who: ::subxt::utils::AccountId32, free: ::core::primitive::u128 },
					#[codec(index = 4)]
					#[doc = "Some balance was reserved (moved from free to reserved)."]
					Reserved { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 5)]
					#[doc = "Some balance was unreserved (moved from reserved to free)."]
					Unreserved { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 6)]
					#[doc = "Some balance was moved from the reserve of the first account to the second account."]
					#[doc = "Final argument indicates the destination balance type."]
					ReserveRepatriated {
						from: ::subxt::utils::AccountId32,
						to: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
						destination_status:
							runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
					},
					#[codec(index = 7)]
					#[doc = "Some amount was deposited (e.g. for transaction fees)."]
					Deposit { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 8)]
					#[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
					Withdraw { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 9)]
					#[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
					Slashed { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 10)]
					#[doc = "Some amount was minted into an account."]
					Minted { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 11)]
					#[doc = "Some amount was burned from an account."]
					Burned { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 12)]
					#[doc = "Some amount was suspended from an account (it can be restored later)."]
					Suspended { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 13)]
					#[doc = "Some amount was restored into an account."]
					Restored { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 14)]
					#[doc = "An account was upgraded."]
					Upgraded { who: ::subxt::utils::AccountId32 },
					#[codec(index = 15)]
					#[doc = "Total issuance was increased by `amount`, creating a credit to be balanced."]
					Issued { amount: ::core::primitive::u128 },
					#[codec(index = 16)]
					#[doc = "Total issuance was decreased by `amount`, creating a debt to be balanced."]
					Rescinded { amount: ::core::primitive::u128 },
					#[codec(index = 17)]
					#[doc = "Some balance was locked."]
					Locked { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 18)]
					#[doc = "Some balance was unlocked."]
					Unlocked { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 19)]
					#[doc = "Some balance was frozen."]
					Frozen { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 20)]
					#[doc = "Some balance was thawed."]
					Thawed { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 21)]
					#[doc = "The `TotalIssuance` was forcefully changed."]
					TotalIssuanceForced {
						old: ::core::primitive::u128,
						new: ::core::primitive::u128,
					},
				}
			}
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct AccountData<_0> {
					pub free: _0,
					pub reserved: _0,
					pub frozen: _0,
					pub flags: runtime_types::pallet_balances::types::ExtraFlags,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum AdjustmentDirection {
					#[codec(index = 0)]
					Increase,
					#[codec(index = 1)]
					Decrease,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct BalanceLock<_0> {
					pub id: [::core::primitive::u8; 8usize],
					pub amount: _0,
					pub reasons: runtime_types::pallet_balances::types::Reasons,
				}
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ExtraFlags(pub ::core::primitive::u128);
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct IdAmount<_0, _1> {
					pub id: _0,
					pub amount: _1,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Reasons {
					#[codec(index = 0)]
					Fee,
					#[codec(index = 1)]
					Misc,
					#[codec(index = 2)]
					All,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ReserveData<_0, _1> {
					pub id: _0,
					pub amount: _1,
				}
			}
		}
		pub mod pallet_block_rewards {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct BlockPayout<_0, _1> {
					pub account_id: _0,
					pub ulixees: _1,
					pub argons: _1,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					RewardCreated {
						maturation_block: ::core::primitive::u32,
						rewards: ::std::vec::Vec<
							runtime_types::pallet_block_rewards::pallet::BlockPayout<
								::subxt::utils::AccountId32,
								::core::primitive::u128,
							>,
						>,
					},
					#[codec(index = 1)]
					RewardUnlocked {
						rewards: ::std::vec::Vec<
							runtime_types::pallet_block_rewards::pallet::BlockPayout<
								::subxt::utils::AccountId32,
								::core::primitive::u128,
							>,
						>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum FreezeReason {
					#[codec(index = 0)]
					MaturationPeriod,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum HoldReason {
					#[codec(index = 0)]
					MaturationPeriod,
				}
			}
		}
		pub mod pallet_block_seal {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					apply { seal: runtime_types::ulx_primitives::inherents::BlockSealInherent },
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "The strength of the given seal did not match calculations"]
					InvalidVoteSealStrength,
					#[codec(index = 1)]
					#[doc = "Vote not submitted by the right miner"]
					InvalidSubmitter,
					#[codec(index = 2)]
					#[doc = "Could not decode the vote bytes"]
					UnableToDecodeVoteAccount,
					#[codec(index = 3)]
					#[doc = "The block author is not a registered miner"]
					UnregisteredBlockAuthor,
					#[codec(index = 4)]
					#[doc = "The merkle proof of vote inclusion in the notebook is invalid"]
					InvalidBlockVoteProof,
					#[codec(index = 5)]
					#[doc = "No vote minimum found at grandparent height"]
					NoGrandparentVoteMinimum,
					#[codec(index = 6)]
					#[doc = "Too many block seals submitted"]
					DuplicateBlockSealProvided,
					#[codec(index = 7)]
					#[doc = "The block vote did not reach the minimum voting power at time of the grandparent block"]
					InsufficientVotingPower,
					#[codec(index = 8)]
					#[doc = "No registered voting key found for the parent block"]
					ParentVotingKeyNotFound,
					#[codec(index = 9)]
					#[doc = "The block vote was not for a valid block"]
					InvalidVoteGrandparentHash,
					#[codec(index = 10)]
					#[doc = "The notebook for this vote was not eligible to vote"]
					IneligibleNotebookUsed,
					#[codec(index = 11)]
					#[doc = "The lookup to verify a vote's authenticity is not available for the given block"]
					NoEligibleVotingRoot,
					#[codec(index = 12)]
					#[doc = "The data domain was not registered"]
					UnregisteredDataDomain,
					#[codec(index = 13)]
					#[doc = "The data domain account is mismatched with the block reward seeker"]
					InvalidDataDomainAccount,
					#[codec(index = 14)]
					#[doc = "Message was not signed by a registered miner"]
					InvalidAuthoritySignature,
					#[codec(index = 15)]
					#[doc = "Could not decode the scale bytes of the votes"]
					CouldNotDecodeVote,
					#[codec(index = 16)]
					#[doc = "Too many notebooks were submitted for the current tick. Should not be possible"]
					MaxNotebooksAtTickExceeded,
					#[codec(index = 17)]
					#[doc = "No closest miner found for vote"]
					NoClosestMinerFoundForVote,
					#[codec(index = 18)]
					#[doc = "The vote signature was invalid"]
					BlockVoteInvalidSignature,
				}
			}
		}
		pub mod pallet_block_seal_spec {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					configure {
						vote_minimum: ::core::option::Option<::core::primitive::u128>,
						compute_difficulty: ::core::option::Option<::core::primitive::u128>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "The maximum number of notebooks at the current tick has been exceeded"]
					MaxNotebooksAtTickExceeded,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					VoteMinimumAdjusted {
						expected_block_votes: ::core::primitive::u128,
						actual_block_votes: ::core::primitive::u128,
						start_vote_minimum: ::core::primitive::u128,
						new_vote_minimum: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					ComputeDifficultyAdjusted {
						expected_block_time: ::core::primitive::u64,
						actual_block_time: ::core::primitive::u64,
						start_difficulty: ::core::primitive::u128,
						new_difficulty: ::core::primitive::u128,
					},
				}
			}
		}
		pub mod pallet_bond {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					offer_fund {
						#[codec(compact)]
						lease_annual_percent_rate: ::core::primitive::u32,
						#[codec(compact)]
						lease_base_fee: ::core::primitive::u128,
						#[codec(compact)]
						amount_offered: ::core::primitive::u128,
						expiration_block: ::core::primitive::u32,
					},
					#[codec(index = 1)]
					#[doc = "Stop offering this fund for new bond. Will not affect existing bond. Unreserved funds"]
					#[doc = "are returned immediately."]
					end_fund { bond_fund_id: ::core::primitive::u32 },
					#[codec(index = 2)]
					#[doc = "Add additional time or funds to the bond fund"]
					extend_fund {
						bond_fund_id: ::core::primitive::u32,
						total_amount_offered: ::core::primitive::u128,
						expiration_block: ::core::primitive::u32,
					},
					#[codec(index = 3)]
					bond_self {
						amount: ::core::primitive::u128,
						bond_until_block: ::core::primitive::u32,
					},
					#[codec(index = 4)]
					lease {
						bond_fund_id: ::core::primitive::u32,
						amount: ::core::primitive::u128,
						lease_until_block: ::core::primitive::u32,
					},
					#[codec(index = 5)]
					return_bond { bond_id: ::core::primitive::u64 },
					#[codec(index = 6)]
					extend_bond {
						bond_id: ::core::primitive::u64,
						total_amount: ::core::primitive::u128,
						bond_until_block: ::core::primitive::u32,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					BadState,
					#[codec(index = 1)]
					BondNotFound,
					#[codec(index = 2)]
					NoMoreBondFundIds,
					#[codec(index = 3)]
					NoMoreBondIds,
					#[codec(index = 4)]
					MinimumBondAmountNotMet,
					#[codec(index = 5)]
					#[doc = "There are too many bond or bond funds expiring in the given expiration block"]
					ExpirationAtBlockOverflow,
					#[codec(index = 6)]
					InsufficientFunds,
					#[codec(index = 7)]
					InsufficientBondFunds,
					#[codec(index = 8)]
					TransactionWouldTakeAccountBelowMinimumBalance,
					#[codec(index = 9)]
					BondFundClosed,
					#[codec(index = 10)]
					#[doc = "This reduction in bond funds offered goes below the amount that is already committed to"]
					#[doc = "bond"]
					BondFundReductionExceedsAllocatedFunds,
					#[codec(index = 11)]
					ExpirationTooSoon,
					#[codec(index = 12)]
					LeaseUntilBlockTooSoon,
					#[codec(index = 13)]
					LeaseUntilPastFundExpiration,
					#[codec(index = 14)]
					NoPermissions,
					#[codec(index = 15)]
					NoBondFundFound,
					#[codec(index = 16)]
					FundExtensionMustBeLater,
					#[codec(index = 17)]
					HoldUnexpectedlyModified,
					#[codec(index = 18)]
					BondFundMaximumBondsExceeded,
					#[codec(index = 19)]
					UnrecoverableHold,
					#[codec(index = 20)]
					BondFundNotFound,
					#[codec(index = 21)]
					BondAlreadyLocked,
					#[codec(index = 22)]
					BondLockedCannotModify,
					#[codec(index = 23)]
					#[doc = "The fee for this bond exceeds the amount of the bond, which is unsafe"]
					FeeExceedsBondAmount,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					BondFundOffered {
						bond_fund_id: ::core::primitive::u32,
						amount_offered: ::core::primitive::u128,
						expiration_block: ::core::primitive::u32,
						offer_account_id: ::subxt::utils::AccountId32,
					},
					#[codec(index = 1)]
					BondFundExtended {
						bond_fund_id: ::core::primitive::u32,
						amount_offered: ::core::primitive::u128,
						expiration_block: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					BondFundEnded {
						bond_fund_id: ::core::primitive::u32,
						amount_still_bonded: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					BondFundExpired {
						bond_fund_id: ::core::primitive::u32,
						offer_account_id: ::subxt::utils::AccountId32,
					},
					#[codec(index = 4)]
					BondedSelf {
						bond_id: ::core::primitive::u64,
						bonded_account_id: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
						completion_block: ::core::primitive::u32,
					},
					#[codec(index = 5)]
					BondLeased {
						bond_fund_id: ::core::primitive::u32,
						bond_id: ::core::primitive::u64,
						bonded_account_id: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
						total_fee: ::core::primitive::u128,
						annual_percent_rate: ::core::primitive::u32,
						completion_block: ::core::primitive::u32,
					},
					#[codec(index = 6)]
					BondExtended {
						bond_fund_id: ::core::option::Option<::core::primitive::u32>,
						bond_id: ::core::primitive::u64,
						amount: ::core::primitive::u128,
						completion_block: ::core::primitive::u32,
						fee_change: ::core::primitive::u128,
						annual_percent_rate: ::core::primitive::u32,
					},
					#[codec(index = 7)]
					BondCompleted {
						bond_fund_id: ::core::option::Option<::core::primitive::u32>,
						bond_id: ::core::primitive::u64,
					},
					#[codec(index = 8)]
					BondFeeRefund {
						bond_fund_id: ::core::primitive::u32,
						bond_id: ::core::primitive::u64,
						bonded_account_id: ::subxt::utils::AccountId32,
						bond_fund_reduction_for_payment: ::core::primitive::u128,
						final_fee: ::core::primitive::u128,
						refund_amount: ::core::primitive::u128,
					},
					#[codec(index = 9)]
					BondLocked {
						bond_id: ::core::primitive::u64,
						bonded_account_id: ::subxt::utils::AccountId32,
					},
					#[codec(index = 10)]
					BondUnlocked {
						bond_id: ::core::primitive::u64,
						bonded_account_id: ::subxt::utils::AccountId32,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum HoldReason {
					#[codec(index = 0)]
					EnterBondFund,
				}
			}
		}
		pub mod pallet_chain_transfer {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					send_to_localchain {
						#[codec(compact)]
						amount: ::core::primitive::u128,
						notary_id: ::core::primitive::u32,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					MaxBlockTransfersExceeded,
					#[codec(index = 1)]
					#[doc = "Insufficient balance to create this transfer"]
					InsufficientFunds,
					#[codec(index = 2)]
					#[doc = "Insufficient balance to fulfill a mainchain transfer"]
					InsufficientNotarizedFunds,
					#[codec(index = 3)]
					#[doc = "The transfer was already submitted in a previous block"]
					InvalidOrDuplicatedLocalchainTransfer,
					#[codec(index = 4)]
					#[doc = "A transfer was submitted in a previous block but the expiration block has passed"]
					NotebookIncludesExpiredLocalchainTransfer,
					#[codec(index = 5)]
					#[doc = "The notary id is not registered"]
					InvalidNotaryUsedForTransfer,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					TransferToLocalchain {
						account_id: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
						transfer_id: ::core::primitive::u32,
						notary_id: ::core::primitive::u32,
						expiration_block: ::core::primitive::u32,
					},
					#[codec(index = 1)]
					TransferToLocalchainExpired {
						account_id: ::subxt::utils::AccountId32,
						transfer_id: ::core::primitive::u32,
						notary_id: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					TransferIn {
						account_id: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
						notary_id: ::core::primitive::u32,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct QueuedTransferOut<_0, _1, _2> {
				pub account_id: _0,
				pub amount: _1,
				pub expiration_block: _2,
				pub notary_id: ::core::primitive::u32,
			}
		}
		pub mod pallet_data_domain {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					set_zone_record {
						domain_hash: ::subxt::utils::H256,
						zone_record: runtime_types::ulx_primitives::data_domain::ZoneRecord<
							::subxt::utils::AccountId32,
						>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "The domain is not registered."]
					DomainNotRegistered,
					#[codec(index = 1)]
					#[doc = "The sender is not the owner of the domain."]
					NotDomainOwner,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A data domain zone record was updated"]
					ZoneRecordUpdated {
						domain_hash: ::subxt::utils::H256,
						zone_record: runtime_types::ulx_primitives::data_domain::ZoneRecord<
							::subxt::utils::AccountId32,
						>,
					},
					#[codec(index = 1)]
					#[doc = "A data domain was registered"]
					DataDomainRegistered {
						domain_hash: ::subxt::utils::H256,
						registration: runtime_types::pallet_data_domain::DataDomainRegistration<
							::subxt::utils::AccountId32,
						>,
					},
					#[codec(index = 2)]
					#[doc = "A data domain was registered"]
					DataDomainRenewed { domain_hash: ::subxt::utils::H256 },
					#[codec(index = 3)]
					#[doc = "A data domain was expired"]
					DataDomainExpired { domain_hash: ::subxt::utils::H256 },
					#[codec(index = 4)]
					#[doc = "A data domain registration was canceled due to a conflicting registration in the same"]
					#[doc = "tick"]
					DataDomainRegistrationCanceled {
						domain_hash: ::subxt::utils::H256,
						registration: runtime_types::pallet_data_domain::DataDomainRegistration<
							::subxt::utils::AccountId32,
						>,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct DataDomainRegistration<_0> {
				pub account_id: _0,
				pub registered_at_tick: ::core::primitive::u32,
			}
		}
		pub mod pallet_grandpa {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Report voter equivocation/misbehavior. This method will verify the"]
					#[doc = "equivocation proof and validate the given key ownership proof"]
					#[doc = "against the extracted offender. If both are valid, the offence"]
					#[doc = "will be reported."]
					report_equivocation {
						equivocation_proof: ::std::boxed::Box<
							runtime_types::sp_consensus_grandpa::EquivocationProof<
								::subxt::utils::H256,
								::core::primitive::u32,
							>,
						>,
						key_owner_proof: runtime_types::sp_session::MembershipProof,
					},
					#[codec(index = 1)]
					#[doc = "Report voter equivocation/misbehavior. This method will verify the"]
					#[doc = "equivocation proof and validate the given key ownership proof"]
					#[doc = "against the extracted offender. If both are valid, the offence"]
					#[doc = "will be reported."]
					#[doc = ""]
					#[doc = "This extrinsic must be called unsigned and it is expected that only"]
					#[doc = "block authors will call it (validated in `ValidateUnsigned`), as such"]
					#[doc = "if the block author is defined it will be defined as the equivocation"]
					#[doc = "reporter."]
					report_equivocation_unsigned {
						equivocation_proof: ::std::boxed::Box<
							runtime_types::sp_consensus_grandpa::EquivocationProof<
								::subxt::utils::H256,
								::core::primitive::u32,
							>,
						>,
						key_owner_proof: runtime_types::sp_session::MembershipProof,
					},
					#[codec(index = 2)]
					#[doc = "Note that the current authority set of the GRANDPA finality gadget has stalled."]
					#[doc = ""]
					#[doc = "This will trigger a forced authority set change at the beginning of the next session, to"]
					#[doc = "be enacted `delay` blocks after that. The `delay` should be high enough to safely assume"]
					#[doc = "that the block signalling the forced change will not be re-orged e.g. 1000 blocks."]
					#[doc = "The block production rate (which may be slowed down because of finality lagging) should"]
					#[doc = "be taken into account when choosing the `delay`. The GRANDPA voters based on the new"]
					#[doc = "authority will start voting on top of `best_finalized_block_number` for new finalized"]
					#[doc = "blocks. `best_finalized_block_number` should be the highest of the latest finalized"]
					#[doc = "block of all validators of the new authority set."]
					#[doc = ""]
					#[doc = "Only callable by root."]
					note_stalled {
						delay: ::core::primitive::u32,
						best_finalized_block_number: ::core::primitive::u32,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Attempt to signal GRANDPA pause when the authority set isn't live"]
					#[doc = "(either paused or already pending pause)."]
					PauseFailed,
					#[codec(index = 1)]
					#[doc = "Attempt to signal GRANDPA resume when the authority set isn't paused"]
					#[doc = "(either live or already pending resume)."]
					ResumeFailed,
					#[codec(index = 2)]
					#[doc = "Attempt to signal GRANDPA change with one already pending."]
					ChangePending,
					#[codec(index = 3)]
					#[doc = "Cannot signal forced change so soon after last."]
					TooSoon,
					#[codec(index = 4)]
					#[doc = "A key ownership proof provided as part of an equivocation report is invalid."]
					InvalidKeyOwnershipProof,
					#[codec(index = 5)]
					#[doc = "An equivocation proof provided as part of an equivocation report is invalid."]
					InvalidEquivocationProof,
					#[codec(index = 6)]
					#[doc = "A given equivocation report is valid but already previously reported."]
					DuplicateOffenceReport,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "New authority set has been applied."]
					NewAuthorities {
						authority_set: ::std::vec::Vec<(
							runtime_types::sp_consensus_grandpa::app::Public,
							::core::primitive::u64,
						)>,
					},
					#[codec(index = 1)]
					#[doc = "Current authority set has been paused."]
					Paused,
					#[codec(index = 2)]
					#[doc = "Current authority set has been resumed."]
					Resumed,
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct StoredPendingChange<_0> {
				pub scheduled_at: _0,
				pub delay: _0,
				pub next_authorities:
					runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<(
						runtime_types::sp_consensus_grandpa::app::Public,
						::core::primitive::u64,
					)>,
				pub forced: ::core::option::Option<_0>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum StoredState<_0> {
				#[codec(index = 0)]
				Live,
				#[codec(index = 1)]
				PendingPause { scheduled_at: _0, delay: _0 },
				#[codec(index = 2)]
				Paused,
				#[codec(index = 3)]
				PendingResume { scheduled_at: _0, delay: _0 },
			}
		}
		pub mod pallet_mining_slot {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					bid {
						bond_id: ::core::option::Option<::core::primitive::u64>,
						reward_destination:
							runtime_types::ulx_primitives::block_seal::RewardDestination<
								::subxt::utils::AccountId32,
							>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					SlotNotTakingBids,
					#[codec(index = 1)]
					TooManyBlockRegistrants,
					#[codec(index = 2)]
					UnableToRotateAuthority,
					#[codec(index = 3)]
					InsufficientOwnershipTokens,
					#[codec(index = 4)]
					InsufficientBalanceForBid,
					#[codec(index = 5)]
					BidTooLow,
					#[codec(index = 6)]
					#[doc = "Internal state has become somehow corrupted and the operation cannot continue."]
					BadInternalState,
					#[codec(index = 7)]
					#[doc = "You must register with rpc hosts so that your miner can be reached for block seal"]
					#[doc = "auditing"]
					RpcHostsAreRequired,
					#[codec(index = 8)]
					BidBondDurationTooShort,
					#[codec(index = 9)]
					CannotRegisteredOverlappingSessions,
					#[codec(index = 10)]
					BadState,
					#[codec(index = 11)]
					BondNotFound,
					#[codec(index = 12)]
					NoMoreBondIds,
					#[codec(index = 13)]
					BondFundClosed,
					#[codec(index = 14)]
					MinimumBondAmountNotMet,
					#[codec(index = 15)]
					LeaseUntilBlockTooSoon,
					#[codec(index = 16)]
					LeaseUntilPastFundExpiration,
					#[codec(index = 17)]
					#[doc = "There are too many bond or bond funds expiring in the given expiration block"]
					ExpirationAtBlockOverflow,
					#[codec(index = 18)]
					InsufficientFunds,
					#[codec(index = 19)]
					InsufficientBondFunds,
					#[codec(index = 20)]
					ExpirationTooSoon,
					#[codec(index = 21)]
					NoPermissions,
					#[codec(index = 22)]
					NoBondFundFound,
					#[codec(index = 23)]
					HoldUnexpectedlyModified,
					#[codec(index = 24)]
					BondFundMaximumBondsExceeded,
					#[codec(index = 25)]
					UnrecoverableHold,
					#[codec(index = 26)]
					BondFundNotFound,
					#[codec(index = 27)]
					BondAlreadyClosed,
					#[codec(index = 28)]
					BondAlreadyLocked,
					#[codec(index = 29)]
					BondLockedCannotModify,
					#[codec(index = 30)]
					#[doc = "The fee for this bond exceeds the amount of the bond, which is unsafe"]
					FeeExceedsBondAmount,
					#[codec(index = 31)]
					AccountWouldBeBelowMinimum,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					NewMiners {
						start_index: ::core::primitive::u32,
						new_miners: runtime_types::bounded_collections::bounded_vec::BoundedVec<
							runtime_types::ulx_primitives::block_seal::MiningRegistration<
								::subxt::utils::AccountId32,
								::core::primitive::u64,
								::core::primitive::u128,
							>,
						>,
					},
					#[codec(index = 1)]
					SlotBidderAdded {
						account_id: ::subxt::utils::AccountId32,
						bid_amount: ::core::primitive::u128,
						index: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					SlotBidderReplaced {
						account_id: ::subxt::utils::AccountId32,
						bond_id: ::core::option::Option<::core::primitive::u64>,
						kept_ownership_bond: ::core::primitive::bool,
					},
					#[codec(index = 3)]
					UnbondedMiner {
						account_id: ::subxt::utils::AccountId32,
						bond_id: ::core::option::Option<::core::primitive::u64>,
						kept_ownership_bond: ::core::primitive::bool,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum HoldReason {
					#[codec(index = 0)]
					RegisterAsMiner,
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct MinerHistory {
				pub authority_index: ::core::primitive::u32,
			}
		}
		pub mod pallet_mint {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum HoldReason {}
			}
		}
		pub mod pallet_notaries {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					propose { meta: runtime_types::ulx_primitives::notary::NotaryMeta },
					#[codec(index = 1)]
					activate { operator_account: ::subxt::utils::AccountId32 },
					#[codec(index = 2)]
					update {
						#[codec(compact)]
						notary_id: ::core::primitive::u32,
						meta: runtime_types::ulx_primitives::notary::NotaryMeta,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					ProposalNotFound,
					#[codec(index = 1)]
					MaxNotariesExceeded,
					#[codec(index = 2)]
					MaxProposalsPerBlockExceeded,
					#[codec(index = 3)]
					NotAnActiveNotary,
					#[codec(index = 4)]
					InvalidNotaryOperator,
					#[codec(index = 5)]
					NoMoreNotaryIds,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A user has proposed operating as a notary"]
					NotaryProposed {
						operator_account: ::subxt::utils::AccountId32,
						meta: runtime_types::ulx_primitives::notary::NotaryMeta,
						expires: ::core::primitive::u32,
					},
					#[codec(index = 1)]
					#[doc = "A notary proposal has been accepted"]
					NotaryActivated {
						notary: runtime_types::ulx_primitives::notary::NotaryRecord<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 2)]
					#[doc = "Notary metadata queued for update"]
					NotaryMetaUpdateQueued {
						notary_id: ::core::primitive::u32,
						meta: runtime_types::ulx_primitives::notary::NotaryMeta,
						effective_block: ::core::primitive::u32,
					},
					#[codec(index = 3)]
					#[doc = "Notary metadata updated"]
					NotaryMetaUpdated {
						notary_id: ::core::primitive::u32,
						meta: runtime_types::ulx_primitives::notary::NotaryMeta,
					},
				}
			}
		}
		pub mod pallet_notebook {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					submit {
						notebooks: ::std::vec::Vec<
							runtime_types::ulx_primitives::notebook::SignedNotebookHeader,
						>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "This notebook has already been submitted"]
					DuplicateNotebookNumber,
					#[codec(index = 1)]
					#[doc = "Notebooks received out of order"]
					MissingNotebookNumber,
					#[codec(index = 2)]
					#[doc = "A notebook was already provided at this tick"]
					NotebookTickAlreadyUsed,
					#[codec(index = 3)]
					#[doc = "The signature of the notebook is invalid"]
					InvalidNotebookSignature,
					#[codec(index = 4)]
					#[doc = "The secret or secret hash of the parent notebook do not match"]
					InvalidSecretProvided,
					#[codec(index = 5)]
					#[doc = "Could not decode the scale bytes of the notebook"]
					CouldNotDecodeNotebook,
					#[codec(index = 6)]
					#[doc = "The notebook digest was included more than once"]
					DuplicateNotebookDigest,
					#[codec(index = 7)]
					#[doc = "The notebook digest was not included"]
					MissingNotebookDigest,
					#[codec(index = 8)]
					#[doc = "The notebook digest did not match the included notebooks"]
					InvalidNotebookDigest,
					#[codec(index = 9)]
					#[doc = "Multiple inherents provided"]
					MultipleNotebookInherentsProvided,
					#[codec(index = 10)]
					#[doc = "Unable to track the notebook change list"]
					InternalError,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					NotebookSubmitted {
						notary_id: ::core::primitive::u32,
						notebook_number: ::core::primitive::u32,
					},
					#[codec(index = 1)]
					NotebookAuditFailure {
						notary_id: ::core::primitive::u32,
						notebook_number: ::core::primitive::u32,
						first_failure_reason: runtime_types::ulx_notary_audit::error::VerifyError,
					},
				}
			}
		}
		pub mod pallet_offences {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Events type."]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "There is an offence reported of the given `kind` happened at the `session_index` and"]
					#[doc = "(kind-specific) time slot. This event is not deposited for duplicate slashes."]
					#[doc = "\\[kind, timeslot\\]."]
					Offence {
						kind: [::core::primitive::u8; 16usize],
						timeslot: ::std::vec::Vec<::core::primitive::u8>,
					},
				}
			}
		}
		pub mod pallet_session {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Sets the session key(s) of the function caller to `keys`."]
					#[doc = "Allows an account to set its session key prior to becoming a validator."]
					#[doc = "This doesn't take effect until the next session."]
					#[doc = ""]
					#[doc = "The dispatch origin of this function must be signed."]
					#[doc = ""]
					#[doc = "## Complexity"]
					#[doc = "- `O(1)`. Actual cost depends on the number of length of `T::Keys::key_ids()` which is"]
					#[doc = "  fixed."]
					set_keys {
						keys: runtime_types::ulx_node_runtime::opaque::SessionKeys,
						proof: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 1)]
					#[doc = "Removes any session key(s) of the function caller."]
					#[doc = ""]
					#[doc = "This doesn't take effect until the next session."]
					#[doc = ""]
					#[doc = "The dispatch origin of this function must be Signed and the account must be either be"]
					#[doc = "convertible to a validator ID using the chain's typical addressing system (this usually"]
					#[doc = "means being a controller account) or directly convertible into a validator ID (which"]
					#[doc = "usually means being a stash account)."]
					#[doc = ""]
					#[doc = "## Complexity"]
					#[doc = "- `O(1)` in number of key types. Actual cost depends on the number of length of"]
					#[doc = "  `T::Keys::key_ids()` which is fixed."]
					purge_keys,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Error for the session pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Invalid ownership proof."]
					InvalidProof,
					#[codec(index = 1)]
					#[doc = "No associated validator ID for account."]
					NoAssociatedValidatorId,
					#[codec(index = 2)]
					#[doc = "Registered duplicate key."]
					DuplicatedKey,
					#[codec(index = 3)]
					#[doc = "No keys are associated with this account."]
					NoKeys,
					#[codec(index = 4)]
					#[doc = "Key setting account is not live, so it's impossible to associate keys."]
					NoAccount,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "New session has happened. Note that the argument is the session index, not the"]
					#[doc = "block number as the type might suggest."]
					NewSession { session_index: ::core::primitive::u32 },
				}
			}
		}
		pub mod pallet_sudo {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
					sudo { call: ::std::boxed::Box<runtime_types::ulx_node_runtime::RuntimeCall> },
					#[codec(index = 1)]
					#[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
					#[doc = "This function does not check the weight of the call, and instead allows the"]
					#[doc = "Sudo user to specify the weight of the call."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					sudo_unchecked_weight {
						call: ::std::boxed::Box<runtime_types::ulx_node_runtime::RuntimeCall>,
						weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 2)]
					#[doc = "Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo"]
					#[doc = "key."]
					set_key { new: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> },
					#[codec(index = 3)]
					#[doc = "Authenticates the sudo key and dispatches a function call with `Signed` origin from"]
					#[doc = "a given account."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					sudo_as {
						who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						call: ::std::boxed::Box<runtime_types::ulx_node_runtime::RuntimeCall>,
					},
					#[codec(index = 4)]
					#[doc = "Permanently removes the sudo key."]
					#[doc = ""]
					#[doc = "**This cannot be un-done.**"]
					remove_key,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Error for the Sudo pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Sender must be the Sudo account."]
					RequireSudo,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A sudo call just took place."]
					Sudid {
						sudo_result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 1)]
					#[doc = "The sudo key has been updated."]
					KeyChanged {
						old: ::core::option::Option<::subxt::utils::AccountId32>,
						new: ::subxt::utils::AccountId32,
					},
					#[codec(index = 2)]
					#[doc = "The key was permanently removed."]
					KeyRemoved,
					#[codec(index = 3)]
					#[doc = "A [sudo_as](Pallet::sudo_as) call just took place."]
					SudoAsDone {
						sudo_result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
				}
			}
		}
		pub mod pallet_ticks {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {}
			}
		}
		pub mod pallet_timestamp {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Set the current time."]
					#[doc = ""]
					#[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
					#[doc = "phase, if this call hasn't been invoked by that time."]
					#[doc = ""]
					#[doc = "The timestamp should be greater than the previous one by the amount specified by"]
					#[doc = "[`Config::MinimumPeriod`]."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _None_."]
					#[doc = ""]
					#[doc = "This dispatch class is _Mandatory_ to ensure it gets executed in the block. Be aware"]
					#[doc = "that changing the complexity of this call could result exhausting the resources in a"]
					#[doc = "block to execute any other calls."]
					#[doc = ""]
					#[doc = "## Complexity"]
					#[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
					#[doc = "- 1 storage read and 1 storage mutation (codec `O(1)` because of `DidUpdate::take` in"]
					#[doc = "  `on_finalize`)"]
					#[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
					set {
						#[codec(compact)]
						now: ::core::primitive::u64,
					},
				}
			}
		}
		pub mod pallet_transaction_payment {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,"]
					#[doc = "has been paid by `who`."]
					TransactionFeePaid {
						who: ::subxt::utils::AccountId32,
						actual_fee: ::core::primitive::u128,
						tip: ::core::primitive::u128,
					},
				}
			}
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct FeeDetails<_0> {
					pub inclusion_fee: ::core::option::Option<
						runtime_types::pallet_transaction_payment::types::InclusionFee<_0>,
					>,
					pub tip: _0,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct InclusionFee<_0> {
					pub base_fee: _0,
					pub len_fee: _0,
					pub adjusted_weight_fee: _0,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct RuntimeDispatchInfo<_0, _1> {
					pub weight: _1,
					pub class: runtime_types::frame_support::dispatch::DispatchClass,
					pub partial_fee: _0,
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ChargeTransactionPayment(#[codec(compact)] pub ::core::primitive::u128);
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum Releases {
				#[codec(index = 0)]
				V1Ancient,
				#[codec(index = 1)]
				V2,
			}
		}
		pub mod pallet_tx_pause {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Pause a call."]
					#[doc = ""]
					#[doc = "Can only be called by [`Config::PauseOrigin`]."]
					#[doc = "Emits an [`Event::CallPaused`] event on success."]
					pause {
						full_name: (
							runtime_types::bounded_collections::bounded_vec::BoundedVec<
								::core::primitive::u8,
							>,
							runtime_types::bounded_collections::bounded_vec::BoundedVec<
								::core::primitive::u8,
							>,
						),
					},
					#[codec(index = 1)]
					#[doc = "Un-pause a call."]
					#[doc = ""]
					#[doc = "Can only be called by [`Config::UnpauseOrigin`]."]
					#[doc = "Emits an [`Event::CallUnpaused`] event on success."]
					unpause {
						ident: (
							runtime_types::bounded_collections::bounded_vec::BoundedVec<
								::core::primitive::u8,
							>,
							runtime_types::bounded_collections::bounded_vec::BoundedVec<
								::core::primitive::u8,
							>,
						),
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "The call is paused."]
					IsPaused,
					#[codec(index = 1)]
					#[doc = "The call is unpaused."]
					IsUnpaused,
					#[codec(index = 2)]
					#[doc = "The call is whitelisted and cannot be paused."]
					Unpausable,
					#[codec(index = 3)]
					NotFound,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "This pallet, or a specific call is now paused."]
					CallPaused {
						full_name: (
							runtime_types::bounded_collections::bounded_vec::BoundedVec<
								::core::primitive::u8,
							>,
							runtime_types::bounded_collections::bounded_vec::BoundedVec<
								::core::primitive::u8,
							>,
						),
					},
					#[codec(index = 1)]
					#[doc = "This pallet, or a specific call is now unpaused."]
					CallUnpaused {
						full_name: (
							runtime_types::bounded_collections::bounded_vec::BoundedVec<
								::core::primitive::u8,
							>,
							runtime_types::bounded_collections::bounded_vec::BoundedVec<
								::core::primitive::u8,
							>,
						),
					},
				}
			}
		}
		pub mod primitive_types {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct U256(pub [::core::primitive::u64; 4usize]);
		}
		pub mod sp_arithmetic {
			use super::runtime_types;
			pub mod fixed_point {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct FixedU128(pub ::core::primitive::u128);
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum ArithmeticError {
				#[codec(index = 0)]
				Underflow,
				#[codec(index = 1)]
				Overflow,
				#[codec(index = 2)]
				DivisionByZero,
			}
		}
		pub mod sp_consensus_grandpa {
			use super::runtime_types;
			pub mod app {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Public(pub [::core::primitive::u8; 32usize]);
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Signature(pub [::core::primitive::u8; 64usize]);
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum Equivocation<_0, _1> {
				#[codec(index = 0)]
				Prevote(
					runtime_types::finality_grandpa::Equivocation<
						runtime_types::sp_consensus_grandpa::app::Public,
						runtime_types::finality_grandpa::Prevote<_0, _1>,
						runtime_types::sp_consensus_grandpa::app::Signature,
					>,
				),
				#[codec(index = 1)]
				Precommit(
					runtime_types::finality_grandpa::Equivocation<
						runtime_types::sp_consensus_grandpa::app::Public,
						runtime_types::finality_grandpa::Precommit<_0, _1>,
						runtime_types::sp_consensus_grandpa::app::Signature,
					>,
				),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct EquivocationProof<_0, _1> {
				pub set_id: ::core::primitive::u64,
				pub equivocation: runtime_types::sp_consensus_grandpa::Equivocation<_0, _1>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct OpaqueKeyOwnershipProof(pub ::std::vec::Vec<::core::primitive::u8>);
		}
		pub mod sp_core {
			use super::runtime_types;
			pub mod crypto {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct KeyTypeId(pub [::core::primitive::u8; 4usize]);
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct OpaqueMetadata(pub ::std::vec::Vec<::core::primitive::u8>);
		}
		pub mod sp_inherents {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct CheckInherentsResult {
				pub okay: ::core::primitive::bool,
				pub fatal_error: ::core::primitive::bool,
				pub errors: runtime_types::sp_inherents::InherentData,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct InherentData {
				pub data: ::subxt::utils::KeyedVec<
					[::core::primitive::u8; 8usize],
					::std::vec::Vec<::core::primitive::u8>,
				>,
			}
		}
		pub mod sp_runtime {
			use super::runtime_types;
			pub mod generic {
				use super::runtime_types;
				pub mod block {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Clone,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct Block<_0, _1> {
						pub header: _0,
						pub extrinsics: ::std::vec::Vec<_1>,
					}
				}
				pub mod digest {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Clone,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct Digest {
						pub logs:
							::std::vec::Vec<runtime_types::sp_runtime::generic::digest::DigestItem>,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Clone,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum DigestItem {
						#[codec(index = 6)]
						PreRuntime(
							[::core::primitive::u8; 4usize],
							::std::vec::Vec<::core::primitive::u8>,
						),
						#[codec(index = 4)]
						Consensus(
							[::core::primitive::u8; 4usize],
							::std::vec::Vec<::core::primitive::u8>,
						),
						#[codec(index = 5)]
						Seal(
							[::core::primitive::u8; 4usize],
							::std::vec::Vec<::core::primitive::u8>,
						),
						#[codec(index = 0)]
						Other(::std::vec::Vec<::core::primitive::u8>),
						#[codec(index = 8)]
						RuntimeEnvironmentUpdated,
					}
				}
				pub mod era {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Clone,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Era {
						#[codec(index = 0)]
						Immortal,
						#[codec(index = 1)]
						Mortal1(::core::primitive::u8),
						#[codec(index = 2)]
						Mortal2(::core::primitive::u8),
						#[codec(index = 3)]
						Mortal3(::core::primitive::u8),
						#[codec(index = 4)]
						Mortal4(::core::primitive::u8),
						#[codec(index = 5)]
						Mortal5(::core::primitive::u8),
						#[codec(index = 6)]
						Mortal6(::core::primitive::u8),
						#[codec(index = 7)]
						Mortal7(::core::primitive::u8),
						#[codec(index = 8)]
						Mortal8(::core::primitive::u8),
						#[codec(index = 9)]
						Mortal9(::core::primitive::u8),
						#[codec(index = 10)]
						Mortal10(::core::primitive::u8),
						#[codec(index = 11)]
						Mortal11(::core::primitive::u8),
						#[codec(index = 12)]
						Mortal12(::core::primitive::u8),
						#[codec(index = 13)]
						Mortal13(::core::primitive::u8),
						#[codec(index = 14)]
						Mortal14(::core::primitive::u8),
						#[codec(index = 15)]
						Mortal15(::core::primitive::u8),
						#[codec(index = 16)]
						Mortal16(::core::primitive::u8),
						#[codec(index = 17)]
						Mortal17(::core::primitive::u8),
						#[codec(index = 18)]
						Mortal18(::core::primitive::u8),
						#[codec(index = 19)]
						Mortal19(::core::primitive::u8),
						#[codec(index = 20)]
						Mortal20(::core::primitive::u8),
						#[codec(index = 21)]
						Mortal21(::core::primitive::u8),
						#[codec(index = 22)]
						Mortal22(::core::primitive::u8),
						#[codec(index = 23)]
						Mortal23(::core::primitive::u8),
						#[codec(index = 24)]
						Mortal24(::core::primitive::u8),
						#[codec(index = 25)]
						Mortal25(::core::primitive::u8),
						#[codec(index = 26)]
						Mortal26(::core::primitive::u8),
						#[codec(index = 27)]
						Mortal27(::core::primitive::u8),
						#[codec(index = 28)]
						Mortal28(::core::primitive::u8),
						#[codec(index = 29)]
						Mortal29(::core::primitive::u8),
						#[codec(index = 30)]
						Mortal30(::core::primitive::u8),
						#[codec(index = 31)]
						Mortal31(::core::primitive::u8),
						#[codec(index = 32)]
						Mortal32(::core::primitive::u8),
						#[codec(index = 33)]
						Mortal33(::core::primitive::u8),
						#[codec(index = 34)]
						Mortal34(::core::primitive::u8),
						#[codec(index = 35)]
						Mortal35(::core::primitive::u8),
						#[codec(index = 36)]
						Mortal36(::core::primitive::u8),
						#[codec(index = 37)]
						Mortal37(::core::primitive::u8),
						#[codec(index = 38)]
						Mortal38(::core::primitive::u8),
						#[codec(index = 39)]
						Mortal39(::core::primitive::u8),
						#[codec(index = 40)]
						Mortal40(::core::primitive::u8),
						#[codec(index = 41)]
						Mortal41(::core::primitive::u8),
						#[codec(index = 42)]
						Mortal42(::core::primitive::u8),
						#[codec(index = 43)]
						Mortal43(::core::primitive::u8),
						#[codec(index = 44)]
						Mortal44(::core::primitive::u8),
						#[codec(index = 45)]
						Mortal45(::core::primitive::u8),
						#[codec(index = 46)]
						Mortal46(::core::primitive::u8),
						#[codec(index = 47)]
						Mortal47(::core::primitive::u8),
						#[codec(index = 48)]
						Mortal48(::core::primitive::u8),
						#[codec(index = 49)]
						Mortal49(::core::primitive::u8),
						#[codec(index = 50)]
						Mortal50(::core::primitive::u8),
						#[codec(index = 51)]
						Mortal51(::core::primitive::u8),
						#[codec(index = 52)]
						Mortal52(::core::primitive::u8),
						#[codec(index = 53)]
						Mortal53(::core::primitive::u8),
						#[codec(index = 54)]
						Mortal54(::core::primitive::u8),
						#[codec(index = 55)]
						Mortal55(::core::primitive::u8),
						#[codec(index = 56)]
						Mortal56(::core::primitive::u8),
						#[codec(index = 57)]
						Mortal57(::core::primitive::u8),
						#[codec(index = 58)]
						Mortal58(::core::primitive::u8),
						#[codec(index = 59)]
						Mortal59(::core::primitive::u8),
						#[codec(index = 60)]
						Mortal60(::core::primitive::u8),
						#[codec(index = 61)]
						Mortal61(::core::primitive::u8),
						#[codec(index = 62)]
						Mortal62(::core::primitive::u8),
						#[codec(index = 63)]
						Mortal63(::core::primitive::u8),
						#[codec(index = 64)]
						Mortal64(::core::primitive::u8),
						#[codec(index = 65)]
						Mortal65(::core::primitive::u8),
						#[codec(index = 66)]
						Mortal66(::core::primitive::u8),
						#[codec(index = 67)]
						Mortal67(::core::primitive::u8),
						#[codec(index = 68)]
						Mortal68(::core::primitive::u8),
						#[codec(index = 69)]
						Mortal69(::core::primitive::u8),
						#[codec(index = 70)]
						Mortal70(::core::primitive::u8),
						#[codec(index = 71)]
						Mortal71(::core::primitive::u8),
						#[codec(index = 72)]
						Mortal72(::core::primitive::u8),
						#[codec(index = 73)]
						Mortal73(::core::primitive::u8),
						#[codec(index = 74)]
						Mortal74(::core::primitive::u8),
						#[codec(index = 75)]
						Mortal75(::core::primitive::u8),
						#[codec(index = 76)]
						Mortal76(::core::primitive::u8),
						#[codec(index = 77)]
						Mortal77(::core::primitive::u8),
						#[codec(index = 78)]
						Mortal78(::core::primitive::u8),
						#[codec(index = 79)]
						Mortal79(::core::primitive::u8),
						#[codec(index = 80)]
						Mortal80(::core::primitive::u8),
						#[codec(index = 81)]
						Mortal81(::core::primitive::u8),
						#[codec(index = 82)]
						Mortal82(::core::primitive::u8),
						#[codec(index = 83)]
						Mortal83(::core::primitive::u8),
						#[codec(index = 84)]
						Mortal84(::core::primitive::u8),
						#[codec(index = 85)]
						Mortal85(::core::primitive::u8),
						#[codec(index = 86)]
						Mortal86(::core::primitive::u8),
						#[codec(index = 87)]
						Mortal87(::core::primitive::u8),
						#[codec(index = 88)]
						Mortal88(::core::primitive::u8),
						#[codec(index = 89)]
						Mortal89(::core::primitive::u8),
						#[codec(index = 90)]
						Mortal90(::core::primitive::u8),
						#[codec(index = 91)]
						Mortal91(::core::primitive::u8),
						#[codec(index = 92)]
						Mortal92(::core::primitive::u8),
						#[codec(index = 93)]
						Mortal93(::core::primitive::u8),
						#[codec(index = 94)]
						Mortal94(::core::primitive::u8),
						#[codec(index = 95)]
						Mortal95(::core::primitive::u8),
						#[codec(index = 96)]
						Mortal96(::core::primitive::u8),
						#[codec(index = 97)]
						Mortal97(::core::primitive::u8),
						#[codec(index = 98)]
						Mortal98(::core::primitive::u8),
						#[codec(index = 99)]
						Mortal99(::core::primitive::u8),
						#[codec(index = 100)]
						Mortal100(::core::primitive::u8),
						#[codec(index = 101)]
						Mortal101(::core::primitive::u8),
						#[codec(index = 102)]
						Mortal102(::core::primitive::u8),
						#[codec(index = 103)]
						Mortal103(::core::primitive::u8),
						#[codec(index = 104)]
						Mortal104(::core::primitive::u8),
						#[codec(index = 105)]
						Mortal105(::core::primitive::u8),
						#[codec(index = 106)]
						Mortal106(::core::primitive::u8),
						#[codec(index = 107)]
						Mortal107(::core::primitive::u8),
						#[codec(index = 108)]
						Mortal108(::core::primitive::u8),
						#[codec(index = 109)]
						Mortal109(::core::primitive::u8),
						#[codec(index = 110)]
						Mortal110(::core::primitive::u8),
						#[codec(index = 111)]
						Mortal111(::core::primitive::u8),
						#[codec(index = 112)]
						Mortal112(::core::primitive::u8),
						#[codec(index = 113)]
						Mortal113(::core::primitive::u8),
						#[codec(index = 114)]
						Mortal114(::core::primitive::u8),
						#[codec(index = 115)]
						Mortal115(::core::primitive::u8),
						#[codec(index = 116)]
						Mortal116(::core::primitive::u8),
						#[codec(index = 117)]
						Mortal117(::core::primitive::u8),
						#[codec(index = 118)]
						Mortal118(::core::primitive::u8),
						#[codec(index = 119)]
						Mortal119(::core::primitive::u8),
						#[codec(index = 120)]
						Mortal120(::core::primitive::u8),
						#[codec(index = 121)]
						Mortal121(::core::primitive::u8),
						#[codec(index = 122)]
						Mortal122(::core::primitive::u8),
						#[codec(index = 123)]
						Mortal123(::core::primitive::u8),
						#[codec(index = 124)]
						Mortal124(::core::primitive::u8),
						#[codec(index = 125)]
						Mortal125(::core::primitive::u8),
						#[codec(index = 126)]
						Mortal126(::core::primitive::u8),
						#[codec(index = 127)]
						Mortal127(::core::primitive::u8),
						#[codec(index = 128)]
						Mortal128(::core::primitive::u8),
						#[codec(index = 129)]
						Mortal129(::core::primitive::u8),
						#[codec(index = 130)]
						Mortal130(::core::primitive::u8),
						#[codec(index = 131)]
						Mortal131(::core::primitive::u8),
						#[codec(index = 132)]
						Mortal132(::core::primitive::u8),
						#[codec(index = 133)]
						Mortal133(::core::primitive::u8),
						#[codec(index = 134)]
						Mortal134(::core::primitive::u8),
						#[codec(index = 135)]
						Mortal135(::core::primitive::u8),
						#[codec(index = 136)]
						Mortal136(::core::primitive::u8),
						#[codec(index = 137)]
						Mortal137(::core::primitive::u8),
						#[codec(index = 138)]
						Mortal138(::core::primitive::u8),
						#[codec(index = 139)]
						Mortal139(::core::primitive::u8),
						#[codec(index = 140)]
						Mortal140(::core::primitive::u8),
						#[codec(index = 141)]
						Mortal141(::core::primitive::u8),
						#[codec(index = 142)]
						Mortal142(::core::primitive::u8),
						#[codec(index = 143)]
						Mortal143(::core::primitive::u8),
						#[codec(index = 144)]
						Mortal144(::core::primitive::u8),
						#[codec(index = 145)]
						Mortal145(::core::primitive::u8),
						#[codec(index = 146)]
						Mortal146(::core::primitive::u8),
						#[codec(index = 147)]
						Mortal147(::core::primitive::u8),
						#[codec(index = 148)]
						Mortal148(::core::primitive::u8),
						#[codec(index = 149)]
						Mortal149(::core::primitive::u8),
						#[codec(index = 150)]
						Mortal150(::core::primitive::u8),
						#[codec(index = 151)]
						Mortal151(::core::primitive::u8),
						#[codec(index = 152)]
						Mortal152(::core::primitive::u8),
						#[codec(index = 153)]
						Mortal153(::core::primitive::u8),
						#[codec(index = 154)]
						Mortal154(::core::primitive::u8),
						#[codec(index = 155)]
						Mortal155(::core::primitive::u8),
						#[codec(index = 156)]
						Mortal156(::core::primitive::u8),
						#[codec(index = 157)]
						Mortal157(::core::primitive::u8),
						#[codec(index = 158)]
						Mortal158(::core::primitive::u8),
						#[codec(index = 159)]
						Mortal159(::core::primitive::u8),
						#[codec(index = 160)]
						Mortal160(::core::primitive::u8),
						#[codec(index = 161)]
						Mortal161(::core::primitive::u8),
						#[codec(index = 162)]
						Mortal162(::core::primitive::u8),
						#[codec(index = 163)]
						Mortal163(::core::primitive::u8),
						#[codec(index = 164)]
						Mortal164(::core::primitive::u8),
						#[codec(index = 165)]
						Mortal165(::core::primitive::u8),
						#[codec(index = 166)]
						Mortal166(::core::primitive::u8),
						#[codec(index = 167)]
						Mortal167(::core::primitive::u8),
						#[codec(index = 168)]
						Mortal168(::core::primitive::u8),
						#[codec(index = 169)]
						Mortal169(::core::primitive::u8),
						#[codec(index = 170)]
						Mortal170(::core::primitive::u8),
						#[codec(index = 171)]
						Mortal171(::core::primitive::u8),
						#[codec(index = 172)]
						Mortal172(::core::primitive::u8),
						#[codec(index = 173)]
						Mortal173(::core::primitive::u8),
						#[codec(index = 174)]
						Mortal174(::core::primitive::u8),
						#[codec(index = 175)]
						Mortal175(::core::primitive::u8),
						#[codec(index = 176)]
						Mortal176(::core::primitive::u8),
						#[codec(index = 177)]
						Mortal177(::core::primitive::u8),
						#[codec(index = 178)]
						Mortal178(::core::primitive::u8),
						#[codec(index = 179)]
						Mortal179(::core::primitive::u8),
						#[codec(index = 180)]
						Mortal180(::core::primitive::u8),
						#[codec(index = 181)]
						Mortal181(::core::primitive::u8),
						#[codec(index = 182)]
						Mortal182(::core::primitive::u8),
						#[codec(index = 183)]
						Mortal183(::core::primitive::u8),
						#[codec(index = 184)]
						Mortal184(::core::primitive::u8),
						#[codec(index = 185)]
						Mortal185(::core::primitive::u8),
						#[codec(index = 186)]
						Mortal186(::core::primitive::u8),
						#[codec(index = 187)]
						Mortal187(::core::primitive::u8),
						#[codec(index = 188)]
						Mortal188(::core::primitive::u8),
						#[codec(index = 189)]
						Mortal189(::core::primitive::u8),
						#[codec(index = 190)]
						Mortal190(::core::primitive::u8),
						#[codec(index = 191)]
						Mortal191(::core::primitive::u8),
						#[codec(index = 192)]
						Mortal192(::core::primitive::u8),
						#[codec(index = 193)]
						Mortal193(::core::primitive::u8),
						#[codec(index = 194)]
						Mortal194(::core::primitive::u8),
						#[codec(index = 195)]
						Mortal195(::core::primitive::u8),
						#[codec(index = 196)]
						Mortal196(::core::primitive::u8),
						#[codec(index = 197)]
						Mortal197(::core::primitive::u8),
						#[codec(index = 198)]
						Mortal198(::core::primitive::u8),
						#[codec(index = 199)]
						Mortal199(::core::primitive::u8),
						#[codec(index = 200)]
						Mortal200(::core::primitive::u8),
						#[codec(index = 201)]
						Mortal201(::core::primitive::u8),
						#[codec(index = 202)]
						Mortal202(::core::primitive::u8),
						#[codec(index = 203)]
						Mortal203(::core::primitive::u8),
						#[codec(index = 204)]
						Mortal204(::core::primitive::u8),
						#[codec(index = 205)]
						Mortal205(::core::primitive::u8),
						#[codec(index = 206)]
						Mortal206(::core::primitive::u8),
						#[codec(index = 207)]
						Mortal207(::core::primitive::u8),
						#[codec(index = 208)]
						Mortal208(::core::primitive::u8),
						#[codec(index = 209)]
						Mortal209(::core::primitive::u8),
						#[codec(index = 210)]
						Mortal210(::core::primitive::u8),
						#[codec(index = 211)]
						Mortal211(::core::primitive::u8),
						#[codec(index = 212)]
						Mortal212(::core::primitive::u8),
						#[codec(index = 213)]
						Mortal213(::core::primitive::u8),
						#[codec(index = 214)]
						Mortal214(::core::primitive::u8),
						#[codec(index = 215)]
						Mortal215(::core::primitive::u8),
						#[codec(index = 216)]
						Mortal216(::core::primitive::u8),
						#[codec(index = 217)]
						Mortal217(::core::primitive::u8),
						#[codec(index = 218)]
						Mortal218(::core::primitive::u8),
						#[codec(index = 219)]
						Mortal219(::core::primitive::u8),
						#[codec(index = 220)]
						Mortal220(::core::primitive::u8),
						#[codec(index = 221)]
						Mortal221(::core::primitive::u8),
						#[codec(index = 222)]
						Mortal222(::core::primitive::u8),
						#[codec(index = 223)]
						Mortal223(::core::primitive::u8),
						#[codec(index = 224)]
						Mortal224(::core::primitive::u8),
						#[codec(index = 225)]
						Mortal225(::core::primitive::u8),
						#[codec(index = 226)]
						Mortal226(::core::primitive::u8),
						#[codec(index = 227)]
						Mortal227(::core::primitive::u8),
						#[codec(index = 228)]
						Mortal228(::core::primitive::u8),
						#[codec(index = 229)]
						Mortal229(::core::primitive::u8),
						#[codec(index = 230)]
						Mortal230(::core::primitive::u8),
						#[codec(index = 231)]
						Mortal231(::core::primitive::u8),
						#[codec(index = 232)]
						Mortal232(::core::primitive::u8),
						#[codec(index = 233)]
						Mortal233(::core::primitive::u8),
						#[codec(index = 234)]
						Mortal234(::core::primitive::u8),
						#[codec(index = 235)]
						Mortal235(::core::primitive::u8),
						#[codec(index = 236)]
						Mortal236(::core::primitive::u8),
						#[codec(index = 237)]
						Mortal237(::core::primitive::u8),
						#[codec(index = 238)]
						Mortal238(::core::primitive::u8),
						#[codec(index = 239)]
						Mortal239(::core::primitive::u8),
						#[codec(index = 240)]
						Mortal240(::core::primitive::u8),
						#[codec(index = 241)]
						Mortal241(::core::primitive::u8),
						#[codec(index = 242)]
						Mortal242(::core::primitive::u8),
						#[codec(index = 243)]
						Mortal243(::core::primitive::u8),
						#[codec(index = 244)]
						Mortal244(::core::primitive::u8),
						#[codec(index = 245)]
						Mortal245(::core::primitive::u8),
						#[codec(index = 246)]
						Mortal246(::core::primitive::u8),
						#[codec(index = 247)]
						Mortal247(::core::primitive::u8),
						#[codec(index = 248)]
						Mortal248(::core::primitive::u8),
						#[codec(index = 249)]
						Mortal249(::core::primitive::u8),
						#[codec(index = 250)]
						Mortal250(::core::primitive::u8),
						#[codec(index = 251)]
						Mortal251(::core::primitive::u8),
						#[codec(index = 252)]
						Mortal252(::core::primitive::u8),
						#[codec(index = 253)]
						Mortal253(::core::primitive::u8),
						#[codec(index = 254)]
						Mortal254(::core::primitive::u8),
						#[codec(index = 255)]
						Mortal255(::core::primitive::u8),
					}
				}
				pub mod header {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Clone,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct Header<_0> {
						pub parent_hash: ::subxt::utils::H256,
						#[codec(compact)]
						pub number: _0,
						pub state_root: ::subxt::utils::H256,
						pub extrinsics_root: ::subxt::utils::H256,
						pub digest: runtime_types::sp_runtime::generic::digest::Digest,
					}
				}
			}
			pub mod transaction_validity {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum InvalidTransaction {
					#[codec(index = 0)]
					Call,
					#[codec(index = 1)]
					Payment,
					#[codec(index = 2)]
					Future,
					#[codec(index = 3)]
					Stale,
					#[codec(index = 4)]
					BadProof,
					#[codec(index = 5)]
					AncientBirthBlock,
					#[codec(index = 6)]
					ExhaustsResources,
					#[codec(index = 7)]
					Custom(::core::primitive::u8),
					#[codec(index = 8)]
					BadMandatory,
					#[codec(index = 9)]
					MandatoryValidation,
					#[codec(index = 10)]
					BadSigner,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum TransactionSource {
					#[codec(index = 0)]
					InBlock,
					#[codec(index = 1)]
					Local,
					#[codec(index = 2)]
					External,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum TransactionValidityError {
					#[codec(index = 0)]
					Invalid(runtime_types::sp_runtime::transaction_validity::InvalidTransaction),
					#[codec(index = 1)]
					Unknown(runtime_types::sp_runtime::transaction_validity::UnknownTransaction),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum UnknownTransaction {
					#[codec(index = 0)]
					CannotLookup,
					#[codec(index = 1)]
					NoUnsignedValidator,
					#[codec(index = 2)]
					Custom(::core::primitive::u8),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ValidTransaction {
					pub priority: ::core::primitive::u64,
					pub requires: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
					pub provides: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
					pub longevity: ::core::primitive::u64,
					pub propagate: ::core::primitive::bool,
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum DispatchError {
				#[codec(index = 0)]
				Other,
				#[codec(index = 1)]
				CannotLookup,
				#[codec(index = 2)]
				BadOrigin,
				#[codec(index = 3)]
				Module(runtime_types::sp_runtime::ModuleError),
				#[codec(index = 4)]
				ConsumerRemaining,
				#[codec(index = 5)]
				NoProviders,
				#[codec(index = 6)]
				TooManyConsumers,
				#[codec(index = 7)]
				Token(runtime_types::sp_runtime::TokenError),
				#[codec(index = 8)]
				Arithmetic(runtime_types::sp_arithmetic::ArithmeticError),
				#[codec(index = 9)]
				Transactional(runtime_types::sp_runtime::TransactionalError),
				#[codec(index = 10)]
				Exhausted,
				#[codec(index = 11)]
				Corruption,
				#[codec(index = 12)]
				Unavailable,
				#[codec(index = 13)]
				RootNotAllowed,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum ExtrinsicInclusionMode {
				#[codec(index = 0)]
				AllExtrinsics,
				#[codec(index = 1)]
				OnlyInherents,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ModuleError {
				pub index: ::core::primitive::u8,
				pub error: [::core::primitive::u8; 4usize],
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum MultiSignature {
				#[codec(index = 0)]
				Ed25519([::core::primitive::u8; 64usize]),
				#[codec(index = 1)]
				Sr25519([::core::primitive::u8; 64usize]),
				#[codec(index = 2)]
				Ecdsa([::core::primitive::u8; 65usize]),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum TokenError {
				#[codec(index = 0)]
				FundsUnavailable,
				#[codec(index = 1)]
				OnlyProvider,
				#[codec(index = 2)]
				BelowMinimum,
				#[codec(index = 3)]
				CannotCreate,
				#[codec(index = 4)]
				UnknownAsset,
				#[codec(index = 5)]
				Frozen,
				#[codec(index = 6)]
				Unsupported,
				#[codec(index = 7)]
				CannotCreateHold,
				#[codec(index = 8)]
				NotExpendable,
				#[codec(index = 9)]
				Blocked,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum TransactionalError {
				#[codec(index = 0)]
				LimitReached,
				#[codec(index = 1)]
				NoLayer,
			}
		}
		pub mod sp_session {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct MembershipProof {
				pub session: ::core::primitive::u32,
				pub trie_nodes: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
				pub validator_count: ::core::primitive::u32,
			}
		}
		pub mod sp_staking {
			use super::runtime_types;
			pub mod offence {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct OffenceDetails<_0, _1> {
					pub offender: _1,
					pub reporters: ::std::vec::Vec<_0>,
				}
			}
		}
		pub mod sp_version {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct RuntimeVersion {
				pub spec_name: ::std::string::String,
				pub impl_name: ::std::string::String,
				pub authoring_version: ::core::primitive::u32,
				pub spec_version: ::core::primitive::u32,
				pub impl_version: ::core::primitive::u32,
				pub apis:
					::std::vec::Vec<([::core::primitive::u8; 8usize], ::core::primitive::u32)>,
				pub transaction_version: ::core::primitive::u32,
				pub state_version: ::core::primitive::u8,
			}
		}
		pub mod sp_weights {
			use super::runtime_types;
			pub mod weight_v2 {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Weight {
					#[codec(compact)]
					pub ref_time: ::core::primitive::u64,
					#[codec(compact)]
					pub proof_size: ::core::primitive::u64,
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct RuntimeDbWeight {
				pub read: ::core::primitive::u64,
				pub write: ::core::primitive::u64,
			}
		}
		pub mod ulx_node_runtime {
			use super::runtime_types;
			pub mod opaque {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SessionKeys {
					pub grandpa: runtime_types::sp_consensus_grandpa::app::Public,
					pub block_seal_authority:
						runtime_types::ulx_primitives::block_seal::app::Public,
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Runtime;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum RuntimeCall {
				#[codec(index = 0)]
				System(runtime_types::frame_system::pallet::Call),
				#[codec(index = 1)]
				Timestamp(runtime_types::pallet_timestamp::pallet::Call),
				#[codec(index = 2)]
				Ticks(runtime_types::pallet_ticks::pallet::Call),
				#[codec(index = 3)]
				MiningSlot(runtime_types::pallet_mining_slot::pallet::Call),
				#[codec(index = 4)]
				Bond(runtime_types::pallet_bond::pallet::Call),
				#[codec(index = 5)]
				Notaries(runtime_types::pallet_notaries::pallet::Call),
				#[codec(index = 6)]
				Notebook(runtime_types::pallet_notebook::pallet::Call),
				#[codec(index = 7)]
				ChainTransfer(runtime_types::pallet_chain_transfer::pallet::Call),
				#[codec(index = 8)]
				BlockSealSpec(runtime_types::pallet_block_seal_spec::pallet::Call),
				#[codec(index = 9)]
				DataDomain(runtime_types::pallet_data_domain::pallet::Call),
				#[codec(index = 12)]
				Session(runtime_types::pallet_session::pallet::Call),
				#[codec(index = 13)]
				BlockSeal(runtime_types::pallet_block_seal::pallet::Call),
				#[codec(index = 14)]
				BlockRewards(runtime_types::pallet_block_rewards::pallet::Call),
				#[codec(index = 15)]
				Grandpa(runtime_types::pallet_grandpa::pallet::Call),
				#[codec(index = 17)]
				ArgonBalances(runtime_types::pallet_balances::pallet::Call),
				#[codec(index = 18)]
				Mint(runtime_types::pallet_mint::pallet::Call),
				#[codec(index = 19)]
				UlixeeBalances(runtime_types::pallet_balances::pallet::Call2),
				#[codec(index = 20)]
				TxPause(runtime_types::pallet_tx_pause::pallet::Call),
				#[codec(index = 22)]
				Sudo(runtime_types::pallet_sudo::pallet::Call),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum RuntimeError {
				#[codec(index = 0)]
				System(runtime_types::frame_system::pallet::Error),
				#[codec(index = 2)]
				Ticks(runtime_types::pallet_ticks::pallet::Error),
				#[codec(index = 3)]
				MiningSlot(runtime_types::pallet_mining_slot::pallet::Error),
				#[codec(index = 4)]
				Bond(runtime_types::pallet_bond::pallet::Error),
				#[codec(index = 5)]
				Notaries(runtime_types::pallet_notaries::pallet::Error),
				#[codec(index = 6)]
				Notebook(runtime_types::pallet_notebook::pallet::Error),
				#[codec(index = 7)]
				ChainTransfer(runtime_types::pallet_chain_transfer::pallet::Error),
				#[codec(index = 8)]
				BlockSealSpec(runtime_types::pallet_block_seal_spec::pallet::Error),
				#[codec(index = 9)]
				DataDomain(runtime_types::pallet_data_domain::pallet::Error),
				#[codec(index = 12)]
				Session(runtime_types::pallet_session::pallet::Error),
				#[codec(index = 13)]
				BlockSeal(runtime_types::pallet_block_seal::pallet::Error),
				#[codec(index = 14)]
				BlockRewards(runtime_types::pallet_block_rewards::pallet::Error),
				#[codec(index = 15)]
				Grandpa(runtime_types::pallet_grandpa::pallet::Error),
				#[codec(index = 17)]
				ArgonBalances(runtime_types::pallet_balances::pallet::Error),
				#[codec(index = 18)]
				Mint(runtime_types::pallet_mint::pallet::Error),
				#[codec(index = 19)]
				UlixeeBalances(runtime_types::pallet_balances::pallet::Error2),
				#[codec(index = 20)]
				TxPause(runtime_types::pallet_tx_pause::pallet::Error),
				#[codec(index = 22)]
				Sudo(runtime_types::pallet_sudo::pallet::Error),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum RuntimeEvent {
				#[codec(index = 0)]
				System(runtime_types::frame_system::pallet::Event),
				#[codec(index = 3)]
				MiningSlot(runtime_types::pallet_mining_slot::pallet::Event),
				#[codec(index = 4)]
				Bond(runtime_types::pallet_bond::pallet::Event),
				#[codec(index = 5)]
				Notaries(runtime_types::pallet_notaries::pallet::Event),
				#[codec(index = 6)]
				Notebook(runtime_types::pallet_notebook::pallet::Event),
				#[codec(index = 7)]
				ChainTransfer(runtime_types::pallet_chain_transfer::pallet::Event),
				#[codec(index = 8)]
				BlockSealSpec(runtime_types::pallet_block_seal_spec::pallet::Event),
				#[codec(index = 9)]
				DataDomain(runtime_types::pallet_data_domain::pallet::Event),
				#[codec(index = 12)]
				Session(runtime_types::pallet_session::pallet::Event),
				#[codec(index = 14)]
				BlockRewards(runtime_types::pallet_block_rewards::pallet::Event),
				#[codec(index = 15)]
				Grandpa(runtime_types::pallet_grandpa::pallet::Event),
				#[codec(index = 16)]
				Offences(runtime_types::pallet_offences::pallet::Event),
				#[codec(index = 17)]
				ArgonBalances(runtime_types::pallet_balances::pallet::Event),
				#[codec(index = 18)]
				Mint(runtime_types::pallet_mint::pallet::Event),
				#[codec(index = 19)]
				UlixeeBalances(runtime_types::pallet_balances::pallet::Event2),
				#[codec(index = 20)]
				TxPause(runtime_types::pallet_tx_pause::pallet::Event),
				#[codec(index = 21)]
				TransactionPayment(runtime_types::pallet_transaction_payment::pallet::Event),
				#[codec(index = 22)]
				Sudo(runtime_types::pallet_sudo::pallet::Event),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum RuntimeFreezeReason {
				#[codec(index = 14)]
				BlockRewards(runtime_types::pallet_block_rewards::pallet::FreezeReason),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum RuntimeHoldReason {
				#[codec(index = 3)]
				MiningSlot(runtime_types::pallet_mining_slot::pallet::HoldReason),
				#[codec(index = 4)]
				Bond(runtime_types::pallet_bond::pallet::HoldReason),
				#[codec(index = 14)]
				BlockRewards(runtime_types::pallet_block_rewards::pallet::HoldReason),
				#[codec(index = 18)]
				Mint(runtime_types::pallet_mint::pallet::HoldReason),
			}
		}
		pub mod ulx_notary_audit {
			use super::runtime_types;
			pub mod error {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum VerifyError {
					#[codec(index = 0)]
					MissingAccountOrigin {
						account_id: ::subxt::utils::AccountId32,
						account_type: runtime_types::ulx_primitives::account::AccountType,
					},
					#[codec(index = 1)]
					HistoryLookupError {
						source: runtime_types::ulx_notary_audit::AccountHistoryLookupError,
					},
					#[codec(index = 2)]
					InvalidAccountChangelist,
					#[codec(index = 3)]
					InvalidChainTransfersList,
					#[codec(index = 4)]
					InvalidBalanceChangeRoot,
					#[codec(index = 5)]
					InvalidHeaderTaxRecorded,
					#[codec(index = 6)]
					InvalidPreviousNonce,
					#[codec(index = 7)]
					InvalidPreviousBalance,
					#[codec(index = 8)]
					InvalidPreviousAccountOrigin,
					#[codec(index = 9)]
					InvalidPreviousBalanceChangeNotebook,
					#[codec(index = 10)]
					InvalidBalanceChange,
					#[codec(index = 11)]
					InvalidBalanceChangeSignature { change_index: ::core::primitive::u16 },
					#[codec(index = 12)]
					InvalidNoteRecipients,
					#[codec(index = 13)]
					BalanceChangeError {
						change_index: ::core::primitive::u16,
						note_index: ::core::primitive::u16,
						message: ::std::string::String,
					},
					#[codec(index = 14)]
					InvalidNetBalanceChangeset,
					#[codec(index = 15)]
					InsufficientBalance {
						balance: ::core::primitive::u128,
						amount: ::core::primitive::u128,
						note_index: ::core::primitive::u16,
						change_index: ::core::primitive::u16,
					},
					#[codec(index = 16)]
					ExceededMaxBalance {
						balance: ::core::primitive::u128,
						amount: ::core::primitive::u128,
						note_index: ::core::primitive::u16,
						change_index: ::core::primitive::u16,
					},
					#[codec(index = 17)]
					BalanceChangeMismatch {
						change_index: ::core::primitive::u16,
						provided_balance: ::core::primitive::u128,
						calculated_balance: ::core::primitive::i128,
					},
					#[codec(index = 18)]
					BalanceChangeNotNetZero {
						sent: ::core::primitive::u128,
						claimed: ::core::primitive::u128,
					},
					#[codec(index = 19)]
					InvalidDomainLeaseAllocation,
					#[codec(index = 20)]
					TaxBalanceChangeNotNetZero {
						sent: ::core::primitive::u128,
						claimed: ::core::primitive::u128,
					},
					#[codec(index = 21)]
					MissingBalanceProof,
					#[codec(index = 22)]
					InvalidPreviousBalanceProof,
					#[codec(index = 23)]
					InvalidNotebookHash,
					#[codec(index = 24)]
					InvalidNotebookHeaderHash,
					#[codec(index = 25)]
					DuplicateChainTransfer,
					#[codec(index = 26)]
					DuplicatedAccountOriginUid,
					#[codec(index = 27)]
					InvalidNotarySignature,
					#[codec(index = 28)]
					NotebookTooOld,
					#[codec(index = 29)]
					CatchupNotebooksMissing,
					#[codec(index = 30)]
					DecodeError,
					#[codec(index = 31)]
					AccountEscrowHoldDoesntExist,
					#[codec(index = 32)]
					AccountAlreadyHasEscrowHold,
					#[codec(index = 33)]
					EscrowHoldNotReadyForClaim {
						current_tick: ::core::primitive::u32,
						claim_tick: ::core::primitive::u32,
					},
					#[codec(index = 34)]
					AccountLocked,
					#[codec(index = 35)]
					MissingEscrowHoldNote,
					#[codec(index = 36)]
					InvalidEscrowHoldNote,
					#[codec(index = 37)]
					InvalidEscrowClaimers,
					#[codec(index = 38)]
					EscrowNoteBelowMinimum,
					#[codec(index = 39)]
					InvalidTaxNoteAccount,
					#[codec(index = 40)]
					InvalidTaxOperation,
					#[codec(index = 41)]
					InsufficientTaxIncluded {
						tax_sent: ::core::primitive::u128,
						tax_owed: ::core::primitive::u128,
						account_id: ::subxt::utils::AccountId32,
					},
					#[codec(index = 42)]
					InsufficientBlockVoteTax,
					#[codec(index = 43)]
					IneligibleTaxVoter,
					#[codec(index = 44)]
					BlockVoteInvalidSignature,
					#[codec(index = 45)]
					InvalidBlockVoteAllocation,
					#[codec(index = 46)]
					InvalidBlockVoteRoot,
					#[codec(index = 47)]
					InvalidBlockVotesCount,
					#[codec(index = 48)]
					InvalidBlockVotingPower,
					#[codec(index = 49)]
					InvalidBlockVoteList,
					#[codec(index = 50)]
					InvalidComputeProof,
					#[codec(index = 51)]
					InvalidBlockVoteSource,
					#[codec(index = 52)]
					InsufficientBlockVoteMinimum,
					#[codec(index = 53)]
					BlockVoteDataDomainMismatch,
					#[codec(index = 54)]
					BlockVoteEscrowReused,
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum AccountHistoryLookupError {
				#[codec(index = 0)]
				RootNotFound,
				#[codec(index = 1)]
				LastChangeNotFound,
				#[codec(index = 2)]
				InvalidTransferToLocalchain,
				#[codec(index = 3)]
				BlockSpecificationNotFound,
			}
		}
		pub mod ulx_primitives {
			use super::runtime_types;
			pub mod account {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum AccountType {
					#[codec(index = 0)]
					Tax,
					#[codec(index = 1)]
					Deposit,
				}
			}
			pub mod apis {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct NotaryNotebookVotes {
					#[codec(compact)]
					pub notary_id: ::core::primitive::u32,
					#[codec(compact)]
					pub notebook_number: ::core::primitive::u32,
					pub raw_votes: ::std::vec::Vec<(
						::std::vec::Vec<::core::primitive::u8>,
						::core::primitive::u128,
					)>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct NotebookAuditResult {
					#[codec(compact)]
					pub notary_id: ::core::primitive::u32,
					#[codec(compact)]
					pub notebook_number: ::core::primitive::u32,
					#[codec(compact)]
					pub tick: ::core::primitive::u32,
					pub raw_votes: ::std::vec::Vec<(
						::std::vec::Vec<::core::primitive::u8>,
						::core::primitive::u128,
					)>,
					pub changed_accounts_root: ::subxt::utils::H256,
					pub account_changelist: ::std::vec::Vec<
						runtime_types::ulx_primitives::balance_change::AccountOrigin,
					>,
					pub used_transfers_to_localchain: ::std::vec::Vec<::core::primitive::u32>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct NotebookAuditSummary {
					#[codec(compact)]
					pub notary_id: ::core::primitive::u32,
					#[codec(compact)]
					pub notebook_number: ::core::primitive::u32,
					#[codec(compact)]
					pub tick: ::core::primitive::u32,
					pub changed_accounts_root: ::subxt::utils::H256,
					pub account_changelist: ::std::vec::Vec<
						runtime_types::ulx_primitives::balance_change::AccountOrigin,
					>,
					pub used_transfers_to_localchain: ::std::vec::Vec<::core::primitive::u32>,
				}
			}
			pub mod balance_change {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct AccountOrigin {
					#[codec(compact)]
					pub notebook_number: ::core::primitive::u32,
					#[codec(compact)]
					pub account_uid: ::core::primitive::u32,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct MerkleProof {
					pub proof: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::subxt::utils::H256,
					>,
					#[codec(compact)]
					pub number_of_leaves: ::core::primitive::u32,
					#[codec(compact)]
					pub leaf_index: ::core::primitive::u32,
				}
			}
			pub mod block_seal {
				use super::runtime_types;
				pub mod app {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Clone,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct Public(pub [::core::primitive::u8; 32usize]);
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Clone,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct Signature(pub [::core::primitive::u8; 64usize]);
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct MiningAuthority<_0, _1> {
					#[codec(compact)]
					pub authority_index: ::core::primitive::u32,
					pub authority_id: _0,
					pub account_id: _1,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct MiningRegistration<_0, _1, _2> {
					pub account_id: _0,
					pub reward_destination:
						runtime_types::ulx_primitives::block_seal::RewardDestination<_0>,
					pub bond_id: ::core::option::Option<_1>,
					#[codec(compact)]
					pub bond_amount: _2,
					#[codec(compact)]
					pub ownership_tokens: _2,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum RewardDestination<_0> {
					#[codec(index = 0)]
					Owner,
					#[codec(index = 1)]
					Account(_0),
				}
			}
			pub mod block_vote {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct BestBlockVoteSeal<_0, _1> {
					pub seal_strength: runtime_types::primitive_types::U256,
					#[codec(compact)]
					pub notary_id: ::core::primitive::u32,
					pub block_vote_bytes: ::std::vec::Vec<::core::primitive::u8>,
					#[codec(compact)]
					pub source_notebook_number: ::core::primitive::u32,
					pub source_notebook_proof:
						runtime_types::ulx_primitives::balance_change::MerkleProof,
					pub closest_miner: (_0, _1),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct BlockVoteT<_0> {
					pub account_id: ::subxt::utils::AccountId32,
					pub block_hash: _0,
					#[codec(compact)]
					pub index: ::core::primitive::u32,
					#[codec(compact)]
					pub power: ::core::primitive::u128,
					pub data_domain_hash: ::subxt::utils::H256,
					pub data_domain_account: ::subxt::utils::AccountId32,
					pub signature: runtime_types::sp_runtime::MultiSignature,
					pub block_rewards_account_id: ::subxt::utils::AccountId32,
				}
			}
			pub mod bond {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Bond<_0, _1, _2, _3> {
					pub bond_fund_id: ::core::option::Option<_2>,
					pub bonded_account_id: _0,
					#[codec(compact)]
					pub annual_percent_rate: _2,
					#[codec(compact)]
					pub base_fee: _1,
					#[codec(compact)]
					pub fee: _1,
					#[codec(compact)]
					pub amount: _1,
					#[codec(compact)]
					pub start_block: _2,
					#[codec(compact)]
					pub completion_block: _2,
					pub is_locked: ::core::primitive::bool,
					#[codec(skip)]
					pub __ignore: ::core::marker::PhantomData<_3>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct BondFund<_0, _1, _2> {
					#[codec(compact)]
					pub lease_annual_percent_rate: _2,
					#[codec(compact)]
					pub lease_base_fee: _1,
					pub offer_account_id: _0,
					#[codec(compact)]
					pub amount_reserved: _1,
					pub offer_expiration_block: _2,
					#[codec(compact)]
					pub amount_bonded: _1,
					pub is_ended: ::core::primitive::bool,
				}
			}
			pub mod data_domain {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Semver {
					pub major: ::core::primitive::u32,
					pub minor: ::core::primitive::u32,
					pub patch: ::core::primitive::u32,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct VersionHost {
					pub datastore_id: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
					pub host: runtime_types::ulx_primitives::host::Host,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ZoneRecord<_0> {
					pub payment_account: _0,
					pub notary_id: ::core::primitive::u32,
					pub versions: ::subxt::utils::KeyedVec<
						runtime_types::ulx_primitives::data_domain::Semver,
						runtime_types::ulx_primitives::data_domain::VersionHost,
					>,
				}
			}
			pub mod digests {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct BlockVoteDigest {
					#[codec(compact)]
					pub voting_power: ::core::primitive::u128,
					#[codec(compact)]
					pub votes_count: ::core::primitive::u32,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct NotebookDigest<_0> {
					pub notebooks: ::std::vec::Vec<
						runtime_types::ulx_primitives::digests::NotebookDigestRecord<_0>,
					>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct NotebookDigestRecord<_0> {
					#[codec(compact)]
					pub notary_id: ::core::primitive::u32,
					#[codec(compact)]
					pub notebook_number: ::core::primitive::u32,
					#[codec(compact)]
					pub tick: ::core::primitive::u32,
					pub audit_first_failure: ::core::option::Option<_0>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ParentVotingKeyDigest {
					pub parent_voting_key: ::core::option::Option<::subxt::utils::H256>,
				}
			}
			pub mod host {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Host(
					pub  runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
				);
			}
			pub mod inherents {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum BlockSealInherent {
					#[codec(index = 0)]
					Vote {
						seal_strength: runtime_types::primitive_types::U256,
						#[codec(compact)]
						notary_id: ::core::primitive::u32,
						#[codec(compact)]
						source_notebook_number: ::core::primitive::u32,
						source_notebook_proof:
							runtime_types::ulx_primitives::balance_change::MerkleProof,
						block_vote: runtime_types::ulx_primitives::block_vote::BlockVoteT<
							::subxt::utils::H256,
						>,
						miner_signature: runtime_types::ulx_primitives::block_seal::app::Signature,
					},
					#[codec(index = 1)]
					Compute,
				}
			}
			pub mod notary {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct NotaryMeta {
					pub public: [::core::primitive::u8; 32usize],
					pub hosts: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::ulx_primitives::host::Host,
					>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct NotaryNotebookKeyDetails {
					#[codec(compact)]
					pub notebook_number: ::core::primitive::u32,
					#[codec(compact)]
					pub tick: ::core::primitive::u32,
					pub block_votes_root: ::subxt::utils::H256,
					pub secret_hash: ::subxt::utils::H256,
					pub parent_secret: ::core::option::Option<::subxt::utils::H256>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct NotaryNotebookVoteDetails<_0> {
					#[codec(compact)]
					pub notary_id: ::core::primitive::u32,
					#[codec(compact)]
					pub version: ::core::primitive::u32,
					#[codec(compact)]
					pub notebook_number: ::core::primitive::u32,
					#[codec(compact)]
					pub tick: ::core::primitive::u32,
					#[codec(compact)]
					pub finalized_block_number: ::core::primitive::u32,
					pub header_hash: ::subxt::utils::H256,
					#[codec(compact)]
					pub block_votes_count: ::core::primitive::u32,
					#[codec(compact)]
					pub block_voting_power: ::core::primitive::u128,
					pub blocks_with_votes: ::std::vec::Vec<_0>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct NotaryNotebookVoteDigestDetails {
					#[codec(compact)]
					pub notary_id: ::core::primitive::u32,
					#[codec(compact)]
					pub notebook_number: ::core::primitive::u32,
					#[codec(compact)]
					pub tick: ::core::primitive::u32,
					#[codec(compact)]
					pub block_votes_count: ::core::primitive::u32,
					#[codec(compact)]
					pub block_voting_power: ::core::primitive::u128,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct NotaryRecord<_0, _1> {
					#[codec(compact)]
					pub notary_id: _1,
					pub operator_account_id: _0,
					#[codec(compact)]
					pub activated_block: _1,
					#[codec(compact)]
					pub meta_updated_block: _1,
					pub meta: runtime_types::ulx_primitives::notary::NotaryMeta,
				}
			}
			pub mod notebook {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum ChainTransfer {
					#[codec(index = 0)]
					ToMainchain {
						account_id: ::subxt::utils::AccountId32,
						#[codec(compact)]
						amount: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					ToLocalchain {
						#[codec(compact)]
						transfer_id: ::core::primitive::u32,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct NotebookHeader {
					#[codec(compact)]
					pub version: ::core::primitive::u16,
					#[codec(compact)]
					pub notebook_number: ::core::primitive::u32,
					#[codec(compact)]
					pub tick: ::core::primitive::u32,
					#[codec(compact)]
					pub finalized_block_number: ::core::primitive::u32,
					#[codec(compact)]
					pub tax: ::core::primitive::u128,
					#[codec(compact)]
					pub notary_id: ::core::primitive::u32,
					pub chain_transfers:
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							runtime_types::ulx_primitives::notebook::ChainTransfer,
						>,
					pub changed_accounts_root: ::subxt::utils::H256,
					pub changed_account_origins:
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							runtime_types::ulx_primitives::balance_change::AccountOrigin,
						>,
					pub block_votes_root: ::subxt::utils::H256,
					#[codec(compact)]
					pub block_votes_count: ::core::primitive::u32,
					pub blocks_with_votes:
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::subxt::utils::H256,
						>,
					#[codec(compact)]
					pub block_voting_power: ::core::primitive::u128,
					pub secret_hash: ::subxt::utils::H256,
					pub parent_secret: ::core::option::Option<::subxt::utils::H256>,
					pub data_domains: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						(::subxt::utils::H256, ::subxt::utils::AccountId32),
					>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SignedNotebookHeader {
					pub header: runtime_types::ulx_primitives::notebook::NotebookHeader,
					pub signature: [::core::primitive::u8; 64usize],
				}
			}
			pub mod providers {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct BlockSealerInfo<_0> {
					pub miner_rewards_account: _0,
					pub block_vote_rewards_account: _0,
				}
			}
			pub mod tick {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Ticker {
					pub tick_duration_millis: ::core::primitive::u64,
					pub genesis_utc_time: ::core::primitive::u64,
					pub ntp_offset_millis: ::core::primitive::i64,
				}
			}
		}
	}
}
