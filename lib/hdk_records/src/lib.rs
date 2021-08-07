/**
 * Helper methods for managing Holochain DHT links and entries as graphs of information
 */

use std::convert::Infallible;
use thiserror::Error;
use hdk::prelude::*;
use hdk_type_serialization_macros::{RevisionHash, DnaAddressable};

pub use hdk::prelude::{CellId, EntryHash, hash_entry};
pub use holo_hash::{DnaHash};
pub use hdk::info::{agent_info, zome_info};

// re-expose MaybeUndefined module
pub use serde_maybe_undefined as maybe_undefined;
pub use serde_maybe_undefined::MaybeUndefined as MaybeUndefined;

// dependencies

mod internals;

mod entry_helpers;
mod link_helpers;
mod identity_helpers;
mod record_helpers;
mod local_index_helpers;
mod rpc_helpers;
mod remote_index_helpers;
// :TODO: finalise this per https://github.com/holochain/holochain/issues/743
//        and https://github.com/holochain/holochain/issues/563
mod foreign_index_helpers;

// API interfaces

pub mod record_interface;

// helper functions API

pub mod entries { pub use crate::entry_helpers::*; }
pub mod links { pub use crate::link_helpers::*; }
pub mod records { pub use crate::record_helpers::*; }
pub mod local_indexes { pub use crate::local_index_helpers::*; }
pub mod rpc { pub use crate::rpc_helpers::*; }
pub mod remote_indexes { pub use crate::remote_index_helpers::*; }
pub mod foreign_indexes { pub use crate::foreign_index_helpers::*; }

// :TODO: these error types may just be duplicating enums from the HDK,
// revisit this once result handling & serialisation have stabilised.

// error and result type for library operations

#[derive(Error, Debug, Clone)]
pub enum DataIntegrityError {
    #[error(transparent)]
    Serialization(#[from] SerializedBytesError),
    #[error(transparent)]
    Infallible(#[from] Infallible),
    #[error(transparent)]
    EntryError(#[from] EntryError),
    #[error(transparent)]
    Wasm(#[from] WasmError),

    #[error("No entry at this address")]
    EntryNotFound,
    #[error("Could not convert entry to requested type")]
    EntryWrongType,
    #[error("No index found at address {0}")]
    IndexNotFound(EntryHash),
    #[error("No results found")]
    EmptyQuery,
    #[error("Index at address {0} with malformed bytes {1:?}")]
    CorruptIndexError(EntryHash, Option<Vec<u8>>),
    #[error("Error in remote call {0}")]
    RemoteRequestError(String),
    #[error("Bad zome RPC response format from {0}")]
    RemoteResponseFormatError(String),
    #[error("Indexing error in remote call {0}")]
    RemoteIndexingError(String),
}

pub type RecordAPIResult<T> = Result<T, DataIntegrityError>;

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

// convert internal cell errors for passing to remote cell

impl From<DataIntegrityError> for CrossCellError {
    fn from(e: DataIntegrityError) -> CrossCellError {
        match e {
            DataIntegrityError::IndexNotFound(entry) => CrossCellError::IndexNotFound(entry),
            _ => CrossCellError::Internal(e.to_string()),
        }
    }
}

// coerce error types to HDK errors for output

impl From<DataIntegrityError> for WasmError {
    fn from(e: DataIntegrityError) -> WasmError {
        WasmError::Guest(e.to_string())
    }
}

impl From<CrossCellError> for DataIntegrityError {
    fn from(e: CrossCellError) -> DataIntegrityError {
        DataIntegrityError::RemoteRequestError(e.to_string())
    }
}

impl From<CrossCellError> for WasmError {
    fn from(e: CrossCellError) -> WasmError {
        WasmError::CallError(e.to_string())
    }
}

// module constants / internals

pub mod identifiers {
    // Holochain DHT storage type IDs
    pub const RECORD_INITIAL_ENTRY_LINK_TAG: &'static [u8] = b"initial_entry";
}
