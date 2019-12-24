use hdk::{
    holochain_json_api::{ json::JsonString, error::JsonError },
    holochain_core_types::{
        entry::Entry::App as AppEntry,
    },
    error::{ ZomeApiResult, ZomeApiError },
    commit_entry,
    entry_address,
};
use holochain_json_derive::{ DefaultJson };

use hdk_graph_helpers::{
    records::{
        read_record_entry,
        update_record,
        delete_record,
    },
    links::{
        link_entries,
        get_links_and_load_entry_data_direct,
    },
    identifiers::{
        RECORD_INITIAL_ENTRY_LINK_TAG
    },
};

use vf_core::type_aliases::{
    UnitId,
    Addressable,
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
pub fn receive_delete_unit(_id: UnitId) -> ZomeApiResult<bool> {
    // delete_record::<Entry>(&id)
    Ok(true)
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
    let anchor_entry = AppEntry(UNIT_ID_ENTRY_TYPE.into(), Some(id).into());
    let anchor_address: Addressable = (entry_address(&anchor_entry)?).into();

    // read linked entry
let _ = hdk::debug("WARG reading...");
    let entries: Vec<(Addressable, Option<Entry>)> = get_links_and_load_entry_data_direct(&anchor_address, UNIT_INITIAL_ENTRY_LINK_TYPE, RECORD_INITIAL_ENTRY_LINK_TAG)?;
let _ = hdk::debug(format!("WARG read {:?}", entries));
    let linked_entry = entries.first();

    match linked_entry {
        Some((_, Some(entry))) => Ok(construct_response(id, &entry)),
        _ => Err(ZomeApiError::Internal("Unit not found".into()))
    }
}

fn handle_update_unit(resource: &UpdateRequest) -> ZomeApiResult<Response> {
    let address = resource.get_id();
    let new_entry = update_record(UNIT_ENTRY_TYPE, &address, resource)?;
    Ok(construct_response(address, &new_entry))
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
