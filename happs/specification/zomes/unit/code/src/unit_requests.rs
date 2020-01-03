// * :TODO: abstract remainder of the logic from here into key_helpers.rs in hdk_graph_helpers
use hdk::{
    holochain_json_api::{ json::JsonString, error::JsonError },
    holochain_core_types::{
        entry::Entry::App as AppEntry,
    },
    error::{ ZomeApiResult, ZomeApiError },
    commit_entry,
    entry_address,
    remove_entry,
};
use holochain_json_derive::{ DefaultJson };

use hdk_graph_helpers::{
    records::{
        update_entry_direct,
    },
    links::{
        link_entries,
        get_linked_addresses_as_type,
    },
    keys::{
        delete_key_index_link,
    },
    local_indexes::{
        query_direct_index,
    },
    identifiers::{
        RECORD_INITIAL_ENTRY_LINK_TAG
    },
    type_wrappers::Addressable,
};

use vf_core::type_aliases::{
    UnitId,
};
use vf_specification::identifiers::{
    UNIT_ENTRY_TYPE,
    UNIT_ID_ENTRY_TYPE,
    UNIT_INITIAL_ENTRY_LINK_TYPE,
};
use vf_specification::unit::{
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
    // create ID anchor entry
    let entry_id = unit.get_symbol().to_string();
    let anchor_entry = AppEntry(UNIT_ID_ENTRY_TYPE.into(), Some(entry_id.clone()).into());
    let anchor_address = commit_entry(&anchor_entry)?;

    // write entry data
    let entry_data: Entry = unit.to_owned().into();
    let entry_resp = entry_data.clone();
    let entry = AppEntry(UNIT_ENTRY_TYPE.into(), entry_data.into());
    let address = commit_entry(&entry)?;

    // create link pointer
    link_entries(&anchor_address, &address, UNIT_INITIAL_ENTRY_LINK_TYPE, RECORD_INITIAL_ENTRY_LINK_TAG)?;

    Ok(construct_response(&entry_id.into(), &entry_resp))
}

fn handle_get_unit(id: &UnitId) -> ZomeApiResult<Response> {
    // determine ID anchor entry address
    let anchor_address = get_unit_anchor_address(id);

    // read linked entry
    let entries: Vec<(Addressable, Option<Entry>)> = query_direct_index(&anchor_address, UNIT_INITIAL_ENTRY_LINK_TYPE, RECORD_INITIAL_ENTRY_LINK_TAG)?;
    let linked_entry = entries.first();

    match linked_entry {
        Some((_, Some(entry))) => Ok(construct_response(id, &entry)),
        _ => Err(ZomeApiError::Internal("No entry at this address".into()))
    }
}

fn get_unit_anchor_address(id: &UnitId) -> Addressable {
    let anchor_entry = AppEntry(UNIT_ID_ENTRY_TYPE.into(), Some(id).into());
    entry_address(&anchor_entry).unwrap().into()
}

fn handle_update_unit(unit: &UpdateRequest) -> ZomeApiResult<Response> {
    let current_id = unit.get_id();
    let new_id = unit.get_symbol();
    let anchor_address = get_unit_anchor_address(current_id);

    // read linked entry
    let entries: Vec<Addressable> = get_linked_addresses_as_type(anchor_address.clone(), UNIT_INITIAL_ENTRY_LINK_TYPE, RECORD_INITIAL_ENTRY_LINK_TAG).into_owned();
    let entry_address = entries.first().unwrap();

    // reindex if `symbol` primary key has been updated
    match &new_id {
        Some(updated_id) => if &current_id.as_ref()[..] != &updated_id[..] {
            // remove old index anchor
            delete_key_index_link(anchor_address.as_ref(), entry_address.as_ref(), UNIT_INITIAL_ENTRY_LINK_TYPE, RECORD_INITIAL_ENTRY_LINK_TAG)?;
            remove_entry(anchor_address.as_ref())?;

            // add new index anchor
            let anchor_entry = AppEntry(UNIT_ID_ENTRY_TYPE.into(), Some(new_id).into());
            let new_anchor_address = commit_entry(&anchor_entry)?;
            link_entries(&new_anchor_address, entry_address.as_ref(), UNIT_INITIAL_ENTRY_LINK_TYPE, RECORD_INITIAL_ENTRY_LINK_TAG)?;
        },
        None => (),
    };

    // perform update of actual entry object
    let new_entry: Entry = update_entry_direct(UNIT_ENTRY_TYPE, entry_address, unit)?;
    let final_id: UnitId = new_entry.get_symbol().into();

    Ok(construct_response(&final_id, &new_entry))
}

fn handle_delete_unit(id: &UnitId) -> ZomeApiResult<bool> {
    // determine ID anchor entry address
    let anchor_address = get_unit_anchor_address(id);

    // read linked entry
    let entries: Vec<Addressable> = get_linked_addresses_as_type(anchor_address.clone(), UNIT_INITIAL_ENTRY_LINK_TYPE, RECORD_INITIAL_ENTRY_LINK_TAG).into_owned();
    let check_entry_addr = entries.first();

    match check_entry_addr {
        None => Err(ZomeApiError::Internal("Entry does not exist".to_string())),
        Some(entry_addr) => {
            remove_entry(anchor_address.as_ref())?;
            remove_entry(entry_addr.as_ref())?;
            Ok(true)
        },
    }
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
