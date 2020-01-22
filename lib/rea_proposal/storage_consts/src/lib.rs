/**
 * Storage constants for zome entry & link type identifiers
 *
 * Used by modules interfacing with the underlying Holochain storage system directly.
 *
 * @package Holo-REA
 */
pub const PROPOSAL_ENTRY_TYPE: &str = "vf_proposal";
pub const PROPOSAL_BASE_ENTRY_TYPE: &str = "vf_proposal_baseurl";
pub const PROPOSAL_INITIAL_ENTRY_LINK_TYPE: &str = "vf_proposal_entry";
pub const PROPOSAL_PUBLISHES_LINK_TYPE: &str = "vf_proposal_published_link_type";
pub const PROPOSAL_PUBLISHES_LINK_TAG: &str = "vf_proposal_published_link_tag";
