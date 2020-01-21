/**
 * Holo-REA fulfillment zome entry type definitions
 *
 * For use in the standard Holo-REA fulfillment zome,
 * or in zomes wishing to embed additional attributes & logic alongside the
 * standard `Fulfillment` data model.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use hc_zome_rea_economic_event_storage_consts::EVENT_BASE_ENTRY_TYPE;
use hc_zome_rea_commitment_storage_consts::COMMITMENT_BASE_ENTRY_TYPE;
use hc_zome_rea_fulfillment_storage_consts::*;
use hc_zome_rea_fulfillment_storage::{Entry as FulfillmentEntry};

pub fn entry_def() -> ValidatingEntryType {
    entry!(
        name: FULFILLMENT_ENTRY_TYPE,
        description: "Represents many-to-many relationships between commitments and economic events that fully or partially satisfy one or more commitments.",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<FulfillmentEntry>| {
            Ok(())
        }
    )
}

pub fn base_entry_def() -> ValidatingEntryType {
    entry!(
        name: FULFILLMENT_BASE_ENTRY_TYPE,
        description: "Base anchor for initial fulfillment addresses to provide lookup functionality",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<Address>| {
            Ok(())
        },
        links: [
            to!(
                FULFILLMENT_ENTRY_TYPE,
                link_type: FULFILLMENT_INITIAL_ENTRY_LINK_TYPE,

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                COMMITMENT_BASE_ENTRY_TYPE,
                link_type: FULFILLMENT_FULFILLS_LINK_TYPE,

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
        name: FULFILLMENT_BASE_ENTRY_TYPE,
        description: "Base anchor for initial fulfillment addresses to provide lookup functionality",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<Address>| {
            Ok(())
        },
        links: [
            to!(
                FULFILLMENT_ENTRY_TYPE,
                link_type: FULFILLMENT_INITIAL_ENTRY_LINK_TYPE,

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                EVENT_BASE_ENTRY_TYPE,
                link_type: FULFILLMENT_FULFILLEDBY_LINK_TYPE,

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
