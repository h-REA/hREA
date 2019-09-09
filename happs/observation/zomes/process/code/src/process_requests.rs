/**
 * Handling for `Process`-related requests
 */

use std::borrow::Cow;
use hdk::{
    holochain_json_api::{ json::JsonString, error::JsonError },
    holochain_persistence_api::cas::content::Address,
    error::{ ZomeApiResult, ZomeApiError },
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
        get_linked_remote_addresses_as_type,
    },
    rpc::{
        RemoteEntryLinkResponse,
        handle_remote_index_request,
    },
};

use vf_observation::type_aliases::{
    ProcessAddress,
    EventAddress,
    CommitmentAddress,
    IntentAddress,
    AgentAddress,
};
use vf_observation::identifiers::{
    PROCESS_BASE_ENTRY_TYPE,
    PROCESS_INITIAL_ENTRY_LINK_TYPE,
    PROCESS_ENTRY_TYPE,
    PROCESS_EVENT_INPUTS_LINK_TYPE, PROCESS_EVENT_INPUTS_LINK_TAG,
    EVENT_INPUT_OF_LINK_TYPE, EVENT_INPUT_OF_LINK_TAG,
    PROCESS_EVENT_OUTPUTS_LINK_TYPE, PROCESS_EVENT_OUTPUTS_LINK_TAG,
    EVENT_OUTPUT_OF_LINK_TYPE, EVENT_OUTPUT_OF_LINK_TAG,
    PROCESS_COMMITMENT_INPUTS_LINK_TYPE, PROCESS_COMMITMENT_INPUTS_LINK_TAG,
    PROCESS_COMMITMENT_OUTPUTS_LINK_TYPE, PROCESS_COMMITMENT_OUTPUTS_LINK_TAG,
    PROCESS_INTENT_INPUTS_LINK_TYPE, PROCESS_INTENT_INPUTS_LINK_TAG,
    PROCESS_INTENT_OUTPUTS_LINK_TYPE, PROCESS_INTENT_OUTPUTS_LINK_TAG,
};
use vf_planning::identifiers::{
    COMMITMENT_BASE_ENTRY_TYPE,
    COMMITMENT_INPUT_OF_LINK_TYPE, COMMITMENT_INPUT_OF_LINK_TAG,
    COMMITMENT_OUTPUT_OF_LINK_TYPE, COMMITMENT_OUTPUT_OF_LINK_TAG,
    INTENT_INPUT_OF_LINK_TYPE, INTENT_INPUT_OF_LINK_TAG,
    INTENT_OUTPUT_OF_LINK_TYPE, INTENT_OUTPUT_OF_LINK_TAG,
};
use vf_observation::process::{
    Entry,
    CreateRequest,
    UpdateRequest,
    ResponseData as Response,
    construct_response,
};

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    inputs: Option<EventAddress>,
    outputs: Option<EventAddress>,
    unplanned_economic_events: Option<EventAddress>,
    committed_inputs: Option<CommitmentAddress>,
    committed_outputs: Option<CommitmentAddress>,
    intended_inputs: Option<IntentAddress>,
    intended_outputs: Option<IntentAddress>,
    working_agents: Option<AgentAddress>,
}

pub fn receive_create_process(process: CreateRequest) -> ZomeApiResult<Response> {
    handle_create_process(&process)
}

pub fn receive_get_process(address: ProcessAddress) -> ZomeApiResult<Response> {
    handle_get_process(&address)
}

pub fn receive_update_process(process: UpdateRequest) -> ZomeApiResult<Response> {
    handle_update_process(&process)
}

pub fn receive_delete_process(address: ProcessAddress) -> ZomeApiResult<bool> {
    delete_record::<Entry>(&address)
}

pub fn receive_query_processes(params: QueryParams) -> ZomeApiResult<Vec<Response>> {
    handle_query_processes(&params)
}

