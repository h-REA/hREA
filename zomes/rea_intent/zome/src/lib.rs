#![feature(proc_macro_hygiene)]
/**
 * Holo-REA intent zome API definition
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

use hc_zome_rea_intent_defs::{ entry_def, base_entry_def };
use hc_zome_rea_intent_rpc::*;
use hc_zome_rea_intent_lib::*;

fn validate(validation_data: hdk::EntryValidationData<Entry>) {
    // CREATE
    if let EntryValidationData::Create{ entry, validation_data: _ } = validation_data {
        let record: Entry = entry;
        let result = record.validate_or_fields();
        if result.is_ok() {
            return record.validate_action();
        }
        return result;
    }

    // UPDATE
    if let EntryValidationData::Modify{ new_entry, old_entry: _, old_entry_header: _, validation_data: _ } = validation_data {
        let record: Entry = new_entry;
        let result = record.validate_or_fields();
        if result.is_ok() {
            return record.validate_action();
        }
        return result;
    }

    // DELETE
    // if let EntryValidationData::Delete{ old_entry, old_entry_header: _, validation_data: _ } = validation_data {

    // }

    Ok(())
}

// Zome entry type wrappers
#[zome]
mod rea_intent_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData::<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn intent_entry_def() -> ValidatingEntryType {
        entry_def()
    }

    #[entry_def]
    fn intent_base_entry_def() -> ValidatingEntryType {
        base_entry_def()
    }

    #[zome_fn("hc_public")]
    fn create_intent(intent: CreateRequest) -> ZomeApiResult<ResponseData> {
        receive_create_intent(intent)
    }

    #[zome_fn("hc_public")]
    fn get_intent(address: IntentAddress) -> ZomeApiResult<ResponseData> {
        receive_get_intent(address)
    }

    #[zome_fn("hc_public")]
    fn update_intent(intent: UpdateRequest) -> ZomeApiResult<ResponseData> {
        receive_update_intent(intent)
    }

    #[zome_fn("hc_public")]
    fn delete_intent(address: IntentAddress) -> ZomeApiResult<bool> {
        receive_delete_intent(address)
    }

    #[zome_fn("hc_public")]
    fn query_intents(params: QueryParams) -> ZomeApiResult<Vec<ResponseData>>{
        receive_query_intents(params)
    }

    // :TODO: wire up remote indexing API if necessary

    // :TODO:
    // receive: |from, payload| {
    //     format!("Received: {} from {}", payload, from)
    // }
}
