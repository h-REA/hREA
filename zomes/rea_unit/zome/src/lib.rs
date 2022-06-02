/**
 * Holo-REA measurement unit zome API definition
 *
 * Defines the top-level zome configuration needed by Holochain's build system
 * to bundle the app. This basically involves wiring up the helper methods from the
 * related `_lib` module into a packaged zome WASM binary.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use hc_zome_rea_unit_rpc::*;
use hc_zome_rea_unit_lib::*;
use vf_attributes_hdk::UnitInternalAddress;

#[hdk_extern]
fn entry_defs(_: ()) -> ExternResult<EntryDefsCallbackResult> {
    Ok(EntryDefsCallbackResult::from(vec![
        PathEntry::entry_def(),
        UnitInternalAddress::entry_def(),
        EntryDef {
            id: UNIT_ENTRY_TYPE.into(),
            visibility: EntryVisibility::Public,
            crdt_type: CrdtType,
            required_validations: 2.into(),
            required_validation_type: RequiredValidationType::default(),
        }
    ]))
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateParams {
    pub unit: CreateRequest,
}

#[hdk_extern]
fn create_unit(CreateParams { unit }: CreateParams) -> ExternResult<ResponseData>{
    Ok(handle_create_unit(UNIT_ENTRY_TYPE, unit)?)
}

#[derive(Debug, Serialize, Deserialize)]
struct ById {
    id: UnitId,
}

#[hdk_extern]
fn get_unit(ById { id }: ById) -> ExternResult<ResponseData> {
    Ok(handle_get_unit(UNIT_ENTRY_TYPE, id)?)
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateParams {
    pub unit: UpdateRequest,
}

#[hdk_extern]
fn update_unit(UpdateParams { unit }: UpdateParams) -> ExternResult<ResponseData> {
    Ok(handle_update_unit(UNIT_ENTRY_TYPE, unit)?)
}

#[hdk_extern]
fn delete_unit(ByRevision { revision_id }: ByRevision) -> ExternResult<bool> {
    Ok(handle_delete_unit(revision_id)?)
}
