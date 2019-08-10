
/**
 * Handling for `Fulfillment` related behaviours as they relate to `EconomicEvent`s
 */

use hdk::{
    PUBLIC_TOKEN,
    holochain_persistence_api::{
        cas::content::Address,
    },
    error::ZomeApiResult,
    utils::get_links_and_load_type,
};
use hdk_graph_helpers::{
    links::create_remote_index_pair,
};

use vf_observation::{
    BRIDGED_PLANNING_DHT,
};

// Entry types

pub const COMMITMENT_BASE_ENTRY_TYPE: &str = "vf_commitment_baseurl";

// Link tags / link field names

pub const EVENT_FULFILLS_LINK_TYPE: &str = "vf_economic_event_fulfills";
pub const COMMITMENT_FULFILLEDBY_LINK_TYPE: &str = "vf_commitment_fulfilled_by";

pub const LINK_TAG_EVENT_FULFILLS: &str = "fulfills";
pub const LINK_TAG_COMMITMENT_FULFILLEDBY: &str = "fulfilled_by";

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
        EVENT_FULFILLS_LINK_TYPE, LINK_TAG_EVENT_FULFILLS,
        COMMITMENT_FULFILLEDBY_LINK_TYPE, LINK_TAG_COMMITMENT_FULFILLEDBY,
        source_entry,
        targets,
    )
}

pub fn get_fulfillments(address: &Address) -> ZomeApiResult<Vec<Address>> {
    get_links_and_load_type(&address, Some(EVENT_FULFILLS_LINK_TYPE.to_string()), Some(LINK_TAG_EVENT_FULFILLS.to_string()))
}
