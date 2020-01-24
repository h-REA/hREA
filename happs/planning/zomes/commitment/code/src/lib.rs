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

// :TODO: split to own zome
use hc_zome_rea_commitment_storage_consts::COMMITMENT_BASE_ENTRY_TYPE;
use hc_zome_rea_intent_storage_consts::INTENT_BASE_ENTRY_TYPE;
use hc_zome_rea_process_storage_consts::{
    PROCESS_BASE_ENTRY_TYPE,
    PROCESS_COMMITMENT_INPUTS_LINK_TYPE,
    PROCESS_COMMITMENT_OUTPUTS_LINK_TYPE,
    PROCESS_INTENT_INPUTS_LINK_TYPE,
    PROCESS_INTENT_OUTPUTS_LINK_TYPE,
};

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

    #[entry_def]
    fn process_base_entry_def() -> ValidatingEntryType {
        entry!(
            name: PROCESS_BASE_ENTRY_TYPE,
            description: "Base anchor for processes being linked to in external networks",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: |_validation_data: hdk::EntryValidationData<Address>| {
                Ok(())
            },
            links: [
                to!(
                    COMMITMENT_BASE_ENTRY_TYPE,
                    link_type: PROCESS_COMMITMENT_INPUTS_LINK_TYPE,
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: | _validation_data: hdk::LinkValidationData| {
                        Ok(())
                    }
                ),
                to!(
                    COMMITMENT_BASE_ENTRY_TYPE,
                    link_type: PROCESS_COMMITMENT_OUTPUTS_LINK_TYPE,
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: | _validation_data: hdk::LinkValidationData| {
                        Ok(())
                    }
                ),
                // :TODO: ideally this would be defined on a separate `PROCESS_BASE_ENTRY_TYPE`
                // in the intent zome.
                // This might point to a need to split `Process` functionality out into its own zome
                // within the planning DNA.
                to!(
                    INTENT_BASE_ENTRY_TYPE,
                    link_type: PROCESS_INTENT_INPUTS_LINK_TYPE,
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: | _validation_data: hdk::LinkValidationData| {
                        Ok(())
                    }
                ),
                to!(
                    INTENT_BASE_ENTRY_TYPE,
                    link_type: PROCESS_INTENT_OUTPUTS_LINK_TYPE,
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: | _validation_data: hdk::LinkValidationData| {
                        Ok(())
                    }
                )
            ]
        )
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
