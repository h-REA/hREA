/**
 * Holo-REA resource specification zome API definition
 *
 * Defines the top-level zome configuration needed by Holochain's build system
 * to bundle the app. This basically involves wiring up the helper methods from the
 * related `_lib` module into a packaged zome WASM binary.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use hc_zome_rea_resource_specification_rpc::*;
use hc_zome_rea_resource_specification_lib::*;
use hc_zome_rea_resource_specification_storage_consts::*;

#[hdk_extern]
fn create_resource_specification(CreateParams { resource_specification }: CreateParams) -> ExternResult<ResponseData> {
    Ok(handle_create_resource_specification(ECONOMIC_RESOURCE_SPECIFICATION_ENTRY_TYPE, resource_specification)?)
}

#[hdk_extern]
fn get_resource_specification(ByAddress { address }: ByAddress<ResourceSpecificationAddress>) -> ExternResult<ResponseData> {
    Ok(handle_get_resource_specification(address)?)
}

#[hdk_extern]
fn get_revision(ByRevision { revision_id }: ByRevision) -> ExternResult<ResponseData> {
    Ok(handle_get_revision(revision_id)?)
}

#[hdk_extern]
fn update_resource_specification(UpdateParams { resource_specification }: UpdateParams) -> ExternResult<ResponseData> {
    Ok(handle_update_resource_specification(resource_specification)?)
}

#[hdk_extern]
fn delete_resource_specification(ByRevision { revision_id }: ByRevision) -> ExternResult<bool> {
    Ok(handle_delete_resource_specification(revision_id)?)
}