pub fn receive_link_committed_inputs(base_entry: CommitmentAddress, target_entries: Vec<Address>) -> ZomeApiResult<RemoteEntryLinkResponse> {
    handle_remote_index_request(
        COMMITMENT_BASE_ENTRY_TYPE,
        COMMITMENT_INPUT_OF_LINK_TYPE, COMMITMENT_INPUT_OF_LINK_TAG,
        PROCESS_COMMITMENT_INPUTS_LINK_TYPE, PROCESS_COMMITMENT_INPUTS_LINK_TAG,
        base_entry.as_ref(), &target_entries
    )
}

pub fn receive_link_committed_outputs(base_entry: CommitmentAddress, target_entries: Vec<Address>) -> ZomeApiResult<RemoteEntryLinkResponse> {
    handle_remote_index_request(
        COMMITMENT_BASE_ENTRY_TYPE,
        COMMITMENT_OUTPUT_OF_LINK_TYPE, COMMITMENT_OUTPUT_OF_LINK_TAG,
        PROCESS_COMMITMENT_OUTPUTS_LINK_TYPE, PROCESS_COMMITMENT_OUTPUTS_LINK_TAG,
        base_entry.as_ref(), &target_entries
    )
}

// :TODO: move to hdk_graph_helpers module

fn handle_get_process(address: &ProcessAddress) -> ZomeApiResult<Response> {
    Ok(construct_response(address, &read_record_entry(address)?, get_link_fields(address)))
}

fn handle_create_process(process: &CreateRequest) -> ZomeApiResult<Response> {
    let (base_address, entry_resp): (ProcessAddress, Entry) = create_record(
        PROCESS_BASE_ENTRY_TYPE, PROCESS_ENTRY_TYPE,
        PROCESS_INITIAL_ENTRY_LINK_TYPE,
        process.to_owned(),
    )?;
    Ok(construct_response(&base_address, &entry_resp, get_link_fields(&base_address)))
}

fn handle_update_process(process: &UpdateRequest) -> ZomeApiResult<Response> {
    let base_address = process.get_id();
    let new_entry = update_record(PROCESS_ENTRY_TYPE, base_address, process)?;
    Ok(construct_response(&base_address, &new_entry, get_link_fields(base_address)))
}

