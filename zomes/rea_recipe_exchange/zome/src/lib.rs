/**
 * hREA recipe_exchange zome API definition
 *
 * Defines the top-level zome configuration needed by Holochain's build system
 * to bundle the app. This basically involves wiring up the helper methods from the
 * related `_lib` module into a packaged zome WASM binary.
 *
 * @package hREA
 */
use hdk::prelude::*;

use hc_zome_rea_recipe_exchange_rpc::*;
use hc_zome_rea_recipe_exchange_lib::*;
use hc_zome_rea_recipe_exchange_storage_consts::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateParams {
    pub recipe_exchange: CreateRequest,
}

#[hdk_extern]
fn create_recipe_exchange(CreateParams { recipe_exchange }: CreateParams) -> ExternResult<ResponseData> {
    Ok(handle_create_recipe_exchange(
        RECIPE_PROCESS_ENTRY_TYPE,
        recipe_exchange,
    )?)
}

#[derive(Debug, Serialize, Deserialize)]
struct ByAddress {
    pub address: RecipeExchangeAddress,
}

#[hdk_extern]
fn get_recipe_exchange(ByAddress { address }: ByAddress) -> ExternResult<ResponseData> {
    Ok(handle_get_recipe_exchange(address)?)
}

#[hdk_extern]
fn get_revision(ByRevision { revision_id }: ByRevision) -> ExternResult<ResponseData> {
    Ok(handle_get_revision(revision_id)?)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateParams {
    pub recipe_exchange: UpdateRequest,
}

#[hdk_extern]
fn update_recipe_exchange(UpdateParams { recipe_exchange }: UpdateParams) -> ExternResult<ResponseData> {
    Ok(handle_update_recipe_exchange(recipe_exchange)?)
}

#[hdk_extern]
fn delete_recipe_exchange(ByRevision { revision_id }: ByRevision) -> ExternResult<bool> {
    Ok(handle_delete_recipe_exchange(revision_id)?)
}
