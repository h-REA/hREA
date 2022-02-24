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
use paste::paste;
use hdk_records::{
    RecordAPIResult,
    records::{
        create_record,
        read_record_entry,
        read_record_entry_by_header,
        update_record,
        delete_record,
    },
};
use hdk_semantic_indexes_client_lib::*;

use hc_zome_rea_satisfaction_storage::*;
use hc_zome_rea_satisfaction_rpc::*;
use hc_zome_rea_satisfaction_lib::construct_response;

pub fn handle_create_satisfaction<S>(entry_def_id: S, satisfaction: CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let (revision_id, satisfaction_address, entry_resp): (_,_, EntryData) = create_record(&entry_def_id, satisfaction.to_owned())?;

    // link entries in the local DNA
    create_index!(Local(satisfaction.satisfied_by(satisfaction.get_satisfied_by()), economic_event.satisfies(&satisfaction_address)))?;

    // :TODO: figure out if necessary/desirable to do bidirectional bridging between observation and other planning DNAs

    construct_response(&satisfaction_address, &revision_id, &entry_resp)
}

pub fn handle_get_satisfaction<S>(entry_def_id: S, address: SatisfactionAddress) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let (revision, base_address, entry) = read_record_entry::<EntryData, EntryStorage, _,_,_>(&entry_def_id, address.as_ref())?;
    construct_response(&base_address, &revision, &entry)
}

pub fn handle_update_satisfaction<S>(entry_def_id: S, satisfaction: UpdateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let (revision_id, base_address, new_entry, prev_entry): (_, SatisfactionAddress, EntryData, EntryData) = update_record(&entry_def_id, &satisfaction.get_revision_id(), satisfaction.to_owned())?;

    if new_entry.satisfied_by != prev_entry.satisfied_by {
        update_index!(Local(
            satisfaction
                .satisfied_by(&vec![new_entry.satisfied_by.to_owned()])
                .not(&vec![prev_entry.satisfied_by]),
            economic_event.satisfies(&base_address)
        ))?;
    }

    construct_response(&base_address, &revision_id, &new_entry)
}

pub fn handle_delete_satisfaction(revision_id: HeaderHash) -> RecordAPIResult<bool>
{
    // read any referencing indexes
    let (base_address, entry) = read_record_entry_by_header::<EntryData, EntryStorage, _>(&revision_id)?;

    // handle link fields
    update_index!(Local(satisfaction.satisfied_by.not(&vec![entry.satisfied_by]), economic_event.satisfies(&base_address)))?;

    delete_record::<EntryStorage>(&revision_id)
}

/// Properties accessor for zome config.
fn read_satisfaction_index_zome(conf: DnaConfigSliceObservation) -> Option<String> {
    Some(conf.satisfaction.index_zome)
}

/// Properties accessor for zome config.
fn read_economic_event_index_zome(conf: DnaConfigSliceObservation) -> Option<String> {
    Some(conf.satisfaction.economic_event_index_zome)
}
