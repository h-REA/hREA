/**
 * Holo-REA resource specification zome library API
 *
 * Contains helper methods that can be used to manipulate `ResourceSpecification` data
 * structures in either the local Holochain zome, or a separate DNA-local zome.
 *
 * @package Holo-REA
 */
use std::borrow::Cow;
use hdk::error::{ ZomeApiResult, ZomeApiError };

use hdk_graph_helpers::{
    records::{
        create_record,
        read_record_entry,
        update_record,
        delete_record,
    },
    links::{
        get_linked_addresses_as_type,
        get_linked_addresses_with_foreign_key_as_type,
    },
    local_indexes::{
        query_direct_index_with_foreign_key,
        query_direct_remote_index_with_foreign_key,
    },
    remote_indexes::{
        RemoteEntryLinkResponse,
        handle_sync_direct_remote_index_destination,
    },
};

use hc_zome_rea_resource_specification_storage_consts::*;
use hc_zome_rea_resource_specification_storage::*;
use hc_zome_rea_resource_specification_rpc::*;

pub fn receive_create_resource_specification(resource_specification: CreateRequest) -> ZomeApiResult<Response> {
    let (base_address, entry_resp): (ResourceSpecificationAddress, Entry) = create_record(
        ECONOMIC_RESOURCE_SPECIFICATION_BASE_ENTRY_TYPE,
        ECONOMIC_RESOURCE_SPECIFICATION_ENTRY_TYPE,
        ECONOMIC_RESOURCE_SPECIFICATION_INITIAL_ENTRY_LINK_TYPE,
        resource_specification.to_owned(),
    )?;
    Ok(construct_response(&base_address, &entry_resp, None))
}
pub fn receive_get_resource_specification(address: ResourceSpecificationAddress) -> ZomeApiResult<Response> {
    Ok(construct_response(&address, &read_record_entry(&address)?, None))
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

fn handle_update_resource_specification(resource_specification: &UpdateRequest) -> ZomeApiResult<Response> {
    let address = resource_specification.get_id();
    let new_entry = update_record(ECONOMIC_RESOURCE_SPECIFICATION_ENTRY_TYPE, &address, resource_specification)?;
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
                            None,
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

/// Create response from input DHT primitives
pub fn construct_response<'a>(
    address: &ResourceSpecificationAddress,
    e: &Entry,
    // :TODO: link conforming resources in associated link registry DNA module
    _conforming_resources : Option<Cow<'a, Vec<ResourceAddress>>>
) -> ResponseData {
    ResponseData {
        resource_specification: Response {
            // entry fields
            id: address.to_owned(),
            name: e.name.to_owned(),
            image: e.image.to_owned(),
            note: e.note.to_owned(),

            // conforming_resources: conforming_resources.map(Cow::into_owned),
        }
    }
}

//---------------- READ ----------------

// @see construct_response
// pub fn get_link_fields<'a>(resource_specification: &ResourceSpecificationAddress) -> (
//     // :TODO:
// ) {
//     (
//         // :TODO:
//     )
// }
