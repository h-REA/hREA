/**
 * Storage constants for zome entry & link type identifiers
 *
 * Used by modules interfacing with the underlying Holochain storage system directly.
 *
 * @package Holo-REA
 */
pub const AGREEMENT_BASE_ENTRY_TYPE: &str = "vf_agreement_baseurl";
pub const AGREEMENT_INITIAL_ENTRY_LINK_TYPE: &str = "vf_agreement_entry";
pub const AGREEMENT_ENTRY_TYPE: &str = "vf_agreement";

pub const AGREEMENT_EVENTS_LINK_TYPE: &str = "vf_agreement_events";
pub const AGREEMENT_EVENTS_LINK_TAG: &str = "economic_events";
pub const AGREEMENT_COMMITMENTS_LINK_TYPE: &str = "vf_agreement_commitments";
pub const AGREEMENT_COMMITMENTS_LINK_TAG: &str = "commitments";
