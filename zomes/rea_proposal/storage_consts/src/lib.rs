/**
 * Storage constants for zome entry & link type identifiers
 *
 * Used by modules interfacing with the underlying Holochain storage system directly.
 *
 * @package Holo-REA
 */
pub const PROPOSAL_ENTRY_TYPE: &str = "vf_proposal";

pub const PROPOSAL_PUBLISHES_LINK_TAG: &str = "publishes";
pub const PROPOSAL_PUBLISHED_TO_LINK_TAG: &str = "published_to";

pub const PROPOSAL_PUBLISHES_READ_API_METHOD: &str = "_internal_read_proposal_proposed_intents";
pub const PROPOSAL_PUBLISHED_TO_READ_API_METHOD: &str = "_internal_read_proposal_participants";
