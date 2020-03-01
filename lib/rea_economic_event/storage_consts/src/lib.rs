/**
 * Storage constants for zome entry & link type identifiers
 *
 * Used by modules interfacing with the underlying Holochain storage system directly.
 *
 * @package Holo-REA
 */
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

// :TODO: replace with a DAG
pub const EVENT_INDEX_ROOT_ENTRY_TYPE: &str = "vf_economic_events_root";
pub const EVENT_INDEX_ROOT_ENTRY_ID: &str = "all_vf_economic_events";
pub const EVENT_INDEX_ENTRY_LINK_TYPE: &str = "vf_economic_event_root_index";
