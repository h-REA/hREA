/**
 * Handling for external request structure for economic event records
 */

use hdk::{
    holochain_persistence_api::{
        cas::content::Address,
    },
    holochain_core_types::link::LinkMatch::Exactly,
    error::ZomeApiResult,
    error::ZomeApiError,
    get_links,
};

use hdk_graph_helpers::{
    records::{
        create_record,
        read_record_entry,
        update_record,
        delete_record,
    },
    links::{
        get_links_and_load_entry_data,
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
use vf_planning::identifiers::{
    FULFILLMENT_FULFILLEDBY_LINK_TYPE,
    FULFILLMENT_FULFILLEDBY_LINK_TAG,
};

// :TODO: pull

pub fn receive_get_economic_event(address: Address) -> ZomeApiResult<EconomicEventResponse> {
    handle_get_economic_event(&address)
}

fn handle_get_economic_event(address: &Address) -> ZomeApiResult<EconomicEventResponse> {
    let entry = read_record_entry(&address)?;

    // It is important to note that there is no need to traverse the graph in any zome API read callbacks.
    // When querying links, we only need to read the target addresses from the links EAV in our DHT.
    // We leave it to the client GraphQL layer to handle fetching the details of associated fulfillments,
    // which would be performed externally as a call to the associated `planning` DHT for "get_fulfillment_ids".
    let fulfillment_links = get_fulfillment_ids(&address)?;

    Ok(construct_response(&address, &entry, &Some(fulfillment_links)))
}

pub fn receive_create_economic_event(event: EconomicEventCreateRequest) -> ZomeApiResult<EconomicEventResponse> {
    handle_create_economic_event(&event)
}

fn handle_create_economic_event(event: &EconomicEventCreateRequest) -> ZomeApiResult<EconomicEventResponse> {
    let (base_address, entry_resp): (Address, EconomicEventEntry) = create_record(
        EVENT_BASE_ENTRY_TYPE, EVENT_ENTRY_TYPE,
        EVENT_INITIAL_ENTRY_LINK_TYPE,
        event.to_owned()
    )?;

    // return entire record structure
    Ok(construct_response(&base_address, &entry_resp, &None))
}

pub fn receive_update_economic_event(event: EconomicEventUpdateRequest) -> ZomeApiResult<EconomicEventResponse> {
    handle_update_economic_event(&event)
}

fn handle_update_economic_event(event: &EconomicEventUpdateRequest) -> ZomeApiResult<EconomicEventResponse> {
    let base_address = event.get_id();
    let new_entry = update_record(EVENT_ENTRY_TYPE, &base_address, event)?;

    // :TODO: link field handling
    let fulfills = get_fulfillment_ids(&base_address)?;

    Ok(construct_response(base_address, &new_entry, &Some(fulfills)))
}

pub fn receive_delete_economic_event(address: Address) -> ZomeApiResult<bool> {
    delete_record::<EconomicEventEntry>(&address)
}

/// Used to load the list of linked Fulfillment IDs
fn get_fulfillment_ids(economic_event: &Address) -> ZomeApiResult<Vec<Address>> {
    Ok(get_links(&economic_event, Exactly(EVENT_FULFILLS_LINK_TYPE), Exactly(EVENT_FULFILLS_LINK_TAG))?.addresses())
}

pub fn receive_query_events(fulfillment: Address) -> ZomeApiResult<Vec<EconomicEventResponse>> {
    handle_query_events(&fulfillment)
}

fn handle_query_events(fulfillment: &Address) -> ZomeApiResult<Vec<EconomicEventResponse>> {
    let entries_result: ZomeApiResult<Vec<(Address, Option<EconomicEventEntry>)>> = get_links_and_load_entry_data(
        &fulfillment,
        FULFILLMENT_FULFILLEDBY_LINK_TYPE, FULFILLMENT_FULFILLEDBY_LINK_TAG,
    );

    match entries_result {
        Ok(entries) => Ok(
            entries.iter()
                .map(|(entry_base_address, maybe_entry)| {
                    // :TODO: avoid cloning entry
                    match maybe_entry {
                        Some(entry) => Ok(construct_response(
                            entry_base_address,
                            &entry,
                            &Some(get_fulfillment_ids(&entry_base_address)?)
                        )),
                        None => Err(ZomeApiError::Internal("referenced entry not found".to_string()))
                    }
                })
                .filter_map(Result::ok)
                .collect()
        ),
        _ => Err(ZomeApiError::Internal("could not load linked addresses".to_string()))
    }
}
