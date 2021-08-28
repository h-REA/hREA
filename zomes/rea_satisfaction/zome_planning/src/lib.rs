/**
 * Holo-REA satisfaction zome API definition
 *
 * Defines the top-level zome configuration needed by Holochain's build system
 * to bundle the app. This basically involves wiring up the helper methods from the
 * related `_lib` module into a packaged zome WASM binary.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use hc_zome_rea_satisfaction_lib_origin::*;
use hc_zome_rea_satisfaction_rpc::*;
use hc_zome_rea_satisfaction_storage_consts::*;
use hc_zome_rea_commitment_storage_consts::COMMITMENT_ENTRY_TYPE;
use hc_zome_rea_intent_storage_consts::INTENT_ENTRY_TYPE;

#[hdk_extern]
fn entry_defs(_: ()) -> ExternResult<EntryDefsCallbackResult> {
    Ok(EntryDefsCallbackResult::from(vec![
        Path::entry_def(),
        EntryDef {
            id: SATISFACTION_ENTRY_TYPE.into(),
            visibility: EntryVisibility::Public,
            crdt_type: CrdtType,
            required_validations: 1.into(),
            required_validation_type: RequiredValidationType::default(),
        }
    ]))
}

#[hdk_extern]
fn create_satisfaction(CreateParams { satisfaction }: CreateParams) -> ExternResult<ResponseData> {
    Ok(receive_create_satisfaction(SATISFACTION_ENTRY_TYPE, INTENT_ENTRY_TYPE, COMMITMENT_ENTRY_TYPE, satisfaction)?)
}

#[hdk_extern]
fn get_satisfaction(ByAddress { address }: ByAddress<SatisfactionAddress>) -> ExternResult<ResponseData> {
    Ok(receive_get_satisfaction(SATISFACTION_ENTRY_TYPE, address)?)
}

#[hdk_extern]
fn update_satisfaction(UpdateParams { satisfaction }: UpdateParams) -> ExternResult<ResponseData> {
    Ok(receive_update_satisfaction(SATISFACTION_ENTRY_TYPE, INTENT_ENTRY_TYPE, COMMITMENT_ENTRY_TYPE, satisfaction)?)
}

#[hdk_extern]
fn delete_satisfaction(ByHeader { address }: ByHeader) -> ExternResult<bool> {
    Ok(receive_delete_satisfaction(SATISFACTION_ENTRY_TYPE, INTENT_ENTRY_TYPE, COMMITMENT_ENTRY_TYPE, address)?)
}

#[hdk_extern]
fn query_satisfactions(params: QueryParams) -> ExternResult<Vec<ResponseData>>{
    Ok(receive_query_satisfactions(INTENT_ENTRY_TYPE, COMMITMENT_ENTRY_TYPE, params)?)
}
