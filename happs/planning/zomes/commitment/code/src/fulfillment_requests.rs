/**
 * Handling for `Fulfillment`-related behaviours as they apply to `Commitment`s
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
    utils::{
        get_links_and_load_type,
    },
};
use hdk_graph_helpers::{
    links::create_remote_query_index,
};

// use super::commitment_requests::{
//     CommitmentResponse,
// };
use vf_planning::{
    commitment::{
        ResponseData as CommitmentResponse,
    },
};

// Entry types

pub const EVENT_BASE_ENTRY_TYPE: &str = "vf_economic_event_baseurl";

// Link tags / link field names

pub const EVENT_FULFILLS_LINK_TYPE: &str = "vf_economic_event_fulfills";
pub const COMMITMENT_FULFILLEDBY_LINK_TYPE: &str = "vf_commitment_fulfilled_by";

pub const LINK_TAG_EVENT_FULFILLS: &str = "fulfills";
pub const LINK_TAG_COMMITMENT_FULFILLEDBY: &str = "fulfilled_by";

pub fn handle_link_fulfillments(economic_event: Address, commitments: Vec<Address>) -> ZomeApiResult<Vec<Address>> {
    link_fulfillments(&economic_event, &commitments)
}

// :TODO: make error handling more robust
// :TODO: construct full final record by reading linked IDs
//        note that this means doing field-level filtering at the zome layer is an efficiency measure, we can skip link reading a lot of the time
pub fn handle_get_fulfillments(economic_event: Address) -> ZomeApiResult<Vec<CommitmentResponse>> {
    // determine address of base 'anchor' entry
    let base_address = entry_address(&Entry::App(EVENT_BASE_ENTRY_TYPE.into(), economic_event.into()))?;

    let commitments = get_links_and_load_type(&base_address, Exactly(EVENT_FULFILLS_LINK_TYPE), Exactly(LINK_TAG_EVENT_FULFILLS))?;

    Ok(commitments)
}

fn link_fulfillments(economic_event_address: &Address, commitments: &Vec<Address>) -> ZomeApiResult<Vec<Address>> {
    create_remote_query_index(
        EVENT_BASE_ENTRY_TYPE,
        EVENT_FULFILLS_LINK_TYPE, LINK_TAG_EVENT_FULFILLS,
        COMMITMENT_FULFILLEDBY_LINK_TYPE, LINK_TAG_COMMITMENT_FULFILLEDBY,
        economic_event_address,
        commitments,
    )
}

pub fn get_fulfilled_by(address: &Address) -> ZomeApiResult<Vec<Address>> {
    get_links_and_load_type(&address, Exactly(COMMITMENT_FULFILLEDBY_LINK_TYPE), Exactly(LINK_TAG_COMMITMENT_FULFILLEDBY))
}
