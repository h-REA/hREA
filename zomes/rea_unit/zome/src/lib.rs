/**
 * hREA measurement unit zome API definition
 *
 * Defines the top-level zome configuration needed by Holochain's build system
 * to bundle the app. This basically involves wiring up the helper methods from the
 * related `_lib` module into a packaged zome WASM binary.
 *
 * @package hREA
 */
use hdk::prelude::*;

use hc_zome_rea_unit_rpc::*;
use hc_zome_rea_unit_lib::*;
use vf_attributes_hdk::UnitInternalAddress;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
    Ok(handle_get_unit(id)?)
}

#[derive(Debug, Serialize, Deserialize)]
struct BySymbol {
    symbol: String,
}

#[hdk_extern]
fn get_unit_by_symbol(BySymbol { symbol }: BySymbol) -> ExternResult<ResponseData> {
    Ok(handle_get_unit_by_symbol(symbol)?)
}

// used by indexing zomes to retrieve indexed record data
#[hdk_extern]
fn __internal_get_unit_by_hash(ByAddress { address }: ByAddress<UnitInternalAddress>) -> ExternResult<ResponseData> {
    Ok(handle_get_unit_by_address(address)?)
}

#[hdk_extern]
fn get_revision(ByRevision { revision_id }: ByRevision) -> ExternResult<ResponseData> {
    Ok(handle_get_revision(revision_id)?)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateParams {
    pub unit: UpdateRequest,
}

#[hdk_extern]
fn update_unit(UpdateParams { unit }: UpdateParams) -> ExternResult<ResponseData> {
    Ok(handle_update_unit(unit)?)
}

#[hdk_extern]
fn delete_unit(ByRevision { revision_id }: ByRevision) -> ExternResult<bool> {
    Ok(handle_delete_unit(revision_id)?)
}
