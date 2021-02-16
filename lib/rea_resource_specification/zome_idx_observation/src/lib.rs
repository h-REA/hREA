#![feature(proc_macro_hygiene)]
/**
 * Holo-REA resource specification index zome API definition
 *
 * Provides remote indexing capability for resource specifications inside the observation
 * DNA such that resources can be queried by their specifications.
 *
 * @package Holo-REA
 */
extern crate serde;
extern crate hdk;
extern crate hdk_proc_macros;

use hdk::prelude::*;
use hdk_proc_macros::zome;

use hc_zome_rea_resource_specification_storage_consts::{ ECONOMIC_RESOURCE_SPECIFICATION_BASE_ENTRY_TYPE, RESOURCE_SPECIFICATION_CONFORMING_RESOURCE_LINK_TYPE };
use hc_zome_rea_economic_resource_storage_consts::RESOURCE_BASE_ENTRY_TYPE;

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

    // :TODO: move to separate zome
    #[entry_def]
    fn resource_specification_base_entry_def() -> ValidatingEntryType {
        entry!(
            name: ECONOMIC_RESOURCE_SPECIFICATION_BASE_ENTRY_TYPE,
            description: "Base anchor for external ResourceSpecification records to provide lookup functionality",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: |_validation_data: hdk::EntryValidationData<Address>| {
                Ok(())
            },
            links: [
                to!(
                    RESOURCE_BASE_ENTRY_TYPE,
                    link_type: RESOURCE_SPECIFICATION_CONFORMING_RESOURCE_LINK_TYPE,
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
