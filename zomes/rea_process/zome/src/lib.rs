/**
 * REA `Process` zome API definition
 *
 * Defines the top-level zome configuration needed by Holochain's build system
 * to bundle the app. This basically involves wiring up the helper methods from the
 * related `_lib` module into a packaged zome WASM binary.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;
use vf_attributes_hdk::{EventAddress, ProcessAddress};
use hdk_records::{
    remote_indexes::{
        RemoteEntryLinkRequest,
        RemoteEntryLinkResponse,
        sync_remote_index,
    },
};

use hc_zome_rea_process_storage_consts::{ PROCESS_ENTRY_TYPE, PROCESS_EVENT_INPUTS_LINK_TAG, PROCESS_EVENT_OUTPUTS_LINK_TAG };
use hc_zome_rea_economic_event_storage_consts::{ EVENT_ENTRY_TYPE, EVENT_INPUT_OF_LINK_TAG, EVENT_OUTPUT_OF_LINK_TAG };

use hc_zome_rea_process_lib::*;
use hc_zome_rea_process_rpc::*;

#[hdk_extern]
fn entry_defs(_: ()) -> ExternResult<EntryDefsCallbackResult> {
    Ok(EntryDefsCallbackResult::from(vec![
        Path::entry_def(),
        EntryDef {
            id: PROCESS_ENTRY_TYPE.into(),
            visibility: EntryVisibility::Public,
            crdt_type: CrdtType,
            required_validations: 2.into(),
            required_validation_type: RequiredValidationType::default(),
        }
    ]))
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateParams {
    pub process: CreateRequest,
}

#[hdk_extern]
fn create_process(CreateParams { process }: CreateParams) -> ExternResult<ResponseData> {
    Ok(receive_create_process(PROCESS_ENTRY_TYPE, process)?)
}

#[derive(Debug, Serialize, Deserialize)]
struct ReadParams {
    pub address: ProcessAddress,
}

#[hdk_extern]
fn get_process(ReadParams { address }: ReadParams) -> ExternResult<ResponseData> {
    Ok(receive_get_process(PROCESS_ENTRY_TYPE, address)?)
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateParams {
    pub process: UpdateRequest,
}

#[hdk_extern]
fn update_process(UpdateParams { process }: UpdateParams) -> ExternResult<ResponseData> {
    Ok(receive_update_process(PROCESS_ENTRY_TYPE, process)?)
}

#[derive(Debug, Serialize, Deserialize)]
struct DeleteParams {
    pub address: RevisionHash,
}

#[hdk_extern]
fn delete_process(DeleteParams { address }: DeleteParams) -> ExternResult<bool> {
    Ok(receive_delete_process(PROCESS_ENTRY_TYPE, address)?)
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchInputs {
    pub params: QueryParams,
}

#[hdk_extern]
fn query_processes(SearchInputs { params }: SearchInputs) -> ExternResult<Vec<ResponseData>>{
    Ok(receive_query_processes(
        PROCESS_ENTRY_TYPE, EVENT_ENTRY_TYPE, COMMITMENT_ENTRY_TYPE, INTENT_ENTRY_TYPE,
        params,
    )?)
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
