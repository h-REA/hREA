/**
 * Holo-REA 'economic event' zome entry & link definitions
 *
 * Definitions for entry & link types, to be used in Holochain zomes
 * which handle the actual data storage. Definitions must be unique within
 * the same DNA.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use vf_planning::identifiers::{
    FULFILLMENT_BASE_ENTRY_TYPE,
    SATISFACTION_BASE_ENTRY_TYPE,
};

use hc_zome_rea_process_storage_consts::PROCESS_BASE_ENTRY_TYPE;
use hc_zome_rea_economic_event_storage_consts::*;
use hc_zome_rea_economic_event_structs_internal::{
    Entry as EconomicEventEntry,
};

pub fn entry_def() -> ValidatingEntryType {
    entry!(
        name: EVENT_ENTRY_TYPE,
        description: "An observed economic flow, as opposed to a flow planned to happen in the future. This could reflect a change in the quantity of an economic resource. It is also defined by its behavior (action) in relation to the economic resource.",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |validation_data: hdk::EntryValidationData<EconomicEventEntry>| {
            // CREATE
            if let EntryValidationData::Create{ entry, validation_data: _ } = validation_data {
                let record: EconomicEventEntry = entry;
                let result = record.validate_or_fields();
                if result.is_ok() {
                    return record.validate_action();
                }
                return result;
            }

            // UPDATE
            if let EntryValidationData::Modify{ new_entry, old_entry: _, old_entry_header: _, validation_data: _ } = validation_data {
                let record: EconomicEventEntry = new_entry;
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
        name: EVENT_BASE_ENTRY_TYPE,
        description: "Base anchor for initial event addresses to provide lookup functionality",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<Address>| {
            Ok(())
        },
        links: [
            to!(
                EVENT_ENTRY_TYPE,
                link_type: EVENT_INITIAL_ENTRY_LINK_TYPE,

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                FULFILLMENT_BASE_ENTRY_TYPE,
                link_type: EVENT_FULFILLS_LINK_TYPE,

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                SATISFACTION_BASE_ENTRY_TYPE,
                link_type: EVENT_SATISFIES_LINK_TYPE,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                PROCESS_BASE_ENTRY_TYPE,
                link_type: EVENT_INPUT_OF_LINK_TYPE,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                PROCESS_BASE_ENTRY_TYPE,
                link_type: EVENT_OUTPUT_OF_LINK_TYPE,
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
