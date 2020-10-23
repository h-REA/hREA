#![feature(proc_macro_hygiene)]
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
use hc_zome_rea_agreement_storage_consts::*;
use hc_zome_rea_agreement_storage::Entry;


#[hdk_extern]
fn init() {
    Ok(())
}

#[hdk_extern]
pub fn validate_agent(validation_data: EntryValidationData::<AgentId>) {
    Ok(())
}

entry_def!(Entry EntryDef {
    id: AGREEMENT_ENTRY_TYPE.into(),
    ..Default::default()
});

// :TODO: investigate "paths" in the context of 'key indexes'
entry_def!(EntryHash EntryDef {
    id: AGREEMENT_BASE_ENTRY_TYPE.into(),
    ..Default::default()
});

#[hdk_extern]
fn create_agreement(agreement: CreateRequest) -> ZomeApiResult<ResponseData> {
    receive_create_agreement(agreement)
}

#[hdk_extern]
fn get_agreement(address: AgreementAddress) -> ZomeApiResult<ResponseData> {
    receive_get_agreement(address)
}

#[hdk_extern]
fn update_agreement(agreement: UpdateRequest) -> ZomeApiResult<ResponseData> {
    receive_update_agreement(agreement)
}

#[hdk_extern]
fn delete_agreement(address: AgreementAddress) -> ZomeApiResult<bool> {
    receive_delete_agreement(address)
}