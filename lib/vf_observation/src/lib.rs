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
pub mod economic_resource;
pub mod process;

pub mod identifiers {
    // :TODO: how to read this from conductor, and determine correct DHT to link to?
    pub const BRIDGED_PLANNING_DHT: &str = "vf_planning";

    pub const EVENT_BASE_ENTRY_TYPE: &str = "vf_economic_event_baseurl";
    pub const EVENT_INITIAL_ENTRY_LINK_TYPE: &str = "vf_economic_event_entry";
    pub const EVENT_ENTRY_TYPE: &str = "vf_economic_event";
    pub const EVENT_FULFILLS_LINK_TYPE: &str = "vf_economic_event_fulfills";
    pub const EVENT_FULFILLS_LINK_TAG: &str = "fulfills";
    pub const EVENT_SATISFIES_LINK_TYPE: &str = "vf_economic_event_satisfies";
    pub const EVENT_SATISFIES_LINK_TAG: &str = "fulfills";
    pub const EVENT_INPUT_OF_LINK_TYPE: &str = "vf_economic_event_input_of";
    pub const EVENT_INPUT_OF_LINK_TAG: &str = "input_of";
    pub const EVENT_OUTPUT_OF_LINK_TYPE: &str = "vf_economic_event_output_of";
    pub const EVENT_OUTPUT_OF_LINK_TAG: &str = "output_of";

    pub const RESOURCE_BASE_ENTRY_TYPE: &str = "vf_economic_resource_baseurl";
    pub const RESOURCE_INITIAL_ENTRY_LINK_TYPE: &str = "vf_economic_resource_entry";
    pub const RESOURCE_ENTRY_TYPE: &str = "vf_economic_resource";
    pub const RESOURCE_CONTAINS_LINK_TYPE: &str = "vf_resource_contains";
    pub const RESOURCE_CONTAINS_LINK_TAG: &str = "contains";
    pub const RESOURCE_CONTAINED_IN_LINK_TYPE: &str = "vf_resource_contained_in";
    pub const RESOURCE_CONTAINED_IN_LINK_TAG: &str = "contained_in";

    pub const PROCESS_BASE_ENTRY_TYPE: &str = "vf_process_baseurl";
    pub const PROCESS_INITIAL_ENTRY_LINK_TYPE: &str = "vf_process_entry";
    pub const PROCESS_ENTRY_TYPE: &str = "vf_process";
    pub const PROCESS_EVENT_INPUTS_LINK_TYPE: &str = "vf_process_inputs";
    pub const PROCESS_EVENT_INPUTS_LINK_TAG: &str = "inputs";
    pub const PROCESS_EVENT_OUTPUTS_LINK_TYPE: &str = "vf_process_outputs";
    pub const PROCESS_EVENT_OUTPUTS_LINK_TAG: &str = "outputs";
    pub const PROCESS_COMMITMENT_INPUTS_LINK_TYPE: &str = "vf_process_committed_inputs";
    pub const PROCESS_COMMITMENT_INPUTS_LINK_TAG: &str = "committed_inputs";
    pub const PROCESS_COMMITMENT_OUTPUTS_LINK_TYPE: &str = "vf_process_committed_outputs";
    pub const PROCESS_COMMITMENT_OUTPUTS_LINK_TAG: &str = "committed_outputs";
    pub const PROCESS_INTENT_INPUTS_LINK_TYPE: &str = "vf_process_intended_inputs";
    pub const PROCESS_INTENT_INPUTS_LINK_TAG: &str = "intended_inputs";
    pub const PROCESS_INTENT_OUTPUTS_LINK_TYPE: &str = "vf_process_intended_outputs";
    pub const PROCESS_INTENT_OUTPUTS_LINK_TAG: &str = "intended_outputs";
}
