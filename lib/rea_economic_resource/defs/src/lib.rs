/**
 * Holo-REA 'economic resource' zome entry type definitions
 *
 * For use in standard REA resource accounting zomes, or in zomes wishing
 * to embed additional attributes & logic alongside the
 * standard `EconomicResource` accounting data.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use vf_specification::identifiers::ECONOMIC_RESOURCE_SPECIFICATION_BASE_ENTRY_TYPE;
use hc_zome_rea_economic_event_structs_internal::identifiers::EVENT_BASE_ENTRY_TYPE;
use hc_zome_rea_economic_resource_structs_internal::identifiers::*;
use hc_zome_rea_economic_resource_structs_internal::Entry;

pub fn entry_def() -> ValidatingEntryType {
    entry!(
        name: RESOURCE_ENTRY_TYPE,
        description: "A resource which is useful to people or the ecosystem.",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |validation_data: hdk::EntryValidationData<Entry>| {
            // CREATE
            if let EntryValidationData::Create{ entry, validation_data: _ } = validation_data {
                let record: Entry = entry;
                return record.validate();
            }

            // UPDATE
            if let EntryValidationData::Modify{ new_entry, old_entry: _, old_entry_header: _, validation_data: _ } = validation_data {
                let record: Entry = new_entry;
                return record.validate();
            }

            // DELETE
            // if let EntryValidationData::Delete{ old_entry, old_entry_header: _, validation_data: _ } = validation_data {

            // }

            Ok(())
        }
    )
}

pub fn base_entry_def() -> ValidatingEntryType {
    entry!(
        name: RESOURCE_BASE_ENTRY_TYPE,
        description: "Base anchor for initial resource addresses to provide lookup functionality",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<Address>| {
            Ok(())
        },
        links: [
            to!(
                RESOURCE_ENTRY_TYPE,
                link_type: RESOURCE_INITIAL_ENTRY_LINK_TYPE,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                RESOURCE_BASE_ENTRY_TYPE,
                link_type: RESOURCE_CONTAINS_LINK_TYPE,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                RESOURCE_BASE_ENTRY_TYPE,
                link_type: RESOURCE_CONTAINED_IN_LINK_TYPE,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                EVENT_BASE_ENTRY_TYPE,
                link_type: RESOURCE_AFFECTED_BY_EVENT_LINK_TYPE,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                ECONOMIC_RESOURCE_SPECIFICATION_BASE_ENTRY_TYPE,
                link_type: RESOURCE_CONFORMS_TO_LINK_TYPE,
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
