/**
 * Holo-REA proposal addressees zome entry type definitions
 *
 * For use in the standard Holo-REA proposal addressees zome,
 * or in zomes wishing to embed additional attributes & logic alongside the
 * standard `ProposedTo` data model.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

// use hc_zome_TODO_storage_consts::TODO_BASE_ENTRY_TYPE;
use hc_zome_rea_proposed_to_storage::Entry;
use hc_zome_rea_proposed_to_storage_consts::*;
use hc_zome_rea_proposal_storage_consts::PROPOSAL_BASE_ENTRY_TYPE;

pub fn entry_def() -> ValidatingEntryType {
    entry!(
        name: PROPOSED_TO_ENTRY_TYPE,
        description: "An agent to which the proposal is to be published.  A proposal can be published to many agents.",
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
        name: PROPOSED_TO_BASE_ENTRY_TYPE,
        description: "Base anchor for initial proposedto addresses to provide lookup functionality",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<Address>| {
            Ok(())
        },
        links: [
            // :TODO: replace with final link definitions
            to!(
                PROPOSED_TO_ENTRY_TYPE,
                link_type: PROPOSED_TO_INITIAL_ENTRY_LINK_TYPE ,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                PROPOSAL_BASE_ENTRY_TYPE,
                link_type: PROPOSED_TO_PROPOSED_LINK_TYPE,
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
