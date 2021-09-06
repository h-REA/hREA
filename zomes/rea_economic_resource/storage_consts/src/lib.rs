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

pub const RESOURCE_CONTAINS_INDEXING_API_METHOD: &str = "_internal_reindex_contained_resources";
pub const RESOURCE_CONTAINS_READ_API_METHOD: &str = "_internal_read_contained_resources";
pub const RESOURCE_CONTAINEDIN_INDEXING_API_METHOD: &str = "_internal_reindex_container_resources";
pub const RESOURCE_CONTAINEDIN_READ_API_METHOD: &str = "_internal_read_container_resource";
pub const RESOURCE_AFFECTED_BY_READ_API_METHOD: &str = "_internal_read_affecting_events";
pub const RESOURCE_SPECIFICATION_RESOURCES_INDEXING_API_METHOD: &str = "index_resource_specification_resources";
