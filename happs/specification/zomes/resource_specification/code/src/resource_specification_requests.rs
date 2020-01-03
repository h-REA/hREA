use std::borrow::Cow;
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
    links::{
        // get_links_and_load_entry_data,
        // replace_entry_link_set,
        get_linked_remote_addresses_as_type,
    },
};

use vf_core::type_aliases::{
    ResourceSpecificationAddress,
    ResourceAddress,
};
use vf_specification::identifiers::{
    ECONOMIC_RESOURCE_SPECIFICATION_ENTRY_TYPE,
    ECONOMIC_RESOURCE_SPECIFICATION_BASE_ENTRY_TYPE,
    ECONOMIC_RESOURCE_SPECIFICATION_INITIAL_ENTRY_LINK_TYPE,
    ECONOMIC_RESOURCE_SPECIFICATION_CONFORMING,
    ECONOMIC_RESOURCE_SPECIFICATION_CONFORMING_TAG,
};
use vf_specification::resource_specification::{
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

pub fn receive_create_resource_specification(resource_specification: CreateRequest) -> ZomeApiResult<Response> {
    let (base_address, entry_resp): (ResourceSpecificationAddress, Entry) = create_record(
        ECONOMIC_RESOURCE_SPECIFICATION_BASE_ENTRY_TYPE,
        ECONOMIC_RESOURCE_SPECIFICATION_ENTRY_TYPE,
        ECONOMIC_RESOURCE_SPECIFICATION_INITIAL_ENTRY_LINK_TYPE,
        resource_specification.to_owned(),
    )?;
    Ok(construct_response(&base_address, &entry_resp, get_link_fields(&base_address)))
}
pub fn receive_get_resource_specification(address: ResourceSpecificationAddress) -> ZomeApiResult<Response> {
    Ok(construct_response(&address, &read_record_entry(&address)?, get_link_fields(&address)))
}

fn get_conforming<'a>(spec: &ResourceSpecificationAddress) -> Cow<'a, Vec<ResourceAddress>> {
    get_linked_remote_addresses_as_type(spec, ECONOMIC_RESOURCE_SPECIFICATION_CONFORMING, ECONOMIC_RESOURCE_SPECIFICATION_CONFORMING_TAG)
}
pub fn receive_update_resource_specification(resource_specification: UpdateRequest) -> ZomeApiResult<Response> {
    handle_update_resource_specification(&resource_specification)
}
pub fn receive_delete_resource_specification(id: ResourceSpecificationAddress) -> ZomeApiResult<bool> {
    delete_record::<Entry>(&id)
}
pub fn receive_query_resource_specifications(params: QueryParams) -> ZomeApiResult<Vec<Response>> {
    handle_query_resource_specifications(&params)
}

fn get_link_fields<'a>(spec: &ResourceSpecificationAddress) -> Option<Cow<'a, Vec<ResourceAddress>>>{
    Some(get_conforming(spec))
}

fn handle_update_resource_specification(resource_specification: &UpdateRequest) -> ZomeApiResult<Response> {
    let address = resource_specification.get_id();
    let new_entry = update_record(ECONOMIC_RESOURCE_SPECIFICATION_ENTRY_TYPE, &address, resource_specification)?;
    // :TODO: handle link fields
    // replace_entry_link_set(address, &resource.get_contained_in(),
    //     RESOURCE_CONTAINED_IN_LINK_TYPE, RESOURCE_CONTAINED_IN_LINK_TAG,
    //     RESOURCE_CONTAINS_LINK_TYPE, RESOURCE_CONTAINS_LINK_TAG,
    // )?;
    Ok(construct_response(address, &new_entry, None))
}

fn handle_query_resource_specifications(_params: &QueryParams) -> ZomeApiResult<Vec<Response>> {
    let entries_result: ZomeApiResult<Vec<(ResourceSpecificationAddress, Option<Entry>)>> = Err(ZomeApiError::Internal("No results found".to_string()));

    // :TODO: implement "all" query and filters

    match entries_result {
        Ok(entries) => Ok(
            entries.iter()
                .map(|(entry_base_address, maybe_entry)| {
                    match maybe_entry {
                        Some(entry) => Ok(construct_response(
                            entry_base_address,
                            &entry,
                            get_link_fields(&entry_base_address),
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
