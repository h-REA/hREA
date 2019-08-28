/**
 * Handling for external request structure for economic event records
 */

use hdk::{
    holochain_persistence_api::{
        cas::content::Address,
    },
    holochain_core_types::link::LinkMatch::Exactly,
    error::ZomeApiResult,
    utils::get_links_and_load_type,
};

use hdk_graph_helpers::{
    records::{
        create_record,
        read_record_entry,
        update_record,
        delete_record,
    },
};
use vf_observation::economic_event::{
    Entry as EconomicEventEntry,
    CreateRequest as EconomicEventCreateRequest,
    UpdateRequest as EconomicEventUpdateRequest,
    ResponseData as EconomicEventResponse,
    construct_response,
};
use vf_observation::identifiers::{
    EVENT_BASE_ENTRY_TYPE,
    EVENT_ENTRY_TYPE,
    EVENT_INITIAL_ENTRY_LINK_TYPE,
    EVENT_FULFILLS_LINK_TYPE,
    EVENT_FULFILLS_LINK_TAG,
};

// :TODO: pull

pub fn handle_get_economic_event(address: Address) -> ZomeApiResult<EconomicEventResponse> {
    let entry = read_record_entry(&address)?;

    // It is important to note that there is no need to traverse the graph in any zome API read callbacks.
    // When querying links, we only need to read the target addresses from the links EAV in our DHT.
    // We leave it to the client GraphQL layer to handle fetching the details of associated fulfillments,
    // which would be performed externally as a call to the associated `planning` DHT for "get_fulfillment_ids".
    let fulfillment_links = get_fulfillment_ids(&address)?;

    Ok(construct_response(&address, entry, &Some(fulfillment_links)))
}

pub fn handle_create_economic_event(event: EconomicEventCreateRequest) -> ZomeApiResult<EconomicEventResponse> {
    let (base_address, entry_resp): (Address, EconomicEventEntry) = create_record(
        EVENT_BASE_ENTRY_TYPE, EVENT_ENTRY_TYPE,
        EVENT_INITIAL_ENTRY_LINK_TYPE,
        event
    )?;

    // return entire record structure
    Ok(construct_response(&base_address, entry_resp, &None))
}

pub fn handle_update_economic_event(event: EconomicEventUpdateRequest) -> ZomeApiResult<EconomicEventResponse> {
    let base_address = event.get_id();
    let new_entry = update_record(EVENT_ENTRY_TYPE, &base_address, &event)?;

    // :TODO: link field handling
    let fulfills = get_fulfillment_ids(&base_address)?;

    Ok(construct_response(base_address, new_entry, &Some(fulfills)))
}

pub fn handle_delete_economic_event(address: Address) -> ZomeApiResult<bool> {
    delete_record::<EconomicEventEntry>(&address)
}

/// Used to load the list of linked Fulfillment IDs
pub fn get_fulfillment_ids(economic_event: &Address) -> ZomeApiResult<Vec<Address>> {
    get_links_and_load_type(&economic_event, Exactly(EVENT_FULFILLS_LINK_TYPE), Exactly(EVENT_FULFILLS_LINK_TAG))
}
