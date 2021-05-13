/**
 * Holo-REA 'process' zome library API
 *
 * Contains helper methods that can be used to manipulate `Process` data
 * structures in either the local Holochain zome, or a separate DNA-local zome.
 *
 * @package Holo-REA
 */
use hdk_records::{
    DataIntegrityError, RecordAPIResult,
    records::{
        create_record,
        read_record_entry,
        read_record_entry_by_header,
        update_record,
        delete_record,
    },
    local_indexes::{
        read_index,
        query_index,
    },
};

use vf_attributes_hdk::{
    ProcessAddress,
    EventAddress,
    CommitmentAddress,
    IntentAddress,
    AgentAddress,
};

pub use hc_zome_rea_process_storage_consts::*;
pub use hc_zome_rea_economic_event_storage_consts::{EVENT_ENTRY_TYPE};
pub use hc_zome_rea_commitment_storage_consts::{COMMITMENT_ENTRY_TYPE};
pub use hc_zome_rea_intent_storage_consts::{INTENT_ENTRY_TYPE};
use hc_zome_rea_process_storage::*;
use hc_zome_rea_process_rpc::*;

use hc_zome_rea_economic_event_storage_consts::{
    EVENT_INPUT_OF_LINK_TAG, EVENT_OUTPUT_OF_LINK_TAG,
};
use hc_zome_rea_commitment_storage_consts::{
    COMMITMENT_INPUT_OF_LINK_TAG, COMMITMENT_OUTPUT_OF_LINK_TAG,
};
use hc_zome_rea_intent_storage_consts::{
    INTENT_INPUT_OF_LINK_TAG, INTENT_OUTPUT_OF_LINK_TAG,
};

pub fn receive_create_process<S>(entry_def_id: S, process: CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    handle_create_process(entry_def_id, process)
}

pub fn receive_get_process<S>(entry_def_id: S, address: ProcessAddress) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    handle_get_process(entry_def_id, &address)
}

pub fn receive_update_process<S>(entry_def_id: S, process: UpdateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    handle_update_process(entry_def_id, process)
}

pub fn receive_delete_process<S>(entry_def_id: S, revision_id: RevisionHash) -> RecordAPIResult<bool> {
    // load the record to ensure it is of the correct type
    let (_base_address, _entry) = read_record_entry_by_header::<EntryData, EntryStorage, _>(&revision_id)?;

    delete_record::<EntryStorage, _>(&revision_id)
}

pub fn receive_query_processes<S>(entry_def_id: S, event_entry_def_id: S, commitment_entry_def_id: S, intent_entry_def_id: S, params: QueryParams) -> RecordAPIResult<Vec<ResponseData>>
    where S: AsRef<str>
{
    handle_query_processes(entry_def_id, event_entry_def_id, commitment_entry_def_id, intent_entry_def_id, &params)
}

// :TODO: move to hdk_records module

fn handle_get_process<S>(entry_def_id: S, address: &ProcessAddress) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let (revision, base_address, entry) = read_record_entry::<EntryData, EntryStorage, _,_>(&entry_def_id, address.as_ref())?;
    construct_response(&base_address, &revision, &entry, get_link_fields(&entry_def_id, &address)?)
}

fn handle_create_process<S>(entry_def_id: S, process: CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let (header_addr, base_address, entry_resp): (_,_, EntryData) = create_record(&entry_def_id, process)?;
    construct_response(&base_address, &header_addr, &entry_resp, get_link_fields(&entry_def_id, &base_address)?)
}

fn handle_update_process<S>(entry_def_id: S, process: UpdateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let address = process.get_revision_id().clone();
    let (revision_id, identity_address, entry, _prev_entry): (_,_, EntryData, EntryData) = update_record(&entry_def_id, &address, process)?;
    construct_response(&identity_address, &revision_id, &entry, get_link_fields(entry_def_id, &identity_address)?)
}

