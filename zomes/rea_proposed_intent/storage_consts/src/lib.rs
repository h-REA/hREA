/**
 * Storage constants for zome entry & link type identifiers
 *
 * Used by modules interfacing with the underlying Holochain storage system directly.
 *
 * @package Holo-REA
 */
pub const PROPOSED_INTENT_ENTRY_TYPE: &str = "vf_proposed_intent";

pub const PROPOSED_INTENT_PUBLISHED_IN_LINK_TYPE: &str = "vf_proposed_intent_published_in";
pub const PROPOSED_INTENT_PUBLISHED_IN_LINK_TAG: &str = "published_in";

pub const PROPOSED_INTENT_PUBLISHES_LINK_TYPE: &str = "vf_proposed_intent_publishes";
pub const PROPOSED_INTENT_PUBLISHES_LINK_TAG: &str = "publishes";

pub const PROPOSED_INTENT_PROPOSAL_INDEXING_API_METHOD: &str = "_internal_reindex_proposals";
pub const PROPOSAL_PROPOSED_INTENT_INDEXING_API_METHOD: &str = "_internal_reindex_proposed_intents";

pub const INTENT_PUBLISHEDIN_INDEXING_API_METHOD: &str = "index_intent_proposals";

pub const PROPOSED_INTENT_PROPOSES_INDEXING_API_METHOD: &str = "_internal_index_";
