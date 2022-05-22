/**
 * Helper methods for managing Holochain DHT links and entries as graphs of information
 */
use thiserror::Error;
use std::convert::Infallible;
use std::string::FromUtf8Error;
use hdk::prelude::*;
pub use hdk_uuid_types::DnaAddressable;

pub use hdk::prelude::{CellId, EntryHash, hash_entry};
pub use holo_hash::{DnaHash};
pub use hdk::info::{agent_info, dna_info};

// re-expose MaybeUndefined module
pub use serde_maybe_undefined as maybe_undefined;
pub use serde_maybe_undefined::MaybeUndefined as MaybeUndefined;
pub use hdk_rpc_errors::{ OtherCellResult, CrossCellError };

// re-export auth resolver entry def IDs; zomes declaring full `entry_defs()` extern
// will have to redeclare these manually since they override any others declared with macros
pub use hc_zome_dna_auth_resolver_lib::CAP_STORAGE_ENTRY_DEF_ID;

mod entry_helpers;
mod link_helpers;
mod identity_helpers;
mod record_helpers;
mod anchored_record_helpers;
mod rpc_helpers;

// API interfaces

pub mod record_interface;

// helper functions API

pub mod identities { pub use crate::identity_helpers::*; }
pub mod entries { pub use crate::entry_helpers::*; }
pub mod links { pub use crate::link_helpers::*; }
pub mod records { pub use crate::record_helpers::*; }
pub mod records_anchored { pub use crate::anchored_record_helpers::*; }
pub mod rpc { pub use crate::rpc_helpers::*; }

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
    #[error("String index with malformed bytes {0:?}")]
    BadStringIndexError(Vec<u8>),
    #[error("Error in remote call {0}")]
    RemoteRequestError(String),
    #[error("Bad zome RPC response format from {0}")]
    RemoteResponseFormatError(String),
    #[error("Indexing error in remote call {0}")]
    RemoteIndexingError(String),
}

pub type RecordAPIResult<T> = Result<T, DataIntegrityError>;

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

impl From<FromUtf8Error> for DataIntegrityError {
    fn from(e: FromUtf8Error) -> DataIntegrityError {
        DataIntegrityError::BadStringIndexError(e.into_bytes())
    }
}

// module constants / internals

pub mod identifiers {
    // Holochain DHT storage type IDs
    pub const RECORD_INITIAL_ENTRY_LINK_TAG: &'static [u8] = b"initial_entry";
    pub const RECORD_IDENTITY_ANCHOR_LINK_TAG: &'static [u8] = b"id|";  // :WARNING: byte length is important here. @see anchored_record_helpers::read_entry_anchor_id
}
