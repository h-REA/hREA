/**
 * Handling for `Satisfaction`-related behaviours as they apply to `Intent`s
 */

use hdk::{
    holochain_core_types::{
        cas::content::Address,
    },
    error::ZomeApiResult,
    utils::get_links_and_load_type,
};

use hdk_graph_helpers::{
    records::create_base_entry,
    link_entries_bidir,
};

pub const COMMITMENT_BASE_ENTRY_TYPE: &str = "vf_commitment_baseurl";
pub const COMMITMENT_SATISFIES_LINK_TYPE: &str = "vf_commitment_satisfies";
pub const COMMITMENT_SATISFIES_LINK_TAG: &str = "satisfies";
pub const INTENT_SATISFIEDBY_LINK_TYPE: &str = "vf_intent_satisfied_by";
pub const INTENT_SATISFIEDBY_LINK_TAG: &str = "satisfied_by";

/// Zome API request handler for applying recriprocal links triggered by foreign zome or DNA
pub fn handle_link_satisfactions(base_entry: Address, target_entries: Vec<Address>) -> ZomeApiResult<Vec<Address>> {
    Ok(link_satisfied_by(&base_entry, &target_entries))
}

/// Internal handler for applying satisfied_by link structure
fn link_satisfied_by(commitment_address: &Address, intent_addresses: &Vec<Address>) -> Vec<Address> {
    // create a base entry for the linking commitment
    let commitment_base = create_base_entry(COMMITMENT_BASE_ENTRY_TYPE.into(), commitment_address).unwrap();

    // link all referenced satisfactions from the commitment
    let results = intent_addresses.iter()
        .map(|target_address| {

            // link event to commitment by `fulfilled`/`fulfilledBy` edge
            link_entries_bidir(
                &commitment_base, &target_address,
                COMMITMENT_SATISFIES_LINK_TYPE, COMMITMENT_SATISFIES_LINK_TAG,
                INTENT_SATISFIEDBY_LINK_TYPE, INTENT_SATISFIEDBY_LINK_TAG,
            );

            // return target base link
            target_address.to_owned()
        })
        .collect();

    // hdk::debug(format!("links inside intent DNA: {:?}", results));

    results
}

pub fn get_satisfied_by(address: &Address) -> ZomeApiResult<Vec<Address>> {
    get_links_and_load_type(&address, Some(INTENT_SATISFIEDBY_LINK_TYPE.to_string()), Some(INTENT_SATISFIEDBY_LINK_TAG.to_string()))
}
