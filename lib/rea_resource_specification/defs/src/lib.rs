/**
 * Holo-REA resource specification zome entry type definitions
 *
 * For use in the standard Holo-REA resource specification zome,
 * or in zomes wishing to embed additional attributes & logic alongside the
 * standard `ResourceSpecification` data model.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use hc_zome_rea_resource_specification_storage_consts::*;
use hc_zome_rea_resource_specification_storage::Entry;

pub fn entry_def() -> ValidatingEntryType {
    entry!(
        name: ECONOMIC_RESOURCE_SPECIFICATION_ENTRY_TYPE,
        description: "Defines the available types of resources that can be found in an REA economic network, as well as their classifications and units of measure.",
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
        name: ECONOMIC_RESOURCE_SPECIFICATION_BASE_ENTRY_TYPE,
        description: "Base anchor for initial resource specification addresses to provide lookup functionality",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<ResourceSpecificationAddress>| {
            Ok(())
        },
        links: [
            to!(
                ECONOMIC_RESOURCE_SPECIFICATION_ENTRY_TYPE,
                link_type: ECONOMIC_RESOURCE_SPECIFICATION_INITIAL_ENTRY_LINK_TYPE,
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
