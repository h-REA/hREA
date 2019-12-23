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
};

use vf_core::type_aliases::{
    UnitAddress,
};
use vf_specification::identifiers::{
    UNIT_ENTRY_TYPE,
    UNIT_BASE_ENTRY_TYPE,
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
    let (base_address, entry_resp): (UnitAddress, Entry) = create_record(
        UNIT_BASE_ENTRY_TYPE,
        UNIT_ENTRY_TYPE,
        UNIT_INITIAL_ENTRY_LINK_TYPE,
        unit.to_owned(),
    )?;
    Ok(construct_response(&base_address, &entry_resp))
}
pub fn receive_get_unit(id: UnitAddress) -> ZomeApiResult<Response> {
    Ok(construct_response(&id, &read_record_entry(&id)?))
}
pub fn receive_update_unit(unit: UpdateRequest) -> ZomeApiResult<Response> {
    handle_update_unit(&unit)
}
pub fn receive_delete_unit(id: UnitAddress) -> ZomeApiResult<bool> {
    delete_record::<Entry>(&id)
}
pub fn receive_query_units(params: QueryParams) -> ZomeApiResult<Vec<Response>> {
    handle_query_units(&params)
}

fn handle_update_unit(resource: &UpdateRequest) -> ZomeApiResult<Response> {
    let address = resource.get_id();
    let new_entry = update_record(UNIT_ENTRY_TYPE, &address, resource)?;
    Ok(construct_response(address, &new_entry))
}

fn handle_query_units(_params: &QueryParams) -> ZomeApiResult<Vec<Response>> {
    let entries_result: ZomeApiResult<Vec<(UnitAddress, Option<Entry>)>> = Err(ZomeApiError::Internal("No results found".to_string()));

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
