/**
 * Holo-REA satisfaction zome library API
 *
 * Contains helper methods that can be used to manipulate `Satisfaction` data
 * structures in either the local Holochain zome, or a separate DNA-local zome.
 *
 * Contains functionality for the "origin" side of an "indirect remote index" pair
 * (@see `hdk_records` README).
 *
 * @package Holo-REA
 */
use paste::paste;
use hdk::prelude::*;
use crate::holo_hash::DnaHash;
use hdk_records::{
    RecordAPIResult, OtherCellResult,
    records::{
        create_record,
        read_record_entry,
        read_record_entry_by_header,
        update_record,
        delete_record,
    },
    rpc::call_zome_method,
};
use hdk_semantic_indexes_client_lib::*;

use hc_zome_rea_satisfaction_storage_consts::*;
use hc_zome_rea_satisfaction_storage::*;
use hc_zome_rea_satisfaction_rpc::*;
use hc_zome_rea_satisfaction_lib::construct_response;

// :SHONK: needed to re-export for zome `entry_defs()` where macro-assigned defs are overridden
pub use hdk_records::CAP_STORAGE_ENTRY_DEF_ID;

/// properties accessor for zome config
fn read_index_zome(conf: DnaConfigSlicePlanning) -> Option<String> {
    Some(conf.satisfaction.index_zome)
}

