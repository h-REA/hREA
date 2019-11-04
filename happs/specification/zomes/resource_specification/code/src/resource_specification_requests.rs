/**
 * Handling for `EconomicResource`-related requests
 */
use std::borrow::Cow;
use hdk::{
    holochain_json_api::{ json::JsonString, error::JsonError },
    error::{
        ZomeApiResult,
        // ZomeApiError
    },
};
use holochain_json_derive::{ DefaultJson };

use hdk_graph_helpers::{
    records::{
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
    // ProcessAddress
};
use vf_specification::identifiers::{
    ECONOMIC_RESOURCE,
    ECONOMIC_RESOURCE_CONFORMING,
    ECONOMIC_RESOURCE_CONFORMING_TAG,
};
use vf_specification::resource_specification::{
    Entry,
    UpdateRequest,
    ResponseData as Response,
    construct_response,
};

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    contains: Option<ResourceSpecificationAddress>,
    contained_in: Option<ResourceSpecificationAddress>,
}

pub fn receive_create_resource_specification(resource: UpdateRequest) -> ZomeApiResult<Response> {
    handle_update_economic_resource(&resource)
}
pub fn receive_get_resource_specification(address: ResourceSpecificationAddress) -> ZomeApiResult<Response> {
    Ok(construct_response(&address, &read_record_entry(&address)?, get_link_fields(&address)))
}

fn get_output_intent_ids<'a>(process: &ResourceSpecificationAddress) -> Cow<'a, Vec<ResourceAddress>> {
    get_linked_remote_addresses_as_type(process, ECONOMIC_RESOURCE_CONFORMING, ECONOMIC_RESOURCE_CONFORMING_TAG)
}
pub fn receive_update_resource_specification(resource: UpdateRequest) -> ZomeApiResult<Response> {
    handle_update_economic_resource(&resource)
}
pub fn receive_delete_resource_specification(address: ResourceSpecificationAddress) -> ZomeApiResult<bool> {
    delete_record::<Entry>(&address)
}

fn get_link_fields<'a>(process: &ResourceSpecificationAddress) -> Option<Cow<'a, Vec<ResourceAddress>>>{
    Some(get_output_intent_ids(process))
}

fn handle_delete_economic_resource(address: &ResourceSpecificationAddress) -> ZomeApiResult<Response> {
    let entry = read_record_entry(&address)?;
    Ok(construct_response(&address, &entry, None))
}

fn handle_update_economic_resource(resource: &UpdateRequest) -> ZomeApiResult<Response> {
    let address = resource.get_id();
    let new_entry = update_record(ECONOMIC_RESOURCE, &address, resource)?;
    // :TODO: handle link fields
    // replace_entry_link_set(address, &resource.get_contained_in(),
    //     RESOURCE_CONTAINED_IN_LINK_TYPE, RESOURCE_CONTAINED_IN_LINK_TAG,
    //     RESOURCE_CONTAINS_LINK_TYPE, RESOURCE_CONTAINS_LINK_TAG,
    // )?;
    Ok(construct_response(address, &new_entry, None))
}

// fn handle_query_economic_resources(params: &QueryParams) -> ZomeApiResult<Vec<Response>> {
//     let mut entries_result: ZomeApiResult<Vec<(ResourceAddress, Option<Entry>)>> = Err(ZomeApiError::Internal("No results found".to_string()));
//     match &params.conforming_resources {
//         Some(conforming_resources) => {
//             entries_result = get_links_and_load_entry_data(
//                 &conforming_resources, ECONOMIC_RESOURCE_CONFORMING, ECONOMIC_RESOURCE_CONFORMING_TAG,
//             );
//         },
//         _ => (),
//     };
// }
