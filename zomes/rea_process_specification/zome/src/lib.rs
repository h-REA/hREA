/**
 * hREA process specification zome API definition
 *
 * Defines the top-level zome configuration needed by Holochain's build system
 * to bundle the app. This basically involves wiring up the helper methods from the
 * related `_lib` module into a packaged zome WASM binary.
 *
 * @package hREA
 */
use hdk::prelude::*;

use hc_zome_rea_process_specification_rpc::*;
use hc_zome_rea_process_specification_lib::*;
use hc_zome_rea_process_specification_storage_consts::*;

#[hdk_extern]
fn create_process_specification(CreateParams { process_specification }: CreateParams) -> ExternResult<ResponseData> {
    Ok(handle_create_process_specification(PROCESS_SPECIFICATION_ENTRY_TYPE, process_specification)?)
}

#[hdk_extern]
fn get_process_specification(ByAddress { address }: ByAddress<ProcessSpecificationAddress>) -> ExternResult<ResponseData> {
    Ok(handle_get_process_specification(address)?)
}

#[hdk_extern]
fn get_revision(ByRevision { revision_id }: ByRevision) -> ExternResult<ResponseData> {
    Ok(handle_get_revision(revision_id)?)
}

#[hdk_extern]
fn update_process_specification(UpdateParams { process_specification }: UpdateParams) -> ExternResult<ResponseData> {
    Ok(handle_update_process_specification(process_specification)?)
}

#[hdk_extern]
fn delete_process_specification(ByRevision { revision_id }: ByRevision) -> ExternResult<bool> {
    Ok(handle_delete_process_specification(revision_id)?)
}
