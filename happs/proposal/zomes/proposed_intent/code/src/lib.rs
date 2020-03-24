#![feature(proc_macro_hygiene)]
extern crate hdk;
extern crate hdk_proc_macros;
/**
 * Holo-REA proposed intents zome API definition
 *
 * Defines the top-level zome configuration needed by Holochain's build system
 * to bundle the app. This basically involves wiring up the helper methods from the
 * related `_lib` module into a packaged zome WASM binary.
 *
 * @package Holo-REA
 */
extern crate serde;

use hdk::prelude::*;
use hdk_proc_macros::zome;

// use hdk_graph_helpers::remote_indexes::RemoteEntryLinkRespnse; // :TODO: wire up remote indexing API if necessary

use hc_zome_rea_proposed_intent_defs::{base_entry_def, entry_def};
use hc_zome_rea_proposed_intent_lib_origin::*;
use hc_zome_rea_proposed_intent_rpc::*;

// Zome entry type wrappers
#[zome]
mod rea_proposed_intent_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn proposed_intent_entry_def() -> ValidatingEntryType {
        entry_def()
    }

    #[entry_def]
    fn proposed_intent_base_entry_def() -> ValidatingEntryType {
        base_entry_def()
    }

    #[zome_fn("hc_public")]
    fn create_proposed_intent(proposed_intent: CreateRequest) -> ZomeApiResult<ResponseData> {
        receive_create_proposed_intent(proposed_intent)
    }

    #[zome_fn("hc_public")]
    fn get_proposed_intent(address: ProposedIntentAddress) -> ZomeApiResult<ResponseData> {
        receive_get_proposed_intent(address)
    }

    #[zome_fn("hc_public")]
    fn delete_proposed_intent(address: ProposedIntentAddress) -> ZomeApiResult<bool> {
        receive_delete_proposed_intent(address)
    }

    // :TODO: wire up remote indexing API if necessary

    // :TODO:
    // receive: |from, payload| {
    //     format!("Received: {} from {}", payload, from)
    // }
}
