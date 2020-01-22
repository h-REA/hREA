/**
 * Holo-REA intent zome entry type definitions
 *
 * For use in the standard Holo-REA intent zome,
 * or in zomes wishing to embed additional attributes & logic alongside the
 * standard `Intent` data model.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use hc_zome_rea_satisfaction_storage_consts::SATISFACTION_BASE_ENTRY_TYPE;
use hc_zome_rea_process_storage_consts::PROCESS_BASE_ENTRY_TYPE;
use hc_zome_rea_intent_storage_consts::*;
use hc_zome_rea_intent_storage::Entry;

pub fn entry_def() -> ValidatingEntryType {
    entry!(
        name: INTENT_ENTRY_TYPE,
        description: "A planned economic flow, which can lead to economic events (sometimes through commitments).",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |validation_data: hdk::EntryValidationData<Entry>| {
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
    )
}

pub fn base_entry_def() -> ValidatingEntryType {
    entry!(
        name: INTENT_BASE_ENTRY_TYPE,
        description: "Base anchor for initial intent addresses to provide lookup functionality",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<Address>| {
            Ok(())
        },
        links: [
            to!(
                INTENT_ENTRY_TYPE,
                link_type: INTENT_INITIAL_ENTRY_LINK_TYPE,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                SATISFACTION_BASE_ENTRY_TYPE,
                link_type: INTENT_SATISFIEDBY_LINK_TYPE,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                PROCESS_BASE_ENTRY_TYPE,
                link_type: INTENT_INPUT_OF_LINK_TYPE,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                PROCESS_BASE_ENTRY_TYPE,
                link_type: INTENT_OUTPUT_OF_LINK_TYPE,
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
