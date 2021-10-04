/**
 * Process query indexes for observation DNA
 *
 * @package Holo-REA
 * @since   2021-08-26
 */
use hdk::prelude::*;
use hdk_semantic_indexes_zome_lib::{
    ByAddress, RecordAPIResult, DataIntegrityError,
    IndexingZomeConfig,
    RemoteEntryLinkRequest,
    RemoteEntryLinkResponse,
    read_index,
    query_index,
    sync_index,
};

use hc_zome_rea_process_rpc::*;
use hc_zome_rea_process_storage_consts::*;
use hc_zome_rea_economic_event_storage_consts::{ EVENT_ENTRY_TYPE, EVENT_INPUT_OF_LINK_TAG, EVENT_OUTPUT_OF_LINK_TAG };
use hc_zome_rea_commitment_storage_consts::{ COMMITMENT_ENTRY_TYPE, COMMITMENT_INPUT_OF_LINK_TAG, COMMITMENT_OUTPUT_OF_LINK_TAG };
use hc_zome_rea_intent_storage_consts::{ INTENT_ENTRY_TYPE, INTENT_INPUT_OF_LINK_TAG, INTENT_OUTPUT_OF_LINK_TAG };

entry_defs![Path::entry_def()];

// :TODO: obviate this with zome-specific configs
#[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
pub struct DnaConfigSlice {
    pub process_index: IndexingZomeConfig,
}

fn read_index_target_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.process_index.record_storage_zome)
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchInputs {
    pub params: QueryParams,
}

const READ_FN_NAME: &str = "get_process";

#[hdk_extern]
fn query_processes(SearchInputs { params }: SearchInputs) -> ExternResult<Vec<ResponseData>>
{
    let mut entries_result: RecordAPIResult<Vec<RecordAPIResult<ResponseData>>> = Err(DataIntegrityError::EmptyQuery);

    // :TODO: proper search logic, not mutually exclusive ID filters

    match &params.inputs {
        Some(inputs) => {
            entries_result = query_index::<ResponseData, ProcessAddress, _,_,_,_,_,_>(&EVENT_ENTRY_TYPE, inputs, &EVENT_INPUT_OF_LINK_TAG, &read_index_target_zome, &READ_FN_NAME);
        },
        _ => (),
    };
    match &params.outputs {
        Some(outputs) => {
            entries_result = query_index::<ResponseData, ProcessAddress, _,_,_,_,_,_>(&EVENT_ENTRY_TYPE, outputs, &EVENT_OUTPUT_OF_LINK_TAG, &read_index_target_zome, &READ_FN_NAME);
        },
        _ => (),
    };
    match &params.committed_inputs {
        Some(committed_inputs) => {
            entries_result = query_index::<ResponseData, ProcessAddress, _,_,_,_,_,_>(&COMMITMENT_ENTRY_TYPE, committed_inputs, &COMMITMENT_INPUT_OF_LINK_TAG, &read_index_target_zome, &READ_FN_NAME);
        },
        _ => (),
    };
    match &params.committed_outputs {
        Some(committed_outputs) => {
            entries_result = query_index::<ResponseData, ProcessAddress, _,_,_,_,_,_>(&COMMITMENT_ENTRY_TYPE, committed_outputs, &COMMITMENT_OUTPUT_OF_LINK_TAG, &read_index_target_zome, &READ_FN_NAME);
        },
        _ => (),
    };
    match &params.intended_inputs {
        Some(intended_inputs) => {
            entries_result = query_index::<ResponseData, ProcessAddress, _,_,_,_,_,_>(&INTENT_ENTRY_TYPE, intended_inputs, &INTENT_INPUT_OF_LINK_TAG, &read_index_target_zome, &READ_FN_NAME);
        },
        _ => (),
    };
    match &params.intended_outputs {
        Some(intended_outputs) => {
            entries_result = query_index::<ResponseData, ProcessAddress, _,_,_,_,_,_>(&INTENT_ENTRY_TYPE, intended_outputs, &INTENT_OUTPUT_OF_LINK_TAG, &read_index_target_zome, &READ_FN_NAME);
        },
        _ => (),
    };

    // :TODO: unplanned_economic_events, working_agents

    // :TODO: return errors for UI, rather than filtering
    Ok(entries_result?.iter()
        .cloned()
        .filter_map(Result::ok)
        .collect())
}

