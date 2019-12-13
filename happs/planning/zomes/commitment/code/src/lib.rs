#![feature(proc_macro_hygiene)]
/**
 * Planning zome API definition
 *
 * # Remarks
 *
 * Defines the top-level zome configuration needed by Holochain's build system
 * to bundle the app. This basically involves wiring up the `entry!` type macros
 * and `define_zome!` definition to the standard Rust code in the rest of this
 * module.
 *
 * @package: HoloREA
 * @author:  pospi <pospi@spadgos.com>
 * @since:   2019-02-06
 */

extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate hdk_graph_helpers;
extern crate vf_planning;

mod commitment_requests;

use hdk::prelude::*;
use hdk_proc_macros::zome;

use vf_planning::type_aliases::CommitmentAddress;
use vf_planning::commitment::{
    Entry as CommitmentEntry,
    CreateRequest as CommitmentCreateRequest,
    UpdateRequest as CommitmentUpdateRequest,
    ResponseData as CommitmentResponse,
};

use commitment_requests::{
    QueryParams,
    receive_get_commitment,
    receive_create_commitment,
    receive_update_commitment,
    receive_delete_commitment,
    receive_query_commitments,
};

use vf_planning::identifiers::{
    COMMITMENT_BASE_ENTRY_TYPE,
    COMMITMENT_INITIAL_ENTRY_LINK_TYPE,
    COMMITMENT_ENTRY_TYPE,
    COMMITMENT_FULFILLEDBY_LINK_TYPE,
    FULFILLMENT_BASE_ENTRY_TYPE,
    COMMITMENT_SATISFIES_LINK_TYPE,
    SATISFACTION_BASE_ENTRY_TYPE,
    COMMITMENT_INPUT_OF_LINK_TYPE,
    COMMITMENT_OUTPUT_OF_LINK_TYPE,
};
use vf_observation::identifiers::{
    PROCESS_BASE_ENTRY_TYPE,
    PROCESS_COMMITMENT_INPUTS_LINK_TYPE, PROCESS_COMMITMENT_OUTPUTS_LINK_TYPE,
    PROCESS_INTENT_INPUTS_LINK_TYPE, PROCESS_INTENT_OUTPUTS_LINK_TYPE,
};
use vf_planning::identifiers::{
    INTENT_BASE_ENTRY_TYPE,
};

// Zome entry type wrappers
#[zome]
mod rea_commitment_planning_zome {

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
        entry!(
            name: COMMITMENT_ENTRY_TYPE,
            description: "A planned economic flow that has been promised by an agent to another agent.",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: |validation_data: hdk::EntryValidationData<CommitmentEntry>| {
                // CREATE
                if let EntryValidationData::Create{ entry, validation_data: _ } = validation_data {
                    let record: CommitmentEntry = entry;
                    let result = record.validate_or_fields();
                    if result.is_ok() {
                        return record.validate_action();
                    }
                    return result;
                }
    
                // UPDATE
                if let EntryValidationData::Modify{ new_entry, old_entry: _, old_entry_header: _, validation_data: _ } = validation_data {
                    let record: CommitmentEntry = new_entry;
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
        )
    }

    #[entry_def]
    fn commitment_base_entry_def() -> ValidatingEntryType {
        entry!(
            name: COMMITMENT_BASE_ENTRY_TYPE,
            description: "Base anchor for initial commitment addresses to provide lookup functionality",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: |_validation_data: hdk::EntryValidationData<Address>| {
                Ok(())
            },
            links: [
                to!(
                    COMMITMENT_ENTRY_TYPE,
                    link_type: COMMITMENT_INITIAL_ENTRY_LINK_TYPE,
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: | _validation_data: hdk::LinkValidationData| {
                        Ok(())
                    }
                ),
                to!(
                    FULFILLMENT_BASE_ENTRY_TYPE,
                    link_type: COMMITMENT_FULFILLEDBY_LINK_TYPE,
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: | _validation_data: hdk::LinkValidationData| {
                        Ok(())
                    }
                ),
                to!(
                    SATISFACTION_BASE_ENTRY_TYPE,
                    link_type: COMMITMENT_SATISFIES_LINK_TYPE,
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: | _validation_data: hdk::LinkValidationData| {
                        Ok(())
                    }
                ),
                to!(
                    PROCESS_BASE_ENTRY_TYPE,
                    link_type: COMMITMENT_INPUT_OF_LINK_TYPE,
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: | _validation_data: hdk::LinkValidationData| {
                        Ok(())
                    }
                ),
                to!(
                    PROCESS_BASE_ENTRY_TYPE,
                    link_type: COMMITMENT_OUTPUT_OF_LINK_TYPE,
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
    fn create_commitment(commitment: CommitmentCreateRequest) -> ZomeApiResult<CommitmentResponse>{
        receive_create_commitment(commitment)
    }

    #[zome_fn("hc_public")]
    fn get_commitment(address: CommitmentAddress) -> ZomeApiResult<CommitmentResponse> {
        receive_get_commitment(address)
    }

    #[zome_fn("hc_public")]
    fn update_commitment(commitment: CommitmentUpdateRequest) -> ZomeApiResult<CommitmentResponse> {
        receive_update_commitment(commitment)
    }

    #[zome_fn("hc_public")]
    fn delete_commitment(address: CommitmentAddress) -> ZomeApiResult<bool> {
        receive_delete_commitment(address)
    }

    #[zome_fn("hc_public")]
    fn query_commitments(params: QueryParams ) -> ZomeApiResult<Vec<CommitmentResponse>> {
        receive_query_commitments(params)
    }






    // :TODO:
    // receive: |from, payload| {
    //     format!("Received: {} from {}", payload, from)
    //   }


}







    

//     functions: [
//         create_commitment: {
//             inputs: |commitment: CommitmentCreateRequest|,
//             outputs: |result: ZomeApiResult<CommitmentResponse>|,
//             handler: receive_create_commitment
//         }
//         get_commitment: {
//             inputs: |address: CommitmentAddress|,
//             outputs: |result: ZomeApiResult<CommitmentResponse>|,
//             handler: receive_get_commitment
//         }
//         update_commitment: {
//             inputs: |commitment: CommitmentUpdateRequest|,
//             outputs: |result: ZomeApiResult<CommitmentResponse>|,
//             handler: receive_update_commitment
//         }
//         delete_commitment: {
//             inputs: |address: CommitmentAddress|,
//             outputs: |result: ZomeApiResult<bool>|,
//             handler: receive_delete_commitment
//         }

//         query_commitments: {
//             inputs: |params: QueryParams|,
//             outputs: |result: ZomeApiResult<Vec<CommitmentResponse>>|,
//             handler: receive_query_commitments
//         }
//     ]
// }
