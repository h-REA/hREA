/**
 * Holo-REA fulfillment zome library API
 *
 * Contains helper methods that can be used to manipulate `Fulfillment` data
 * structures in either the local Holochain zome, or a separate DNA-local zome.
 *
 * Contains functionality for the "destination" side of an "indirect remote index" pair
 * (@see `hdk_records` README).
 *
 * @package Holo-REA
 */
use hdk_records::{
    RecordAPIResult, DataIntegrityError,
    records::{
        create_record,
        read_record_entry,
        update_record,
        delete_record,
    },
    local_indexes::{
        create_index,
        update_index,
        query_index,
    },
};

use hc_zome_rea_fulfillment_storage::*;
use hc_zome_rea_fulfillment_rpc::*;
use hc_zome_rea_fulfillment_lib::construct_response;
use hc_zome_rea_fulfillment_storage_consts::*;
use hc_zome_rea_economic_event_storage_consts::{EVENT_FULFILLS_LINK_TAG};

pub fn receive_create_fulfillment<S>(entry_def_id: S, event_entry_def_id: S, fulfillment: CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    handle_create_fulfillment(entry_def_id, event_entry_def_id, &fulfillment)
}

pub fn receive_get_fulfillment<S>(entry_def_id: S, address: FulfillmentAddress) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    handle_get_fulfillment(entry_def_id, &address)
}

pub fn receive_update_fulfillment<S>(entry_def_id: S, event_entry_def_id: S, fulfillment: UpdateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    handle_update_fulfillment(entry_def_id, event_entry_def_id, &fulfillment)
}

pub fn receive_delete_fulfillment(revision_id: RevisionHash) -> RecordAPIResult<bool> {
    delete_record::<EntryStorage, _>(&revision_id)
}

pub fn receive_query_fulfillments<S>(event_entry_def_id: S, params: QueryParams) -> RecordAPIResult<Vec<ResponseData>>
    where S: AsRef<str>
{
    handle_query_fulfillments(event_entry_def_id, &params)
}

fn handle_create_fulfillment<S>(entry_def_id: S, event_entry_def_id: S, fulfillment: &CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let (revision_id, fulfillment_address, entry_resp): (_,_, EntryData) = create_record(&entry_def_id, fulfillment.to_owned())?;

    // link entries in the local DNA
    let _results = create_index(
        &entry_def_id, &fulfillment_address,
        &event_entry_def_id, fulfillment.get_fulfilled_by(),
        FULFILLMENT_FULFILLEDBY_LINK_TAG, EVENT_FULFILLS_LINK_TAG,
    )?;

    // :TODO: figure out if necessary/desirable to do bidirectional bridging between observation and other planning DNAs

    construct_response(&fulfillment_address, &revision_id, &entry_resp)
}

fn handle_update_fulfillment<S>(entry_def_id: S, event_entry_def_id: S, fulfillment: &UpdateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let (revision_id, base_address, new_entry, prev_entry): (_, FulfillmentAddress, EntryData, EntryData) = update_record(&entry_def_id, &fulfillment.get_revision_id(), fulfillment.to_owned())?;

    if new_entry.fulfilled_by != prev_entry.fulfilled_by {
        let _results = update_index(
            &entry_def_id, &base_address,
            &event_entry_def_id,
            FULFILLMENT_FULFILLEDBY_LINK_TAG, EVENT_FULFILLS_LINK_TAG,
            vec![new_entry.fulfilled_by.clone()].as_slice(), vec![prev_entry.fulfilled_by].as_slice(),
        )?;
    }

    construct_response(&base_address, &revision_id, &new_entry)
}

/// Read an individual fulfillment's details
fn handle_get_fulfillment<S>(entry_def_id: S, address: &FulfillmentAddress) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let (revision, base_address, entry) = read_record_entry::<EntryData, EntryStorage, _,_>(&entry_def_id, address.as_ref())?;
    construct_response(&base_address, &revision, &entry)
}

fn handle_query_fulfillments<S>(event_entry_def_id: S, params: &QueryParams) -> RecordAPIResult<Vec<ResponseData>>
    where S: AsRef<str>
{
    let mut entries_result: RecordAPIResult<Vec<RecordAPIResult<(RevisionHash, FulfillmentAddress, EntryData)>>> = Err(DataIntegrityError::EmptyQuery);

    match &params.fulfilled_by {
        Some(fulfilled_by) => {
            entries_result = query_index::<EntryData, EntryStorage, _,_,_,_>(
                &event_entry_def_id,
                fulfilled_by, EVENT_FULFILLS_LINK_TAG,
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
