/**
 * Observation module datatypes & behaviours
*/

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate vf_core;

pub use vf_core::{ type_aliases, measurement };

pub mod proposal;
pub mod proposed_intent;
pub mod proposed_to;

pub mod identifiers {
    pub const PROPOSAL_ENTRY_TYPE: &str = "vf_proposal";
    pub const PROPOSAL_BASE_ENTRY_TYPE: &str = "vf_proposal_baseurl";
    pub const PROPOSAL_INITIAL_ENTRY_LINK_TYPE: &str = "vf_proposal_entry";
    pub const PROPOSAL_PUBLISHES_LINK_TYPE: &str = "vf_proposal_published_link_type";
    pub const PROPOSAL_PUBLISHES_LINK_TAG: &str = "vf_proposal_published_link_tag";

    pub const PROPOSED_INTENT_ENTRY_TYPE: &str = "vf_proposed_intent";
    pub const PROPOSED_INTENT_BASE_ENTRY_TYPE: &str = "vf_proposed_intent_baseurl";
    pub const PROPOSED_INTENT_INITIAL_ENTRY_LINK_TYPE: &str = "vf_proposed_intent_entry";
    pub const PROPOSED_INTENT_PUBLISHED_IN_LINK_TYPE: &str = "vf_proposed_intent_published_in_link_type";
    pub const PROPOSED_INTENT_PUBLISHED_IN_LINK_TAG: &str = "vf_proposed_intent_published_in_link_tag";

    pub const PROPOSED_TO_ENTRY_TYPE: &str = "vf_proposed_to";
    pub const PROPOSED_TO_BASE_ENTRY_TYPE: &str = "vf_proposed_to_id";
    pub const PROPOSED_TO_INITIAL_ENTRY_LINK_TYPE: &str = "vf_proposed_to_entry";
    pub const PROPOSED_TO_PROPOSED_LINK_TYPE: &str = "proposed_to_proposed_link_type";
    pub const PROPOSED_TO_PROPOSED_LINK_TAG: &str = "proposed_to_proposed_link_tag";
}
