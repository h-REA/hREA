/**
 * Helper methods for managing Holochain DHT links and entries as graphs of information
 */
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate hdk;

pub mod type_wrappers;
pub mod maybe_undefined;
pub mod record_interface;

pub mod entry_helpers;
pub mod key_helpers;
pub mod local_index_helpers;
pub mod remote_index_helpers;
pub mod record_helpers;
pub mod link_helpers;
pub mod rpc_helpers;

// submodule exports

pub use maybe_undefined::MaybeUndefined as MaybeUndefined;

use entry_helpers as entries;
pub use key_helpers as keys;
pub use local_index_helpers as local_indexes;
pub use remote_index_helpers as remote_indexes;
pub use record_helpers as records;
pub use link_helpers as links;
pub use rpc_helpers as rpc;

pub mod identifiers {
    pub const RECORD_INITIAL_ENTRY_LINK_TAG: &str = "initial_entry";
}
