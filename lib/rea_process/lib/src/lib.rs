/**
 * Holo-REA 'process' zome library API
 *
 * Contains helper methods that can be used to manipulate `Process` data
 * structures in either the local Holochain zome, or a separate DNA-local zome.
 *
 * @package Holo-REA
 */
use std::borrow::Cow;
use hdk::prelude::*;

use hdk_records::{
    records::{
        create_record,
        read_record_entry,
        update_record,
        delete_record,
    },
    links::{
        // get_linked_addresses_as_type,
        // get_linked_addresses_with_foreign_key_as_type,
    },
    local_indexes::{
        // query_direct_index_with_foreign_key,
        // query_direct_remote_index_with_foreign_key,
    },
    // remote_indexes::{
    //     RemoteEntryLinkResponse,
    //     handle_sync_direct_remote_index_destination,
    // },
};

use vf_core::type_aliases::{
    ProcessAddress,
    EventAddress,
    CommitmentAddress,
    IntentAddress,
    AgentAddress,
};

use hc_zome_rea_process_storage_consts::*;
use hc_zome_rea_process_storage::*;
use hc_zome_rea_process_rpc::*;

use hc_zome_rea_economic_event_storage_consts::{
    EVENT_INPUT_OF_LINK_TYPE, EVENT_INPUT_OF_LINK_TAG,
    EVENT_OUTPUT_OF_LINK_TYPE, EVENT_OUTPUT_OF_LINK_TAG,
};
use hc_zome_rea_commitment_storage_consts::{
    COMMITMENT_ENTRY_TYPE,
    COMMITMENT_INPUT_OF_LINK_TAG, COMMITMENT_OUTPUT_OF_LINK_TAG,
};
use hc_zome_rea_intent_storage_consts::{
    INTENT_ENTRY_TYPE,
    INTENT_INPUT_OF_LINK_TAG, INTENT_OUTPUT_OF_LINK_TAG,
};

pub fn receive_create_process(process: CreateRequest) -> ZomeApiResult<ResponseData> {
    handle_create_process(&process)
}

pub fn receive_get_process(address: ProcessAddress) -> ZomeApiResult<ResponseData> {
    handle_get_process(&address)
}

pub fn receive_update_process(process: UpdateRequest) -> ZomeApiResult<ResponseData> {
    handle_update_process(&process)
}

pub fn receive_delete_process(address: ProcessAddress) -> ZomeApiResult<bool> {
    delete_record::<Entry>(&address)
}

pub fn receive_query_processes(params: QueryParams) -> ZomeApiResult<Vec<ResponseData>> {
    handle_query_processes(&params)
}

// :TODO: move to hdk_records module

fn handle_get_process(address: &ProcessAddress) -> ZomeApiResult<ResponseData> {
    Ok(construct_response(address, &read_record_entry(address)?, get_link_fields(address)))
}

fn handle_create_process(process: &CreateRequest) -> ZomeApiResult<ResponseData> {
    let (base_address, entry_resp): (ProcessAddress, Entry) = create_record(
        PROCESS_BASE_ENTRY_TYPE, PROCESS_ENTRY_TYPE,
        PROCESS_INITIAL_ENTRY_LINK_TYPE,
        process.to_owned(),
    )?;
    Ok(construct_response(&base_address, &entry_resp, get_link_fields(&base_address)))
}

fn handle_update_process(process: &UpdateRequest) -> ZomeApiResult<ResponseData> {
    let base_address = process.get_id();
    let new_entry = update_record(PROCESS_ENTRY_TYPE, base_address, process)?;
    Ok(construct_response(&base_address, &new_entry, get_link_fields(base_address)))
}

