/**
 * Holo-REA process specification zome API definition
 *
 * Defines the top-level zome configuration needed by Holochain's build system
 * to bundle the app. This basically involves wiring up the helper methods from the
 * related `_lib` module into a packaged zome WASM binary.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use hc_zome_rea_process_specification_rpc::*;
use hc_zome_rea_process_specification_lib::*;
use hc_zome_rea_process_specification_storage_consts::*;

#[hdk_extern]
fn entry_defs(_: ()) -> ExternResult<EntryDefsCallbackResult> {
    Ok(EntryDefsCallbackResult::from(vec![
        Path::entry_def(),
        EntryDef {
            id: PROCESS_SPECIFICATION_ENTRY_TYPE.into(),
            visibility: EntryVisibility::Public,
            crdt_type: CrdtType,
            required_validations: 2.into(),
            required_validation_type: RequiredValidationType::default(),
        }
    ]))
}

#[hdk_extern]
fn create_process_specification(CreateParams { process_specification }: CreateParams) -> ExternResult<ResponseData> {
    Ok(receive_create_process_specification(PROCESS_SPECIFICATION_ENTRY_TYPE, process_specification)?)
}

#[hdk_extern]
fn get_process_specification(ByAddress { address }: ByAddress<ProcessSpecificationAddress>) -> ExternResult<ResponseData> {
    Ok(receive_get_process_specification(PROCESS_SPECIFICATION_ENTRY_TYPE, address)?)
}

#[hdk_extern]
fn update_process_specification(UpdateParams { process_specification }: UpdateParams) -> ExternResult<ResponseData> {
    Ok(receive_update_process_specification(PROCESS_SPECIFICATION_ENTRY_TYPE, process_specification)?)
}

#[hdk_extern]
fn delete_process_specification(ByHeader { address }: ByHeader) -> ExternResult<bool> {
    Ok(receive_delete_process_specification(address)?)
}
