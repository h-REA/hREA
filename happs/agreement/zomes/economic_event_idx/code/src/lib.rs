#![feature(proc_macro_hygiene)]
/**
 * Holo-REA economic event index zome API definition
 *
 * Provides remote indexing capability for the economic events of agreement records.
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

use vf_core::type_aliases::{ AgreementAddress, EventAddress };
use hc_zome_rea_agreement_storage_consts::{ AGREEMENT_BASE_ENTRY_TYPE, AGREEMENT_EVENTS_LINK_TYPE, AGREEMENT_EVENTS_LINK_TAG };
use hc_zome_rea_economic_event_storage_consts::{ EVENT_BASE_ENTRY_TYPE, EVENT_REALIZATION_OF_LINK_TYPE, EVENT_REALIZATION_OF_LINK_TAG };

#[zome]
mod rea_commitment_index_zome {
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
            name: EVENT_BASE_ENTRY_TYPE,
            description: "Base anchor for external EconomicEvent records to provide lookup functionality",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: |_validation_data: hdk::EntryValidationData<Address>| {
                Ok(())
            },
            links: [
                to!(
                    AGREEMENT_BASE_ENTRY_TYPE,
                    link_type: EVENT_REALIZATION_OF_LINK_TYPE,
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: | _validation_data: hdk::LinkValidationData| {
                        Ok(())
                    }
                ),
                from!(
                    AGREEMENT_BASE_ENTRY_TYPE,
                    link_type: AGREEMENT_EVENTS_LINK_TYPE,
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
    fn index_events(base_entry: AgreementAddress, target_entries: Vec<EventAddress>, removed_entries: Vec<EventAddress>) -> ZomeApiResult<RemoteEntryLinkResponse> {
        handle_sync_direct_remote_index_destination(
            EVENT_BASE_ENTRY_TYPE,
            EVENT_REALIZATION_OF_LINK_TYPE, EVENT_REALIZATION_OF_LINK_TAG,
            AGREEMENT_EVENTS_LINK_TYPE, AGREEMENT_EVENTS_LINK_TAG,
            &base_entry, target_entries, removed_entries
        )
    }
}
