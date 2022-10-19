/**
 * Helper methods for managing Holochain DHT links and entries as graphs of information
 */
use thiserror::Error;
use std::convert::Infallible;
use hdk::prelude::*;

pub use hdk::prelude::{CellId, EntryHash, hash_entry};
pub use holo_hash::{DnaHash};
pub use hdk::{
    info::{agent_info, dna_info},
    link::get_links,
    prelude::{WasmError, SignedActionHashed},
};
pub use hdk_semantic_indexes_error::*;
pub use hdk_uuid_types::DnaAddressable;

// re-expose MaybeUndefined module
pub use serde_maybe_undefined as maybe_undefined;
pub use serde_maybe_undefined::MaybeUndefined as MaybeUndefined;
pub use hdk_rpc_errors::{ OtherCellResult, CrossCellError };

mod entry_helpers;
mod identity_helpers;
mod record_helpers;
mod anchored_record_helpers;
mod rpc_helpers;
mod metadata_helpers;

// API interfaces

pub mod record_interface;

// helper functions API

pub mod identities { pub use crate::identity_helpers::*; }
pub mod entries { pub use crate::entry_helpers::*; }
pub mod records { pub use crate::record_helpers::*; }
pub mod records_anchored { pub use crate::anchored_record_helpers::*; }
pub mod rpc { pub use crate::rpc_helpers::*; }
pub mod metadata { pub use crate::metadata_helpers::*; }

// externally-facing structs

pub use metadata_helpers::{ RevisionMeta, RecordMeta };

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
    #[error(transparent)]
    SemanticIndexingError(#[from] SemanticIndexError),

    #[error("An Agent is already associated with the currently authenticated user")]
    AgentAlreadyLinked,
    #[error("No Agent data is associated with the currently authenticated user")]
    AgentNotLinked,
    #[error("No entry at this address")]
    EntryNotFound,
    #[error("Could not convert entry to requested type")]
    EntryWrongType,
    #[error("Conflicting revisions found: {0:?}")]
    UpdateConflict(Vec<ActionHash>),

    #[error("Error in remote call {0}")]
    RemoteRequestError(String),
    #[error("Bad zome RPC response format from {0}")]
    RemoteResponseFormatError(String),
    #[error("Indexing error in remote call {0}")]
    RemoteIndexingError(String),
    #[error("No index found at address {0}")]
    IndexNotFound(EntryHash),
    #[error("DNA misconfiguration detected- local index zome request error for '{0}': {1}")]
    LocalIndexNotConfigured(String, String),
    #[error("Mismatching units in arithmetic operation. Attempting to add or subtract {0:?} with {1:?}")]
    MismatchingUnits(Option<String>, Option<String>),
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
        wasm_error!(WasmErrorInner::Guest(e.to_string()))
    }
}

impl From<CrossCellError> for DataIntegrityError {
    fn from(e: CrossCellError) -> DataIntegrityError {
        DataIntegrityError::RemoteRequestError(e.to_string())
    }
}

// module constants / internals

pub mod identifiers {
    // Holochain DHT storage type IDs
    pub const RECORD_INITIAL_ENTRY_LINK_TAG: &'static [u8] = b"initial_entry";
    pub const RECORD_IDENTITY_ANCHOR_LINK_TAG: &'static [u8] = b"id|";  // :WARNING: byte length is important here. @see anchored_record_helpers::read_entry_anchor_id
}
