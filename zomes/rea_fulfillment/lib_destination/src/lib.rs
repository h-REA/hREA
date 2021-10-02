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
    RecordAPIResult,
    records::{
        create_record,
        read_record_entry,
        read_record_entry_by_header,
        update_record,
        delete_record,
    },
};
use hdk_semantic_indexes_client_lib::{
    create_foreign_index,
    update_foreign_index,
};

use hc_zome_rea_fulfillment_storage::*;
use hc_zome_rea_fulfillment_rpc::*;
use hc_zome_rea_fulfillment_lib::construct_response;
use hc_zome_rea_fulfillment_storage_consts::*;

pub fn handle_create_fulfillment<S>(entry_def_id: S, fulfillment: CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let (revision_id, fulfillment_address, entry_resp): (_,_, EntryData) = create_record(&entry_def_id, fulfillment.to_owned())?;

    // link entries in the local DNA
    let _results = create_foreign_index(
        read_foreign_index_zome,
        &FULFILLMENT_FULFILLEDBY_INDEXING_API_METHOD,
        &fulfillment_address,
        read_foreign_event_index_zome,
        &EVENT_FULFILLS_INDEXING_API_METHOD,
        fulfillment.get_fulfilled_by(),
    )?;

    // :TODO: figure out if necessary/desirable to do bidirectional bridging between observation and other planning DNAs

    construct_response(&fulfillment_address, &revision_id, &entry_resp)
}

pub fn handle_get_fulfillment<S>(entry_def_id: S, address: FulfillmentAddress) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let (revision, base_address, entry) = read_record_entry::<EntryData, EntryStorage, _,_>(&entry_def_id, address.as_ref())?;
    construct_response(&base_address, &revision, &entry)
}

pub fn handle_update_fulfillment<S>(entry_def_id: S, fulfillment: UpdateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let (revision_id, base_address, new_entry, prev_entry): (_, FulfillmentAddress, EntryData, EntryData) = update_record(&entry_def_id, &fulfillment.get_revision_id(), fulfillment.to_owned())?;

    if new_entry.fulfilled_by != prev_entry.fulfilled_by {
        let _results = update_foreign_index(
            read_foreign_index_zome,
            &FULFILLMENT_FULFILLEDBY_INDEXING_API_METHOD,
            &base_address,
            read_foreign_event_index_zome,
            &EVENT_FULFILLS_INDEXING_API_METHOD,
            vec![new_entry.fulfilled_by.clone()].as_slice(),
            vec![prev_entry.fulfilled_by].as_slice(),
        )?;
    }

    construct_response(&base_address, &revision_id, &new_entry)
}

pub fn handle_delete_fulfillment(revision_id: RevisionHash) -> RecordAPIResult<bool>
{
    // read any referencing indexes
    let (base_address, fulfillment) = read_record_entry_by_header::<EntryData, EntryStorage, _>(&revision_id)?;

    // handle link fields
    let _results = update_foreign_index(
        read_foreign_index_zome,
        &FULFILLMENT_FULFILLEDBY_INDEXING_API_METHOD,
        &base_address,
        read_foreign_event_index_zome,
        &EVENT_FULFILLS_INDEXING_API_METHOD,
        vec![].as_slice(),
        vec![fulfillment.fulfilled_by].as_slice(),
    )?;

    delete_record::<EntryStorage, _>(&revision_id)
}

/// Properties accessor for zome config.
fn read_foreign_event_index_zome(conf: DnaConfigSliceObservation) -> Option<String> {
    Some(conf.fulfillment.economic_event_index_zome)
}

/// Properties accessor for zome config.
fn read_foreign_index_zome(conf: DnaConfigSliceObservation) -> Option<String> {
    Some(conf.fulfillment.index_zome)
}
