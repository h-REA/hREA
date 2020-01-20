/**
 * Holo-REA process specification zome entry type definitions
 *
 * For use in the standard Holo-REA process specification zome,
 * or in zomes wishing to embed additional attributes & logic alongside the
 * standard `ProcessSpecification` data model.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use hc_zome_rea_process_specification_storage_consts::*;
use hc_zome_rea_process_specification_storage::Entry;

pub fn entry_def() -> ValidatingEntryType {
    entry!(
        name: PROCESS_SPECIFICATION_ENTRY_TYPE,
        description: "Defines the available types of Processes that are performed in an REA economic network.",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<Entry>| {
            Ok(())
        },
        links: [
        ]
    )
}

pub fn base_entry_def() -> ValidatingEntryType {
    entry!(
        name: PROCESS_SPECIFICATION_BASE_ENTRY_TYPE,
        description: "Base anchor for initial process specification addresses to provide lookup functionality",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<ProcessSpecificationAddress>| {
            Ok(())
        },
        links: [
            to!(
                PROCESS_SPECIFICATION_ENTRY_TYPE,
                link_type: PROCESS_SPECIFICATION_INITIAL_ENTRY_LINK_TYPE,
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
