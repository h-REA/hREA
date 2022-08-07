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
use hc_zome_rea_intent_storage::*;
use hc_zome_rea_intent_storage_consts::*;

#[hdk_extern]
fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
    match op {
        Op::StoreRecord { .. } => Ok(ValidateCallbackResult::Valid),
        Op::StoreEntry(StoreEntry { entry, .. }) => validate_entry(entry),
        Op::RegisterCreateLink { .. } => Ok(ValidateCallbackResult::Valid),
        Op::RegisterDeleteLink { .. } => Ok(ValidateCallbackResult::Valid),
        Op::RegisterUpdate { .. } => Ok(ValidateCallbackResult::Valid),
        Op::RegisterDelete { .. } => Ok(ValidateCallbackResult::Valid),
        Op::RegisterAgentActivity { .. } => Ok(ValidateCallbackResult::Valid),
    }
}

fn validate_entry(entry: Entry) -> ExternResult<ValidateCallbackResult> {
    match EntryStorage::try_from(&entry) {
        Ok(event_storage) => {
            let record = event_storage.entry();
            record.validate_or_fields()
                .and_then(|()| { record.validate_action() })
                .and_then(|()| { Ok(ValidateCallbackResult::Valid) })
                .or_else(|e| { Ok(ValidateCallbackResult::Invalid(e)) })
        },
        _ => Ok(ValidateCallbackResult::Valid),
    }
}

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
