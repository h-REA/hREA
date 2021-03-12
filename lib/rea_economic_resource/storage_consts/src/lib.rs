/**
 * Storage constants for zome entry & link type identifiers
 *
 * Used by modules interfacing with the underlying Holochain storage system directly.
 *
 * @package Holo-REA
 */
pub const RESOURCE_ENTRY_TYPE: &str = "vf_economic_resource";

pub const RESOURCE_CONTAINS_LINK_TAG: &str = "contains";
pub const RESOURCE_CONTAINED_IN_LINK_TAG: &str = "contained_in";
pub const RESOURCE_AFFECTED_BY_EVENT_LINK_TAG: &str = "affected_by";
pub const RESOURCE_CONFORMS_TO_LINK_TAG: &str = "conforms_to";
