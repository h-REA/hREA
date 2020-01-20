/**
 * Holo-REA measurement unit zome library API
 *
 * Contains helper methods that can be used to manipulate `Unit` data
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

use hc_zome_rea_unit_storage_consts::*;
use hc_zome_rea_unit_storage::*;
use hc_zome_rea_unit_rpc::*;

pub fn receive_create_unit(unit: CreateRequest) -> ZomeApiResult<Response> {
    handle_create_unit(&unit)
}
pub fn receive_get_unit(id: UnitId) -> ZomeApiResult<Response> {
    handle_get_unit(&id)
}
pub fn receive_update_unit(unit: UpdateRequest) -> ZomeApiResult<Response> {
    handle_update_unit(&unit)
}
pub fn receive_delete_unit(id: UnitId) -> ZomeApiResult<bool> {
    handle_delete_unit(&id)
}
pub fn receive_query_units(params: QueryParams) -> ZomeApiResult<Vec<Response>> {
    handle_query_units(&params)
}

fn handle_create_unit(unit: &CreateRequest) -> ZomeApiResult<Response> {
    let (entry_id, entry_resp) = create_anchored_record(UNIT_ID_ENTRY_TYPE, UNIT_INITIAL_ENTRY_LINK_TYPE, UNIT_ENTRY_TYPE, unit.to_owned())?;
    Ok(construct_response(&entry_id.into(), &entry_resp))
}

fn handle_get_unit(id: &UnitId) -> ZomeApiResult<Response> {
    let entry = read_anchored_record_entry(&UNIT_ID_ENTRY_TYPE.to_string(), UNIT_INITIAL_ENTRY_LINK_TYPE, id.as_ref())?;
    Ok(construct_response(id, &entry))
}

fn handle_update_unit(unit: &UpdateRequest) -> ZomeApiResult<Response> {
    let (new_id, new_entry) = update_anchored_record(UNIT_ID_ENTRY_TYPE, UNIT_INITIAL_ENTRY_LINK_TYPE, UNIT_ENTRY_TYPE, unit)?;
    Ok(construct_response(&new_id.into(), &new_entry))
}

fn handle_delete_unit(id: &UnitId) -> ZomeApiResult<bool> {
    delete_anchored_record::<Entry>(UNIT_ID_ENTRY_TYPE, UNIT_INITIAL_ENTRY_LINK_TYPE, id.as_ref())
}

fn handle_query_units(_params: &QueryParams) -> ZomeApiResult<Vec<Response>> {
    let entries_result: ZomeApiResult<Vec<(UnitId, Option<Entry>)>> = Err(ZomeApiError::Internal("No results found".to_string()));

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

pub fn construct_response<'a>(
    id: &UnitId, e: &Entry
) -> ResponseData {
    ResponseData {
        unit: Response {
            // entry fields
            id: id.to_owned(),
            label: e.label.to_owned(),
            symbol: e.symbol.to_owned(),
        }
    }
}

//---------------- READ ----------------

// @see construct_response
// pub fn get_link_fields<'a>(unit: &UnitAddress) -> (
//     // :TODO:
// ) {
//     (
//         // :TODO:
//     )
// }
