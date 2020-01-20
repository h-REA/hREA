#![feature(proc_macro_hygiene)]
// :TODO: documentation
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate hdk_graph_helpers;

extern crate vf_specification;

mod unit_requests;

use hdk::prelude::*;
use hdk_proc_macros::zome;

use vf_specification::type_aliases::{
    UnitId,
};
use vf_specification::unit::{
    Entry,
    CreateRequest,
    UpdateRequest,
    ResponseData,
};
use unit_requests::{
    QueryParams,
    receive_create_unit,
    receive_get_unit,
    receive_update_unit,
    receive_delete_unit,
    receive_query_units,
};

use vf_specification::identifiers::{
    UNIT_ENTRY_TYPE,
    UNIT_ID_ENTRY_TYPE,
    UNIT_INITIAL_ENTRY_LINK_TYPE,
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
    fn unit_entry_def() -> ValidatingEntryType {
        entry!(
            name: UNIT_ENTRY_TYPE,
            description: "Unit",
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
    fn unit_id_entry_def() -> ValidatingEntryType {
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

    // receive: |from, payload| {
    //   format!("Received: {} from {}", payload, from)
    // }

    #[zome_fn("hc_public")]
    fn create_unit(unit: CreateRequest) -> ZomeApiResult<ResponseData>{
        receive_create_unit(unit)
    }

    #[zome_fn("hc_public")]
    fn get_unit(id: UnitId) -> ZomeApiResult<ResponseData> {
        receive_get_unit(id)
    }

    #[zome_fn("hc_public")]
    fn update_unit(unit: UpdateRequest) -> ZomeApiResult<ResponseData> {
        receive_update_unit(unit)
    }

    #[zome_fn("hc_public")]
    fn delete_unit(id: UnitId) -> ZomeApiResult<bool> {
        receive_delete_unit(id)
    }

    #[zome_fn("hc_public")]
    fn query_units(params: QueryParams) -> ZomeApiResult<Vec<ResponseData>> {
        receive_query_units(params)
    }
}
