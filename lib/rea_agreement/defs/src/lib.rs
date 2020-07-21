/**
 * Holo-REA agreement zome entry type definitions
 *
 * For use in the standard Holo-REA agreement zome,
 * or in zomes wishing to embed additional attributes & logic alongside the
 * standard `Agreement` data model.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use hc_zome_rea_agreement_storage_consts::*;
use hc_zome_rea_agreement_storage::Entry;

pub fn entry_def() -> ValidatingEntryType {
    entry!(
        name: AGREEMENT_ENTRY_TYPE,
        description: "Simple module for managing shared agreements between agents in order to reference agreements against commitments made and actions taken.",
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
        name: AGREEMENT_BASE_ENTRY_TYPE,
        description: "Base anchor for initial agreement addresses to provide lookup functionality",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<Address>| {
            Ok(())
        },
        links: [
            to!(
                AGREEMENT_ENTRY_TYPE,
                link_type: AGREEMENT_INITIAL_ENTRY_LINK_TYPE,
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
