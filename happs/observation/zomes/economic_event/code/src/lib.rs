#![feature(proc_macro_hygiene)]
/**
 * Observations zome API definition
 *
 * # Remarks
 *
 * Defines the top-level zome configuration needed by Holochain's build system
 * to bundle the app. This basically involves wiring up the `entry!` type macros
 * and `define_zome!` definition to the standard Rust code in the rest of this
 * module.
 */

extern crate hdk;

use hdk::prelude::*;
use hdk_proc_macros::zome;

use hc_zome_rea_economic_event_defs::{ entry_def, base_entry_def };
use hc_zome_rea_economic_event_lib::*;
use hc_zome_rea_economic_event_structs_rpc::*;
use hc_zome_rea_economic_resource_structs_rpc::CreateRequest as EconomicResourceCreateRequest;

#[zome]
mod rea_economic_event_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData::<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn event_entry_def() -> ValidatingEntryType {
        entry_def()
    }

    #[entry_def]
    fn event_base_entry_def() -> ValidatingEntryType {
        base_entry_def()
    }

    #[zome_fn("hc_public")]
    fn create_event(event: CreateRequest, new_inventoried_resource: Option<EconomicResourceCreateRequest>) -> ZomeApiResult<ResponseData> {
        receive_create_economic_event(event, new_inventoried_resource)
    }

    #[zome_fn("hc_public")]
    fn get_event(address: EventAddress) -> ZomeApiResult<ResponseData> {
        receive_get_economic_event(address)
    }

    #[zome_fn("hc_public")]
    fn update_event(event: UpdateRequest) -> ZomeApiResult<ResponseData> {
        receive_update_economic_event(event)
    }

    #[zome_fn("hc_public")]
    fn delete_event(address: EventAddress) -> ZomeApiResult<bool> {
        receive_delete_economic_event(address)
    }

    #[zome_fn("hc_public")]
    fn query_events(params: QueryParams) -> ZomeApiResult<Vec<ResponseData>> {
        receive_query_events(params)
    }



    // :TODO:
    // receive: |from, payload| {
    //   format!("Received: {} from {}", payload, from)
    // }



}
