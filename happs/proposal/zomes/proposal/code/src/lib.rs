#![feature(proc_macro_hygiene)]
// :TODO: documentation
extern crate hdk;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate hdk_graph_helpers;

extern crate vf_proposal;

mod proposal_requests;

use hdk::prelude::*;
use hdk_proc_macros::zome;

use vf_proposal::type_aliases::{
    ProposalAddress,
};
use vf_proposal::proposal::{
    Entry,
    CreateRequest,
    UpdateRequest,
    ResponseData,
};
use proposal_requests::{
    receive_create_proposal,
    receive_get_proposal,
    receive_update_proposal,
    receive_delete_proposal,
};

use vf_proposal::identifiers::{
    PROPOSAL_ENTRY_TYPE,
    PROPOSAL_BASE_ENTRY_TYPE,
    PROPOSAL_INITIAL_ENTRY_LINK_TYPE,
};

#[zome]
mod rea_specification_unit_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData::<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn proposal_entry_def() -> ValidatingEntryType {
        entry!(
            name: PROPOSAL_ENTRY_TYPE,
            description: "Proposed To",
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

    #[entry_def]
    fn proposal_id_entry_def() -> ValidatingEntryType {
        entry!(
            name: PROPOSAL_BASE_ENTRY_TYPE,
            description: "Proposed To Anchor",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: |_validation_data: hdk::EntryValidationData<ProposalAddress>| {
                Ok(())
            },
            links: [
                to!(
                    PROPOSAL_ENTRY_TYPE,
                    link_type: PROPOSAL_INITIAL_ENTRY_LINK_TYPE,
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

    // receive: |from, payload| {
    //   format!("Received: {} from {}", payload, from)
    // }

    #[zome_fn("hc_public")]
    fn create_proposal(prop_to: CreateRequest) -> ZomeApiResult<ResponseData>{
        receive_create_proposal(prop_to)
    }

    #[zome_fn("hc_public")]
    fn get_proposal(id: ProposalAddress) -> ZomeApiResult<ResponseData> {
        receive_get_proposal(id)
    }

    #[zome_fn("hc_public")]
    fn update_proposal(prop_to: UpdateRequest) -> ZomeApiResult<ResponseData> {
        receive_update_proposal(prop_to)
    }

    #[zome_fn("hc_public")]
    fn delete_proposal(id: ProposalAddress) -> ZomeApiResult<bool> {
        receive_delete_proposal(id)
    }
}
