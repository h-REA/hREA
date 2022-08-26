/**
 * Commitment zome API definition
 *
 * # Remarks
 *
 * Defines the top-level zome configuration needed by Holochain's build system
 * to bundle the app. This basically involves wiring up the `entry!` type macros
 * and `define_zome!` definition to the standard Rust code in the rest of this
 * module.
 *
 * @package: HoloREA
 * @since:   2019-02-06
 */
use hdk::prelude::*;

use hc_zome_rea_commitment_rpc::*;
use hc_zome_rea_commitment_lib::*;
use hc_zome_rea_commitment_storage_consts::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateParams {
    pub commitment: CreateRequest,
}

#[hdk_extern]
fn create_commitment(CreateParams { commitment }: CreateParams) -> ExternResult<ResponseData> {
    Ok(handle_create_commitment(COMMITMENT_ENTRY_TYPE, commitment)?)
}

#[derive(Debug, Serialize, Deserialize)]
struct ByAddress {
    pub address: CommitmentAddress,
}

#[hdk_extern]
fn get_commitment(ByAddress { address }: ByAddress) -> ExternResult<ResponseData> {
    Ok(handle_get_commitment(address)?)
}

#[hdk_extern]
fn get_revision(ByRevision { revision_id }: ByRevision) -> ExternResult<ResponseData> {
    Ok(handle_get_revision(revision_id)?)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateParams {
    pub commitment: UpdateRequest,
}

#[hdk_extern]
fn update_commitment(UpdateParams { commitment }: UpdateParams) -> ExternResult<ResponseData> {
    Ok(handle_update_commitment(commitment)?)
}

#[hdk_extern]
fn delete_commitment(ByRevision { revision_id }: ByRevision) -> ExternResult<bool> {
    Ok(handle_delete_commitment(revision_id)?)
}
