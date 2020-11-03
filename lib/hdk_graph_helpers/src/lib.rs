/**
 * Helper methods for managing Holochain DHT links and entries as graphs of information
 */
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate hdk;

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

pub mod identifiers {
    // Holochain DHT storage type IDs
    pub const RECORD_INITIAL_ENTRY_LINK_TAG: &str = "initial_entry";
    pub const ANCHOR_POINTER_LINK_TAG: &str = "referenced_entry";

    // Error message strings
    pub const ERR_MSG_ENTRY_NOT_FOUND: &str = "No entry at this address";
    pub const ERR_MSG_ENTRY_WRONG_TYPE: &str = "Could not convert entry to requested type";
    pub const ERR_MSG_REMOTE_INDEXING_ERR: &str = "Indexing error in remote DNA call ";
    pub const ERR_MSG_REMOTE_REQUEST_ERR: &str = "Error in zome RPC call ";
    pub const ERR_MSG_REMOTE_RESPONSE_FORMAT_ERR: &str = "Bad zome RPC response format from ";
    pub const ERR_MSG_INDEX_NOT_FOUND: &str = "Given index does not exist";
}

pub fn error(reason: &str) -> HdkError {
    HdkError::Wasm(WasmError::Zome(String::from(reason)))
}
