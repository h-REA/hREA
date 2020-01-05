// * :TODO: abstract remainder of the logic from here into key_helpers.rs in hdk_graph_helpers
use hdk::{
    holochain_json_api::{ json::JsonString, error::JsonError },
    error::{ ZomeApiResult, ZomeApiError },
};
use holochain_json_derive::{ DefaultJson };

use hdk_graph_helpers::{
    records::{
        create_anchored_record,
        read_anchored_record_entry,
        update_anchored_record,
        delete_anchored_record,
    },
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
