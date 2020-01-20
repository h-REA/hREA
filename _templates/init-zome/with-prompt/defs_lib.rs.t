---
to: lib/<%= h.changeCase.snake(zome_name) %>/defs/src/lib.rs
---
/**
 * <%=zome_friendly_name%> zome entry type definitions
 *
 * For use in the standard <%=zome_friendly_name%> zome,
 * or in zomes wishing to embed additional attributes & logic alongside the
 * standard `<%= h.changeCase.pascal(record_type_name) %>` data model.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use hc_zome_TODO_storage_consts::TODO_BASE_ENTRY_TYPE;
use hc_zome_<%= h.changeCase.snake(zome_name) %>_storage_consts::*;
use hc_zome_<%= h.changeCase.snake(zome_name) %>_storage::Entry;
use hc_zome_<%= h.changeCase.snake(zome_name) %>_rpc::<%= h.changeCase.pascal(record_type_name) %>Address;

pub fn entry_def() -> ValidatingEntryType {
    entry!(
        name: <%= h.changeCase.constant(record_type_name) %>_ENTRY_TYPE,
        description: "<%=record_type_description%>",
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
        name: <%= h.changeCase.constant(record_type_name) %>_BASE_ENTRY_TYPE,
        description: "Base anchor for initial <%= h.changeCase.lower(record_type_name) %> addresses to provide lookup functionality",
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
                TODO_BASE_ENTRY_TYPE,
                link_type: <%= h.changeCase.constant(record_type_name) %>_PARENTS_LINK_TYPE,
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
