/**
 * Storage constants for zome entry & link type identifiers
 *
 * Used by modules interfacing with the underlying Holochain storage system directly.
 *
 * @package Holo-REA
 */
pub const FULFILLMENT_BASE_ENTRY_TYPE: &str = "vf_fulfillment_baseurl";
pub const FULFILLMENT_INITIAL_ENTRY_LINK_TYPE: &str = "vf_fulfillment_entry";
pub const FULFILLMENT_ENTRY_TYPE: &str = "vf_fulfillment";
pub const FULFILLMENT_FULFILLS_LINK_TYPE: &str = "vf_fulfillment_fulfills";
pub const FULFILLMENT_FULFILLS_LINK_TAG: &str = "fulfills";
pub const FULFILLMENT_FULFILLEDBY_LINK_TYPE: &str = "vf_fulfillment_fulfilled_by";
pub const FULFILLMENT_FULFILLEDBY_LINK_TAG: &str = "fulfilled_by";

pub const REPLICATE_CREATE_API_METHOD: &str = "create_fulfillment";
pub const REPLICATE_UPDATE_API_METHOD: &str = "update_fulfillment";
pub const REPLICATE_DELETE_API_METHOD: &str = "delete_fulfillment";
pub const COMMITMENT_FULFILLEDBY_INDEXING_API_METHOD: &str = "_internal_reindex_fulfillments";
pub const FULFILLMENT_FULFILLS_INDEXING_API_METHOD: &str = "_internal_reindex_commitments";
pub const EVENT_FULFILLS_INDEXING_API_METHOD: &str = "_internal_reindex_fulfillments";
pub const FULFILLMENT_FULFILLEDBY_INDEXING_API_METHOD: &str = "_internal_reindex_events";
