#![feature(proc_macro_hygiene)]
// :TODO: documentation
extern crate hdk;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate hdk_graph_helpers;

extern crate vf_proposal;

mod proposed_to_requests;

use hdk::prelude::*;
use hdk_proc_macros::zome;

use vf_proposal::type_aliases::{
    ProposedToAddress,
};
use vf_proposal::proposed_to::{
    Entry,
    CreateRequest,
    UpdateRequest,
    ResponseData,
};
use proposed_to_requests::{
    receive_create_proposed_to,
    receive_get_proposed_to,
    receive_update_proposed_to,
    receive_delete_proposed_to,
};

use vf_proposal::identifiers::{
    PROPOSED_TO_ENTRY_TYPE,
    PROPOSED_TO_BASE_ENTRY_TYPE,
    PROPOSED_TO_INITIAL_ENTRY_LINK_TYPE,
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
    fn proposed_to_entry_def() -> ValidatingEntryType {
        entry!(
            name: PROPOSED_TO_ENTRY_TYPE,
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
    fn proposed_to_id_entry_def() -> ValidatingEntryType {
        entry!(
            name: PROPOSED_TO_BASE_ENTRY_TYPE,
            description: "Proposed To Anchor",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: |_validation_data: hdk::EntryValidationData<ProposedToAddress>| {
                Ok(())
            },
            links: [
                to!(
                    PROPOSED_TO_ENTRY_TYPE,
                    link_type: PROPOSED_TO_INITIAL_ENTRY_LINK_TYPE,
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
    fn create_proposed_to(prop_to: CreateRequest) -> ZomeApiResult<ResponseData>{
        receive_create_proposed_to(prop_to)
    }

    #[zome_fn("hc_public")]
    fn get_proposed_to(id: ProposedToAddress) -> ZomeApiResult<ResponseData> {
        receive_get_proposed_to(id)
    }

    #[zome_fn("hc_public")]
    fn update_proposed_to(prop_to: UpdateRequest) -> ZomeApiResult<ResponseData> {
        receive_update_proposed_to(prop_to)
    }

    #[zome_fn("hc_public")]
    fn delete_proposed_to(id: ProposedToAddress) -> ZomeApiResult<bool> {
        receive_delete_proposed_to(id)
    }
}