#[hdk_extern]
fn _internal_read_process_inputs(ByAddress { address }: ByAddress<ProcessAddress>) -> ExternResult<Vec<EconomicEventAddress>> {
    Ok(read_index(&PROCESS_ENTRY_TYPE, &address, &PROCESS_EVENT_INPUTS_LINK_TAG)?)
}

#[hdk_extern]
fn _internal_reindex_input_events(indexes: RemoteEntryLinkRequest<EconomicEventAddress, ProcessAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_index(
        &EVENT_ENTRY_TYPE, &remote_entry,
        &PROCESS_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        &EVENT_INPUT_OF_LINK_TAG, &PROCESS_EVENT_INPUTS_LINK_TAG,
    )?)
}

#[hdk_extern]
fn _internal_read_process_outputs(ByAddress { address }: ByAddress<ProcessAddress>) -> ExternResult<Vec<EconomicEventAddress>> {
    Ok(read_index(&PROCESS_ENTRY_TYPE, &address, &PROCESS_EVENT_OUTPUTS_LINK_TAG)?)
}

#[hdk_extern]
fn _internal_reindex_output_events(indexes: RemoteEntryLinkRequest<EconomicEventAddress, ProcessAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_index(
        &EVENT_ENTRY_TYPE, &remote_entry,
        &PROCESS_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        &EVENT_OUTPUT_OF_LINK_TAG, &PROCESS_EVENT_OUTPUTS_LINK_TAG,
    )?)
}

#[hdk_extern]
fn _internal_read_process_committed_inputs(ByAddress { address }: ByAddress<ProcessAddress>) -> ExternResult<Vec<CommitmentAddress>> {
    Ok(read_index(&PROCESS_ENTRY_TYPE, &address, &PROCESS_COMMITMENT_INPUTS_LINK_TAG)?)
}

#[hdk_extern]
fn index_input_commitments(indexes: RemoteEntryLinkRequest<CommitmentAddress, ProcessAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_index(
        &COMMITMENT_ENTRY_TYPE, &remote_entry,
        &PROCESS_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        &COMMITMENT_INPUT_OF_LINK_TAG, &PROCESS_COMMITMENT_INPUTS_LINK_TAG,
    )?)
}

#[hdk_extern]
fn _internal_read_process_committed_outputs(ByAddress { address }: ByAddress<ProcessAddress>) -> ExternResult<Vec<CommitmentAddress>> {
    Ok(read_index(&PROCESS_ENTRY_TYPE, &address, &PROCESS_COMMITMENT_OUTPUTS_LINK_TAG)?)
}

#[hdk_extern]
fn index_output_commitments(indexes: RemoteEntryLinkRequest<CommitmentAddress, ProcessAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_index(
        &COMMITMENT_ENTRY_TYPE, &remote_entry,
        &PROCESS_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        &COMMITMENT_OUTPUT_OF_LINK_TAG, &PROCESS_COMMITMENT_OUTPUTS_LINK_TAG,
    )?)
}

#[hdk_extern]
fn _internal_read_process_intended_inputs(ByAddress { address }: ByAddress<ProcessAddress>) -> ExternResult<Vec<IntentAddress>> {
    Ok(read_index(&PROCESS_ENTRY_TYPE, &address, &PROCESS_INTENT_INPUTS_LINK_TAG)?)
}

#[hdk_extern]
fn index_input_intents(indexes: RemoteEntryLinkRequest<IntentAddress, ProcessAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_index(
        &INTENT_ENTRY_TYPE, &remote_entry,
        &PROCESS_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        &INTENT_INPUT_OF_LINK_TAG, &PROCESS_INTENT_INPUTS_LINK_TAG,
    )?)
}

#[hdk_extern]
fn _internal_read_process_intended_outputs(ByAddress { address }: ByAddress<ProcessAddress>) -> ExternResult<Vec<IntentAddress>> {
    Ok(read_index(&PROCESS_ENTRY_TYPE, &address, &PROCESS_INTENT_OUTPUTS_LINK_TAG)?)
}

#[hdk_extern]
fn index_output_intents(indexes: RemoteEntryLinkRequest<IntentAddress, ProcessAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_index(
        &INTENT_ENTRY_TYPE, &remote_entry,
        &PROCESS_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        &INTENT_OUTPUT_OF_LINK_TAG, &PROCESS_INTENT_OUTPUTS_LINK_TAG,
    )?)
}
