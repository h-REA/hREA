/**
 * Handling for `Satisfaction`-related behaviours as they apply to `Commitment`s
 */

use hdk::{
    PUBLIC_TOKEN,
    THIS_INSTANCE,
    holochain_persistence_api::{
        cas::content::Address,
    },
    holochain_core_types::link::LinkMatch::Exactly,
    error::ZomeApiResult,
    utils::get_links_and_load_type,
};

use hdk_graph_helpers::{
    links::create_remote_index_pair,
};

pub const INTENT_BASE_ENTRY_TYPE: &str = "vf_intent_baseurl";
pub const COMMITMENT_SATISFIES_LINK_TYPE: &str = "vf_commitment_satisfies";
pub const COMMITMENT_SATISFIES_LINK_TAG: &str = "satisfies";
pub const INTENT_SATISFIEDBY_LINK_TYPE: &str = "vf_intent_satisfied_by";
pub const INTENT_SATISFIEDBY_LINK_TAG: &str = "satisfied_by";

// pub fn handle_link_satisfactions(commitment: Address, intents: Vec<Address>) -> ZomeApiResult<Vec<Address>> {
//     link_satisfactions(&commitment, &intents)
// }

pub fn link_satisfactions(base_address: &Address, targets: &Vec<Address>) -> ZomeApiResult<Vec<Address>> {
    create_remote_index_pair(
        THIS_INSTANCE,
        "intent",
        "link_satisfactions",
        // &PUBLIC_TOKEN,
        Address::from(PUBLIC_TOKEN.to_string()),
        INTENT_BASE_ENTRY_TYPE,
        COMMITMENT_SATISFIES_LINK_TYPE, COMMITMENT_SATISFIES_LINK_TAG,
        INTENT_SATISFIEDBY_LINK_TYPE, INTENT_SATISFIEDBY_LINK_TAG,
        base_address,
        targets,
    )
}

pub fn get_satisfactions(address: &Address) -> ZomeApiResult<Vec<Address>> {
    get_links_and_load_type(&address, Exactly(COMMITMENT_SATISFIES_LINK_TYPE), Exactly(COMMITMENT_SATISFIES_LINK_TAG))
}
