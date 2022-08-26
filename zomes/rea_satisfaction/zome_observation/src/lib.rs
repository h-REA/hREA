/**
 * Holo-REA satisfaction remote index zome API definition
 *
 * Manages indexes for querying `EconomicEvents` against remote `Satisfactions`.
 *
 * Defines the top-level zome configuration needed by Holochain's build system
 * to bundle the app. This basically involves wiring up the helper methods from the
 * related `_lib` module into a packaged zome WASM binary.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use hc_zome_rea_satisfaction_lib_destination::*;
use hc_zome_rea_satisfaction_rpc::*;
use hc_zome_rea_satisfaction_storage_consts::*;

#[hdk_extern]
fn satisfaction_created(CreateParams { satisfaction }: CreateParams) -> ExternResult<ResponseData> {
    Ok(handle_create_satisfaction(
        SATISFACTION_ENTRY_TYPE,
        satisfaction,
    )?)
}

#[hdk_extern]
fn get_satisfaction(
    ByAddress { address }: ByAddress<SatisfactionAddress>,
) -> ExternResult<ResponseData> {
    Ok(handle_get_satisfaction(address)?)
}

#[hdk_extern]
fn get_revision(ByRevision { revision_id }: ByRevision) -> ExternResult<ResponseData> {
    Ok(handle_get_revision(revision_id)?)
}

#[hdk_extern]
fn satisfaction_updated(UpdateParams { satisfaction }: UpdateParams) -> ExternResult<ResponseData> {
    Ok(handle_update_satisfaction(satisfaction)?)
}

#[hdk_extern]
fn satisfaction_deleted(ByRevision { revision_id }: ByRevision) -> ExternResult<bool> {
    Ok(handle_delete_satisfaction(revision_id)?)
}
