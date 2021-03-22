#![feature(proc_macro_hygiene)]
/**
 * Commitment zome API definition
 *
 * # Remarks
 *
 * Defines the top-level zome configuration needed by Holochain's build system
 * to bundle the app. This basically involves wiring up the `entry!` type macros
 * and `define_zome!` definition to the standard Rust code in the rest of this
 * module.
 *
 * @package: HoloREA
 * @since:   2019-02-06
 */
extern crate serde;
extern crate hdk;
extern crate hdk_proc_macros;

use hdk::prelude::*;
use hdk_proc_macros::zome;

use hc_zome_rea_commitment_defs::{ entry_def, base_entry_def };
use hc_zome_rea_commitment_rpc::*;
use hc_zome_rea_commitment_lib::*;

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
mod rea_commitment_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData::<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn commitment_entry_def() -> ValidatingEntryType {
        entry_def()
    }

    #[entry_def]
    fn commitment_base_entry_def() -> ValidatingEntryType {
        base_entry_def()
    }

    #[zome_fn("hc_public")]
    fn create_commitment(commitment: CreateRequest) -> ZomeApiResult<ResponseData> {
        receive_create_commitment(commitment)
    }

    #[zome_fn("hc_public")]
    fn get_commitment(address: CommitmentAddress) -> ZomeApiResult<ResponseData> {
        receive_get_commitment(address)
    }

    #[zome_fn("hc_public")]
    fn update_commitment(commitment: UpdateRequest) -> ZomeApiResult<ResponseData> {
        receive_update_commitment(commitment)
    }

    #[zome_fn("hc_public")]
    fn delete_commitment(address: CommitmentAddress) -> ZomeApiResult<bool> {
        receive_delete_commitment(address)
    }

    #[zome_fn("hc_public")]
    fn query_commitments(params: QueryParams) -> ZomeApiResult<Vec<ResponseData>>{
        receive_query_commitments(params)
    }

    // :TODO:
    // receive: |from, payload| {
    //     format!("Received: {} from {}", payload, from)
    //   }

}
