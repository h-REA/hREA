/**
 * Handling for `Satisfaction`-related behaviours as they apply to `Commitment`s
 */

use hdk::{
    PUBLIC_TOKEN,
    THIS_INSTANCE,
    holochain_core_types::{
        cas::content::Address,
        json::JsonString,
    },
    error::ZomeApiResult,
    utils::get_links_and_load_type,
};

use hdk_graph_helpers::{
    records::create_base_entry,
    link_entries_bidir,
    link_remote_entries,
};

pub const INTENT_BASE_ENTRY_TYPE: &str = "vf_intent_baseurl";
pub const COMMITMENT_SATISFIES_LINK_TYPE: &str = "vf_commitment_satisfies";
pub const COMMITMENT_SATISFIES_LINK_TAG: &str = "satisfies";
pub const INTENT_SATISFIEDBY_LINK_TYPE: &str = "vf_intent_satisfied_by";
pub const INTENT_SATISFIEDBY_LINK_TAG: &str = "satisfied_by";

pub fn link_satisfactions(base_address: &Address, targets: &Vec<Address>) -> ZomeApiResult<Vec<Address>> {
    // abort if targets are empty
    if targets.len() == 0 { return Ok(vec![]) }

    // link all referenced satisfactions to the commitment
    let commitment_results = targets.iter()
        .map(|target_address| {
            // create a base entry for the targeted intent
            let target_base = create_base_entry(INTENT_BASE_ENTRY_TYPE.into(), target_address).unwrap();

            // write forward links within same zome
            link_entries_bidir(
                &base_address, &target_base,
                COMMITMENT_SATISFIES_LINK_TYPE, COMMITMENT_SATISFIES_LINK_TAG,
                INTENT_SATISFIEDBY_LINK_TYPE, INTENT_SATISFIEDBY_LINK_TAG
            );

            // return target base link
            target_address.to_owned()
        })
        .collect();

    // hdk::debug(format!("Local links: {:?}", commitment_results));

    // trigger reciprocal links in foreign zome
    // :TODO: figure out how to decode this into a Rust struct via the type system, with stronger types in place of JsonString
    let _foreign_result: JsonString = link_remote_entries(
        THIS_INSTANCE,
        "intent",
        Address::from(PUBLIC_TOKEN.to_string()),
        "link_satisfactions",
        &base_address,
        targets,
    )?;

    // hdk::debug(format!("result from intent DNA: {:?}", _foreign_result));

    Ok(commitment_results)
}

pub fn get_satisfactions(address: &Address) -> ZomeApiResult<Vec<Address>> {
    get_links_and_load_type(&address, Some(COMMITMENT_SATISFIES_LINK_TYPE.to_string()), Some(COMMITMENT_SATISFIES_LINK_TAG.to_string()))
}
