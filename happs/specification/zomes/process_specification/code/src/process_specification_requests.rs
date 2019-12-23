use hdk::{
    holochain_json_api::{ json::JsonString, error::JsonError },
    error::{ ZomeApiResult, ZomeApiError },
};
use holochain_json_derive::{ DefaultJson };

use hdk_graph_helpers::{
    records::{
        create_record,
        read_record_entry,
        update_record,
        delete_record,
    },
};

use vf_core::type_aliases::{
    ProcessSpecificationAddress,
};
use vf_specification::identifiers::{
    PROCESS_SPECIFICATION_ENTRY_TYPE,
    PROCESS_SPECIFICATION_BASE_ENTRY_TYPE,
    PROCESS_SPECIFICATION_INITIAL_ENTRY_LINK_TYPE,
};
use vf_specification::process_specification::{
    Entry,
    UpdateRequest,
    CreateRequest,
    ResponseData as Response,
    construct_response,
};

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
}

pub fn receive_create_process_specification(process_specification: CreateRequest) -> ZomeApiResult<Response> {
    let (base_address, entry_resp): (ProcessSpecificationAddress, Entry) = create_record(
        PROCESS_SPECIFICATION_BASE_ENTRY_TYPE,
        PROCESS_SPECIFICATION_ENTRY_TYPE,
        PROCESS_SPECIFICATION_INITIAL_ENTRY_LINK_TYPE,
        process_specification.to_owned(),
    )?;
    Ok(construct_response(&base_address, &entry_resp))
}
pub fn receive_get_process_specification(address: ProcessSpecificationAddress) -> ZomeApiResult<Response> {
    Ok(construct_response(&address, &read_record_entry(&address)?))
}
pub fn receive_update_process_specification(process_specification: UpdateRequest) -> ZomeApiResult<Response> {
    handle_update_process_specification(&process_specification)
}
pub fn receive_delete_process_specification(address: ProcessSpecificationAddress) -> ZomeApiResult<bool> {
    delete_record::<Entry>(&address)
}
pub fn receive_query_process_specifications(params: QueryParams) -> ZomeApiResult<Vec<Response>> {
    handle_query_process_specifications(&params)
}

fn handle_update_process_specification(process_specification: &UpdateRequest) -> ZomeApiResult<Response> {
    let address = process_specification.get_id();
    let new_entry = update_record(PROCESS_SPECIFICATION_ENTRY_TYPE, &address, process_specification)?;
    Ok(construct_response(address, &new_entry))
}

fn handle_query_process_specifications(_params: &QueryParams) -> ZomeApiResult<Vec<Response>> {
    let entries_result: ZomeApiResult<Vec<(ProcessSpecificationAddress, Option<Entry>)>> = Err(ZomeApiError::Internal("No results found".to_string()));

    // :TODO: implement "all" query and filters

    match entries_result {
        Ok(entries) => Ok(
            entries.iter()
                .map(|(entry_base_address, maybe_entry)| {
                    match maybe_entry {
                        Some(entry) => Ok(construct_response(
                            entry_base_address,
                            &entry,
                        )),
                        None => Err(ZomeApiError::Internal("referenced entry not found".to_string()))
                    }
                })
                .filter_map(Result::ok)
                .collect()
        ),
        Err(e) => Err(e)
    }
}
