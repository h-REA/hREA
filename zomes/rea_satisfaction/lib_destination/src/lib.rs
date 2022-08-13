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
        read_record_entry_by_action,
        update_record,
        delete_record,
    },
};
use hdk_semantic_indexes_client_lib::*;

use hc_zome_rea_satisfaction_storage::*;
use hc_zome_rea_satisfaction_rpc::*;

use hc_zome_rea_satisfaction_lib::construct_response;



/// properties accessor for zome config
fn read_index_zome(conf: DnaConfigSliceObservation) -> Option<String> {
    Some(conf.satisfaction.index_zome)
}

pub fn handle_create_satisfaction<S>(entry_def_id: S, satisfaction: CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str> + std::fmt::Display,
{
    let (meta, satisfaction_address, entry_resp): (_,_, EntryData) = create_record::<EntryTypes,_,_,_,_,_,_,_,_>(read_index_zome, &entry_def_id, satisfaction.to_owned())?;

    // link entries in the local DNA
    let r1 = create_index!(satisfaction.satisfied_by(satisfaction.get_satisfied_by()), economic_event.satisfies(&satisfaction_address));
    hdk::prelude::debug!("handle_create_satisfaction::satisfied_by index (destination) {:?}", r1);

    // :TODO: figure out if necessary/desirable to do bidirectional bridging between observation and other planning DNAs

    construct_response(&satisfaction_address, &meta, &entry_resp)
}

pub fn handle_get_satisfaction(address: SatisfactionAddress) -> RecordAPIResult<ResponseData>
{
    let (meta, base_address, entry) = read_record_entry::<EntryData, EntryStorage, _>(address.as_ref())?;
    construct_response(&base_address, &meta, &entry)
}

pub fn handle_update_satisfaction(satisfaction: UpdateRequest) -> RecordAPIResult<ResponseData>
{
    let (meta, base_address, new_entry, prev_entry): (_, SatisfactionAddress, EntryData, EntryData) = update_record(&satisfaction.get_revision_id(), satisfaction.to_owned())?;

    if new_entry.satisfied_by != prev_entry.satisfied_by {
        let e = update_index!(
            satisfaction
                .satisfied_by(&vec![new_entry.satisfied_by.to_owned()])
                .not(&vec![prev_entry.satisfied_by]),
            economic_event.satisfies(&base_address)
        );
        hdk::prelude::debug!("handle_update_satisfaction::satisfied_by index (destination) {:?}", e);
    }

    construct_response(&base_address, &meta, &new_entry)
}

pub fn handle_delete_satisfaction(revision_id: ActionHash) -> RecordAPIResult<bool>
{
    // read any referencing indexes
    let (_meta, base_address, entry) = read_record_entry_by_action::<EntryData, EntryStorage, _>(&revision_id)?;

    // handle link fields
    let e = update_index!(satisfaction.satisfied_by.not(&vec![entry.satisfied_by]), economic_event.satisfies(&base_address));
    hdk::prelude::debug!("handle_delete_satisfaction::satisfied_by index (destination) {:?}", e);

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
