/**
 * Holo-REA process specification zome library API
 *
 * Contains helper methods that can be used to manipulate `ProcessSpecification` data
 * structures in either the local Holochain zome, or a separate DNA-local zome.
 *
 * @package Holo-REA
 */
use hdk::error::{ ZomeApiResult, ZomeApiError };

use hdk_records::{
    records::{
        create_record,
        read_record_entry,
        update_record,
        delete_record,
    },
};

use hc_zome_rea_process_specification_storage_consts::*;
use hc_zome_rea_process_specification_storage::*;
use hc_zome_rea_process_specification_rpc::*;

pub fn receive_create_process_specification(process_specification: CreateRequest) -> ZomeApiResult<ResponseData> {
    let (base_address, entry_resp): (ProcessSpecificationAddress, Entry) = create_record(
        PROCESS_SPECIFICATION_BASE_ENTRY_TYPE,
        PROCESS_SPECIFICATION_ENTRY_TYPE,
        PROCESS_SPECIFICATION_INITIAL_ENTRY_LINK_TYPE,
        process_specification.to_owned(),
    )?;
    Ok(construct_response(&base_address, &entry_resp))
}
pub fn receive_get_process_specification(address: ProcessSpecificationAddress) -> ZomeApiResult<ResponseData> {
    Ok(construct_response(&address, &read_record_entry(&address)?))
}
pub fn receive_update_process_specification(process_specification: UpdateRequest) -> ZomeApiResult<ResponseData> {
    handle_update_process_specification(&process_specification)
}
pub fn receive_delete_process_specification(address: ProcessSpecificationAddress) -> ZomeApiResult<bool> {
    delete_record::<Entry>(&address)
}
pub fn receive_query_process_specifications(params: QueryParams) -> ZomeApiResult<Vec<ResponseData>> {
    handle_query_process_specifications(&params)
}

fn handle_update_process_specification(process_specification: &UpdateRequest) -> ZomeApiResult<ResponseData> {
    let address = process_specification.get_id();
    let new_entry = update_record(PROCESS_SPECIFICATION_ENTRY_TYPE, &address, process_specification)?;
    Ok(construct_response(address, &new_entry))
}

fn handle_query_process_specifications(_params: &QueryParams) -> ZomeApiResult<Vec<ResponseData>> {
    let entries_result: ZomeApiResult<Vec<(ProcessSpecificationAddress, Option<Entry>)>> = Err(ZomeApiError::Internal("No results found".to_string()));

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


/// Create response from input DHT primitives
pub fn construct_response<'a>(
    address: &ProcessSpecificationAddress, e: &Entry,
) -> ResponseData {
    ResponseData {
        process_specification: Response {
            id: address.to_owned(),
            name: e.name.to_owned(),
            note: e.note.to_owned(),
        }
    }
}

//---------------- READ ----------------

// @see construct_response
// pub fn get_link_fields<'a>(process_specification: &ProcessSpecificationAddress) -> (
//     // :TODO:
// ) {
//     (
//         // :TODO:
//     )
// }
