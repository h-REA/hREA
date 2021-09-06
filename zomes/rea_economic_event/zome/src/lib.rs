/**
 * REA `EconomicEvent` zome API definition
 *
 * Defines the top-level zome configuration needed by Holochain's build system
 * to bundle the app. This basically involves wiring up the helper methods from the
 * related `_lib` module into a packaged zome WASM binary.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use hc_zome_rea_economic_event_lib::*;
use hc_zome_rea_economic_event_rpc::*;
use hc_zome_rea_economic_event_storage::*;
use hc_zome_rea_economic_resource_rpc::CreateRequest as EconomicResourceCreateRequest;

#[hdk_extern]
fn entry_defs(_: ()) -> ExternResult<EntryDefsCallbackResult> {
    Ok(EntryDefsCallbackResult::from(vec![
        Path::entry_def(),
        EntryDef {
            id: EVENT_ENTRY_TYPE.into(),
            visibility: EntryVisibility::Public,
            crdt_type: CrdtType,
            required_validations: 2.into(),
            required_validation_type: RequiredValidationType::default(),
        }
    ]))
}

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

#[derive(Debug, Serialize, Deserialize)]
struct CreateParams {
    pub event: CreateRequest,
    pub new_inventoried_resource: Option<EconomicResourceCreateRequest>,
}

#[hdk_extern]
fn create_event(CreateParams { event, new_inventoried_resource }: CreateParams) -> ExternResult<ResponseData> {
    Ok(receive_create_economic_event(
        EVENT_ENTRY_TYPE, PROCESS_ENTRY_TYPE,
        event, new_inventoried_resource,
    )?)
}

#[derive(Debug, Serialize, Deserialize)]
struct ByAddress {
    pub address: EventAddress,
}

#[hdk_extern]
fn get_event(ByAddress { address }: ByAddress) -> ExternResult<ResponseData> {
    Ok(receive_get_economic_event(EVENT_ENTRY_TYPE, address)?)
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateParams {
    pub event: UpdateRequest,
}

#[hdk_extern]
fn update_event(UpdateParams { event }: UpdateParams) -> ExternResult<ResponseData> {
    Ok(receive_update_economic_event(EVENT_ENTRY_TYPE, event)?)
}

#[derive(Debug, Serialize, Deserialize)]
struct ByHeader {
    pub address: RevisionHash,
}

#[hdk_extern]
fn delete_event(ByHeader { address }: ByHeader) -> ExternResult<bool> {
    Ok(receive_delete_economic_event(address)?)
}

#[hdk_extern]
fn get_all_events(_: ()) -> ExternResult<Vec<ResponseData>> {
    Ok(receive_get_all_economic_events(EVENT_ENTRY_TYPE)?)
}
