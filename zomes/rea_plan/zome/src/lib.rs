/**
 * Holo-REA Plan zome API definition
 *
 * Defines the top-level zome configuration needed by Holochain's build system
 * to bundle the app. This basically involves wiring up the helper methods from the
 * related `_lib` module into a packaged zome WASM binary.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use hc_zome_rea_plan_rpc::*;
use hc_zome_rea_plan_lib::*;

#[hdk_extern]
fn entry_defs(_: ()) -> ExternResult<EntryDefsCallbackResult> {
    Ok(EntryDefsCallbackResult::from(vec![
        PathEntry::entry_def(),
        PlanAddress::entry_def(),
        EntryDef {
            id: PLAN_ENTRY_TYPE.into(),
            visibility: EntryVisibility::Public,
            crdt_type: CrdtType,
            required_validations: 2.into(),
            required_validation_type: RequiredValidationType::default(),
        }
    ]))
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateParams {
    pub plan: CreateRequest,
}

#[hdk_extern]
fn create_plan(CreateParams { plan }: CreateParams) -> ExternResult<ResponseData> {
    Ok(handle_create_plan(PLAN_ENTRY_TYPE, plan)?)
}

#[derive(Debug, Serialize, Deserialize)]
struct ReadParams {
    pub address: PlanAddress,
}

#[hdk_extern]
fn get_plan(ReadParams { address }: ReadParams) -> ExternResult<ResponseData> {
    Ok(handle_get_plan(PLAN_ENTRY_TYPE, address)?)
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateParams {
    pub plan: UpdateRequest,
}

#[hdk_extern]
fn update_plan(UpdateParams { plan }: UpdateParams) -> ExternResult<ResponseData> {
    Ok(handle_update_plan(PLAN_ENTRY_TYPE, plan)?)
}

#[hdk_extern]
fn delete_plan(ByHeader { address }: ByHeader) -> ExternResult<bool> {
    Ok(handle_delete_plan(address)?)
}
