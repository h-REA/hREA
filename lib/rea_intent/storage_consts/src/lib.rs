/**
 * Storage constants for zome entry & link type identifiers
 *
 * Used by modules interfacing with the underlying Holochain storage system directly.
 *
 * @package Holo-REA
 */
pub const INTENT_BASE_ENTRY_TYPE: &str = "vf_intent_baseurl";
pub const INTENT_INITIAL_ENTRY_LINK_TYPE: &str = "vf_intent_entry";
pub const INTENT_ENTRY_TYPE: &str = "vf_intent";
pub const INTENT_SATISFIEDBY_LINK_TYPE: &str = "vf_intent_satisfied_by";
pub const INTENT_SATISFIEDBY_LINK_TAG: &str = "satisfied_by";
pub const INTENT_INPUT_OF_LINK_TYPE: &str = "vf_intent_input_of";
pub const INTENT_INPUT_OF_LINK_TAG: &str = "input_of";
pub const INTENT_OUTPUT_OF_LINK_TYPE: &str = "vf_intent_output_of";
pub const INTENT_OUTPUT_OF_LINK_TAG: &str = "output_of";

pub const INTENT_PUBLISHED_IN_LINK_TYPE: &str = "vf_intent_published_in";
pub const INTENT_PUBLISHED_IN_LINK_TAG: &str = "published_in";

pub const BRIDGED_OBSERVATION_DHT: &str = "vf_observation";
