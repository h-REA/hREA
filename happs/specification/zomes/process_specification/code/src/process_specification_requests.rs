use hdk::{
    error::{
        ZomeApiResult,
    },
};

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

pub fn receive_create_process_specification(resource: CreateRequest) -> ZomeApiResult<Response> {
    let (base_address, entry_resp): (ProcessSpecificationAddress, Entry) = create_record(
        PROCESS_SPECIFICATION_BASE_ENTRY_TYPE,
        PROCESS_SPECIFICATION_ENTRY_TYPE,
        PROCESS_SPECIFICATION_INITIAL_ENTRY_LINK_TYPE,
        resource.to_owned(),
    )?;
    Ok(construct_response(&base_address, &entry_resp))
}
pub fn receive_get_process_specification(address: ProcessSpecificationAddress) -> ZomeApiResult<Response> {
    Ok(construct_response(&address, &read_record_entry(&address)?))
}
pub fn receive_update_process_specification(resource: UpdateRequest) -> ZomeApiResult<Response> {
    handle_update_economic_resource(&resource)
}
pub fn receive_delete_process_specification(address: ProcessSpecificationAddress) -> ZomeApiResult<bool> {
    delete_record::<Entry>(&address)
}

fn handle_update_economic_resource(resource: &UpdateRequest) -> ZomeApiResult<Response> {
    let address = resource.get_id();
    let new_entry = update_record(PROCESS_SPECIFICATION_ENTRY_TYPE, &address, resource)?;
    Ok(construct_response(address, &new_entry))
}

