/**
 * Storage constants for zome entry & link type identifiers
 *
 * Used by modules interfacing with the underlying Holochain storage system directly.
 *
 * @package Holo-REA
 */
pub const PROPOSED_TO_ENTRY_TYPE: &str = "vf_proposed_to";

pub const PROPOSED_TO_PROPOSED_LINK_TAG: &str = "proposed";
pub const PROPOSED_TO_PROPOSED_TO_LINK_TAG: &str = "proposed_to";

pub const PROPOSED_TO_PROPOSAL_INDEXING_API_METHOD: &str = "_internal_reindex_proposals";
pub const PROPOSAL_PROPOSED_TO_INDEXING_API_METHOD: &str = "_internal_reindex_proposed_to";
