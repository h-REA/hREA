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

use hc_zome_rea_fulfillment_storage::*;
use hc_zome_rea_fulfillment_rpc::*;
use hc_zome_rea_fulfillment_lib::construct_response;

// :SHONK: needed to re-export for zome `entry_defs()` where macro-assigned defs are overridden
pub use hdk_records::CAP_STORAGE_ENTRY_DEF_ID;

/// properties accessor for zome config
fn read_index_zome(conf: DnaConfigSliceObservation) -> Option<String> {
    Some(conf.fulfillment.index_zome)
}

pub fn handle_create_fulfillment<S>(entry_def_id: S, fulfillment: CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str> + std::fmt::Display,
{
    let (meta, fulfillment_address, entry_resp): (_,_, EntryData) = create_record(read_index_zome, &entry_def_id, fulfillment.to_owned())?;

    // link entries in the local DNA
    let e = create_index!(fulfillment.fulfilled_by(fulfillment.get_fulfilled_by()), economic_event.fulfills(&fulfillment_address));
    hdk::prelude::debug!("handle_create_fulfillment::fulfilled_by index (destination) {:?}", e);

    // :TODO: figure out if necessary/desirable to do bidirectional bridging between observation and other planning DNAs

    construct_response(&fulfillment_address, &meta, &entry_resp)
}

pub fn handle_get_fulfillment<S>(entry_def_id: S, address: FulfillmentAddress) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let (meta, base_address, entry) = read_record_entry::<EntryData, EntryStorage, _,_,_>(&entry_def_id, address.as_ref())?;
    construct_response(&base_address, &meta, &entry)
}

pub fn handle_update_fulfillment<S>(entry_def_id: S, fulfillment: UpdateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let (meta, base_address, new_entry, prev_entry): (_, FulfillmentAddress, EntryData, EntryData) = update_record(&entry_def_id, &fulfillment.get_revision_id(), fulfillment.to_owned())?;

    if new_entry.fulfilled_by != prev_entry.fulfilled_by {
        let e = update_index!(
            fulfillment
                .fulfilled_by(&vec![new_entry.fulfilled_by.clone()])
                .not(&vec![prev_entry.fulfilled_by]),
            economic_event.fulfills(&base_address)
        );
        hdk::prelude::debug!("handle_update_fulfillment::fulfilled_by index (destination) {:?}", e);
    }

    construct_response(&base_address, &meta, &new_entry)
}

pub fn handle_delete_fulfillment(revision_id: HeaderHash) -> RecordAPIResult<bool>
{
    // read any referencing indexes
    let (_meta, base_address, fulfillment) = read_record_entry_by_header::<EntryData, EntryStorage, _>(&revision_id)?;

    // handle link fields
    let e = update_index!(fulfillment.fulfilled_by.not(&vec![fulfillment.fulfilled_by]), economic_event.fulfills(&base_address));
    hdk::prelude::debug!("handle_delete_fulfillment::fulfilled_by index (destination) {:?}", e);

    delete_record::<EntryStorage>(&revision_id)
}

/// Properties accessor for zome config.
fn read_economic_event_index_zome(conf: DnaConfigSliceObservation) -> Option<String> {
    Some(conf.fulfillment.economic_event_index_zome)
}

/// Properties accessor for zome config.
fn read_fulfillment_index_zome(conf: DnaConfigSliceObservation) -> Option<String> {
    Some(conf.fulfillment.index_zome)
}
