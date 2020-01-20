#![feature(proc_macro_hygiene)]
/**
 * Holo-REA measurement unit zome API definition
 *
 * Defines the top-level zome configuration needed by Holochain's build system
 * to bundle the app. This basically involves wiring up the helper methods from the
 * related `_lib` module into a packaged zome WASM binary.
 *
 * @package Holo-REA
 */
extern crate serde;
extern crate hdk;
extern crate hdk_proc_macros;

use hdk::prelude::*;
use hdk_proc_macros::zome;

use hc_zome_rea_unit_defs::{ entry_def, id_anchor_entry_def };
use hc_zome_rea_unit_storage_consts::*;
use hc_zome_rea_unit_rpc::*;
use hc_zome_rea_unit_lib::*;


// Zome entry type wrappers
#[zome]
mod rea_unit_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData::<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn unit_entry_def() -> ValidatingEntryType {
        entry_def()
    }

    #[entry_def]
    fn unit_base_entry_def() -> ValidatingEntryType {
        id_anchor_entry_def()
    }

    #[zome_fn("hc_public")]
    fn create_unit(unit: CreateRequest) -> ZomeApiResult<ResponseData>{
        receive_create_unit(unit)
    }

    #[zome_fn("hc_public")]
    fn get_unit(id: UnitId) -> ZomeApiResult<ResponseData> {
        receive_get_unit(id)
    }

    #[zome_fn("hc_public")]
    fn update_unit(unit: UpdateRequest) -> ZomeApiResult<ResponseData> {
        receive_update_unit(unit)
    }

    #[zome_fn("hc_public")]
    fn delete_unit(id: UnitId) -> ZomeApiResult<bool> {
        receive_delete_unit(id)
    }

    #[zome_fn("hc_public")]
    fn query_units(params: QueryParams) -> ZomeApiResult<Vec<ResponseData>> {
        receive_query_units(params)
    }

    // :TODO: wire up remote indexing API if necessary

    // :TODO:
    // receive: |from, payload| {
    //     format!("Received: {} from {}", payload, from)
    // }
}
