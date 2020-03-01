#![feature(proc_macro_hygiene)]
/**
 * Holo-REA process index zome API definition
 *
 * Provides remote indexing capability for processes inside the planning DNA such
 * that intents & commitments can be queried by the processes they are part of.
 *
 * @package Holo-REA
 */
extern crate serde;
extern crate hdk;
extern crate hdk_proc_macros;

use hdk::prelude::*;
use hdk_proc_macros::zome;

use hc_zome_rea_commitment_storage_consts::COMMITMENT_BASE_ENTRY_TYPE;
use hc_zome_rea_intent_storage_consts::INTENT_BASE_ENTRY_TYPE;
use hc_zome_rea_process_storage_consts::{
    PROCESS_BASE_ENTRY_TYPE,
    PROCESS_COMMITMENT_INPUTS_LINK_TYPE,
    PROCESS_COMMITMENT_OUTPUTS_LINK_TYPE,
    PROCESS_INTENT_INPUTS_LINK_TYPE,
    PROCESS_INTENT_OUTPUTS_LINK_TYPE,
};

#[zome]
mod rea_resource_specification_index_zome {
    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData::<AgentId>) {
        Ok(())
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
}
