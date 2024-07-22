use alloc::vec::Vec;
use codec::{Decode, Encode, MaxEncodedLen};
use core::convert::{TryFrom, TryInto};
use frame_support::pallet_prelude::TypeInfo;
use sp_core::{ConstU32, H256};
use sp_debug_derive::RuntimeDebug;
use sp_runtime::BoundedVec;

#[derive(Encode, Decode, Clone, PartialEq, Eq, TypeInfo, RuntimeDebug)]
pub struct BitcoinSyncStatus {
	pub confirmed_block: BitcoinBlock,
	pub synched_block: Option<BitcoinBlock>,
	pub oldest_allowed_block_height: BitcoinHeight,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, TypeInfo, RuntimeDebug)]
pub struct BitcoinBlock {
	#[codec(compact)]
	pub block_height: BitcoinHeight,
	pub block_hash: BitcoinBlockHash,
}

impl BitcoinBlock {
	pub fn new(block_height: BitcoinHeight, block_hash: BitcoinBlockHash) -> Self {
		Self { block_height, block_hash }
	}
}

#[derive(RuntimeDebug, Encode, Decode, Clone, PartialEq, Eq, TypeInfo)]
pub enum BitcoinError {
	InvalidLockTime,
	InvalidByteLength,
}

#[derive(
	Clone,
	Copy,
	PartialEq,
	Eq,
	Ord,
	PartialOrd,
	Encode,
	Decode,
	TypeInfo,
	RuntimeDebug,
	MaxEncodedLen,
)]
#[repr(transparent)]
pub struct BitcoinPubkeyHash(pub [u8; 20]);

#[derive(Clone, Copy, PartialEq, Eq, Encode, Decode, TypeInfo, RuntimeDebug, MaxEncodedLen)]
#[repr(transparent)]
pub struct CompressedBitcoinPubkey(pub [u8; 33]);
impl From<[u8; 33]> for CompressedBitcoinPubkey {
	fn from(value: [u8; 33]) -> Self {
		Self(value)
	}
}

#[derive(Clone, PartialEq, Eq, Encode, Decode, TypeInfo, RuntimeDebug, MaxEncodedLen)]
#[repr(transparent)]
pub struct BitcoinSignature(pub BoundedVec<u8, ConstU32<73>>);

impl TryFrom<Vec<u8>> for BitcoinSignature {
	type Error = Vec<u8>;
	fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
		Ok(Self(value.try_into()?))
	}
}

/// A Script Pubkey for a Bitcoin UTXO. Supported types are:
/// - P2WSH (Pay to Witness Script Hash)
#[derive(Clone, PartialEq, Eq, Encode, Decode, TypeInfo, RuntimeDebug, Copy)]
#[repr(transparent)]
pub enum BitcoinCosignScriptPubkey {
	/// Pay to Witness Script Hash
	P2WSH { wscript_hash: H256 },
}

#[derive(Clone, PartialEq, Eq, Encode, Decode, TypeInfo, RuntimeDebug)]
#[repr(transparent)]
pub struct BitcoinScriptPubkey(pub BoundedVec<u8, ConstU32<34>>); // allow p2wsh, p2tr max
impl TryFrom<Vec<u8>> for BitcoinScriptPubkey {
	type Error = Vec<u8>;
	fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
		Ok(Self(value.try_into()?))
	}
}

/// A Bitcoin sighash. This is a 32-byte hash that is used to sign a Bitcoin transaction.
#[derive(Clone, PartialEq, Eq, Encode, Decode, TypeInfo, RuntimeDebug)]
#[repr(transparent)]
pub struct BitcoinSighash(pub [u8; 32]);

#[derive(Encode, Decode, Clone, PartialEq, Eq, TypeInfo, RuntimeDebug, Ord, PartialOrd)]
pub struct UtxoRef {
	pub txid: H256Le,
	#[codec(compact)]
	pub output_index: u32,
}

