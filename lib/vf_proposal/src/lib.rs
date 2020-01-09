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
 
     pub const PROPOSED_INTENT_ENTRY_TYPE: &str = "vf_proposed_intent";
     pub const PROPOSED_INTENT_BASE_ENTRY_TYPE: &str = "vf_proposed_intent_baseurl";
     pub const PROPOSED_INTENT_INITIAL_ENTRY_LINK_TYPE: &str = "vf_proposed_intent_entry";
 
     pub const PROPOSED_TO_ENTRY_TYPE: &str = "vf_proposed_to";
     pub const PROPOSED_TO_ID_ENTRY_TYPE: &str = "vf_proposed_to_id";
     pub const PROPOSED_TO_INITIAL_ENTRY_LINK_TYPE: &str = "vf_proposed_to_entry";
 }
 