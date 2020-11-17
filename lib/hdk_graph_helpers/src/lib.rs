/**
 * Helper methods for managing Holochain DHT links and entries as graphs of information
 */

use std::convert::Infallible;
use thiserror::Error;
pub use paste;
use hdk3::prelude::*;
pub use hdk3::prelude::{EntryHash, hash_entry};

// dependencies

mod internals;

mod entry_helpers;
mod anchor_helpers;
mod key_helpers;
mod local_index_helpers;
mod remote_index_helpers;
mod record_helpers;
mod link_helpers;
mod rpc_helpers;

// API interfaces

pub mod type_wrappers;  // :TODO: make private once unit_requests.rs dependency lifted
pub mod maybe_undefined;
pub use maybe_undefined::MaybeUndefined as MaybeUndefined;
pub mod record_interface;

// helper functions API

pub mod entries { pub use crate::entry_helpers::*; }
pub mod anchors { pub use crate::anchor_helpers::*; }
pub mod links { pub use crate::link_helpers::*; }
pub mod keys { pub use crate::key_helpers::*; }
pub mod local_indexes { pub use crate::local_index_helpers::*; }
pub mod remote_indexes { pub use crate::remote_index_helpers::*; }
pub mod rpc { pub use crate::rpc_helpers::*; }
pub mod records { pub use crate::record_helpers::*; }

#[derive(Error, Debug)]
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
    #[error("Given index does not exist")]
    IndexNotFound,
    #[error("Error in remote call {0}")]
    RemoteRequestError(String),
    #[error("Bad zome RPC response format from {0}")]
    RemoteResponseFormatError(String),
    #[error("Indexing error in remote call {0}")]
    RemoteIndexingError(String),
}

pub type GraphAPIResult<T> = Result<T, DataIntegrityError>;

pub mod identifiers {
    // Holochain DHT storage type IDs
    pub const RECORD_INITIAL_ENTRY_LINK_TAG: &'static [u8] = b"initial_entry";
}
