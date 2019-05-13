/**
 * Handling for `Fulfillment`-related behaviours as they apply to `Commitment`s
 */

use hdk::{
    holochain_core_types::{
        cas::content::Address,
        error::HolochainError,
        json::JsonString,
        entry::Entry,
    },
    error::ZomeApiResult,
    link_entries,
    entry_address,
    commit_entry,
    utils::{
        get_links_and_load_type,
    },
};
use holochain_core_types_derive::{ DefaultJson };

// use super::commitment_requests::{
//     CommitmentResponse,
// };
use vf_planning::{
    commitment::{
        Entry as CommitmentEntry,
    },
};

// Entry types

pub const EVENT_BASE_ENTRY_TYPE: &str = "vf_economic_event_baseurl";

// Link tags / link field names

pub const LINK_TAG_EVENT_FULFILLS: &str = "fulfills";
pub const LINK_TAG_COMMITMENT_FULFILLEDBY: &str = "fulfilledBy";

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct LinkResponseStatus {
    base_address: Address,
    links_result: Vec<Vec<ZomeApiResult<Address>>>
}

pub fn handle_link_fulfillments(economic_event: Address, commitments: Vec<Address>) -> ZomeApiResult<LinkResponseStatus> {
    // create a base entry pointer for the referenced event
    let base_entry = Entry::App(EVENT_BASE_ENTRY_TYPE.into(), economic_event.into());
    let base_address: Address = commit_entry(&base_entry)?;

    let commitment_results = commitments.iter()
        .map(|target_address| {
            vec![
                // link to the indexes of the external `Event` via `EconomicEvent.fulfills`.
                // Used by the GraphQL layer to load commitment entries as relationships under an `EconomicEvent`,
                link_entries(&base_address, &target_address, LINK_TAG_EVENT_FULFILLS),
                // link from the `Commitment` entry via `Commitment.fulfilledBy`.
                // Used for querying `EconomicEvent` entry addresses as a sub-field of the main `Commitment` record,
                // and to otherwise filter commitments by the event that fulfills them.
                link_entries(&target_address, &base_address, LINK_TAG_COMMITMENT_FULFILLEDBY)
            ]
        })
        .collect();

    Ok(LinkResponseStatus {
        base_address: base_address.clone().into(),
        links_result: commitment_results,
    })
}

// :TODO: make error handling more robust
// :TODO: construct full final record by reading linked IDs
//        note that this means doing field-level filtering at the zome layer is an efficiency measure, we can skip link reading a lot of the time
pub fn handle_get_fulfillments(economic_event: Address) -> ZomeApiResult<Vec<CommitmentEntry>> {
    // determine address of base 'anchor' entry
    let base_address = entry_address(&Entry::App(EVENT_BASE_ENTRY_TYPE.into(), economic_event.into()))?;

    let commitments = get_links_and_load_type(&base_address, LINK_TAG_EVENT_FULFILLS)?;

    Ok(commitments)
}
