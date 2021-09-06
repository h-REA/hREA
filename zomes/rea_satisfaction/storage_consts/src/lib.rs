/**
 * Storage constants for zome entry & link type identifiers
 *
 * Used by modules interfacing with the underlying Holochain storage system directly.
 *
 * @package Holo-REA
 */
pub const SATISFACTION_BASE_ENTRY_TYPE: &str = "vf_satisfaction_baseurl";
pub const SATISFACTION_INITIAL_ENTRY_LINK_TYPE: &str = "vf_satisfaction_entry";
pub const SATISFACTION_ENTRY_TYPE: &str = "vf_satisfaction";
pub const SATISFACTION_SATISFIES_LINK_TYPE: &str = "vf_satisfaction_satisfies";
pub const SATISFACTION_SATISFIES_LINK_TAG: &str = "satisfies";
pub const SATISFACTION_SATISFIEDBY_LINK_TYPE: &str = "vf_satisfaction_satisfied_by";
pub const SATISFACTION_SATISFIEDBY_LINK_TAG: &str = "satisfied_by";

pub const REPLICATE_CREATE_API_METHOD: &str = "create_satisfaction";
pub const REPLICATE_UPDATE_API_METHOD: &str = "update_satisfaction";
pub const REPLICATE_DELETE_API_METHOD: &str = "delete_satisfaction";
pub const CHECK_COMMITMENT_API_METHOD: &str = "get_commitment";
pub const INTENT_INDEXING_API_METHOD: &str = "_internal_reindex_satisfactions";
pub const COMMITMENT_INDEXING_API_METHOD: &str = "_internal_reindex_satisfactions";
pub const EVENT_INDEXING_API_METHOD: &str = "_internal_reindex_satisfactions";
pub const SATISFACTION_SATISFIEDBY_INDEXING_API_METHOD: &str = "_internal_reindex_satisfiedby"; // :NOTE: same in both observation and planning zome APIs
pub const SATISFACTION_SATISFIES_INDEXING_API_METHOD: &str = "_internal_reindex_intents";
