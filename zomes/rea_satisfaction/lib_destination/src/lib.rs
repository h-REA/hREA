/**
 * Holo-REA satisfaction zome library API
 *
 * Contains helper methods that can be used to manipulate `Satisfaction` data
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

use hc_zome_rea_economic_event_storage_consts::{EVENT_SATISFIES_LINK_TAG};
use hc_zome_rea_satisfaction_storage_consts::*;
use hc_zome_rea_satisfaction_storage::*;
use hc_zome_rea_satisfaction_rpc::*;
use hc_zome_rea_satisfaction_lib::construct_response;

pub fn receive_create_satisfaction<S>(entry_def_id: S, event_entry_def_id: S, satisfaction: CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    handle_create_satisfaction(entry_def_id, event_entry_def_id, &satisfaction)
}

pub fn receive_get_satisfaction<S>(entry_def_id: S, address: SatisfactionAddress) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    handle_get_satisfaction(entry_def_id, &address)
}

pub fn receive_update_satisfaction<S>(entry_def_id: S, event_entry_def_id: S, satisfaction: UpdateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    handle_update_satisfaction(entry_def_id, event_entry_def_id, &satisfaction)
}

pub fn receive_delete_satisfaction(revision_id: RevisionHash) -> RecordAPIResult<bool> {
    delete_record::<EntryStorage, _>(&revision_id)
}

pub fn receive_query_satisfactions<S>(event_entry_def_id: S, params: QueryParams) -> RecordAPIResult<Vec<ResponseData>>
    where S: AsRef<str>
{
    handle_query_satisfactions(event_entry_def_id, &params)
}

fn handle_create_satisfaction<S>(entry_def_id: S, event_entry_def_id: S, satisfaction: &CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let (revision_id, satisfaction_address, entry_resp): (_,_, EntryData) = create_record(&entry_def_id, satisfaction.to_owned())?;

    // link entries in the local DNA
    let _results = create_index(
        &entry_def_id, &satisfaction_address,
        &event_entry_def_id, satisfaction.get_satisfied_by(),
        SATISFACTION_SATISFIEDBY_LINK_TAG, EVENT_SATISFIES_LINK_TAG,
    )?;

    // :TODO: figure out if necessary/desirable to do bidirectional bridging between observation and other planning DNAs

    construct_response(&satisfaction_address, &revision_id, &entry_resp)
}

fn handle_update_satisfaction<S>(entry_def_id: S, event_entry_def_id: S, satisfaction: &UpdateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let (revision_id, base_address, new_entry, prev_entry): (_, SatisfactionAddress, EntryData, EntryData) = update_record(&entry_def_id, &satisfaction.get_revision_id(), satisfaction.to_owned())?;

    if new_entry.satisfied_by != prev_entry.satisfied_by {
        let _results = update_index(
            &entry_def_id, &base_address,
            &event_entry_def_id,
            SATISFACTION_SATISFIEDBY_LINK_TAG, EVENT_SATISFIES_LINK_TAG,
            vec![new_entry.satisfied_by.clone()].as_slice(), vec![prev_entry.satisfied_by].as_slice(),
        )?;
    }

    construct_response(&base_address, &revision_id, &new_entry)
}

/// Read an individual fulfillment's details
fn handle_get_satisfaction<S>(entry_def_id: S, address: &SatisfactionAddress) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let (revision, base_address, entry) = read_record_entry::<EntryData, EntryStorage, _,_>(&entry_def_id, address.as_ref())?;
    construct_response(&base_address, &revision, &entry)
}

fn handle_query_satisfactions<S>(event_entry_def_id: S, params: &QueryParams) -> RecordAPIResult<Vec<ResponseData>>
    where S: AsRef<str>
{
    let mut entries_result: RecordAPIResult<Vec<RecordAPIResult<(RevisionHash, SatisfactionAddress, EntryData)>>> = Err(DataIntegrityError::EmptyQuery);

    match &params.satisfied_by {
        Some(satisfied_by) => {
            entries_result = query_index::<EntryData, EntryStorage, _,_,_,_>(
                &event_entry_def_id,
                satisfied_by, EVENT_SATISFIES_LINK_TAG,
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
fn handle_list_output(entries_result: Vec<RecordAPIResult<(RevisionHash, SatisfactionAddress, EntryData)>>) -> RecordAPIResult<Vec<RecordAPIResult<ResponseData>>>
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
