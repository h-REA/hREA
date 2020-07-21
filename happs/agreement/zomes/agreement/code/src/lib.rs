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
extern crate serde;
extern crate hdk;
extern crate hdk_proc_macros;

use hdk::prelude::*;
use hdk_proc_macros::zome;

use hc_zome_rea_agreement_defs::{ entry_def, base_entry_def };
use hc_zome_rea_agreement_rpc::*;
use hc_zome_rea_agreement_lib::*;


// Zome entry type wrappers
#[zome]
mod rea_agreement_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData::<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn agreement_entry_def() -> ValidatingEntryType {
        entry_def()
    }

    #[entry_def]
    fn agreement_base_entry_def() -> ValidatingEntryType {
        base_entry_def()
    }

    #[zome_fn("hc_public")]
    fn create_agreement(agreement: CreateRequest) -> ZomeApiResult<ResponseData> {
        receive_create_agreement(agreement)
    }

    #[zome_fn("hc_public")]
    fn get_agreement(address: AgreementAddress) -> ZomeApiResult<ResponseData> {
        receive_get_agreement(address)
    }

    #[zome_fn("hc_public")]
    fn update_agreement(agreement: UpdateRequest) -> ZomeApiResult<ResponseData> {
        receive_update_agreement(agreement)
    }

    #[zome_fn("hc_public")]
    fn delete_agreement(address: AgreementAddress) -> ZomeApiResult<bool> {
        receive_delete_agreement(address)
    }

    // #[zome_fn("hc_public")]
    // fn query_agreements(params: QueryParams) -> ZomeApiResult<Vec<ResponseData>>{
    //     receive_query_agreements(params)
    // }

    // :TODO: wire up remote indexing API if necessary

    // :TODO:
    // receive: |from, payload| {
    //     format!("Received: {} from {}", payload, from)
    // }
}
