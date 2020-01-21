/**
 * Holo-REA commitment zome entry type definitions
 *
 * For use in the standard Holo-REA commitment zome,
 * or in zomes wishing to embed additional attributes & logic alongside the
 * standard `Commitment` data model.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use hc_zome_rea_commitment_storage_consts::*;
use hc_zome_rea_commitment_storage::Entry;

use hc_zome_rea_process_storage_consts::PROCESS_BASE_ENTRY_TYPE;
use hc_zome_rea_satisfaction_storage_consts::SATISFACTION_BASE_ENTRY_TYPE;
use vf_planning::identifiers::{
    FULFILLMENT_BASE_ENTRY_TYPE,
};

pub fn entry_def() -> ValidatingEntryType {
    entry!(
        name: COMMITMENT_ENTRY_TYPE,
        description: "A planned economic flow that has been promised by an agent to another agent.",
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
