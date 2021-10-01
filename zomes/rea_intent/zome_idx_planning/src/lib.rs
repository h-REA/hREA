/**
 * Intent query indexes for planning DNA
 *
 * @package Holo-REA
 * @since   2021-08-29
 */
use hdk::prelude::*;
use hdk_semantic_indexes_zome_lib::{
    ByAddress,
    IndexingZomeConfig,
    RemoteEntryLinkRequest,
    RemoteEntryLinkResponse,
    read_index,
    sync_index,
};

use hc_zome_rea_intent_rpc::*;
use hc_zome_rea_intent_lib::generate_query_handler;
use hc_zome_rea_intent_storage_consts::*;
use hc_zome_rea_satisfaction_storage_consts::{ SATISFACTION_ENTRY_TYPE, SATISFACTION_SATISFIES_LINK_TAG };
use hc_zome_rea_process_storage_consts::{ PROCESS_ENTRY_TYPE, PROCESS_INTENT_INPUTS_LINK_TAG, PROCESS_INTENT_OUTPUTS_LINK_TAG };
use hc_zome_rea_proposed_intent_storage_consts::{ PROPOSED_INTENT_ENTRY_TYPE, PROPOSED_INTENT_PUBLISHES_LINK_TAG, INTENT_PUBLISHEDIN_INDEXING_API_METHOD };

entry_defs![Path::entry_def()];

// :TODO: obviate this with zome-specific configs
#[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
pub struct DnaConfigSlice {
    pub intent_index: IndexingZomeConfig,
}

fn read_index_target_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.intent_index.record_storage_zome)
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchInputs {
    pub params: QueryParams,
}

#[hdk_extern]
fn query_intents(SearchInputs { params }: SearchInputs) -> ExternResult<Vec<ResponseData>>
{
    let handler = generate_query_handler(
        read_index_target_zome,
        SATISFACTION_ENTRY_TYPE,
        PROCESS_ENTRY_TYPE,
        PROPOSED_INTENT_ENTRY_TYPE,
    );

    Ok(handler(&params)?)
}

#[hdk_extern]
fn _internal_read_intent_process_inputs(ByAddress { address }: ByAddress<IntentAddress>) -> ExternResult<Vec<ProcessAddress>> {
    Ok(read_index(&INTENT_ENTRY_TYPE, &address, &INTENT_INPUT_OF_LINK_TAG)?)
}

#[hdk_extern]
fn _internal_reindex_process_inputs(indexes: RemoteEntryLinkRequest<ProcessAddress, IntentAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_index(
        &PROCESS_ENTRY_TYPE, &remote_entry,
        &INTENT_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        PROCESS_INTENT_INPUTS_LINK_TAG, INTENT_INPUT_OF_LINK_TAG,
    )?)
}

#[hdk_extern]
fn _internal_read_intent_process_outputs(ByAddress { address }: ByAddress<IntentAddress>) -> ExternResult<Vec<ProcessAddress>> {
    Ok(read_index(&INTENT_ENTRY_TYPE, &address, &INTENT_OUTPUT_OF_LINK_TAG)?)
}

#[hdk_extern]
fn _internal_reindex_process_outputs(indexes: RemoteEntryLinkRequest<ProcessAddress, IntentAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_index(
        &PROCESS_ENTRY_TYPE, &remote_entry,
        &INTENT_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        PROCESS_INTENT_OUTPUTS_LINK_TAG, INTENT_OUTPUT_OF_LINK_TAG,
    )?)
}

#[hdk_extern]
fn _internal_read_intent_satisfactions(ByAddress { address }: ByAddress<IntentAddress>) -> ExternResult<Vec<SatisfactionAddress>> {
    Ok(read_index(&INTENT_ENTRY_TYPE, &address, &INTENT_SATISFIEDBY_LINK_TAG)?)
}

#[hdk_extern]
fn _internal_reindex_satisfactions(indexes: RemoteEntryLinkRequest<SatisfactionAddress, IntentAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_index(
        &SATISFACTION_ENTRY_TYPE, &remote_entry,
        &INTENT_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        &SATISFACTION_SATISFIES_LINK_TAG, &INTENT_SATISFIEDBY_LINK_TAG,
    )?)
}

#[hdk_extern]
fn read_intent_published_in(ByAddress { address }: ByAddress<IntentAddress>) -> ExternResult<Vec<ProposedIntentAddress>> {
    Ok(read_index(&INTENT_ENTRY_TYPE, &address, &INTENT_SATISFIEDBY_LINK_TAG)?)
}

#[hdk_extern]
fn index_intent_proposed_in(indexes: RemoteEntryLinkRequest<ProposedIntentAddress, IntentAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_index(
        &PROPOSED_INTENT_ENTRY_TYPE, &remote_entry,
        &INTENT_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        &PROPOSED_INTENT_PUBLISHES_LINK_TAG, &INTENT_PUBLISHEDIN_INDEXING_API_METHOD,
    )?)
}
