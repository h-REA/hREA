/**
 * Holo-REA proposed intents: maintains relationships between coordinated proposals and the individual intents that describe their planned enaction. zome entry type definitions
 *
 * For use in the standard Holo-REA proposed intents: maintains relationships between coordinated proposals and the individual intents that describe their planned enaction. zome,
 * or in zomes wishing to embed additional attributes & logic alongside the
 * standard `ProposedIntent` data model.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use hc_zome_rea_proposed_intent_storage::Entry;
use hc_zome_rea_proposed_intent_storage_consts::*;

use hc_zome_rea_intent_storage_consts::INTENT_BASE_ENTRY_TYPE;

use hc_zome_rea_proposal_storage_consts::PROPOSAL_BASE_ENTRY_TYPE;

pub fn entry_def() -> ValidatingEntryType {
    entry!(
        name: PROPOSED_INTENT_ENTRY_TYPE,
        description: "Represents many-to-many relationships between Proposals and Intents, supporting including intents in multiple proposals, as well as a proposal including multiple intents.",
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
        name: PROPOSED_INTENT_BASE_ENTRY_TYPE,
        description: "Base anchor for initial proposedintent addresses to provide lookup functionality",
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
                PROPOSED_INTENT_ENTRY_TYPE,
                link_type: PROPOSED_INTENT_INITIAL_ENTRY_LINK_TYPE,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                INTENT_BASE_ENTRY_TYPE,
                link_type: PROPOSED_INTENT_PUBLISHES_LINK_TYPE,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                PROPOSAL_BASE_ENTRY_TYPE,
                link_type: PROPOSED_INTENT_PUBLISHED_IN_LINK_TYPE,
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
