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

use vf_observation::type_aliases::{
    ResourceAddress,
};
use vf_observation::identifiers::{
    RESOURCE_ENTRY_TYPE,
    RESOURCE_CONTAINS_LINK_TYPE,
    RESOURCE_CONTAINS_LINK_TAG,
    RESOURCE_CONTAINED_IN_LINK_TYPE,
    RESOURCE_CONTAINED_IN_LINK_TAG,
};
use vf_observation::economic_resource::{
    Entry,
    UpdateRequest,
    ResponseData as Response,
    get_link_fields,
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
    Ok(construct_response(&address, &entry, get_link_fields(&address)))
}

fn handle_update_economic_resource(resource: &UpdateRequest) -> ZomeApiResult<Response> {
    let address = resource.get_id();
    let new_entry = update_record(RESOURCE_ENTRY_TYPE, &address, resource)?;

    // :TODO: handle link fields
    replace_entry_link_set(address, &resource.get_contained_in(),
        RESOURCE_CONTAINED_IN_LINK_TYPE, RESOURCE_CONTAINED_IN_LINK_TAG,
        RESOURCE_CONTAINS_LINK_TYPE, RESOURCE_CONTAINS_LINK_TAG,
    )?;

    // :TODO: optimise this- should pass results from `replace_entry_link_set` instead of retrieving from `get_link_fields` where updates
    Ok(construct_response(address, &new_entry, get_link_fields(address)))
}

fn handle_query_economic_resources(params: &QueryParams) -> ZomeApiResult<Vec<Response>> {
    let mut entries_result: ZomeApiResult<Vec<(ResourceAddress, Option<Entry>)>> = Err(ZomeApiError::Internal("No results found".to_string()));

    match &params.contains {
        Some(contains) => {
            entries_result = get_links_and_load_entry_data(
                &contains, RESOURCE_CONTAINED_IN_LINK_TYPE, RESOURCE_CONTAINED_IN_LINK_TAG,
            );
        },
        _ => (),
    };
    match &params.contained_in {
        Some(contained_in) => {
            entries_result = get_links_and_load_entry_data(
                contained_in, RESOURCE_CONTAINS_LINK_TYPE, RESOURCE_CONTAINS_LINK_TAG,
            );
        },
        _ => (),
    };

    match entries_result {
        Ok(entries) => Ok(
            entries.iter()
                .map(|(entry_base_address, maybe_entry)| {
                    match maybe_entry {
                        Some(entry) => Ok(construct_response(
                            entry_base_address, &entry, get_link_fields(entry_base_address)
                        )),
                        None => Err(ZomeApiError::Internal("referenced entry not found".to_string()))
                    }
                })
                .filter_map(Result::ok)
                .collect()
        ),
        _ => Err(ZomeApiError::Internal("could not load linked addresses".to_string()))
    }
}
