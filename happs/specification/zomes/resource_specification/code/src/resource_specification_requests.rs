/**
 * Handling for `EconomicResource`-related requests
 */

use hdk::{
    holochain_json_api::{ json::JsonString, error::JsonError },
    error::{ ZomeApiResult, ZomeApiError },
};
use holochain_json_derive::{ DefaultJson };

use hdk_graph_helpers::{
    records::{
        read_record_entry,
        update_record,
    },
    links::{
        get_links_and_load_entry_data,
        replace_entry_link_set,
    },
};

use vf_specification::type_aliases::{
    ResourceAddress,
};
use vf_specification::identifiers::{
    ECONOMIC_RESOURCE_CONFORMING,
    ECONOMIC_RESOURCE_CONFORMING_TAG,
};
use vf_specification::economic_resource::{
    Entry,
    UpdateRequest,
    ResponseData as Response,
    construct_response,
};

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    contains: Option<ResourceAddress>,
    contained_in: Option<ResourceAddress>,
}

pub fn receive_get_economic_resource(address: ResourceAddress) -> ZomeApiResult<Response> {
    handle_get_economic_resource(&address)
}

pub fn receive_update_economic_resource(resource: UpdateRequest) -> ZomeApiResult<Response> {
    handle_update_economic_resource(&resource)
}

pub fn receive_query_economic_resources(params: QueryParams) -> ZomeApiResult<Vec<Response>> {
    handle_query_economic_resources(&params)
}

fn handle_get_economic_resource(address: &ResourceAddress) -> ZomeApiResult<Response> {
    let entry = read_record_entry(&address)?;
    Ok(construct_response(&address, &entry))
}

fn handle_update_economic_resource(resource: &UpdateRequest) -> ZomeApiResult<Response> {
    let address = resource.get_id();
    let new_entry = update_record(RESOURCE_ENTRY_TYPE, &address, resource)?;

    // :TODO: handle link fields
    // replace_entry_link_set(address, &resource.get_contained_in(),
    //     RESOURCE_CONTAINED_IN_LINK_TYPE, RESOURCE_CONTAINED_IN_LINK_TAG,
    //     RESOURCE_CONTAINS_LINK_TYPE, RESOURCE_CONTAINS_LINK_TAG,
    // )?;
    Ok(construct_response(address, &new_entry))
}

fn handle_query_economic_resources(params: &QueryParams) -> ZomeApiResult<Vec<Response>> {
    let mut entries_result: ZomeApiResult<Vec<(ResourceAddress, Option<Entry>)>> = Err(ZomeApiError::Internal("No results found".to_string()));
    match &params.conforming_resources {
        Some(conforming_resources) => {
            entries_result = get_links_and_load_entry_data(
                &conforming_resources, ECONOMIC_RESOURCE_CONFORMING, ECONOMIC_RESOURCE_CONFORMING_TAG,
            );
        },
        _ => (),
    };
}
