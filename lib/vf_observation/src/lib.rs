/**
 * Observation module datatypes & behaviours
 */

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate vf_core;

pub use vf_core::{ type_aliases, measurement };

pub mod economic_event;

pub mod identifiers {
    // :TODO: how to read this from conductor, and determine correct DHT to link to?
    pub const BRIDGED_PLANNING_DHT: &str = "vf_planning";

    pub const EVENT_BASE_ENTRY_TYPE: &str = "vf_economic_event_baseurl";
    pub const EVENT_ENTRY_TYPE: &str = "vf_economic_event";

    pub const EVENT_FULFILLS_LINK_TYPE: &str = "vf_economic_event_fulfills";
    pub const EVENT_FULFILLS_LINK_TAG: &str = "fulfills";
}
