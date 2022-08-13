/**
 * Holo-REA fulfillment remote index zome API definition
 *
 * Manages indexes for querying `EconomicEvents` against remote `Fulfillments`.
 *
 * Defines the top-level zome configuration needed by Holochain's build system
 * to bundle the app. This basically involves wiring up the helper methods from the
 * related `_lib` module into a packaged zome WASM binary.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use hc_zome_rea_fulfillment_lib_destination::*;
use hc_zome_rea_fulfillment_rpc::*;
use hc_zome_rea_fulfillment_storage_consts::*;

#[hdk_extern]
fn fulfillment_created(CreateParams { fulfillment }: CreateParams) -> ExternResult<ResponseData> {
    Ok(handle_create_fulfillment(FULFILLMENT_ENTRY_TYPE, fulfillment)?)
}

#[hdk_extern]
fn get_fulfillment(ByAddress { address }: ByAddress<FulfillmentAddress>) -> ExternResult<ResponseData> {
    Ok(handle_get_fulfillment(address)?)
}

#[hdk_extern]
fn fulfillment_updated(UpdateParams { fulfillment }: UpdateParams) -> ExternResult<ResponseData> {
    Ok(handle_update_fulfillment(fulfillment)?)
}

#[hdk_extern]
fn fulfillment_deleted(ByRevision { revision_id }: ByRevision) -> ExternResult<bool> {
    Ok(handle_delete_fulfillment(revision_id)?)
}
