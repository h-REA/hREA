/**
 * Holo-REA 'process' zome library API
 *
 * Contains helper methods that can be used to manipulate `Process` data
 * structures in either the local Holochain zome, or a separate DNA-local zome.
 *
 * @package Holo-REA
 */
use hdk_records::{
    RecordAPIResult,
    records::{
        create_record,
        read_record_entry,
        read_record_entry_by_header,
        update_record,
        delete_record,
    },
};
use hdk_semantic_indexes_client_lib::{
    read_local_index,
};

use vf_attributes_hdk::{
    ProcessAddress,
    EventAddress,
    CommitmentAddress,
    IntentAddress,
    AgentAddress,
};

pub use hc_zome_rea_process_storage_consts::*;
use hc_zome_rea_process_storage::*;
use hc_zome_rea_process_rpc::*;

pub fn handle_create_process<S>(entry_def_id: S, process: CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let (header_addr, base_address, entry_resp): (_,_, EntryData) = create_record(&entry_def_id, process)?;
    construct_response(&base_address, &header_addr, &entry_resp, get_link_fields(&base_address)?)
}

pub fn handle_get_process<S>(entry_def_id: S, address: ProcessAddress) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let (revision, base_address, entry) = read_record_entry::<EntryData, EntryStorage, _,_>(&entry_def_id, address.as_ref())?;
    construct_response(&base_address, &revision, &entry, get_link_fields(&address)?)
}

pub fn handle_update_process<S>(entry_def_id: S, process: UpdateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let address = process.get_revision_id().clone();
    let (revision_id, identity_address, entry, _prev_entry): (_,_, EntryData, EntryData) = update_record(&entry_def_id, &address, process)?;
    construct_response(&identity_address, &revision_id, &entry, get_link_fields(&identity_address)?)
}

pub fn handle_delete_process<S>(_entry_def_id: S, revision_id: RevisionHash) -> RecordAPIResult<bool>
{
    // load the record to ensure it is of the correct type
    let (_base_address, _entry) = read_record_entry_by_header::<EntryData, EntryStorage, _>(&revision_id)?;

    delete_record::<EntryStorage, _>(&revision_id)
}

/// Create response from input DHT primitives
fn construct_response<'a>(
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

/// Properties accessor for zome config.
fn read_foreign_index_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.process.index_zome)
}

// @see construct_response
fn get_link_fields(process: &ProcessAddress) -> RecordAPIResult<(
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
)> {
    Ok((
        read_local_index(read_foreign_index_zome, &PROCESS_EVENT_INPUTS_READ_API_METHOD, process)?,
        read_local_index(read_foreign_index_zome, &PROCESS_EVENT_OUTPUTS_READ_API_METHOD, process)?,
        vec![],  // :TODO: unplanned_economic_events
        read_local_index(read_foreign_index_zome, &PROCESS_COMMITMENT_INPUTS_READ_API_METHOD, process)?,
        read_local_index(read_foreign_index_zome, &PROCESS_COMMITMENT_OUTPUTS_READ_API_METHOD, process)?,
        read_local_index(read_foreign_index_zome, &PROCESS_INTENT_INPUTS_READ_API_METHOD, process)?,
        read_local_index(read_foreign_index_zome, &PROCESS_INTENT_OUTPUTS_READ_API_METHOD, process)?,
        vec![], // :TODO: next_processes
        vec![], // :TODO: previous_processes
        vec![], // :TODO: working_agents
        vec![], // :TODO: trace
        vec![], // :TODO: track
    ))
}
