/**
 * Holo-REA intent zome API definition
 *
 * Defines the top-level zome configuration needed by Holochain's build system
 * to bundle the app. This basically involves wiring up the helper methods from the
 * related `_lib` module into a packaged zome WASM binary.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use hc_zome_rea_intent_rpc::*;
use hc_zome_rea_intent_lib::*;
use hc_zome_rea_intent_storage_consts::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateParams {
    pub intent: CreateRequest,
}

#[hdk_extern]
fn create_intent(CreateParams { intent }: CreateParams) -> ExternResult<ResponseData> {
    Ok(handle_create_intent(
        INTENT_ENTRY_TYPE,
        intent,
    )?)
}

#[derive(Debug, Serialize, Deserialize)]
struct ByAddress {
    pub address: IntentAddress,
}

#[hdk_extern]
fn get_intent(ByAddress { address }: ByAddress) -> ExternResult<ResponseData> {
    Ok(handle_get_intent(address)?)
}

#[hdk_extern]
fn get_revision(ByRevision { revision_id }: ByRevision) -> ExternResult<ResponseData> {
    Ok(handle_get_revision(revision_id)?)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateParams {
    pub intent: UpdateRequest,
}

#[hdk_extern]
fn update_intent(UpdateParams { intent }: UpdateParams) -> ExternResult<ResponseData> {
    Ok(handle_update_intent(intent)?)
}

#[hdk_extern]
fn delete_intent(ByRevision { revision_id }: ByRevision) -> ExternResult<bool> {
    Ok(handle_delete_intent(revision_id)?)
}
