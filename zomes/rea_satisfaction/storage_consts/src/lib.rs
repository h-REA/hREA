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

pub const BRIDGED_OBSERVATION_DHT: &str = "vf_observation";
