
/**
 * Handling for `Fulfillment` related behaviours as they relate to `EconomicEvent`s
 */

use hdk::{
    PUBLIC_TOKEN,
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

use super::{
    BRIDGED_PLANNING_DHT,
    EVENT_FULFILLS_LINK_TYPE,
    EVENT_FULFILLS_LINK_TAG,
    COMMITMENT_BASE_ENTRY_TYPE,
    COMMITMENT_FULFILLEDBY_LINK_TYPE,
    COMMITMENT_FULFILLEDBY_LINK_TAG,
};

pub fn handle_link_fulfillments(economic_event: Address, commitments: Vec<Address>) -> ZomeApiResult<Vec<Address>> {
    link_fulfillments(&economic_event, &commitments)
}

pub fn link_fulfillments(source_entry: &Address, targets: &Vec<Address>) -> ZomeApiResult<Vec<Address>> {
    create_remote_index_pair(
        BRIDGED_PLANNING_DHT,
        "commitment",
        "link_fulfillments",
        // &PUBLIC_TOKEN,
        Address::from(PUBLIC_TOKEN.to_string()),
        COMMITMENT_BASE_ENTRY_TYPE,
        EVENT_FULFILLS_LINK_TYPE, EVENT_FULFILLS_LINK_TAG,
        COMMITMENT_FULFILLEDBY_LINK_TYPE, COMMITMENT_FULFILLEDBY_LINK_TAG,
        source_entry,
        targets,
    )
}

pub fn get_fulfillments(address: &Address) -> ZomeApiResult<Vec<Address>> {
    get_links_and_load_type(&address, Exactly(EVENT_FULFILLS_LINK_TYPE), Exactly(EVENT_FULFILLS_LINK_TAG))
}
