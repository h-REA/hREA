#![feature(proc_macro_hygiene)]
/**
 * Holo-REA agreement index zome API definition
 *
 * Provides remote indexing capability for the Agreements of Commitment records.
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

use vf_core::type_aliases::{ CommitmentAddress, AgreementAddress };
use hc_zome_rea_commitment_storage_consts::{ COMMITMENT_BASE_ENTRY_TYPE, COMMITMENT_CLAUSE_OF_LINK_TYPE, COMMITMENT_CLAUSE_OF_LINK_TAG };
use hc_zome_rea_agreement_storage_consts::{ AGREEMENT_BASE_ENTRY_TYPE, AGREEMENT_COMMITMENTS_LINK_TYPE, AGREEMENT_COMMITMENTS_LINK_TAG };

#[zome]
mod rea_agreement_index_zome {
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
            name: AGREEMENT_BASE_ENTRY_TYPE,
            description: "Base anchor for external Agreement records to provide lookup functionality",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: |_validation_data: hdk::EntryValidationData<Address>| {
                Ok(())
            },
            links: [
                to!(
                    COMMITMENT_BASE_ENTRY_TYPE,
                    link_type: AGREEMENT_COMMITMENTS_LINK_TYPE,
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: | _validation_data: hdk::LinkValidationData| {
                        Ok(())
                    }
                ),
                from!(
                    COMMITMENT_BASE_ENTRY_TYPE,
                    link_type: COMMITMENT_CLAUSE_OF_LINK_TYPE,
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
    fn index_commitments(base_entry: CommitmentAddress, target_entries: Vec<AgreementAddress>, removed_entries: Vec<AgreementAddress>) -> ZomeApiResult<RemoteEntryLinkResponse> {
        handle_sync_direct_remote_index_destination(
            AGREEMENT_BASE_ENTRY_TYPE,
            AGREEMENT_COMMITMENTS_LINK_TYPE, AGREEMENT_COMMITMENTS_LINK_TAG,
            COMMITMENT_CLAUSE_OF_LINK_TYPE, COMMITMENT_CLAUSE_OF_LINK_TAG,
            &base_entry, target_entries, removed_entries
        )
    }
}
