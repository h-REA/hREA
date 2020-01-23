#![feature(proc_macro_hygiene)]
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
extern crate serde;
extern crate serde_json;
extern crate hdk;
extern crate hdk_proc_macros;

use hdk::prelude::*;
use hdk_proc_macros::zome;

use hc_zome_rea_fulfillment_defs::{ entry_def, remote_entry_def };
use hc_zome_rea_fulfillment_rpc::*;
use hc_zome_rea_fulfillment_lib_destination::*;

#[zome]
mod rea_fulfillment_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData::<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn fulfillment_entry_def() -> ValidatingEntryType {
        entry_def()
    }

    #[entry_def]
    fn fulfillment_base_entry_def() -> ValidatingEntryType {
        remote_entry_def()
    }

    #[zome_fn("hc_public")]
    fn fulfillment_created(fulfillment: CreateRequest) -> ZomeApiResult<ResponseData> {
        receive_create_fulfillment(fulfillment)
    }

    #[zome_fn("hc_public")]
    fn fulfillment_updated(fulfillment: UpdateRequest) -> ZomeApiResult<ResponseData> {
        receive_update_fulfillment(fulfillment)
    }

    #[zome_fn("hc_public")]
    fn fulfillment_deleted(address: FulfillmentAddress) -> ZomeApiResult<bool> {
        receive_delete_fulfillment(address)
    }

    #[zome_fn("hc_public")]
    fn get_fulfillment(address: FulfillmentAddress) -> ZomeApiResult<ResponseData> {
        receive_get_fulfillment(address)
    }

    #[zome_fn("hc_public")]
    fn query_fulfillments(params: QueryParams) -> ZomeApiResult<Vec<ResponseData>> {
        receive_query_fulfillments(params)
    }

    // :TODO:
    // receive: |from, payload| {
    //     format!("Received: {} from {}", payload, from)
    //   }


}
