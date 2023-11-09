/**
 * hREA recipe_process zome API definition
 *
 * Defines the top-level zome configuration needed by Holochain's build system
 * to bundle the app. This basically involves wiring up the helper methods from the
 * related `_lib` module into a packaged zome WASM binary.
 *
 * @package hREA
 */
use hdk::prelude::*;

use hc_zome_rea_recipe_process_rpc::*;
use hc_zome_rea_recipe_process_lib::*;
use hc_zome_rea_recipe_process_storage_consts::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateParams {
    pub recipe_process: CreateRequest,
}

#[hdk_extern]
fn create_recipe_process(CreateParams { recipe_process }: CreateParams) -> ExternResult<ResponseData> {
    Ok(handle_create_recipe_process(
        RECIPE_PROCESS_ENTRY_TYPE,
        recipe_process,
    )?)
}

#[derive(Debug, Serialize, Deserialize)]
struct ByAddress {
    pub address: RecipeProcessAddress,
}

#[hdk_extern]
fn get_recipe_process(ByAddress { address }: ByAddress) -> ExternResult<ResponseData> {
    Ok(handle_get_recipe_process(address)?)
}

#[hdk_extern]
fn get_revision(ByRevision { revision_id }: ByRevision) -> ExternResult<ResponseData> {
    Ok(handle_get_revision(revision_id)?)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateParams {
    pub recipe_process: UpdateRequest,
}

#[hdk_extern]
fn update_recipe_process(UpdateParams { recipe_process }: UpdateParams) -> ExternResult<ResponseData> {
    Ok(handle_update_recipe_process(recipe_process)?)
}

#[hdk_extern]
fn delete_recipe_process(ByRevision { revision_id }: ByRevision) -> ExternResult<bool> {
    Ok(handle_delete_recipe_process(revision_id)?)
}
