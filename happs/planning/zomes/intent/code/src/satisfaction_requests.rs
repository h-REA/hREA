/**
 * Handling for `Satisfaction`-related behaviours as they apply to `Intent`s
 */

use hdk::{
    holochain_persistence_api::{
        cas::content::Address,
    },
    holochain_core_types::{
        link::LinkMatch::Exactly,
        entry::{
            Entry,
        },
    },
    error::ZomeApiResult,
    entry_address,
    utils::get_links_and_load_type,
};

use hdk_graph_helpers::{
    links::create_remote_query_index,
};

use vf_planning::intent::ResponseData as IntentResponse;
use super::{
    COMMITMENT_BASE_ENTRY_TYPE,
    COMMITMENT_SATISFIES_LINK_TYPE,
    COMMITMENT_SATISFIES_LINK_TAG,
    INTENT_SATISFIEDBY_LINK_TYPE,
    INTENT_SATISFIEDBY_LINK_TAG,
};

/// Zome API request handler for applying recriprocal links triggered by foreign zome or DNA
pub fn handle_link_satisfactions(base_entry: Address, target_entries: Vec<Address>) -> ZomeApiResult<Vec<Address>> {
    link_satisfied_by(&base_entry, &target_entries)
}

// :TODO: make error handling more robust
// :TODO: construct full final record by reading linked IDs
//        note that this means doing field-level filtering at the zome layer is an efficiency measure, we can skip link reading a lot of the time
pub fn handle_get_satisfactions(commitment: Address) -> ZomeApiResult<Vec<IntentResponse>> {
    // determine address of base 'anchor' entry
    let base_address = entry_address(&Entry::App(COMMITMENT_BASE_ENTRY_TYPE.into(), commitment.into()))?;

    let commitments = get_links_and_load_type(&base_address, Exactly(COMMITMENT_SATISFIES_LINK_TYPE), Exactly(COMMITMENT_SATISFIES_LINK_TAG))?;

    Ok(commitments)
}

/// Internal handler for applying satisfied_by link structure
fn link_satisfied_by(commitment_address: &Address, intent_addresses: &Vec<Address>) -> ZomeApiResult<Vec<Address>> {
    create_remote_query_index(
        COMMITMENT_BASE_ENTRY_TYPE,
        COMMITMENT_SATISFIES_LINK_TYPE, COMMITMENT_SATISFIES_LINK_TAG,
        INTENT_SATISFIEDBY_LINK_TYPE, INTENT_SATISFIEDBY_LINK_TAG,
        commitment_address,
        intent_addresses,
    )
}

pub fn get_satisfied_by(address: &Address) -> ZomeApiResult<Vec<Address>> {
    get_links_and_load_type(&address, Exactly(INTENT_SATISFIEDBY_LINK_TYPE), Exactly(INTENT_SATISFIEDBY_LINK_TAG))
}
