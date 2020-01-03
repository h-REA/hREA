/**
 * Observation module datatypes & behaviours
 */

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate vf_core;

pub use vf_core::{ type_aliases, measurement };

pub mod process_specification;
pub mod resource_specification;
pub mod unit;

pub mod identifiers {
    pub const ECONOMIC_RESOURCE_SPECIFICATION_ENTRY_TYPE: &str = "vf_resource_specification";
    pub const ECONOMIC_RESOURCE_SPECIFICATION_BASE_ENTRY_TYPE: &str = "vf_resource_specification_baseurl";
    pub const ECONOMIC_RESOURCE_SPECIFICATION_INITIAL_ENTRY_LINK_TYPE: &str = "vf_resource_specification_entry";

    pub const ECONOMIC_RESOURCE_SPECIFICATION_CONFORMING: &str = "ECONOMIC_RESOURCE_SPECIFICATION_CONFORMING";
    pub const ECONOMIC_RESOURCE_SPECIFICATION_CONFORMING_TAG: &str = "ECONOMIC_RESOURCE_SPECIFICATION_CONFORMING_TAG";

    pub const PROCESS_SPECIFICATION_ENTRY_TYPE: &str = "vf_process_specification";
    pub const PROCESS_SPECIFICATION_BASE_ENTRY_TYPE: &str = "vf_process_specification_baseurl";
    pub const PROCESS_SPECIFICATION_INITIAL_ENTRY_LINK_TYPE: &str = "vf_process_specification_entry";

    pub const UNIT_ENTRY_TYPE: &str = "vf_unit";
    pub const UNIT_ID_ENTRY_TYPE: &str = "vf_unit_id";
    pub const UNIT_INITIAL_ENTRY_LINK_TYPE: &str = "vf_unit_entry";
}
