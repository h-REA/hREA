---
to: <%=dna_path%>/zomes/<%= h.changeCase.snake(foreign_record_name) %>_idx/code/src/lib.rs
---
#![feature(proc_macro_hygiene)]
/**
 * Holo-REA <%= h.changeCase.lower(foreign_record_name) %> index zome API definition
 *
 * Provides remote indexing capability for the <%= h.inflection.pluralize(foreign_record_name) %> of <%= local_record_name %> records.
 *
 * @package Holo-REA
 */
extern crate serde;
extern crate hdk;
extern crate hdk_proc_macros;

use hdk::prelude::*;
use hdk_proc_macros::zome;

use hdk_graph_helpers::{
    remote_indexes::{
        RemoteEntryLinkResponse,
        handle_sync_direct_remote_index_destination,
    },
};

use vf_core::type_aliases::{ <%= h.changeCase.pascal(local_record_name) %>Address, <%= h.changeCase.pascal(foreign_record_name) %>Address };
use hc_zome_rea_<%= h.changeCase.snake(local_record_name) %>_storage_consts::{ <%= h.changeCase.constant(local_record_name) %>_BASE_ENTRY_TYPE, <%= h.changeCase.constant(local_record_name) %>_TODO_LINK_TYPE, <%= h.changeCase.constant(local_record_name) %>_TODO_LINK_TAG };
use hc_zome_rea_<%= h.changeCase.snake(foreign_record_name) %>_storage_consts::{ <%= h.changeCase.constant(foreign_record_name) %>_BASE_ENTRY_TYPE, <%= h.changeCase.constant(foreign_record_name) %>_TODO_LINK_TYPE, <%= h.changeCase.constant(foreign_record_name) %>_TODO_LINK_TAG };

#[zome]
mod rea_<%= h.changeCase.snake(foreign_record_name) %>_index_zome {
    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData::<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn index_base_entry_def() -> ValidatingEntryType {
        entry!(
            name: <%= h.changeCase.constant(foreign_record_name) %>_BASE_ENTRY_TYPE,
            description: "Base anchor for external <%= h.changeCase.pascal(foreign_record_name) %> records to provide lookup functionality",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: |_validation_data: hdk::EntryValidationData<Address>| {
                Ok(())
            },
            links: [
                to!(
                    <%= h.changeCase.constant(local_record_name) %>_BASE_ENTRY_TYPE,
                    link_type: <%= h.changeCase.constant(foreign_record_name) %>_TODO_RECIPROCAL_LINK_TYPE,
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: | _validation_data: hdk::LinkValidationData| {
                        Ok(())
                    }
                ),
                from!(
                    <%= h.changeCase.constant(local_record_name) %>_BASE_ENTRY_TYPE,
                    link_type: <%= h.changeCase.constant(local_record_name) %>_TODO_FWD_LINK_TYPE,
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

    #[zome_fn("hc_public")]
    fn index_commitments(base_entry: <%= h.changeCase.pascal(local_record_name) %>Address, target_entries: Vec<<%= h.changeCase.pascal(foreign_record_name) %>Address>, removed_entries: Vec<<%= h.changeCase.pascal(foreign_record_name) %>Address>) -> ZomeApiResult<RemoteEntryLinkResponse> {
        handle_sync_direct_remote_index_destination(
            <%= h.changeCase.constant(foreign_record_name) %>_BASE_ENTRY_TYPE,
            <%= h.changeCase.constant(foreign_record_name) %>_TODO_RECIPROCAL_LINK_TYPE, <%= h.changeCase.constant(foreign_record_name) %>_TODO_RECIPROCAL_LINK_TAG,
            <%= h.changeCase.constant(local_record_name) %>_TODO_FWD_LINK_TYPE, <%= h.changeCase.constant(local_record_name) %>_TODO_FWD_LINK_TAG,
            &base_entry, target_entries, removed_entries
        )
    }
}
