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
use hc_zome_rea_agreement_storage::{AgreementAddress};

entry_defs![
    Path::entry_def(),
];

#[hdk_extern]
fn init() {
    Ok(())
}

#[hdk_extern]
pub fn validate_agent(validation_data: EntryValidationData::<AgentId>) {
    Ok(())
}

#[hdk_extern]
fn create_agreement(agreement: CreateRequest) -> ExternResult<ResponseData> {
    receive_create_agreement(agreement)
}

#[hdk_extern]
fn get_agreement(address: AgreementAddress) -> ExternResult<ResponseData> {
    receive_get_agreement(address)
}

#[hdk_extern]
fn update_agreement(agreement: UpdateRequest) -> ExternResult<ResponseData> {
    receive_update_agreement(agreement)
}

#[hdk_extern]
fn delete_agreement(address: HeaderHash) -> ExternResult<bool> {
    receive_delete_agreement(address)
}
