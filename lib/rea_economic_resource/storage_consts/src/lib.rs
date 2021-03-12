/**
 * Storage constants for zome entry & link type identifiers
 *
 * Used by modules interfacing with the underlying Holochain storage system directly.
 *
 * @package Holo-REA
 */
pub const RESOURCE_ENTRY_TYPE: &str = "vf_economic_resource";

pub const RESOURCE_CONTAINS_LINK_TYPE: &str = "contains";
pub const RESOURCE_CONTAINED_IN_LINK_TYPE: &str = "contained_in";
pub const RESOURCE_AFFECTED_BY_EVENT_LINK_TYPE: &str = "affected_by";
pub const RESOURCE_CONFORMS_TO_LINK_TYPE: &str = "conforms_to";

// :TODO: replace with cell-to-cell comms determined by record IDs
pub const BRIDGED_SPECIFICATION_DHT: &str = "vf_specification";
