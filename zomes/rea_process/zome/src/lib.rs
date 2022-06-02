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

use hc_zome_rea_process_storage_consts::*;
use hc_zome_rea_process_lib::*;
use hc_zome_rea_process_rpc::*;

#[hdk_extern]
fn entry_defs(_: ()) -> ExternResult<EntryDefsCallbackResult> {
    Ok(EntryDefsCallbackResult::from(vec![
        PathEntry::entry_def(),
        ProcessAddress::entry_def(),
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
    Ok(handle_create_process(PROCESS_ENTRY_TYPE, process)?)
}

#[derive(Debug, Serialize, Deserialize)]
struct ReadParams {
    pub address: ProcessAddress,
}

#[hdk_extern]
fn get_process(ReadParams { address }: ReadParams) -> ExternResult<ResponseData> {
    Ok(handle_get_process(PROCESS_ENTRY_TYPE, address)?)
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateParams {
    pub process: UpdateRequest,
}

#[hdk_extern]
fn update_process(UpdateParams { process }: UpdateParams) -> ExternResult<ResponseData> {
    Ok(handle_update_process(PROCESS_ENTRY_TYPE, process)?)
}

#[hdk_extern]
fn delete_process(ByRevision { revision_id }: ByRevision) -> ExternResult<bool> {
    Ok(handle_delete_process(PROCESS_ENTRY_TYPE, revision_id)?)
}
