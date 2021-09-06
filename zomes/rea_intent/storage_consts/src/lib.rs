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

pub const INTENT_SATISFIEDBY_READ_API_METHOD: &str = "_internal_read_intent_satisfactions";

pub const INTENT_INPUT_READ_API_METHOD: &str = "_internal_read_intent_process_inputs";
pub const INTENT_INPUT_INDEXING_API_METHOD: &str = "_internal_reindex_process_inputs";
pub const PROCESS_INPUT_INDEXING_API_METHOD: &str = "index_process_input_intents";

pub const INTENT_OUTPUT_READ_API_METHOD: &str = "_internal_read_intent_process_outputs";
pub const INTENT_OUTPUT_INDEXING_API_METHOD: &str = "_internal_reindex_process_outputs";
pub const PROCESS_OUTPUT_INDEXING_API_METHOD: &str = "index_process_output_intents";
