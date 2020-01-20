/**
 * Holo-REA measurement unit zome entry type definitions
 *
 * For use in the standard Holo-REA measurement unit zome,
 * or in zomes wishing to embed additional attributes & logic alongside the
 * standard `Unit` data model.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use hc_zome_rea_unit_storage_consts::*;
use hc_zome_rea_unit_storage::Entry;
use hc_zome_rea_unit_rpc::UnitId;

pub fn entry_def() -> ValidatingEntryType {
    entry!(
        name: UNIT_ENTRY_TYPE,
        description: "Units of measure used to quantify resources.",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<Entry>| {
            Ok(())
        },
        links: [
        ]
    )
}

pub fn id_anchor_entry_def() -> ValidatingEntryType {
    entry!(
        name: UNIT_ID_ENTRY_TYPE,
        description: "Unit ID (anchor)",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<UnitId>| {
            Ok(())
        },
        links: [
            to!(
                UNIT_ENTRY_TYPE,
                link_type: UNIT_INITIAL_ENTRY_LINK_TYPE,
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
