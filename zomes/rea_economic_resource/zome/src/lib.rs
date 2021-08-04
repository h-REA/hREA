/**
 * REA `EconomicResource` zome API definition
 *
 * Defines the top-level zome configuration needed by Holochain's build system
 * to bundle the app. This basically involves wiring up the helper methods from the
 * related `_lib` module into a packaged zome WASM binary.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use hc_zome_rea_economic_resource_lib::*;
use hc_zome_rea_economic_resource_rpc::*;
use hc_zome_rea_economic_resource_storage::*;
use hc_zome_rea_economic_event_rpc::{
    ResourceResponseData as ResponseData,
    CreateRequest as EventCreateRequest,
};

#[hdk_extern]
fn entry_defs(_: ()) -> ExternResult<EntryDefsCallbackResult> {
    Ok(EntryDefsCallbackResult::from(vec![
        Path::entry_def(),
        EntryDef {
            id: RESOURCE_ENTRY_TYPE.into(),
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
        Ok(resource_storage) => {
            let record = resource_storage.entry();
            record.validate()
                .and_then(|()| { Ok(ValidateCallbackResult::Valid) })
                .or_else(|e| { Ok(ValidateCallbackResult::Invalid(e)) })
        },
        _ => Ok(ValidateCallbackResult::Valid),
    }
}

// :TODO: The signature of this method, and its decoupling from the EconomicEvent zome, means that resources can be
//        instantiated from the receiving inventory. Is this desirable? What are the repercussions?
#[hdk_extern]
fn _internal_create_resource(params: CreationPayload) -> ExternResult<(RevisionHash, ResourceAddress, EntryData)>
{
    Ok(receive_create_economic_resource(
        RESOURCE_ENTRY_TYPE, ECONOMIC_RESOURCE_SPECIFICATION_ENTRY_TYPE,
        params,
    )?)
}

#[hdk_extern]
fn _internal_update_inventory(event: EventCreateRequest) -> ExternResult<Vec<(RevisionHash, ResourceAddress, EntryData, EntryData)>>
{
    Ok(receive_update_inventory_from_event(RESOURCE_ENTRY_TYPE, event)?)
}

#[derive(Debug, Serialize, Deserialize)]
struct ByAddress {
    pub address: ResourceAddress,
}

#[hdk_extern]
fn get_resource(ByAddress { address }: ByAddress) -> ExternResult<ResponseData> {
    Ok(receive_get_economic_resource(
        RESOURCE_ENTRY_TYPE, EVENT_ENTRY_TYPE, PROCESS_ENTRY_TYPE,
        address,
    )?)
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateParams {
    pub resource: UpdateRequest,
}

#[hdk_extern]
fn update_resource(UpdateParams { resource }: UpdateParams) -> ExternResult<ResponseData> {
    Ok(receive_update_economic_resource(
        RESOURCE_ENTRY_TYPE, EVENT_ENTRY_TYPE, PROCESS_ENTRY_TYPE,
        resource
    )?)
}

#[hdk_extern]
fn get_all_resources(_: ()) -> ExternResult<Vec<ResponseData>> {
    Ok(receive_get_all_economic_resources(RESOURCE_ENTRY_TYPE, EVENT_ENTRY_TYPE, PROCESS_ENTRY_TYPE)?)
}

#[hdk_extern]
fn query_resources(params: QueryParams) -> ExternResult<Vec<ResponseData>> {
    Ok(receive_query_economic_resources(
        RESOURCE_ENTRY_TYPE, EVENT_ENTRY_TYPE, PROCESS_ENTRY_TYPE,
        params
    )?)
}
