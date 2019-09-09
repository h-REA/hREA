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
    MaybeUndefined,
    records::{
        create_record,
        read_record_entry,
        update_record,
        delete_record,
    },
    links::{
        link_entries_bidir,
        get_links_and_load_entry_data,
        get_linked_addresses_as_type,
        replace_entry_link_set,
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
    EVENT_FULFILLS_LINK_TYPE, EVENT_FULFILLS_LINK_TAG,
    EVENT_SATISFIES_LINK_TYPE, EVENT_SATISFIES_LINK_TAG,
    EVENT_INPUT_OF_LINK_TYPE, EVENT_INPUT_OF_LINK_TAG,
    EVENT_OUTPUT_OF_LINK_TYPE, EVENT_OUTPUT_OF_LINK_TAG,
    PROCESS_EVENT_INPUTS_LINK_TYPE, PROCESS_EVENT_INPUTS_LINK_TAG,
    PROCESS_EVENT_OUTPUTS_LINK_TYPE, PROCESS_EVENT_OUTPUTS_LINK_TAG,
};
use vf_planning::identifiers::{
    FULFILLMENT_FULFILLEDBY_LINK_TYPE, FULFILLMENT_FULFILLEDBY_LINK_TAG,
    SATISFACTION_SATISFIEDBY_LINK_TYPE, SATISFACTION_SATISFIEDBY_LINK_TAG,
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

    // handle link fields
    if let EconomicEventCreateRequest { input_of: MaybeUndefined::Some(input_of), .. } = event {
        let _results = link_entries_bidir(
            base_address.as_ref(),
            input_of.as_ref(),
            EVENT_INPUT_OF_LINK_TYPE, EVENT_INPUT_OF_LINK_TAG,
            PROCESS_EVENT_INPUTS_LINK_TYPE, PROCESS_EVENT_INPUTS_LINK_TAG,
        );
    };
    if let EconomicEventCreateRequest { output_of: MaybeUndefined::Some(output_of), .. } = event {
        let _results = link_entries_bidir(
            base_address.as_ref(),
            output_of.as_ref(),
            EVENT_OUTPUT_OF_LINK_TYPE, EVENT_OUTPUT_OF_LINK_TAG,
            PROCESS_EVENT_OUTPUTS_LINK_TYPE, PROCESS_EVENT_OUTPUTS_LINK_TAG,
        );
    };

    // :TODO: pass results from link creation rather than re-reading
    Ok(construct_response(&base_address, &entry_resp, get_link_fields(&base_address)))
}

fn handle_get_economic_event(address: &EventAddress) -> ZomeApiResult<EconomicEventResponse> {
    let entry = read_record_entry(&address)?;
    Ok(construct_response(address, &entry, get_link_fields(address)))
}

fn handle_update_economic_event(event: &EconomicEventUpdateRequest) -> ZomeApiResult<EconomicEventResponse> {
    let address = event.get_id();
    let new_entry = update_record(EVENT_ENTRY_TYPE, &address, event)?;

    // handle link fields
    replace_entry_link_set(address, &event.input_of,
        EVENT_INPUT_OF_LINK_TYPE, EVENT_INPUT_OF_LINK_TAG,
        PROCESS_EVENT_INPUTS_LINK_TYPE, PROCESS_EVENT_INPUTS_LINK_TAG,
    );
    replace_entry_link_set(address, &event.output_of,
        EVENT_OUTPUT_OF_LINK_TYPE, EVENT_OUTPUT_OF_LINK_TAG,
        PROCESS_EVENT_OUTPUTS_LINK_TYPE, PROCESS_EVENT_OUTPUTS_LINK_TAG,
    );

    // :TODO: optimise this- should pass results from `replace_entry_link_set` instead of retrieving from `get_link_fields` where updates
    Ok(construct_response(address, &new_entry, get_link_fields(address)))
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
                input_of, PROCESS_EVENT_INPUTS_LINK_TYPE, PROCESS_EVENT_INPUTS_LINK_TAG,
            );
        },
        _ => (),
    };
    match &params.output_of {
        Some(output_of) => {
            entries_result = get_links_and_load_entry_data(
                output_of, PROCESS_EVENT_OUTPUTS_LINK_TYPE, PROCESS_EVENT_OUTPUTS_LINK_TAG,
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
                            entry_base_address, &entry, get_link_fields(entry_base_address),
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

// field list retrieval internals

// @see construct_response
fn get_link_fields<'a>(event: &EventAddress) -> (
    Option<ProcessAddress>,
    Option<ProcessAddress>,
    Option<Cow<'a, Vec<FulfillmentAddress>>>,
    Option<Cow<'a, Vec<SatisfactionAddress>>>,
) {
    (
        get_linked_addresses_as_type(event, EVENT_INPUT_OF_LINK_TYPE, EVENT_INPUT_OF_LINK_TAG).into_owned().pop(),
        get_linked_addresses_as_type(event, EVENT_OUTPUT_OF_LINK_TYPE, EVENT_OUTPUT_OF_LINK_TAG).into_owned().pop(),
        Some(get_linked_addresses_as_type(event, EVENT_FULFILLS_LINK_TYPE, EVENT_FULFILLS_LINK_TAG)),
        Some(get_linked_addresses_as_type(event, EVENT_SATISFIES_LINK_TYPE, EVENT_SATISFIES_LINK_TAG)),
    )
}
