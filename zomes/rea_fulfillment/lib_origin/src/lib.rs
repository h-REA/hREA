/**
 * Holo-REA fulfillment zome library API
 *
 * Contains helper methods that can be used to manipulate `Fulfillment` data
 * structures in either the local Holochain zome, or a separate DNA-local zome.
 *
 * Contains functionality for the "origin" side of an "indirect remote index" pair
 * (@see `hdk_records` README).

 * @package Holo-REA
 */
use hdk_records::{
    RecordAPIResult, OtherCellResult, DataIntegrityError,
    records::{
        create_record,
        read_record_entry,
        read_record_entry_by_header,
        update_record,
        delete_record,
    },
    local_indexes::{
        create_index,
        update_index,
        query_index,
    },
    rpc::call_zome_method,
};

use hc_zome_rea_commitment_storage_consts::{COMMITMENT_FULFILLEDBY_LINK_TAG};
use hc_zome_rea_fulfillment_storage_consts::*;
use hc_zome_rea_fulfillment_storage::*;
use hc_zome_rea_fulfillment_rpc::*;
use hc_zome_rea_fulfillment_lib::construct_response;

pub fn receive_create_fulfillment<S>(entry_def_id: S, commitment_entry_def_id: S, fulfillment: CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    handle_create_fulfillment(entry_def_id, commitment_entry_def_id, &fulfillment)
}

pub fn receive_get_fulfillment<S>(entry_def_id: S, address: FulfillmentAddress) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    handle_get_fulfillment(entry_def_id, &address)
}

pub fn receive_update_fulfillment<S>(entry_def_id: S, commitment_entry_def_id: S, fulfillment: UpdateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    handle_update_fulfillment(entry_def_id, commitment_entry_def_id, &fulfillment)
}

pub fn receive_delete_fulfillment<S>(entry_def_id: S, commitment_entry_def_id: S, address: RevisionHash) -> RecordAPIResult<bool>
    where S: AsRef<str>
{
    handle_delete_fulfillment(entry_def_id, commitment_entry_def_id, &address)
}

pub fn receive_query_fulfillments<S>(entry_def_id: S, params: QueryParams) -> RecordAPIResult<Vec<ResponseData>>
    where S: AsRef<str>
{
    handle_query_fulfillments(entry_def_id, &params)
}

fn handle_create_fulfillment<S>(entry_def_id: S, commitment_entry_def_id: S, fulfillment: &CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let (revision_id, fulfillment_address, entry_resp): (_,_, EntryData) = create_record(&entry_def_id, fulfillment.to_owned())?;

    // link entries in the local DNA
    let _results = create_index(
        &entry_def_id, &fulfillment_address,
        &commitment_entry_def_id, fulfillment.get_fulfills(),
        FULFILLMENT_FULFILLS_LINK_TAG, COMMITMENT_FULFILLEDBY_LINK_TAG,
    )?;

    // update in the associated foreign DNA as well
    let _pingback: OtherCellResult<ResponseData> = call_zome_method(
        fulfillment.get_fulfills(),
        &REPLICATE_CREATE_API_METHOD,
        CreateParams { fulfillment: fulfillment.to_owned() },
    );
    // :TODO: report any error

    construct_response(&fulfillment_address, &revision_id, &entry_resp)
}

/// Read an individual fulfillment's details
fn handle_get_fulfillment<S>(entry_def_id: S, address: &FulfillmentAddress) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let (revision, base_address, entry) = read_record_entry::<EntryData, EntryStorage, _,_>(&entry_def_id, address.as_ref())?;
    construct_response(&base_address, &revision, &entry)
}

fn handle_update_fulfillment<S>(entry_def_id: S, commitment_entry_def_id: S, fulfillment: &UpdateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let (revision_id, base_address, new_entry, prev_entry): (_, FulfillmentAddress, EntryData, EntryData) = update_record(&entry_def_id, &fulfillment.get_revision_id(), fulfillment.to_owned())?;

    // update commitment indexes in local DNA
    if new_entry.fulfills != prev_entry.fulfills {
        let _results = update_index(
            &entry_def_id, &base_address,
            &commitment_entry_def_id,
            FULFILLMENT_FULFILLS_LINK_TAG, COMMITMENT_FULFILLEDBY_LINK_TAG,
            vec![new_entry.fulfills.clone()].as_slice(), vec![prev_entry.fulfills].as_slice(),
        )?;
    }

    // update fulfillment records in remote DNA (and by proxy, event indexes in remote DNA)
    if new_entry.fulfilled_by != prev_entry.fulfilled_by {
        let _pingback: OtherCellResult<ResponseData> = call_zome_method(
            // :TODO: update to intelligently call remote DNAs if new & old target record are not in same network
            &prev_entry.fulfilled_by,
            &REPLICATE_UPDATE_API_METHOD,
            UpdateParams { fulfillment: fulfillment.to_owned() },
        );
        // :TODO: report any error
    }

    construct_response(&base_address, &revision_id, &new_entry)
}

fn handle_delete_fulfillment<S>(entry_def_id: S, commitment_entry_def_id: S, revision_id: &RevisionHash) -> RecordAPIResult<bool>
    where S: AsRef<str>
{
    let (base_address, entry) = read_record_entry_by_header::<EntryData, EntryStorage, _>(&revision_id)?;

    // update commitment indexes in local DNA
    let _results = update_index(
        &entry_def_id, &base_address,
        &commitment_entry_def_id,
        FULFILLMENT_FULFILLS_LINK_TAG, COMMITMENT_FULFILLEDBY_LINK_TAG,
        vec![].as_slice(), vec![entry.fulfills].as_slice(),
    )?;

    // update fulfillment records in remote DNA (and by proxy, event indexes in remote DNA)
    let _pingback: OtherCellResult<ResponseData> = call_zome_method(
        &entry.fulfilled_by,
        &REPLICATE_DELETE_API_METHOD,
        ByHeader { address: revision_id.to_owned() },
    );
    // :TODO: report any error

    delete_record::<EntryStorage, _>(&revision_id)
}

fn handle_query_fulfillments<S>(commitment_entry_def_id: S, params: &QueryParams) -> RecordAPIResult<Vec<ResponseData>>
    where S: AsRef<str>
{
    let mut entries_result: RecordAPIResult<Vec<RecordAPIResult<(RevisionHash, FulfillmentAddress, EntryData)>>> = Err(DataIntegrityError::EmptyQuery);

    // :TODO: proper search logic, not mutually exclusive ID filters
    match &params.fulfills {
        Some(fulfills) => {
            entries_result = query_index::<EntryData, EntryStorage, _,_,_,_>(
                &commitment_entry_def_id,
                fulfills, COMMITMENT_FULFILLEDBY_LINK_TAG,
            );
        },
        _ => (),
    };

    match entries_result {
        Err(DataIntegrityError::EmptyQuery) => Ok(vec![]),
        Err(e) => Err(e),
        _ => {
            Ok(handle_list_output(entries_result?)?.iter().cloned()
                .filter_map(Result::ok)
                .collect()
            )
        },
    }
}

// :DUPE: query-list-output-no-links
fn handle_list_output(entries_result: Vec<RecordAPIResult<(RevisionHash, FulfillmentAddress, EntryData)>>) -> RecordAPIResult<Vec<RecordAPIResult<ResponseData>>>
{
    Ok(entries_result.iter()
        .cloned()
        .filter_map(Result::ok)
        .map(|(revision_id, entry_base_address, entry)| {
            construct_response(
                &entry_base_address, &revision_id, &entry,
            )
        })
        .collect()
    )
}
