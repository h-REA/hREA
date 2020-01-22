/**
 * Holo-REA satisfaction zome entry type definitions
 *
 * For use in the standard Holo-REA satisfaction zome,
 * or in zomes wishing to embed additional attributes & logic alongside the
 * standard `Satisfaction` data model.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use hc_zome_rea_economic_event_storage_consts::EVENT_BASE_ENTRY_TYPE;
use hc_zome_rea_commitment_storage_consts::COMMITMENT_BASE_ENTRY_TYPE;
use hc_zome_rea_intent_storage_consts::INTENT_BASE_ENTRY_TYPE;
use hc_zome_rea_satisfaction_storage_consts::*;
use hc_zome_rea_satisfaction_storage::Entry;

pub fn entry_def() -> ValidatingEntryType {
    entry!(
        name: SATISFACTION_ENTRY_TYPE,
        description: "Represents many-to-many relationships between intents and commitments or economic events that fully or partially satisfy one or more intents.",
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
        name: SATISFACTION_BASE_ENTRY_TYPE,
        description: "Base anchor for initial satisfaction addresses to provide lookup functionality",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<Address>| {
            Ok(())
        },
        links: [
            to!(
                SATISFACTION_ENTRY_TYPE,
                link_type: SATISFACTION_INITIAL_ENTRY_LINK_TYPE,

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                COMMITMENT_BASE_ENTRY_TYPE,
                link_type: SATISFACTION_SATISFIEDBY_LINK_TYPE,

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                INTENT_BASE_ENTRY_TYPE,
                link_type: SATISFACTION_SATISFIES_LINK_TYPE,

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

/// Used on the upstream side of the link to build reciprocal query indexes
/// :WARNING: incompatible with `base_entry_def`- do not use both in the same DNA!
///
/// :TODO: this should probably be moved out into a separate module, both of which should
/// consume one containing `entry_def`. But... probably, this kind of micro-optimisation
/// will be done by the compiler when configured for production.
///
pub fn remote_entry_def() -> ValidatingEntryType {
    entry!(
        name: SATISFACTION_BASE_ENTRY_TYPE,
        description: "Base anchor for initial satisfaction addresses to provide lookup functionality",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<Address>| {
            Ok(())
        },
        links: [
            to!(
                SATISFACTION_ENTRY_TYPE,
                link_type: SATISFACTION_INITIAL_ENTRY_LINK_TYPE,

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                EVENT_BASE_ENTRY_TYPE,
                link_type: SATISFACTION_SATISFIEDBY_LINK_TYPE,

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
