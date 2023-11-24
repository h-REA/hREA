/**
 * hREA Plan zome API definition
 *
 * Defines the top-level zome configuration needed by Holochain's build system
 * to bundle the app. This basically involves wiring up the helper methods from the
 * related `_lib` module into a packaged zome WASM binary.
 *
 * @package hREA
 */
use hdk::prelude::*;

use hc_zome_rea_plan_rpc::*;
use hc_zome_rea_plan_lib::*;

#[hdk_extern]
fn create_plan(CreateParams { plan }: CreateParams) -> ExternResult<ResponseData> {
    Ok(handle_create_plan(PLAN_ENTRY_TYPE, plan)?)
}

#[hdk_extern]
fn get_plan(ReadParams { address }: ReadParams) -> ExternResult<ResponseData> {
    Ok(handle_get_plan(address)?)
}

#[hdk_extern]
fn get_revision(ByRevision { revision_id }: ByRevision) -> ExternResult<ResponseData> {
    Ok(handle_get_revision(revision_id)?)
}

#[hdk_extern]
fn update_plan(UpdateParams { plan }: UpdateParams) -> ExternResult<ResponseData> {
    Ok(handle_update_plan(plan)?)
}

#[hdk_extern]
fn delete_plan(ByRevision { revision_id }: ByRevision) -> ExternResult<bool> {
    Ok(handle_delete_plan(revision_id)?)
}
