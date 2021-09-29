---
to: lib/<%= h.changeCase.snake(zome_name) %>/lib/src/lib.rs
---
/**
 * <%=zome_friendly_name%> zome library API
 *
 * Contains helper methods that can be used to manipulate `<%= h.changeCase.pascal(record_type_name) %>` data
 * structures in either the local Holochain zome, or a separate DNA-local zome.
 *
 * @package Holo-REA
 */
use std::borrow::Cow;
use hdk::error::{ ZomeApiResult, ZomeApiError };

use hdk_records::{
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

use hc_zome_<%= h.changeCase.snake(zome_name) %>_storage_consts::*;
use hc_zome_<%= h.changeCase.snake(zome_name) %>_storage::*;
use hc_zome_<%= h.changeCase.snake(zome_name) %>_rpc::*;

use hc_zome_TODO_storage_consts::{
    TODO_PARENT_OF_LINK_TYPE, TODO_PARENT_OF_LINK_TAG,
};

pub fn handle_create_<%= h.changeCase.snake(record_type_name) %>(<%= h.changeCase.snake(record_type_name) %>: CreateRequest) -> ZomeApiResult<ResponseData> {
    handle_create_<%= h.changeCase.snake(record_type_name) %>(&<%= h.changeCase.snake(record_type_name) %>)
}

pub fn handle_get_<%= h.changeCase.snake(record_type_name) %>(address: <%= h.changeCase.pascal(record_type_name) %>Address) -> ZomeApiResult<ResponseData> {
    handle_get_<%= h.changeCase.snake(record_type_name) %>(&address)
}

pub fn handle_update_<%= h.changeCase.snake(record_type_name) %>(<%= h.changeCase.snake(record_type_name) %>: UpdateRequest) -> ZomeApiResult<ResponseData> {
    handle_update_<%= h.changeCase.snake(record_type_name) %>(&<%= h.changeCase.snake(record_type_name) %>)
}

pub fn handle_delete_<%= h.changeCase.snake(record_type_name) %>(address: <%= h.changeCase.pascal(record_type_name) %>Address) -> ZomeApiResult<bool> {
    delete_record::<Entry>(&address)
}

pub fn handle_query_<%= h.inflection.pluralize(h.changeCase.snake(record_type_name)) %>(params: QueryParams) -> ZomeApiResult<Vec<ResponseData>> {
    handle_query_<%= h.inflection.pluralize(h.changeCase.snake(record_type_name)) %>(&params)
}

fn handle_get_<%= h.changeCase.snake(record_type_name) %>(address: &<%= h.changeCase.pascal(record_type_name) %>Address) -> ZomeApiResult<ResponseData> {
    Ok(construct_response(address, &read_record_entry(address)?, get_link_fields(address)))
}

fn handle_create_<%= h.changeCase.snake(record_type_name) %>(<%= h.changeCase.snake(record_type_name) %>: &CreateRequest) -> ZomeApiResult<ResponseData> {
    let (base_address, entry_resp): (<%= h.changeCase.pascal(record_type_name) %>Address, Entry) = create_record(
        <%= h.changeCase.constant(record_type_name) %>_BASE_ENTRY_TYPE, <%= h.changeCase.constant(record_type_name) %>_ENTRY_TYPE,
        <%= h.changeCase.constant(record_type_name) %>_INITIAL_ENTRY_LINK_TYPE,
        <%= h.changeCase.snake(record_type_name) %>.to_owned(),
    )?;
    Ok(construct_response(&base_address, &entry_resp, get_link_fields(&base_address)))
}

fn handle_update_<%= h.changeCase.snake(record_type_name) %>(<%= h.changeCase.snake(record_type_name) %>: &UpdateRequest) -> ZomeApiResult<ResponseData> {
    let base_address = <%= h.changeCase.snake(record_type_name) %>.get_id();
    let new_entry = update_record(<%= h.changeCase.constant(record_type_name) %>_ENTRY_TYPE, base_address, <%= h.changeCase.snake(record_type_name) %>)?;
    Ok(construct_response(&base_address, &new_entry, get_link_fields(base_address)))
}

fn handle_query_<%= h.inflection.pluralize(h.changeCase.snake(record_type_name)) %>(params: &QueryParams) -> ZomeApiResult<Vec<ResponseData>> {
    let mut entries_result: ZomeApiResult<Vec<(<%= h.changeCase.pascal(record_type_name) %>Address, Option<Entry>)>> = Err(ZomeApiError::Internal("No results found".to_string()));

    // :TODO: replace with real query filter logic
    match &params.parents {
        Some(parents) => {
            entries_result = query_direct_index_with_foreign_key(parents, TODO_PARENT_OF_LINK_TYPE, TODO_PARENT_OF_LINK_TAG);
        },
        _ => (),
    };

    match entries_result {
        Ok(entries) => Ok(
            entries.iter()
                .map(|(entry_base_address, maybe_entry)| {
                    match maybe_entry {
                        Some(entry) => Ok(construct_response(
                            entry_base_address,
                            &entry,
                            get_link_fields(entry_base_address),
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

/// Create response from input DHT primitives
pub fn construct_response<'a>(
    address: &<%= h.changeCase.pascal(record_type_name) %>Address, e: &Entry, (
        // :TODO:
     ): (
        // :TODO:
    ),
) -> ResponseData {
    ResponseData {
        <%= h.changeCase.snake(record_type_name) %>: Response {
            id: address.to_owned(),
            // :TODO:
        }
    }
}

//---------------- READ ----------------

// @see construct_response
pub fn get_link_fields<'a>(<%= h.changeCase.snake(record_type_name) %>: &<%= h.changeCase.pascal(record_type_name) %>Address) -> (
    // :TODO:
) {
    (
        // :TODO:
    )
}
