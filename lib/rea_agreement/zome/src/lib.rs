/**
 * Holo-REA agreement zome API definition
 *
 * Defines the top-level zome configuration needed by Holochain's build system
 * to bundle the app. This basically involves wiring up the helper methods from the
 * related `_lib` module into a packaged zome WASM binary.
 *
 * @package Holo-REA
 */
use hdk3::prelude::*;

use hc_zome_rea_agreement_rpc::*;
use hc_zome_rea_agreement_lib::*;

#[hdk_extern]
fn entry_defs(_: ()) -> ExternResult<EntryDefsCallbackResult> {
    Ok(EntryDefsCallbackResult::from(vec![
        Path::entry_def(),
        EntryDef {
            id: AGREEMENT_ENTRY_TYPE.into(),
            visibility: EntryVisibility::Public,
            crdt_type: CrdtType,
            required_validations: 2.into(),
            required_validation_type: RequiredValidationType::default(),
        }
    ]))
}

#[hdk_extern]
fn create_agreement(agreement: CreateRequest) -> ExternResult<ResponseData> {
    Ok(receive_create_agreement(AGREEMENT_ENTRY_TYPE, agreement)?)
}

#[hdk_extern]
fn get_agreement(address: AgreementAddress) -> ExternResult<ResponseData> {
    Ok(receive_get_agreement(AGREEMENT_ENTRY_TYPE, address)?)
}

#[hdk_extern]
fn update_agreement(agreement: UpdateRequest) -> ExternResult<ResponseData> {
    Ok(receive_update_agreement(AGREEMENT_ENTRY_TYPE, agreement)?)
}

#[hdk_extern]
fn delete_agreement(address: RevisionHash) -> ExternResult<bool> {
    Ok(receive_delete_agreement(address)?)
}
