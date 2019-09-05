/**
 * Handling for external request structure for economic event records
 */

use std::borrow::Cow;
use hdk::{
    holochain_json_api::{ json::JsonString, error::JsonError },
    error::ZomeApiResult,
    error::ZomeApiError,
};
use holochain_json_derive::{ DefaultJson };

use hdk_graph_helpers::{
    records::{
        create_record,
        read_record_entry,
        update_record,
        delete_record,
    },
    links::{
        get_links_and_load_entry_data,
        get_linked_addresses_as_type,
    },
};

use vf_observation::type_aliases::{
    EventAddress,
    ProcessAddress,
    CommitmentAddress,
    IntentAddress,
    FulfillmentAddress,
    SatisfactionAddress,
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
    EVENT_SATISFIES_LINK_TYPE,
    EVENT_SATISFIES_LINK_TAG,
};
use vf_planning::identifiers::{
    FULFILLMENT_FULFILLEDBY_LINK_TYPE,
    FULFILLMENT_FULFILLEDBY_LINK_TAG,
    SATISFACTION_SATISFIEDBY_LINK_TYPE,
    SATISFACTION_SATISFIEDBY_LINK_TAG,
};

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    input_of: Option<ProcessAddress>,
    output_of: Option<ProcessAddress>,
    satisfies: Option<IntentAddress>,
    fulfills: Option<CommitmentAddress>,
}

// API gateway entrypoints. All methods must accept parameters by value.

pub fn receive_create_economic_event(event: EconomicEventCreateRequest) -> ZomeApiResult<EconomicEventResponse> {
    handle_create_economic_event(&event)
}

pub fn receive_get_economic_event(address: EventAddress) -> ZomeApiResult<EconomicEventResponse> {
    handle_get_economic_event(&address)
}

pub fn receive_update_economic_event(event: EconomicEventUpdateRequest) -> ZomeApiResult<EconomicEventResponse> {
    handle_update_economic_event(&event)
}

pub fn receive_delete_economic_event(address: EventAddress) -> ZomeApiResult<bool> {
    delete_record::<EconomicEventEntry>(&address)
}

pub fn receive_query_events(params: QueryParams) -> ZomeApiResult<Vec<EconomicEventResponse>> {
    handle_query_events(&params)
}

// API logic handlers

fn handle_create_economic_event(event: &EconomicEventCreateRequest) -> ZomeApiResult<EconomicEventResponse> {
    let (base_address, entry_resp): (EventAddress, EconomicEventEntry) = create_record(
        EVENT_BASE_ENTRY_TYPE, EVENT_ENTRY_TYPE,
        EVENT_INITIAL_ENTRY_LINK_TYPE,
        event.to_owned()
    )?;

    // return entire record structure
    Ok(construct_response(&base_address, &entry_resp, None, None))
}

fn handle_get_economic_event(address: &EventAddress) -> ZomeApiResult<EconomicEventResponse> {
    let entry = read_record_entry(&address)?;

    Ok(construct_response(address, &entry,
        Some(get_fulfillment_ids(address)),
        Some(get_satisfaction_ids(address)),
    ))
}

fn handle_update_economic_event(event: &EconomicEventUpdateRequest) -> ZomeApiResult<EconomicEventResponse> {
    let address = event.get_id();
    let new_entry = update_record(EVENT_ENTRY_TYPE, &address, event)?;

    Ok(construct_response(address, &new_entry,
        Some(get_fulfillment_ids(&address)),
        Some(get_satisfaction_ids(&address)),
    ))
}

fn handle_query_events(params: &QueryParams) -> ZomeApiResult<Vec<EconomicEventResponse>> {
    let mut entries_result: ZomeApiResult<Vec<(EventAddress, Option<EconomicEventEntry>)>> = Err(ZomeApiError::Internal("No results found".to_string()));

    // :TODO: implement proper AND search rather than exclusive operations
    match &params.satisfies {
        Some(satisfies) => {
            entries_result = get_links_and_load_entry_data(
                satisfies, SATISFACTION_SATISFIEDBY_LINK_TYPE, SATISFACTION_SATISFIEDBY_LINK_TAG,
            );
        },
        _ => (),
    };
    match &params.fulfills {
        Some(fulfills) => {
            entries_result = get_links_and_load_entry_data(
                fulfills, FULFILLMENT_FULFILLEDBY_LINK_TYPE, FULFILLMENT_FULFILLEDBY_LINK_TAG,
            );
        },
        _ => (),
    };
    match &params.input_of {
        Some(input_of) => {
            entries_result = get_links_and_load_entry_data(
                input_of, SATISFACTION_SATISFIEDBY_LINK_TYPE, SATISFACTION_SATISFIEDBY_LINK_TAG,
            );
        },
        _ => (),
    };
    match &params.output_of {
        Some(output_of) => {
            entries_result = get_links_and_load_entry_data(
                output_of, SATISFACTION_SATISFIEDBY_LINK_TYPE, SATISFACTION_SATISFIEDBY_LINK_TAG,
            );
        },
        _ => (),
    };

    match entries_result {
        Ok(entries) => Ok(
            entries.iter()
                .map(|(entry_base_address, maybe_entry)| {
                    match maybe_entry {
                        Some(entry) => Ok(construct_response(
                            entry_base_address, &entry,
                            Some(get_fulfillment_ids(&entry_base_address)),
                            Some(get_satisfaction_ids(&entry_base_address)),
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

// Internals

/// Used to load the list of linked Fulfillment IDs
fn get_fulfillment_ids<'a>(economic_event: &EventAddress) -> Cow<'a, Vec<FulfillmentAddress>> {
    get_linked_addresses_as_type(economic_event, EVENT_FULFILLS_LINK_TYPE, EVENT_FULFILLS_LINK_TAG)
}

/// Used to load the list of linked Satisfaction IDs
fn get_satisfaction_ids<'a>(economic_event: &EventAddress) -> Cow<'a, Vec<SatisfactionAddress>> {
    get_linked_addresses_as_type(economic_event, EVENT_SATISFIES_LINK_TYPE, EVENT_SATISFIES_LINK_TAG)
}
