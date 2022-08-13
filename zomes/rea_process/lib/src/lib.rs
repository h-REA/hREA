/**
 * Holo-REA 'process' zome library API
 *
 * Contains helper methods that can be used to manipulate `Process` data
 * structures in either the local Holochain zome, or a separate DNA-local zome.
 *
 * @package Holo-REA
 */
use paste::paste;
use hdk_records::{
    RecordAPIResult,
    records::{
        create_record,
        read_record_entry,
        read_record_entry_by_action,
        update_record,
        delete_record,
    },
    metadata::read_revision_metadata_abbreviated,
    MaybeUndefined, SignedActionHashed,
};
use hdk_semantic_indexes_client_lib::*;

use hc_zome_rea_process_storage::*;
use hc_zome_rea_process_rpc::*;


/// properties accessor for zome config
fn read_index_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.process.index_zome)
}

pub fn handle_create_process<S>(entry_def_id: S, process: CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str> + std::fmt::Display,
{
    let (meta, base_address, entry_resp): (_,_, EntryData) = create_record::<EntryTypes,_,_,_,_,_,_,_,_>(read_index_zome, &entry_def_id, process.to_owned())?;

    // handle link fields
    // :TODO: propogate errors
    if let CreateRequest { planned_within: MaybeUndefined::Some(planned_within), .. } = &process {
        let e = create_index!(process.planned_within(planned_within), plan.processes(&base_address));
        hdk::prelude::debug!("handle_create_process::planned_within index {:?}", e);
    };

    // :TODO: pass results from link creation rather than re-reading
    construct_response(&base_address, &meta, &entry_resp, get_link_fields(&base_address)?)
}

pub fn handle_get_process(address: ProcessAddress) -> RecordAPIResult<ResponseData>
{
    let (meta, base_address, entry) = read_record_entry::<EntryData, EntryStorage, _>(address.as_ref())?;
    construct_response(&base_address, &meta, &entry, get_link_fields(&address)?)
}

pub fn handle_update_process(process: UpdateRequest) -> RecordAPIResult<ResponseData>
{
    let address = process.get_revision_id().clone();
    let (meta, base_address, new_entry, prev_entry): (_,_, EntryData, EntryData) = update_record(&address, process)?;

    // handle link fields
    if new_entry.planned_within != prev_entry.planned_within {
        let new_value = match &new_entry.planned_within { Some(val) => vec![val.to_owned()], None => vec![] };
        let prev_value = match &prev_entry.planned_within { Some(val) => vec![val.to_owned()], None => vec![] };
        let e = update_index!(
            process
                .planned_within(new_value.as_slice())
                .not(prev_value.as_slice()),
            plan.processes(&base_address)
        );
        hdk::prelude::debug!("handle_update_process::planned_within index {:?}", e);
    }
    construct_response(&base_address, &meta, &new_entry, get_link_fields(&base_address)?)
}

pub fn handle_delete_process(revision_id: ActionHash) -> RecordAPIResult<bool>
{
    // load the record to ensure it is of the correct type
    let (_meta, base_address, entry) = read_record_entry_by_action::<EntryData, EntryStorage, _>(&revision_id)?;

    // handle link fields
    if let Some(plan_address) = entry.planned_within {
        let e = update_index!(process.planned_within.not(&vec![plan_address]), plan.processes(&base_address));
        hdk::prelude::debug!("handle_delete_process::planned_within index {:?}", e);
    }

    delete_record::<EntryStorage>(&revision_id)
}

/// Create response from input DHT primitives
fn construct_response<'a>(
    address: &ProcessAddress, meta: &SignedActionHashed, e: &EntryData, (
        observed_inputs, observed_outputs,
        unplanned_economic_events,
        committed_inputs, committed_outputs,
        intended_inputs, intended_outputs,
        next_processes, previous_processes,
        working_agents,
        trace, track,
     ): (
        Vec<EconomicEventAddress>, Vec<EconomicEventAddress>,
        Vec<EconomicEventAddress>,
        Vec<CommitmentAddress>, Vec<CommitmentAddress>,
        Vec<IntentAddress>, Vec<IntentAddress>,
        Vec<ProcessAddress>, Vec<ProcessAddress>,
        Vec<AgentAddress>,
        Vec<EconomicEventAddress>, Vec<EconomicEventAddress>,
    ),
) -> RecordAPIResult<ResponseData> {
    Ok(ResponseData {
        process: Response {
            // entry fields
            id: address.to_owned(),
            revision_id: meta.as_hash().to_owned(),
            meta: read_revision_metadata_abbreviated(meta)?,
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
            observed_inputs: observed_inputs.to_owned(),
            observed_outputs: observed_outputs.to_owned(),
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
fn read_process_index_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.process.index_zome)
}
/// Properties accessor for zome config
fn read_plan_index_zome(conf: DnaConfigSlice) -> Option<String> {
    conf.process.plan_index_zome
}

// @see construct_response
fn get_link_fields(process: &ProcessAddress) -> RecordAPIResult<(
    Vec<EconomicEventAddress>,
    Vec<EconomicEventAddress>,
    Vec<EconomicEventAddress>,
    Vec<CommitmentAddress>,
    Vec<CommitmentAddress>,
    Vec<IntentAddress>,
    Vec<IntentAddress>,
    Vec<ProcessAddress>,
    Vec<ProcessAddress>,
    Vec<AgentAddress>,
    Vec<EconomicEventAddress>,
    Vec<EconomicEventAddress>,
)> {
    Ok((
        read_index!(process(process).observed_inputs)?,
        read_index!(process(process).observed_outputs)?,
        vec![],  // :TODO: unplanned_economic_events
        read_index!(process(process).committed_inputs)?,
        read_index!(process(process).committed_outputs)?,
        read_index!(process(process).intended_inputs)?,
        read_index!(process(process).intended_outputs)?,
        vec![], // :TODO: next_processes
        vec![], // :TODO: previous_processes
        vec![], // :TODO: working_agents
        vec![], // :TODO: trace
        vec![], // :TODO: track
    ))
}
