/**
 * Holo-REA 'process' zome entry type definitions
 *
 * For use in standard REA resource accounting zomes, or in zomes wishing
 * to embed additional attributes & logic alongside the
 * standard `Process` data model.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use hc_zome_rea_process_storage_consts::*;
use hc_zome_rea_process_structs_internal::Entry;

use hc_zome_rea_commitment_storage_consts::COMMITMENT_BASE_ENTRY_TYPE;
use hc_zome_rea_intent_storage_consts::INTENT_BASE_ENTRY_TYPE;
use hc_zome_rea_economic_event_storage_consts::EVENT_BASE_ENTRY_TYPE;

pub fn entry_def() -> ValidatingEntryType {
    entry!(
        name: PROCESS_ENTRY_TYPE,
        description: "An activity that changes inputs into outputs.  It could transform or transport economic resource(s).",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<Entry>| {
            Ok(())
        }
    )
}

pub fn base_entry_def() -> ValidatingEntryType {
    entry!(
        name: PROCESS_BASE_ENTRY_TYPE,
        description: "Base anchor for initial process addresses to provide lookup functionality",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<Address>| {
            Ok(())
        },
        links: [
            to!(
                PROCESS_ENTRY_TYPE,
                link_type: PROCESS_INITIAL_ENTRY_LINK_TYPE,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                EVENT_BASE_ENTRY_TYPE,
                link_type: PROCESS_EVENT_INPUTS_LINK_TYPE,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                EVENT_BASE_ENTRY_TYPE,
                link_type: PROCESS_EVENT_OUTPUTS_LINK_TYPE,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
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
