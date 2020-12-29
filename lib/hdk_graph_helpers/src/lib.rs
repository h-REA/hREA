/**
 * Helper methods for managing Holochain DHT links and entries as graphs of information
 */

use std::convert::Infallible;
use thiserror::Error;
pub use paste;
use hdk3::prelude::*;
pub use hdk3::prelude::{EntryHash, hash_entry, hdk_entry};

// dependencies

mod internals;

mod entry_helpers;
mod link_helpers;
mod identity_helpers;
mod record_helpers;
mod local_index_helpers;
mod remote_index_helpers;

// API interfaces

pub mod type_wrappers;  // :TODO: make private once unit_requests.rs dependency lifted
pub mod maybe_undefined;
pub use maybe_undefined::MaybeUndefined as MaybeUndefined;
pub mod record_interface;

// helper functions API

pub mod entries { pub use crate::entry_helpers::*; }
pub mod links { pub use crate::link_helpers::*; }
pub mod records { pub use crate::record_helpers::*; }
pub mod local_indexes { pub use crate::local_index_helpers::*; }
pub mod remote_indexes { pub use crate::remote_index_helpers::*; }

// error and result type for library operations; seamlessly coerced from HDK method results

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
    #[error(transparent)]
    HdkError(#[from] HdkError),

    #[error("No entry at this address")]
    EntryNotFound,
    #[error("Could not convert entry to requested type")]
    EntryWrongType,
    #[error("No index found at address {0}")]
    IndexNotFound(EntryHash),
    #[error("Error in remote call {0}")]
    RemoteRequestError(String),
    #[error("Bad zome RPC response format from {0}")]
    RemoteResponseFormatError(String),
    #[error("Indexing error in remote call {0}")]
    RemoteIndexingError(String),
}

pub type GraphAPIResult<T> = Result<T, DataIntegrityError>;

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
    #[error("Zome call was made which the caller was unauthorized to make")]
    Unauthorized,
    #[error("Internal error in remote zome call: {0}")]
    Internal(String),
}

impl From<DataIntegrityError> for CrossCellError {
    fn from(e: DataIntegrityError) -> CrossCellError {
        match e {
            DataIntegrityError::IndexNotFound(entry) => CrossCellError::IndexNotFound(entry),
            _ => CrossCellError::Internal(e.to_string()),
        }
    }
}

impl From<HdkError> for CrossCellError {
    fn from(e: HdkError) -> CrossCellError {
        match e {
            HdkError::EntryError(e) => match e {
                EntryError::EntryTooLarge(size) => CrossCellError::EntryTooLarge(size),
                EntryError::SerializedBytes(e) => CrossCellError::Serialization(e),
            },
            HdkError::SerializedBytes(e) => CrossCellError::Serialization(e),
            HdkError::UnauthorizedZomeCall(_,_,_,_) => CrossCellError::Unauthorized,
            HdkError::ZomeCallNetworkError(msg) => CrossCellError::NetworkError(msg),
            HdkError::Wasm(e) => CrossCellError::Internal(e.to_string()),
        }
    }
}

pub type OtherCellResult<T> = Result<T, CrossCellError>;

// module constants / internals

pub mod identifiers {
    // Holochain DHT storage type IDs
    pub const RECORD_INITIAL_ENTRY_LINK_TAG: &'static [u8] = b"initial_entry";
}
