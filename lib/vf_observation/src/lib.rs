/**
 * Observation module datatypes & behaviours
 */
extern crate vf_core;

pub use vf_core::{ type_aliases, measurement };

pub mod identifiers {
    // :TODO: how to read this from conductor, and determine correct DHT to link to?
    pub const BRIDGED_PLANNING_DHT: &str = "vf_planning";
}
