use argon_notary_audit::VerifyError;
use scale_info::scale;
use sp_core::H256;
use tracing::error;

#[derive(Debug, PartialEq, Clone, thiserror::Error)]
pub enum Error {
	#[error("Notary not found")]
	NotaryNotFound,
	#[error("Empty Notarization Proposed")]
	EmptyNotarizationProposed,
	#[error("An invalid balance change was submitted ({change_index}.{note_index}): {message}")]
	BalanceChangeError { change_index: usize, note_index: usize, message: String },
	#[error(
    	"Account balance tip invalid (change: {change_index}, expected previous: {stored_tip:?} vs provided: {provided_tip:?})",
	)]
	BalanceTipMismatch { change_index: usize, stored_tip: Option<H256>, provided_tip: Option<H256> },
	#[error(
		"Invalid transfer to localchain (expired, already applied, or invalid) for account (change: {change_index}.{note_index})"
	)]
	TransferToLocalchainNotFound { note_index: usize, change_index: usize },
	#[error("Invalid amount claimed for Localchain transfer. (change: {change_index}.{note_index}; expected: {amount}, provided: {provided})" )]
	TransferToLocalchainInvalidAmount {
		note_index: usize,
		change_index: usize,
		provided: u128,
		amount: u128,
	},

	#[error("{0}")]
	Database(String),

	#[error("Internal error: {0}")]
	InternalError(String),

	#[error("Block sync error: {0}")]
	BlockSyncError(String),

	#[error("The requested notebook is not finalized. Please retry this operation later.")]
	NotebookNotFinalized,

	#[error("Invalid balance proof requested")]
	InvalidBalanceProofRequested,

	#[error("Missing account origin")]
	MissingAccountOrigin,

	#[error("Invalid proof of previous balance provided")]
	InvalidBalanceProof,

	#[error("Mainchain API error: {0}")]
	MainchainApiError(String),

	#[error("Error verifying balance change: {0}")]
	BalanceChangeVerifyError(#[from] VerifyError),

	#[error("Error encoding/decoding: {0}")]
	EncodingError(#[from] scale::Error),

	#[error("The current notebook has reached the maximum number of transfers it can include. Please wait for the next notebook.")]
	MaxNotebookChainTransfersReached,

	#[error("Cross-notary proofs are not implemented yet!!!")]
	CrossNotaryProofsNotImplemented,

	#[error("Unsigned notebook header")]
	UnsignedNotebookHeader,
}

impl From<Error> for i32 {
	fn from(e: Error) -> i32 {
		match e {
			Error::NotaryNotFound => 0,
			Error::BalanceChangeError { .. } => 1,
			Error::BalanceTipMismatch { .. } => 2,
			Error::TransferToLocalchainNotFound { .. } => 3,
			Error::TransferToLocalchainInvalidAmount { .. } => 4,
			Error::Database(_) => 5,
			Error::InternalError(_) => 6,
			Error::BlockSyncError(_) => 7,
			Error::NotebookNotFinalized => 8,
			Error::InvalidBalanceProofRequested => 9,
			Error::MissingAccountOrigin => 10,
			Error::InvalidBalanceProof => 11,
			Error::MainchainApiError(_) => 12,
			Error::BalanceChangeVerifyError(_) => 13,
			Error::EncodingError(_) => 14,
			Error::MaxNotebookChainTransfersReached => 15,
			Error::CrossNotaryProofsNotImplemented => 16,
			Error::UnsignedNotebookHeader => 17,
			Error::EmptyNotarizationProposed => 18,
		}
	}
}

impl From<sqlx::Error> for Error {
	fn from(e: sqlx::Error) -> Self {
		Self::Database(e.to_string())
	}
}
impl From<subxt::Error> for Error {
	fn from(e: subxt::Error) -> Self {
		Self::MainchainApiError(e.to_string())
	}
}

impl From<serde_json::Error> for Error {
	fn from(e: serde_json::Error) -> Self {
		Self::InternalError(e.to_string())
	}
}
