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

// Holochain DHT record & link type names

pub const LINK_TYPE_INITIAL_ENTRY: &str = "record_initial_entry";
pub const LINK_TAG_INITIAL_ENTRY: &str = LINK_TYPE_INITIAL_ENTRY;
