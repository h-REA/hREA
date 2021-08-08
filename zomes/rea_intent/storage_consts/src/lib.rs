/**
 * Storage constants for zome entry & link type identifiers
 *
 * Used by modules interfacing with the underlying Holochain storage system directly.
 *
 * @package Holo-REA
 */
pub const INTENT_ENTRY_TYPE: &str = "vf_intent";
pub const INTENT_SATISFIEDBY_LINK_TAG: &str = "satisfied_by";
pub const INTENT_INPUT_OF_LINK_TAG: &str = "input_of";
pub const INTENT_OUTPUT_OF_LINK_TAG: &str = "output_of";

pub const INTENT_PUBLISHED_IN_LINK_TAG: &str = "published_in";

pub const PROCESS_INPUT_INDEXING_API_METHOD: &str = "index_process_input_intents";
pub const PROCESS_OUTPUT_INDEXING_API_METHOD: &str = "index_process_output_intents";
