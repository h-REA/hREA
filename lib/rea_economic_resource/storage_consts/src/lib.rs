/**
 * Storage constants for zome entry & link type identifiers
 *
 * Used by modules interfacing with the underlying Holochain storage system directly.
 *
 * @package Holo-REA
 */
pub const RESOURCE_BASE_ENTRY_TYPE: &str = "vf_economic_resource_baseurl";
pub const RESOURCE_INITIAL_ENTRY_LINK_TYPE: &str = "vf_economic_resource_entry";
pub const RESOURCE_ENTRY_TYPE: &str = "vf_economic_resource";
pub const RESOURCE_CONTAINS_LINK_TYPE: &str = "vf_resource_contains";
pub const RESOURCE_CONTAINS_LINK_TAG: &str = "contains";
pub const RESOURCE_CONTAINED_IN_LINK_TYPE: &str = "vf_resource_contained_in";
pub const RESOURCE_CONTAINED_IN_LINK_TAG: &str = "contained_in";
pub const RESOURCE_AFFECTED_BY_EVENT_LINK_TYPE: &str = "vf_economic_resource_affected_by";
pub const RESOURCE_AFFECTED_BY_EVENT_LINK_TAG: &str = "affected_by";
pub const RESOURCE_CONFORMS_TO_LINK_TYPE: &str = "vf_economic_resource_conforms_to";
pub const RESOURCE_CONFORMS_TO_LINK_TAG: &str = "conforms_to";

// :TODO: replace with a DAG
pub const RESOURCE_INDEX_ROOT_ENTRY_TYPE: &str = "vf_economic_resources_root";
pub const RESOURCE_INDEX_ROOT_ENTRY_ID: &str = "all_vf_economic_resources";
pub const RESOURCE_INDEX_ENTRY_LINK_TYPE: &str = "vf_economic_resource_root_index";

pub const BRIDGED_SPECIFICATION_DHT: &str = "vf_specification";
