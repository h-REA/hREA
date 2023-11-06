/**
 * hREA recipe_flow zome API definition
 *
 * Defines the top-level zome configuration needed by Holochain's build system
 * to bundle the app. This basically involves wiring up the helper methods from the
 * related `_lib` module into a packaged zome WASM binary.
 *
 * @package hREA
 */
use hdk::prelude::*;

use hc_zome_rea_recipe_flow_rpc::*;
use hc_zome_rea_recipe_flow_lib::*;
use hc_zome_rea_recipe_flow_storage_consts::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateParams {
    pub recipe_flow: CreateRequest,
}

#[hdk_extern]
fn create_recipe_flow(CreateParams { recipe_flow }: CreateParams) -> ExternResult<ResponseData> {
    Ok(handle_create_recipe_flow(
        INTENT_ENTRY_TYPE,
        recipe_flow,
    )?)
}

#[derive(Debug, Serialize, Deserialize)]
struct ByAddress {
    pub address: RecipeFlowAddress,
}

#[hdk_extern]
fn get_recipe_flow(ByAddress { address }: ByAddress) -> ExternResult<ResponseData> {
    Ok(handle_get_recipe_flow(address)?)
}

#[hdk_extern]
fn get_revision(ByRevision { revision_id }: ByRevision) -> ExternResult<ResponseData> {
    Ok(handle_get_revision(revision_id)?)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateParams {
    pub recipe_flow: UpdateRequest,
}

#[hdk_extern]
fn update_recipe_flow(UpdateParams { recipe_flow }: UpdateParams) -> ExternResult<ResponseData> {
    Ok(handle_update_recipe_flow(recipe_flow)?)
}

#[hdk_extern]
fn delete_recipe_flow(ByRevision { revision_id }: ByRevision) -> ExternResult<bool> {
    Ok(handle_delete_recipe_flow(revision_id)?)
}
