/**
 * Handling for `Fulfillment`-related behaviours as they apply to `Commitment`s
 */

use hdk::{
    holochain_core_types::{
        cas::content::Address,
        entry::{
            Entry,
        },
    },
    error::ZomeApiResult,
    entry_address,
    utils::{
        get_links_and_load_type,
    },
};
use hdk_graph_helpers::{
    link_entries_bidir,
    create_base_entry,
    // link_remote_entries,
};

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

pub fn link_fulfillments(source_entry: &Address, targets: &Vec<Address>) -> Vec<Vec<Address>> {
    // create a base entry pointer for the referenced event
    let base_address = create_base_entry(EVENT_BASE_ENTRY_TYPE.into(), &source_entry);

    // link all referenced fulfillments to the event
    let commitment_results = targets.iter()
        .map(|target_address| {
            // link event to commitment by `fulfilled`/`fulfilledBy` edge
            link_entries_bidir(&base_address, &target_address, LINK_TAG_EVENT_FULFILLS, LINK_TAG_COMMITMENT_FULFILLEDBY)
        })
        .collect();

    commitment_results
}

pub fn handle_link_fulfillments(economic_event: Address, commitments: Vec<Address>) -> ZomeApiResult<Vec<Vec<Address>>> {
    Ok(link_fulfillments(&economic_event, &commitments))
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
