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

use vf_planning::{
    commitment::{
        ResponseData as CommitmentResponse,
    },
};
use super::{
    EVENT_BASE_ENTRY_TYPE,
    EVENT_FULFILLS_LINK_TYPE,
    EVENT_FULFILLS_LINK_TAG,
    COMMITMENT_FULFILLEDBY_LINK_TYPE,
    COMMITMENT_FULFILLEDBY_LINK_TAG,
};

pub fn handle_link_fulfillments(economic_event: Address, commitments: Vec<Address>) -> ZomeApiResult<Vec<Address>> {
    link_fulfillments(&economic_event, &commitments)
}

// :TODO: make error handling more robust
// :TODO: construct full final record by reading linked IDs
//        note that this means doing field-level filtering at the zome layer is an efficiency measure, we can skip link reading a lot of the time
pub fn handle_get_fulfillments(economic_event: Address) -> ZomeApiResult<Vec<CommitmentResponse>> {
    // determine address of base 'anchor' entry
    let base_address = entry_address(&Entry::App(EVENT_BASE_ENTRY_TYPE.into(), economic_event.into()))?;

    let fulfillments = get_links_and_load_type(&base_address, Exactly(EVENT_FULFILLS_LINK_TYPE), Exactly(EVENT_FULFILLS_LINK_TAG))?;

    Ok(fulfillments)
}

fn link_fulfillments(economic_event_address: &Address, commitments: &Vec<Address>) -> ZomeApiResult<Vec<Address>> {
    create_remote_query_index(
        EVENT_BASE_ENTRY_TYPE,
        EVENT_FULFILLS_LINK_TYPE, EVENT_FULFILLS_LINK_TAG,
        COMMITMENT_FULFILLEDBY_LINK_TYPE, COMMITMENT_FULFILLEDBY_LINK_TAG,
        economic_event_address,
        commitments,
    )
}

pub fn get_fulfilled_by(address: &Address) -> ZomeApiResult<Vec<Address>> {
    get_links_and_load_type(&address, Exactly(COMMITMENT_FULFILLEDBY_LINK_TYPE), Exactly(COMMITMENT_FULFILLEDBY_LINK_TAG))
}
