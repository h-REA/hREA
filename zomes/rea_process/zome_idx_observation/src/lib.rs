/**
 * Process query indexes for observation DNA
 *
 * @package Holo-REA
 * @since   2021-08-26
 */
use hdk::prelude::*;
use hdk_records::{
    index_retrieval::IndexingZomeConfig,
    remote_indexes::{
        RemoteEntryLinkRequest,
        RemoteEntryLinkResponse,
        sync_remote_index,
    },
};

use hc_zome_rea_process_rpc::*;
use hc_zome_rea_process_lib::generate_query_handler;
use hc_zome_rea_process_storage_consts::*;
use hc_zome_rea_economic_event_storage_consts::{ EVENT_ENTRY_TYPE, EVENT_INPUT_OF_LINK_TAG, EVENT_OUTPUT_OF_LINK_TAG };
use hc_zome_rea_commitment_storage_consts::{ COMMITMENT_ENTRY_TYPE, COMMITMENT_INPUT_OF_LINK_TAG, COMMITMENT_OUTPUT_OF_LINK_TAG };
use hc_zome_rea_intent_storage_consts::{ INTENT_ENTRY_TYPE, INTENT_INPUT_OF_LINK_TAG, INTENT_OUTPUT_OF_LINK_TAG };

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

#[hdk_extern]
fn query_processes(SearchInputs { params }: SearchInputs) -> ExternResult<Vec<ResponseData>>
{
    let handler = generate_query_handler(
        read_index_target_zome,
        EVENT_ENTRY_TYPE,
        COMMITMENT_ENTRY_TYPE,
        INTENT_ENTRY_TYPE,
    );

    Ok(handler(&params)?)
}

#[hdk_extern]
fn _internal_reindex_input_events(indexes: RemoteEntryLinkRequest<EventAddress, ProcessAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_remote_index(
        &EVENT_ENTRY_TYPE, &remote_entry,
        &PROCESS_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        &EVENT_INPUT_OF_LINK_TAG, &PROCESS_EVENT_INPUTS_LINK_TAG,
    )?)
}

#[hdk_extern]
fn _internal_reindex_output_events(indexes: RemoteEntryLinkRequest<EventAddress, ProcessAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_remote_index(
        &EVENT_ENTRY_TYPE, &remote_entry,
        &PROCESS_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        &EVENT_OUTPUT_OF_LINK_TAG, &PROCESS_EVENT_OUTPUTS_LINK_TAG,
    )?)
}

#[hdk_extern]
fn index_input_commitments(indexes: RemoteEntryLinkRequest<CommitmentAddress, ProcessAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_remote_index(
        &COMMITMENT_ENTRY_TYPE, &remote_entry,
        &PROCESS_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        &COMMITMENT_INPUT_OF_LINK_TAG, &PROCESS_COMMITMENT_INPUTS_LINK_TAG,
    )?)
}

#[hdk_extern]
fn index_output_commitments(indexes: RemoteEntryLinkRequest<CommitmentAddress, ProcessAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_remote_index(
        &COMMITMENT_ENTRY_TYPE, &remote_entry,
        &PROCESS_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        &COMMITMENT_OUTPUT_OF_LINK_TAG, &PROCESS_COMMITMENT_OUTPUTS_LINK_TAG,
    )?)
}

#[hdk_extern]
fn index_input_intents(indexes: RemoteEntryLinkRequest<IntentAddress, ProcessAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_remote_index(
        &INTENT_ENTRY_TYPE, &remote_entry,
        &PROCESS_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        &INTENT_INPUT_OF_LINK_TAG, &PROCESS_INTENT_INPUTS_LINK_TAG,
    )?)
}

#[hdk_extern]
fn index_output_intents(indexes: RemoteEntryLinkRequest<IntentAddress, ProcessAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_remote_index(
        &INTENT_ENTRY_TYPE, &remote_entry,
        &PROCESS_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        &INTENT_OUTPUT_OF_LINK_TAG, &PROCESS_INTENT_OUTPUTS_LINK_TAG,
    )?)
}
