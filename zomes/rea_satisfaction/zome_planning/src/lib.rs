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

#[hdk_extern]
fn entry_defs(_: ()) -> ExternResult<EntryDefsCallbackResult> {
    Ok(EntryDefsCallbackResult::from(vec![
        PathEntry::entry_def(),
        SatisfactionAddress::entry_def(),
        EntryDef {
            id: CAP_STORAGE_ENTRY_DEF_ID.into(),
            visibility: EntryVisibility::Private,
            required_validations: 1.into(),
            required_validation_type: RequiredValidationType::default(),
        },
        EntryDef {
            id: SATISFACTION_ENTRY_TYPE.into(),
            visibility: EntryVisibility::Public,
            required_validations: 1.into(),
            required_validation_type: RequiredValidationType::default(),
        },
    ]))
}

#[hdk_extern]
fn create_satisfaction(CreateParams { satisfaction }: CreateParams) -> ExternResult<ResponseData> {
    Ok(handle_create_satisfaction(
        SATISFACTION_ENTRY_TYPE,
        satisfaction,
    )?)
}

#[hdk_extern]
fn get_satisfaction(
    ByAddress { address }: ByAddress<SatisfactionAddress>,
) -> ExternResult<ResponseData> {
    Ok(handle_get_satisfaction(SATISFACTION_ENTRY_TYPE, address)?)
}

#[hdk_extern]
fn update_satisfaction(UpdateParams { satisfaction }: UpdateParams) -> ExternResult<ResponseData> {
    Ok(handle_update_satisfaction(
        SATISFACTION_ENTRY_TYPE,
        satisfaction,
    )?)
}

#[hdk_extern]
fn delete_satisfaction(ByRevision { revision_id }: ByRevision) -> ExternResult<bool> {
    Ok(handle_delete_satisfaction(revision_id)?)
}
