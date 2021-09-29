#![feature(proc_macro_hygiene)]
/**
 * Beef industry `EconomicEvent` zome API definition
 *
 * Zome module which extends the standard REA `EconomicEvent` module to provide
 * industry-specific resource attributes applicable in the beef trade.
 *
 * Note that no `EconomicEvent` logic is extended or modified in this example,
 * extension of this module is only necessary for the additions to `EconomicResource`
 * fields.
 *
 * @package Holo-REA
 */
extern crate serde;
extern crate hdk;
extern crate hdk_proc_macros;

use hdk::prelude::*;
use hdk_proc_macros::zome;

use hc_zome_rea_economic_event_defs::{ entry_def, base_entry_def };
use hc_zome_rea_economic_event_lib::*;
use hc_zome_rea_economic_event_rpc::*;
use hc_zome_rea_economic_resource_rpc::CreateRequest as EconomicResourceCreateRequest;

#[zome]
mod beef_economic_event_zome {

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
        handle_create_economic_event(event, new_inventoried_resource)
    }

    #[zome_fn("hc_public")]
    fn get_event(address: EventAddress) -> ZomeApiResult<ResponseData> {
        handle_get_economic_event(address)
    }

    #[zome_fn("hc_public")]
    fn update_event(event: UpdateRequest) -> ZomeApiResult<ResponseData> {
        handle_update_economic_event(event)
    }

    #[zome_fn("hc_public")]
    fn delete_event(address: EventAddress) -> ZomeApiResult<bool> {
        handle_delete_economic_event(address)
    }

    #[zome_fn("hc_public")]
    fn query_events(params: QueryParams) -> ZomeApiResult<Vec<ResponseData>> {
        handle_query_events(params)
    }



    // :TODO:
    // receive: |from, payload| {
    //   format!("Received: {} from {}", payload, from)
    // }



}