pub fn handle_create_satisfaction<S>(entry_def_id: S, satisfaction: CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str> + std::fmt::Display,
{
    let (revision_id, satisfaction_address, entry_resp): (_,_, EntryData) = create_record(read_index_zome, &entry_def_id, satisfaction.to_owned())?;

    // link entries in the local DNA
    let r1 = create_index!(satisfaction.satisfies(satisfaction.get_satisfies()), intent.satisfied_by(&satisfaction_address));
    hdk::prelude::debug!("handle_create_satisfaction::satisfies index (origin) {:?}", r1);

    // link entries which may be local or remote
    let event_or_commitment = satisfaction.get_satisfied_by();
    if is_satisfiedby_local_commitment(event_or_commitment)? {
      // links to local commitment, create link index pair
      let r2 = create_index!(satisfaction.satisfied_by(event_or_commitment), commitment.satisfies(&satisfaction_address));
      hdk::prelude::debug!("handle_create_satisfaction::satisfied_by index (origin) {:?}", r2);
    } else {
      // links to remote event, ping associated foreign DNA & fail if there's an error
      // :TODO: consider the implications of this in loosely coordinated multi-network spaces
      // we assign a type to the response so that call_zome_method can
      // effectively deserialize the response without failing
      let result: OtherCellResult<ResponseData> = call_zome_method(
        event_or_commitment,
        &REPLICATE_CREATE_API_METHOD,
        CreateParams { satisfaction: satisfaction.to_owned() },
      );
      hdk::prelude::debug!("handle_create_satisfaction::call_zome_method::{:?} {:?}", REPLICATE_CREATE_API_METHOD, result);
    }

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

    // update intent indexes in local DNA
    if new_entry.satisfies != prev_entry.satisfies {
        let e = update_index!(
            satisfaction
                .satisfies(&vec![new_entry.satisfies.to_owned()])
                .not(&vec![prev_entry.satisfies]),
            intent.satisfied_by(&base_address)
        );
        hdk::prelude::debug!("handle_update_satisfaction::satisfies index (origin) {:?}", e);
    }

    // update commitment / event indexes in local and/or remote DNA
    if new_entry.satisfied_by != prev_entry.satisfied_by {
        let new_dna: &DnaHash = new_entry.satisfied_by.as_ref();
        let prev_dna: &DnaHash = prev_entry.satisfied_by.as_ref();
        let same_dna = *new_dna == *prev_dna;

        if same_dna {
            if is_satisfiedby_local_commitment(&prev_entry.satisfied_by)? {
                // both values were local, update the index directly
                let e = update_index!(
                    satisfaction
                        .satisfied_by(&vec![new_entry.satisfied_by.to_owned()])
                        .not(&vec![prev_entry.satisfied_by]),
                    commitment.satisfies(&base_address)
                );
                hdk::prelude::debug!("handle_update_satisfaction::satisfied_by index (origin) {:?}", e);
            } else {
                // both values were remote and in the same DNA, forward the update
                let result: OtherCellResult<ResponseData> = call_zome_method(
                    &prev_entry.satisfied_by,
                    &REPLICATE_UPDATE_API_METHOD,
                    UpdateParams { satisfaction: satisfaction.to_owned() },
                );
                hdk::prelude::debug!("handle_update_satisfaction::call_zome_method::{:?} {:?}", REPLICATE_UPDATE_API_METHOD, result);
            }
        } else {
            if is_satisfiedby_local_commitment(&prev_entry.satisfied_by)? {
                // previous value was local, clear the index directly
                let e = update_index!(satisfaction.satisfied_by.not(&vec![prev_entry.satisfied_by]), commitment.satisfies(&base_address));
                hdk::prelude::debug!("handle_update_satisfaction::satisfied_by index (origin) {:?}", e);
            } else {
                // previous value was remote, handle the remote update as a deletion
                let result: OtherCellResult<ResponseData> = call_zome_method(
                    &prev_entry.satisfied_by,
                    &REPLICATE_DELETE_API_METHOD,
                    ByHeader { address: satisfaction.get_revision_id().to_owned() },
                );
                hdk::prelude::debug!("handle_update_satisfaction::call_zome_method::{:?} {:?}", REPLICATE_DELETE_API_METHOD, result);
            }

            if is_satisfiedby_local_commitment(&new_entry.satisfied_by)? {
                // new value was local, add the index directly
                let e = update_index!(satisfaction.satisfied_by(&vec![new_entry.satisfied_by.to_owned()]), commitment.satisfies(&base_address));
                hdk::prelude::debug!("handle_update_satisfaction::satisfied_by index (origin) {:?}", e);
            } else {
                // new value was remote, handle the remote update as a creation
                let result: OtherCellResult<ResponseData> = call_zome_method(
                    &new_entry.satisfied_by,
                    &REPLICATE_CREATE_API_METHOD,
                    CreateParams { satisfaction: CreateRequest {
                        satisfied_by: new_entry.satisfied_by.to_owned(),
                        satisfies: new_entry.satisfies.to_owned(),
                        resource_quantity: new_entry.resource_quantity.to_owned().into(),
                        effort_quantity: new_entry.effort_quantity.to_owned().into(),
                        note: new_entry.note.to_owned().into(),
                    } },
                );
                hdk::prelude::debug!("handle_update_satisfaction::call_zome_method::{:?} {:?}", REPLICATE_CREATE_API_METHOD, result);
            }
        }

        // :TODO: ensure correct number of operations succeeded
    }

    construct_response(&base_address, &revision_id, &new_entry)
}

pub fn handle_delete_satisfaction(revision_id: HeaderHash) -> RecordAPIResult<bool>
{
    let (base_address, entry) = read_record_entry_by_header::<EntryData, EntryStorage, _>(&revision_id)?;

    // update intent indexes in local DNA
    let e = update_index!(satisfaction.satisfies.not(&vec![entry.satisfies]), intent.satisfied_by(&base_address));
    hdk::prelude::debug!("handle_delete_satisfaction::satisfies index (origin) {:?}", e);

    // update commitment & event indexes in local or remote DNAs
    let event_or_commitment = entry.satisfied_by.to_owned();
    if is_satisfiedby_local_commitment(&event_or_commitment)? {
        let e = update_index!(satisfaction.satisfied_by.not(&vec![entry.satisfied_by]), commitment.satisfies(&base_address));
        hdk::prelude::debug!("handle_delete_satisfaction::satisfied_by index (origin) {:?}", e);
    } else {
        // links to remote event, ping associated foreign DNA & fail if there's an error
        // :TODO: consider the implications of this in loosely coordinated multi-network spaces
        let result: OtherCellResult<ResponseData> = call_zome_method(
            &event_or_commitment,
            &REPLICATE_DELETE_API_METHOD,
            ByHeader { address: revision_id.to_owned() },
        );
        hdk::prelude::debug!("handle_delete_satisfaction::call_zome_method::{:?} {:?}", REPLICATE_DELETE_API_METHOD, result);
    }

    delete_record::<EntryStorage>(&revision_id)
}

fn is_satisfiedby_local_commitment(event_or_commitment: &EventOrCommitmentAddress) -> RecordAPIResult<bool> {
    let this_dna = dna_info()?.hash;
    let target_dna: &DnaHash = event_or_commitment.as_ref();

    Ok(this_dna == *target_dna)
}

/// Properties accessor for zome config.
fn read_satisfaction_index_zome(conf: DnaConfigSlicePlanning) -> Option<String> {
    Some(conf.satisfaction.index_zome)
}

/// Properties accessor for zome config.
fn read_intent_index_zome(conf: DnaConfigSlicePlanning) -> Option<String> {
    Some(conf.satisfaction.intent_index_zome)
}

/// Properties accessor for zome config.
fn read_commitment_index_zome(conf: DnaConfigSlicePlanning) -> Option<String> {
    Some(conf.satisfaction.commitment_index_zome)
}
