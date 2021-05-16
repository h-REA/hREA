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