pub type UtxoId = u64;
#[derive(Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct UtxoValue {
	pub utxo_id: UtxoId,
	pub script_pubkey: BitcoinCosignScriptPubkey,
	#[codec(compact)]
	pub satoshis: Satoshis,
	#[codec(compact)]
	pub submitted_at_height: BitcoinHeight,
	#[codec(compact)]
	pub watch_for_spent_until_height: BitcoinHeight,
}

#[derive(Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub enum BitcoinRejectedReason {
	/// The UTXO has a different number of satoshis than what is in the blockchain
	SatoshisMismatch,
	/// The UTXO has been spent
	Spent,
	/// We failed to confirm the utxo before the expiration lookup time
	LookupExpired,
	/// This UTXO is already tracked
	DuplicateUtxo,
}

pub type BitcoinBlockHash = H256Le;
#[cfg(feature = "bitcoin")]
mod bitcoin_compat {
	use alloc::vec::Vec;
	use bitcoin::hashes::{FromSliceError, Hash};
	use sp_core::H256;
	use sp_runtime::BoundedVec;

	use crate::bitcoin::{
		BitcoinCosignScriptPubkey, BitcoinPubkeyHash, BitcoinScriptPubkey, BitcoinSignature,
		CompressedBitcoinPubkey, H256Le, UtxoRef,
	};

	impl From<bitcoin::BlockHash> for H256Le {
		fn from(h: bitcoin::BlockHash) -> Self {
			let mut inner = [0u8; 32];
			inner.copy_from_slice(&h[..]);
			H256Le(inner)
		}
	}

	impl TryInto<bitcoin::BlockHash> for H256Le {
		type Error = FromSliceError;
		fn try_into(self) -> Result<bitcoin::BlockHash, Self::Error> {
			bitcoin::BlockHash::from_slice(&self.0)
		}
	}

	impl From<&bitcoin::BlockHash> for H256Le {
		fn from(h: &bitcoin::BlockHash) -> Self {
			let mut inner = [0u8; 32];
			inner.copy_from_slice(&h[..]);
			H256Le(inner)
		}
	}

	impl From<bitcoin::Txid> for H256Le {
		fn from(h: bitcoin::Txid) -> Self {
			let mut inner = [0u8; 32];
			inner.copy_from_slice(&h[..]);
			H256Le(inner)
		}
	}

	impl From<H256Le> for bitcoin::Txid {
		fn from(h: H256Le) -> Self {
			let hash = bitcoin::hashes::sha256d::Hash::from_bytes_ref(&h.0);
			bitcoin::Txid::from_raw_hash(*hash)
		}
	}

	impl From<bitcoin::FilterHeader> for H256Le {
		fn from(h: bitcoin::FilterHeader) -> Self {
			let mut inner = [0u8; 32];
			inner.copy_from_slice(&h[..]);
			H256Le(inner)
		}
	}

	impl From<bitcoin::OutPoint> for UtxoRef {
		fn from(outpoint: bitcoin::OutPoint) -> Self {
			Self { txid: outpoint.txid.into(), output_index: outpoint.vout }
		}
	}

	impl From<bitcoin::PubkeyHash> for BitcoinPubkeyHash {
		fn from(h: bitcoin::PubkeyHash) -> Self {
			let mut inner = [0u8; 20];
			inner.copy_from_slice(&h.to_raw_hash()[..]);
			BitcoinPubkeyHash(inner)
		}
	}

	impl From<BitcoinPubkeyHash> for bitcoin::PubkeyHash {
		fn from(h: BitcoinPubkeyHash) -> Self {
			let hash = bitcoin::hashes::hash160::Hash::from_bytes_ref(&h.0);
			bitcoin::PubkeyHash::from_raw_hash(*hash)
		}
	}

	impl TryInto<bitcoin::ecdsa::Signature> for BitcoinSignature {
		type Error = bitcoin::ecdsa::Error;
		fn try_into(self) -> Result<bitcoin::ecdsa::Signature, Self::Error> {
			bitcoin::ecdsa::Signature::from_slice(self.0.as_slice())
		}
	}

