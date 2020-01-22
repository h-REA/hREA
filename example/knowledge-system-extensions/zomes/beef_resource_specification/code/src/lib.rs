#![feature(proc_macro_hygiene)]
/**
 * Custom resource knowledge system with domain-specific beef industry
 * knowledge API.
 *
 * @package Holo-REA
 * @since   2020-01-20
 */
extern crate serde;
extern crate serde_json;
extern crate hdk;
extern crate hdk_proc_macros;

use hdk::prelude::*;
use hdk_proc_macros::zome;

use hc_zome_rea_resource_specification_defs::{ entry_def, base_entry_def };
use hc_zome_rea_resource_specification_rpc::*;
use hc_zome_rea_resource_specification_lib::*;


// Zome entry type wrappers
#[zome]
mod beef_industry_resource_specification_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData::<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn resource_specification_entry_def() -> ValidatingEntryType {
        entry_def()
    }

    #[entry_def]
    fn resource_specification_base_entry_def() -> ValidatingEntryType {
        base_entry_def()
    }

    // :TODO: instead of direct creation, pass through a domain-specific data structure
    #[zome_fn("hc_public")]
    fn create_resource_specification(resource_specification: CreateRequest) -> ZomeApiResult<ResponseData> {
        receive_create_resource_specification(resource_specification)
    }

    #[zome_fn("hc_public")]
    fn get_resource_specification(address: ResourceSpecificationAddress) -> ZomeApiResult<ResponseData> {
        receive_get_resource_specification(address)
    }

    // :TODO: instead of direct updates, pass through a domain-specific data structure
    #[zome_fn("hc_public")]
    fn update_resource_specification(resource_specification: UpdateRequest) -> ZomeApiResult<ResponseData> {
        receive_update_resource_specification(resource_specification)
    }

    // :TODO: handle deletion of domain-specific entries & links
    #[zome_fn("hc_public")]
    fn delete_resource_specification(address: ResourceSpecificationAddress) -> ZomeApiResult<bool> {
        receive_delete_resource_specification(address)
    }

    #[zome_fn("hc_public")]
    fn query_resource_specifications(params: QueryParams) -> ZomeApiResult<Vec<ResponseData>>{
        receive_query_resource_specifications(params)
    }

    // :TODO: domain-specific query APIs for retrieving ResourceSpecification IDs
    //        that can be passed to other zomes to filter against referencing datasets

    // :TODO: wire up remote indexing API to track hashes of all resources in the supply-chain

    // :TODO:
    // receive: |from, payload| {
    //     format!("Received: {} from {}", payload, from)
    // }
}