fn handle_query_processes<S>(entry_def_id: S, event_entry_def_id: S, commitment_entry_def_id: S, intent_entry_def_id: S, params: &QueryParams) -> RecordAPIResult<Vec<ResponseData>>
    where S: AsRef<str>
{
    let mut entries_result: RecordAPIResult<Vec<RecordAPIResult<(RevisionHash, ProcessAddress, EntryData)>>> = Err(DataIntegrityError::EmptyQuery);

    // :TODO: proper search logic, not mutually exclusive ID filters

    match &params.inputs {
        Some(inputs) => {
            entries_result = query_index::<EntryData, EntryStorage, _,_,_,_>(&event_entry_def_id, inputs, &EVENT_INPUT_OF_LINK_TAG);
        },
        _ => (),
    };
    match &params.outputs {
        Some(outputs) => {
            entries_result = query_index::<EntryData, EntryStorage, _,_,_,_>(&event_entry_def_id, outputs, &EVENT_OUTPUT_OF_LINK_TAG);
        },
        _ => (),
    };
    match &params.committed_inputs {
        Some(committed_inputs) => {
            entries_result = query_index::<EntryData, EntryStorage, _,_,_,_>(&commitment_entry_def_id, committed_inputs, &COMMITMENT_INPUT_OF_LINK_TAG);
        },
        _ => (),
    };
    match &params.committed_outputs {
        Some(committed_outputs) => {
            entries_result = query_index::<EntryData, EntryStorage, _,_,_,_>(&commitment_entry_def_id, committed_outputs, &COMMITMENT_OUTPUT_OF_LINK_TAG);
        },
        _ => (),
    };
    match &params.intended_inputs {
        Some(intended_inputs) => {
            entries_result = query_index::<EntryData, EntryStorage, _,_,_,_>(&intent_entry_def_id, intended_inputs, &INTENT_INPUT_OF_LINK_TAG);
        },
        _ => (),
    };
    match &params.intended_outputs {
        Some(intended_outputs) => {
            entries_result = query_index::<EntryData, EntryStorage, _,_,_,_>(&intent_entry_def_id, intended_outputs, &INTENT_OUTPUT_OF_LINK_TAG);
        },
        _ => (),
    };

    // :TODO: unplanned_economic_events, working_agents

    match entries_result {
        Ok(entries) =>
            Ok(handle_list_output(entry_def_id, entries)?.iter().cloned()
                .filter_map(Result::ok)
                .collect()
            ),
        _ => Err(DataIntegrityError::EmptyQuery)
    }
}

fn handle_list_output<S>(entry_def_id: S, entries_result: Vec<RecordAPIResult<(RevisionHash, ProcessAddress, EntryData)>>) -> RecordAPIResult<Vec<RecordAPIResult<ResponseData>>>
    where S: AsRef<str>
{
    Ok(entries_result.iter()
        .cloned()
        .filter_map(Result::ok)
        .map(|(revision_id, entry_base_address, entry)| {
            construct_response(
                &entry_base_address, &revision_id, &entry,
                get_link_fields(&entry_def_id, &entry_base_address)?
            )
        })
        .collect()
    )
}

/// Create response from input DHT primitives
pub fn construct_response<'a>(
    address: &ProcessAddress, revision_id: &RevisionHash, e: &EntryData, (
        inputs, outputs,
        unplanned_economic_events,
        committed_inputs, committed_outputs,
        intended_inputs, intended_outputs,
        next_processes, previous_processes,
        working_agents,
        trace, track
     ): (
        Vec<EventAddress>, Vec<EventAddress>,
        Vec<EventAddress>,
        Vec<CommitmentAddress>, Vec<CommitmentAddress>,
        Vec<IntentAddress>, Vec<IntentAddress>,
        Vec<ProcessAddress>, Vec<ProcessAddress>,
        Vec<AgentAddress>,
        Vec<EventAddress>, Vec<EventAddress>,
    ),
) -> RecordAPIResult<ResponseData> {
    Ok(ResponseData {
        process: Response {
            // entry fields
            id: address.to_owned(),
            revision_id: revision_id.to_owned(),
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
            inputs: inputs.to_owned(),
            outputs: outputs.to_owned(),
            unplanned_economic_events: unplanned_economic_events.to_owned(),
            committed_inputs: committed_inputs.to_owned(),
            committed_outputs: committed_outputs.to_owned(),
            intended_inputs: intended_inputs.to_owned(),
            intended_outputs: intended_outputs.to_owned(),
            next_processes: next_processes.to_owned(),
            previous_processes: previous_processes.to_owned(),
            working_agents: working_agents.to_owned(),
            trace: trace.to_owned(),
            track: track.to_owned(),
        }
    })
}

//---------------- READ ----------------

// @see construct_response
pub fn get_link_fields<'a, S>(entry_def_id: S, process: &ProcessAddress) -> RecordAPIResult<(
    Vec<EventAddress>,
    Vec<EventAddress>,
    Vec<EventAddress>,
    Vec<CommitmentAddress>,
    Vec<CommitmentAddress>,
    Vec<IntentAddress>,
    Vec<IntentAddress>,
    Vec<ProcessAddress>,
    Vec<ProcessAddress>,
    Vec<AgentAddress>,
    Vec<EventAddress>,
    Vec<EventAddress>,
)>
    where S: AsRef<str>,
{
    Ok((
        read_index(&entry_def_id, process, &PROCESS_EVENT_INPUTS_LINK_TAG)?,
        read_index(&entry_def_id, process, &PROCESS_EVENT_OUTPUTS_LINK_TAG)?,
        vec![],  // :TODO: unplanned_economic_events
        read_index(&entry_def_id, process, &PROCESS_COMMITMENT_INPUTS_LINK_TAG)?,
        read_index(&entry_def_id, process, &PROCESS_COMMITMENT_OUTPUTS_LINK_TAG)?,
        read_index(&entry_def_id, process, &PROCESS_INTENT_INPUTS_LINK_TAG)?,
        read_index(&entry_def_id, process, &PROCESS_INTENT_OUTPUTS_LINK_TAG)?,
        vec![], // :TODO: next_processes
        vec![], // :TODO: previous_processes
        vec![], // :TODO: working_agents
        vec![], // :TODO: trace
        vec![], // :TODO: track
    ))
}
