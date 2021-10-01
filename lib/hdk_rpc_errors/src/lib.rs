use thiserror::Error;
use hdk::prelude::*;

use holo_hash::{EntryHash, DnaHash};

// serializable error and result type for communicating errors between cells

#[derive(Error, Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub enum CrossCellError {
    #[error(transparent)]
    Serialization(#[from] SerializedBytesError),
    #[error(transparent)]
    Wasm(#[from] WasmError),

    #[error("Entry size of {0} exceeded maximum allowable")]
    EntryTooLarge(usize),
    #[error("No index found at address {0}")]
    IndexNotFound(EntryHash),
    #[error("A remote zome call was made but there was a network error: {0}")]
    NetworkError(String),
    #[error("Zome call unauthorized for {0}.{1}::{2} by agent {3}")]
    Unauthorized(CellId, ZomeName, FunctionName, AgentPubKey),
    #[error("Cross-DNA authentication for remote DNA {0} failed: {1}")]
    CellAuthFailed(DnaHash, String),
    #[error("Internal error in remote zome call: {0}")]
    Internal(String),
    #[error("Local zome call failed: {0} zome is not configured for target {1}")]
    NotConfigured(ZomeName, FunctionName),
}

pub type OtherCellResult<T> = Result<T, CrossCellError>;

impl From<CrossCellError> for WasmError {
    fn from(e: CrossCellError) -> WasmError {
        WasmError::CallError(e.to_string())
    }
}