fn handle_query_processes(params: &QueryParams) -> ZomeApiResult<Vec<ResponseData>> {
    let mut entries_result: ZomeApiResult<Vec<(ProcessAddress, Option<Entry>)>> = Err(ZomeApiError::Internal("No results found".to_string()));

    // :TODO: proper search logic, not mutually exclusive ID filters

    match &params.inputs {
        Some(inputs) => {
            entries_result = query_direct_index_with_foreign_key(inputs, EVENT_INPUT_OF_LINK_TYPE, EVENT_INPUT_OF_LINK_TAG);
        },
        _ => (),
    };
    match &params.outputs {
        Some(outputs) => {
            entries_result = query_direct_index_with_foreign_key(outputs, EVENT_OUTPUT_OF_LINK_TYPE, EVENT_OUTPUT_OF_LINK_TAG);
        },
        _ => (),
    };
    match &params.committed_inputs {
        Some(committed_inputs) => {
            entries_result = query_direct_remote_index_with_foreign_key(
                committed_inputs, COMMITMENT_BASE_ENTRY_TYPE,
                COMMITMENT_INPUT_OF_LINK_TYPE, COMMITMENT_INPUT_OF_LINK_TAG,
            );
        },
        _ => (),
    };
    match &params.committed_outputs {
        Some(committed_outputs) => {
            entries_result = query_direct_remote_index_with_foreign_key(
                committed_outputs, COMMITMENT_BASE_ENTRY_TYPE,
                COMMITMENT_OUTPUT_OF_LINK_TYPE, COMMITMENT_OUTPUT_OF_LINK_TAG,
            );
        },
        _ => (),
    };
    match &params.intended_inputs {
        Some(intended_inputs) => {
            entries_result = query_direct_remote_index_with_foreign_key(
                intended_inputs, INTENT_BASE_ENTRY_TYPE,
                INTENT_INPUT_OF_LINK_TYPE, INTENT_INPUT_OF_LINK_TAG,
            );
        },
        _ => (),
    };
    match &params.intended_outputs {
        Some(intended_outputs) => {
            entries_result = query_direct_remote_index_with_foreign_key(
                intended_outputs, INTENT_BASE_ENTRY_TYPE,
                INTENT_OUTPUT_OF_LINK_TYPE, INTENT_OUTPUT_OF_LINK_TAG,
            );
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

/// Create response from input DHT primitives
pub fn construct_response<'a>(
    address: &ProcessAddress, e: &Entry, (
        inputs, outputs,
        unplanned_economic_events,
        committed_inputs, committed_outputs,
        intended_inputs, intended_outputs,
        next_processes, previous_processes,
        working_agents,
        trace, track
     ): (
        Option<Cow<'a, Vec<EventAddress>>>, Option<Cow<'a, Vec<EventAddress>>>,
        Option<Cow<'a, Vec<EventAddress>>>,
        Option<Cow<'a, Vec<CommitmentAddress>>>, Option<Cow<'a, Vec<CommitmentAddress>>>,
        Option<Cow<'a, Vec<IntentAddress>>>, Option<Cow<'a, Vec<IntentAddress>>>,
        Option<Cow<'a, Vec<ProcessAddress>>>, Option<Cow<'a, Vec<ProcessAddress>>>,
        Option<Cow<'a, Vec<AgentAddress>>>,
        Option<Cow<'a, Vec<EventAddress>>>, Option<Cow<'a, Vec<EventAddress>>>,
    ),
) -> ResponseData {
    ResponseData {
        process: Response {
            // entry fields
            id: address.to_owned(),
            name: e.name.to_owned(),
            has_beginning: e.has_beginning.to_owned(),
            has_end: e.has_end.to_owned(),
            before: e.before.to_owned(),
            after: e.after.to_owned(),
            classified_as: e.classified_as.to_owned(),
            based_on: e.based_on.to_owned(),
            planned_within: e.planned_within.to_owned(),
            note: e.note.to_owned(),
            in_scope_of: e.in_scope_of.to_owned(),
            finished: e.finished.to_owned(),
            deletable: true,    // :TODO:

            // link fields
            inputs: inputs.map(Cow::into_owned),
            outputs: outputs.map(Cow::into_owned),
            unplanned_economic_events: unplanned_economic_events.map(Cow::into_owned),
            committed_inputs: committed_inputs.map(Cow::into_owned),
            committed_outputs: committed_outputs.map(Cow::into_owned),
            intended_inputs: intended_inputs.map(Cow::into_owned),
            intended_outputs: intended_outputs.map(Cow::into_owned),
            next_processes: next_processes.map(Cow::into_owned),
            previous_processes: previous_processes.map(Cow::into_owned),
            working_agents: working_agents.map(Cow::into_owned),
            trace: trace.map(Cow::into_owned),
            track: track.map(Cow::into_owned),
        }
    }
}

//---------------- READ ----------------

// @see construct_response
pub fn get_link_fields<'a>(process: &ProcessAddress) -> (
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
    get_linked_addresses_with_foreign_key_as_type(process, PROCESS_COMMITMENT_INPUTS_LINK_TYPE, PROCESS_COMMITMENT_INPUTS_LINK_TAG)
}

fn get_output_commitment_ids<'a>(process: &ProcessAddress) -> Cow<'a, Vec<CommitmentAddress>> {
    get_linked_addresses_with_foreign_key_as_type(process, PROCESS_COMMITMENT_OUTPUTS_LINK_TYPE, PROCESS_COMMITMENT_OUTPUTS_LINK_TAG)
}

fn get_input_intent_ids<'a>(process: &ProcessAddress) -> Cow<'a, Vec<IntentAddress>> {
    get_linked_addresses_with_foreign_key_as_type(process, PROCESS_INTENT_INPUTS_LINK_TYPE, PROCESS_INTENT_INPUTS_LINK_TAG)
}

fn get_output_intent_ids<'a>(process: &ProcessAddress) -> Cow<'a, Vec<IntentAddress>> {
    get_linked_addresses_with_foreign_key_as_type(process, PROCESS_INTENT_OUTPUTS_LINK_TYPE, PROCESS_INTENT_OUTPUTS_LINK_TAG)
}
