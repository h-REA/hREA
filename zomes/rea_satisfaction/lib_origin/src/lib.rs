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
    RecordAPIResult,
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

pub fn handle_create_satisfaction<S>(entry_def_id: S, satisfaction: CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let (revision_id, satisfaction_address, entry_resp): (_,_, EntryData) = create_record(&entry_def_id, satisfaction.to_owned())?;

    // link entries in the local DNA
    create_index!(satisfaction.satisfies(satisfaction.get_satisfies()), intent.satisfied_by(&satisfaction_address))?;

    // link entries which may be local or remote
    let event_or_commitment = satisfaction.get_satisfied_by();
    if is_satisfiedby_local_commitment(event_or_commitment)? {
        // links to local commitment, create link index pair
        create_index!(satisfaction.satisfied_by(event_or_commitment), commitment.satisfies(&satisfaction_address))?;
    } else {
        // links to remote event, ping associated foreign DNA & fail if there's an error
        // :TODO: consider the implications of this in loosely coordinated multi-network spaces
        call_zome_method(
            event_or_commitment,
            &REPLICATE_CREATE_API_METHOD,
            CreateParams { satisfaction: satisfaction.to_owned() },
        )?;
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
        update_index!(
            satisfaction
                .satisfies(&vec![new_entry.satisfies.to_owned()])
                .not(&vec![prev_entry.satisfies]),
            intent.satisfied_by(&base_address)
        )?;
    }

    // update commitment / event indexes in local and/or remote DNA
    if new_entry.satisfied_by != prev_entry.satisfied_by {
        let new_dna: &DnaHash = new_entry.satisfied_by.as_ref();
        let prev_dna: &DnaHash = prev_entry.satisfied_by.as_ref();
        let same_dna = *new_dna == *prev_dna;

        if same_dna {
            if is_satisfiedby_local_commitment(&prev_entry.satisfied_by)? {
                // both values were local, update the index directly
                update_index!(
                    satisfaction
                        .satisfied_by(&vec![new_entry.satisfied_by.to_owned()])
                        .not(&vec![prev_entry.satisfied_by]),
                    commitment.satisfies(&base_address)
                )?;
            } else {
                // both values were remote and in the same DNA, forward the update
                call_zome_method(
                    &prev_entry.satisfied_by,
                    &REPLICATE_UPDATE_API_METHOD,
                    UpdateParams { satisfaction: satisfaction.to_owned() },
                )?;
            }
        } else {
            if is_satisfiedby_local_commitment(&prev_entry.satisfied_by)? {
                // previous value was local, clear the index directly
                update_index!(satisfaction.satisfied_by.not(&vec![prev_entry.satisfied_by]), commitment.satisfies(&base_address))?;
            } else {
                // previous value was remote, handle the remote update as a deletion
                call_zome_method(
                    &prev_entry.satisfied_by,
                    &REPLICATE_DELETE_API_METHOD,
                    ByHeader { address: satisfaction.get_revision_id().to_owned() },
                )?;
            }

            if is_satisfiedby_local_commitment(&new_entry.satisfied_by)? {
                // new value was local, add the index directly
                update_index!(satisfaction.satisfied_by(&vec![new_entry.satisfied_by.to_owned()]), commitment.satisfies(&base_address))?;
            } else {
                // new value was remote, handle the remote update as a creation
                call_zome_method(
                    &new_entry.satisfied_by,
                    &REPLICATE_CREATE_API_METHOD,
                    CreateParams { satisfaction: CreateRequest {
                        satisfied_by: new_entry.satisfied_by.to_owned(),
                        satisfies: new_entry.satisfies.to_owned(),
                        resource_quantity: new_entry.resource_quantity.to_owned().into(),
                        effort_quantity: new_entry.effort_quantity.to_owned().into(),
                        note: new_entry.note.to_owned().into(),
                    } },
                )?;
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
    update_index!(satisfaction.satisfies.not(&vec![entry.satisfies]), intent.satisfied_by(&base_address))?;

    // update commitment & event indexes in local or remote DNAs
    let event_or_commitment = entry.satisfied_by.to_owned();
    if is_satisfiedby_local_commitment(&event_or_commitment)? {
        update_index!(satisfaction.satisfied_by.not(&vec![entry.satisfied_by]), commitment.satisfies(&base_address))?;
    } else {
        // links to remote event, ping associated foreign DNA & fail if there's an error
        // :TODO: consider the implications of this in loosely coordinated multi-network spaces
        call_zome_method(
            &event_or_commitment,
            &REPLICATE_DELETE_API_METHOD,
            ByHeader { address: revision_id.to_owned() },
        )?;
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
