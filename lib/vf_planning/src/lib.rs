/**
 * Planning module datatypes & behaviours
 */

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate hdk_graph_helpers;
extern crate vf_core;

pub use vf_core::{ type_aliases, measurement };

pub mod commitment;
pub mod fulfillment;
pub mod intent;
pub mod satisfaction;

pub mod identifiers {
    pub const BRIDGED_OBSERVATION_DHT: &str = "vf_observation";

    pub const COMMITMENT_BASE_ENTRY_TYPE: &str = "vf_commitment_baseurl";
    pub const COMMITMENT_ENTRY_TYPE: &str = "vf_commitment";
    pub const COMMITMENT_FULFILLEDBY_LINK_TYPE: &str = "vf_commitment_fulfilled_by";
    pub const COMMITMENT_FULFILLEDBY_LINK_TAG: &str = "fulfills";
    pub const COMMITMENT_SATISFIES_LINK_TYPE: &str = "vf_commitment_satisfies";
    pub const COMMITMENT_SATISFIES_LINK_TAG: &str = "satisfies";

    pub const FULFILLMENT_BASE_ENTRY_TYPE: &str = "vf_fulfillment_baseurl";
    pub const FULFILLMENT_ENTRY_TYPE: &str = "vf_fulfillment";
    pub const FULFILLMENT_FULFILLS_LINK_TYPE: &str = "vf_fulfillment_fulfills";
    pub const FULFILLMENT_FULFILLS_LINK_TAG: &str = "fulfills";
    pub const FULFILLMENT_FULFILLEDBY_LINK_TYPE: &str = "vf_fulfillment_fulfilled_by";
    pub const FULFILLMENT_FULFILLEDBY_LINK_TAG: &str = "fulfilled_by";

    pub const INTENT_BASE_ENTRY_TYPE: &str = "vf_intent_baseurl";
    pub const INTENT_ENTRY_TYPE: &str = "vf_intent";

    pub const INTENT_SATISFIEDBY_LINK_TYPE: &str = "vf_intent_satisfied_by";
    pub const INTENT_SATISFIEDBY_LINK_TAG: &str = "satisfied_by";
}