fn handle_query_processes(params: &QueryParams) -> ZomeApiResult<Vec<Response>> {
    let mut entries_result: ZomeApiResult<Vec<(ProcessAddress, Option<Entry>)>> = Err(ZomeApiError::Internal("No results found".to_string()));

    // :TODO: proper search logic, not mutually exclusive ID filters

    match &params.inputs {
        Some(inputs) => {
            entries_result = get_links_and_load_entry_data(inputs, EVENT_INPUT_OF_LINK_TYPE, EVENT_INPUT_OF_LINK_TAG);
        },
        _ => (),
    };
    match &params.outputs {
        Some(outputs) => {
            entries_result = get_links_and_load_entry_data(outputs, EVENT_OUTPUT_OF_LINK_TYPE, EVENT_OUTPUT_OF_LINK_TAG);
        },
        _ => (),
    };
    match &params.committed_inputs {
        Some(committed_inputs) => {
            entries_result = get_links_and_load_entry_data(committed_inputs, COMMITMENT_INPUT_OF_LINK_TYPE, COMMITMENT_INPUT_OF_LINK_TAG);
        },
        _ => (),
    };
    match &params.committed_outputs {
        Some(committed_outputs) => {
            entries_result = get_links_and_load_entry_data(committed_outputs, COMMITMENT_OUTPUT_OF_LINK_TYPE, COMMITMENT_OUTPUT_OF_LINK_TAG);
        },
        _ => (),
    };
    match &params.intended_inputs {
        Some(intended_inputs) => {
            entries_result = get_links_and_load_entry_data(intended_inputs, INTENT_INPUT_OF_LINK_TYPE, INTENT_INPUT_OF_LINK_TAG);
        },
        _ => (),
    };
    match &params.intended_outputs {
        Some(intended_outputs) => {
            entries_result = get_links_and_load_entry_data(intended_outputs, INTENT_OUTPUT_OF_LINK_TYPE, INTENT_OUTPUT_OF_LINK_TAG);
        },
        _ => (),
    };

    // :TODO: unplanned_economic_events, working_agents

    match entries_result {
        Ok(entries) => Ok(
            entries.iter()
                .map(|(entry_base_address, maybe_entry)| {
                    match maybe_entry {
                        Some(entry) => Ok(construct_response(
                            entry_base_address,
                            &entry,
                            get_link_fields(entry_base_address),
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
fn get_link_fields<'a>(process: &ProcessAddress) -> (
    Option<Cow<'a, Vec<EventAddress>>>,
    Option<Cow<'a, Vec<EventAddress>>>,
    Option<Cow<'a, Vec<EventAddress>>>,
    Option<Cow<'a, Vec<CommitmentAddress>>>,
    Option<Cow<'a, Vec<CommitmentAddress>>>,
    Option<Cow<'a, Vec<IntentAddress>>>,
    Option<Cow<'a, Vec<IntentAddress>>>,
    Option<Cow<'a, Vec<ProcessAddress>>>,
    Option<Cow<'a, Vec<ProcessAddress>>>,
    Option<Cow<'a, Vec<AgentAddress>>>,
    Option<Cow<'a, Vec<EventAddress>>>,
    Option<Cow<'a, Vec<EventAddress>>>,
) {
    (
        Some(get_input_event_ids(process)),
        Some(get_output_event_ids(process)),
        None,  // :TODO: unplanned_economic_events
        Some(get_input_commitment_ids(process)),
        Some(get_output_commitment_ids(process)),
        Some(get_input_intent_ids(process)),
        Some(get_output_intent_ids(process)),
        None, // :TODO: next_processes
        None, // :TODO: previous_processes
        None, // :TODO: working_agents
        None, // :TODO: trace
        None, // :TODO: track
    )
}

fn get_input_event_ids<'a>(process: &ProcessAddress) -> Cow<'a, Vec<EventAddress>> {
    get_linked_addresses_as_type(process, PROCESS_EVENT_INPUTS_LINK_TYPE, PROCESS_EVENT_INPUTS_LINK_TAG)
}

fn get_output_event_ids<'a>(process: &ProcessAddress) -> Cow<'a, Vec<EventAddress>> {
    get_linked_addresses_as_type(process, PROCESS_EVENT_OUTPUTS_LINK_TYPE, PROCESS_EVENT_OUTPUTS_LINK_TAG)
}

fn get_input_commitment_ids<'a>(process: &ProcessAddress) -> Cow<'a, Vec<CommitmentAddress>> {
    get_linked_remote_addresses_as_type(process, PROCESS_COMMITMENT_INPUTS_LINK_TYPE, PROCESS_COMMITMENT_INPUTS_LINK_TAG)
}

fn get_output_commitment_ids<'a>(process: &ProcessAddress) -> Cow<'a, Vec<CommitmentAddress>> {
    get_linked_remote_addresses_as_type(process, PROCESS_COMMITMENT_OUTPUTS_LINK_TYPE, PROCESS_COMMITMENT_OUTPUTS_LINK_TAG)
}

fn get_input_intent_ids<'a>(process: &ProcessAddress) -> Cow<'a, Vec<IntentAddress>> {
    get_linked_remote_addresses_as_type(process, PROCESS_INTENT_INPUTS_LINK_TYPE, PROCESS_INTENT_INPUTS_LINK_TAG)
}

fn get_output_intent_ids<'a>(process: &ProcessAddress) -> Cow<'a, Vec<IntentAddress>> {
    get_linked_remote_addresses_as_type(process, PROCESS_INTENT_OUTPUTS_LINK_TYPE, PROCESS_INTENT_OUTPUTS_LINK_TAG)
}
