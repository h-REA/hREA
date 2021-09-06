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
use hdk::prelude::*;
use hdk_records::{
    RecordAPIResult, DataIntegrityError,
    records::{
        create_record,
        read_record_entry,
        read_record_entry_by_header,
        update_record,
        delete_record,
    },
    local_indexes::{
        query_index,
    },
    foreign_indexes::{
        create_foreign_index,
        update_foreign_index,
    },
};

use hc_zome_rea_economic_event_storage_consts::{EVENT_SATISFIES_LINK_TAG};
use hc_zome_rea_satisfaction_storage_consts::*;
use hc_zome_rea_satisfaction_storage::*;
use hc_zome_rea_satisfaction_rpc::*;
use hc_zome_rea_satisfaction_lib::construct_response;

pub fn receive_create_satisfaction<S>(entry_def_id: S, satisfaction: CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    handle_create_satisfaction(entry_def_id, &satisfaction)
}

pub fn receive_get_satisfaction<S>(entry_def_id: S, address: SatisfactionAddress) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    handle_get_satisfaction(entry_def_id, &address)
}

pub fn receive_update_satisfaction<S>(entry_def_id: S, satisfaction: UpdateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    handle_update_satisfaction(entry_def_id, &satisfaction)
}

pub fn receive_delete_satisfaction(revision_id: RevisionHash) -> RecordAPIResult<bool> {
    // read any referencing indexes
    let (base_address, entry) = read_record_entry_by_header::<EntryData, EntryStorage, _>(&revision_id)?;

    // handle link fields
    let _results = update_foreign_index(
        read_foreign_index_zome,
        &SATISFACTION_SATISFIEDBY_INDEXING_API_METHOD,
        &base_address,
        read_foreign_event_index_zome,
        &EVENT_INDEXING_API_METHOD,
        vec![].as_slice(), vec![entry.satisfied_by].as_slice(),
    )?;

    delete_record::<EntryStorage, _>(&revision_id)
}

/// Properties accessor for zome config.
fn read_foreign_index_zome(conf: DnaConfigSliceObservation) -> Option<String> {
    Some(conf.satisfaction.index_zome)
}

/// Properties accessor for zome config.
fn read_foreign_event_index_zome(conf: DnaConfigSliceObservation) -> Option<String> {
    Some(conf.satisfaction.economic_event_index_zome)
}

fn handle_create_satisfaction<S>(entry_def_id: S, satisfaction: &CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let (revision_id, satisfaction_address, entry_resp): (_,_, EntryData) = create_record(&entry_def_id, satisfaction.to_owned())?;

    // link entries in the local DNA
    let _results = create_foreign_index(
        read_foreign_index_zome,
        &SATISFACTION_SATISFIEDBY_INDEXING_API_METHOD,
        &satisfaction_address,
        read_foreign_event_index_zome,
        &EVENT_INDEXING_API_METHOD,
        satisfaction.get_satisfied_by(),
    )?;

    // :TODO: figure out if necessary/desirable to do bidirectional bridging between observation and other planning DNAs

    construct_response(&satisfaction_address, &revision_id, &entry_resp)
}

fn handle_update_satisfaction<S>(entry_def_id: S, satisfaction: &UpdateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let (revision_id, base_address, new_entry, prev_entry): (_, SatisfactionAddress, EntryData, EntryData) = update_record(&entry_def_id, &satisfaction.get_revision_id(), satisfaction.to_owned())?;

    if new_entry.satisfied_by != prev_entry.satisfied_by {
        let _results = update_foreign_index(
        read_foreign_index_zome,
            &SATISFACTION_SATISFIEDBY_INDEXING_API_METHOD,
            &base_address,
            read_foreign_event_index_zome,
            &EVENT_INDEXING_API_METHOD,
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

const READ_FN_NAME: &str = "get_satisfaction";

pub fn generate_query_handler<S, C, F>(
    foreign_zome_name_from_config: F,
    event_entry_def_id: S,
) -> impl FnOnce(&QueryParams) -> RecordAPIResult<Vec<ResponseData>>
    where S: AsRef<str>,
        C: std::fmt::Debug,
        SerializedBytes: TryInto<C, Error = SerializedBytesError>,
        F: Fn(C) -> Option<String>,
{
    move |params| {
        let mut entries_result: RecordAPIResult<Vec<RecordAPIResult<ResponseData>>> = Err(DataIntegrityError::EmptyQuery);

        match &params.satisfied_by {
            Some(satisfied_by) => {
                entries_result = query_index::<ResponseData, SatisfactionAddress, C,F,_,_,_,_>(
                    &event_entry_def_id,
                    satisfied_by, EVENT_SATISFIES_LINK_TAG,
                    &foreign_zome_name_from_config, &READ_FN_NAME,
                );
            },
            _ => (),
        };

        // :TODO: return errors for UI, rather than filtering
        Ok(entries_result?.iter()
            .cloned()
            .filter_map(Result::ok)
            .collect())
    }
}
