/**
 * Helper methods for managing Holochain DHT links and entries as graphs of information
 */
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate hdk;

mod maybe_undefined;
pub mod record_helpers;
pub mod record_interface;
pub mod link_helpers;

// submodule exports

pub use maybe_undefined::MaybeUndefined as MaybeUndefined;
pub use record_helpers as records;
pub use link_helpers as links;

pub mod identifiers {
    pub const RECORD_INITIAL_ENTRY_LINK_TAG: &str = "initial_entry";
}
