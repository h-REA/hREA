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
use hc_zome_rea_process_storage_consts::PROCESS_ENTRY_TYPE;

#[hdk_extern]
fn validate(validation_data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    let element = validation_data.element;
    let entry = element.into_inner().1;
    let entry = match entry {
        ElementEntry::Present(e) => e,
        _ => return Ok(ValidateCallbackResult::Valid),
    };

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

#[hdk_extern]
fn entry_defs(_: ()) -> ExternResult<EntryDefsCallbackResult> {
    Ok(EntryDefsCallbackResult::from(vec![
        Path::entry_def(),
        EntryDef {
            id: CAP_STORAGE_ENTRY_DEF_ID.into(),
            visibility: EntryVisibility::Private,
            crdt_type: CrdtType,
            required_validations: 1.into(),
            required_validation_type: RequiredValidationType::default(),
        },
        EntryDef {
            id: INTENT_ENTRY_TYPE.into(),
            visibility: EntryVisibility::Public,
            crdt_type: CrdtType,
            required_validations: 2.into(),
            required_validation_type: RequiredValidationType::default(),
        }
    ]))
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateParams {
    pub intent: CreateRequest,
}

#[hdk_extern]
fn create_intent(CreateParams { intent }: CreateParams) -> ExternResult<ResponseData> {
    Ok(receive_create_intent(
        INTENT_ENTRY_TYPE, PROCESS_ENTRY_TYPE,
        intent,
    )?)
}

#[derive(Debug, Serialize, Deserialize)]
struct ByAddress {
    pub address: IntentAddress,
}

#[hdk_extern]
fn get_intent(ByAddress { address }: ByAddress) -> ExternResult<ResponseData> {
    Ok(receive_get_intent(INTENT_ENTRY_TYPE, address)?)
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateParams {
    pub intent: UpdateRequest,
}

#[hdk_extern]
fn update_intent(UpdateParams { intent }: UpdateParams) -> ExternResult<ResponseData> {
    Ok(receive_update_intent(INTENT_ENTRY_TYPE, PROCESS_ENTRY_TYPE, intent)?)
}

#[derive(Debug, Serialize, Deserialize)]
struct ByHeader {
    pub address: RevisionHash,
}

#[hdk_extern]
fn delete_intent(ByHeader { address }: ByHeader) -> ExternResult<bool> {
    Ok(receive_delete_intent(
        INTENT_ENTRY_TYPE, PROCESS_ENTRY_TYPE,
        address,
    )?)
}