	impl TryFrom<bitcoin::ecdsa::Signature> for BitcoinSignature {
		type Error = Vec<u8>;
		fn try_from(sig: bitcoin::ecdsa::Signature) -> Result<Self, Self::Error> {
			Ok(Self(sig.serialize().to_vec().try_into()?))
		}
	}

	impl TryInto<bitcoin::CompressedPublicKey> for CompressedBitcoinPubkey {
		type Error = bitcoin::secp256k1::Error;
		fn try_into(self) -> Result<bitcoin::CompressedPublicKey, Self::Error> {
			bitcoin::CompressedPublicKey::from_slice(&self.0)
		}
	}

	impl From<bitcoin::CompressedPublicKey> for CompressedBitcoinPubkey {
		fn from(pubkey: bitcoin::CompressedPublicKey) -> Self {
			Self(pubkey.to_bytes())
		}
	}

	impl From<bitcoin::PublicKey> for CompressedBitcoinPubkey {
		fn from(pubkey: bitcoin::PublicKey) -> Self {
			pubkey.inner.serialize().into()
		}
	}
	impl TryInto<bitcoin::PublicKey> for CompressedBitcoinPubkey {
		type Error = bitcoin::key::FromSliceError;
		fn try_into(self) -> Result<bitcoin::PublicKey, Self::Error> {
			bitcoin::PublicKey::from_slice(&self.0)
		}
	}

	impl BitcoinCosignScriptPubkey {
		pub fn to_script_bytes(&self) -> Vec<u8> {
			let script_buf: bitcoin::ScriptBuf = (*self).into();
			script_buf.to_bytes()
		}
	}

	#[derive(Debug)]
	pub enum BitcoinScriptPubkeyError {
		UnsupportedScript,
	}
	impl TryFrom<bitcoin::ScriptBuf> for BitcoinCosignScriptPubkey {
		type Error = BitcoinScriptPubkeyError;
		fn try_from(script: bitcoin::ScriptBuf) -> Result<Self, Self::Error> {
			if script.is_p2wsh() {
				let mut inner = [0u8; 32];
				inner.copy_from_slice(&script.as_bytes()[2..]);
				return Ok(BitcoinCosignScriptPubkey::P2WSH { wscript_hash: H256(inner) });
			}
			Err(BitcoinScriptPubkeyError::UnsupportedScript)
		}
	}

	impl TryInto<BitcoinScriptPubkey> for bitcoin::ScriptBuf {
		type Error = Vec<u8>;
		fn try_into(self) -> Result<BitcoinScriptPubkey, Self::Error> {
			Ok(BitcoinScriptPubkey(BoundedVec::try_from(self.to_bytes())?))
		}
	}

	impl From<BitcoinScriptPubkey> for bitcoin::ScriptBuf {
		fn from(val: BitcoinScriptPubkey) -> Self {
			bitcoin::ScriptBuf::from_bytes(val.0.into_inner())
		}
	}

	impl TryInto<BitcoinCosignScriptPubkey> for bitcoin::Address {
		type Error = BitcoinScriptPubkeyError;
		fn try_into(self) -> Result<BitcoinCosignScriptPubkey, Self::Error> {
			self.script_pubkey().try_into()
		}
	}

	impl From<BitcoinCosignScriptPubkey> for bitcoin::ScriptBuf {
		fn from(val: BitcoinCosignScriptPubkey) -> Self {
			match val {
				BitcoinCosignScriptPubkey::P2WSH { wscript_hash } => {
					let bytes = wscript_hash.to_fixed_bytes();
					let raw_hash = bitcoin::hashes::sha256::Hash::from_bytes_ref(&bytes);
					let script_hash = bitcoin::WScriptHash::from_raw_hash(*raw_hash);
					bitcoin::ScriptBuf::new_p2wsh(&script_hash)
				},
			}
		}
	}
}
pub type BitcoinHeight = u64;
pub type Satoshis = u64;

pub const SATOSHIS_PER_BITCOIN: u64 = 100_000_000;

/// Represents a bitcoin 32 bytes hash digest encoded in little-endian

#[derive(Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo, Ord, PartialOrd)]
#[repr(transparent)]
pub struct H256Le(pub [u8; 32]);
