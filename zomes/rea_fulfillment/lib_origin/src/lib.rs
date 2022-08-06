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
use paste::paste;
use hdk_records::{
    RecordAPIResult, OtherCellResult,
    MaybeUndefined,
    records::{
        create_record,
        read_record_entry,
        read_record_entry_by_action,
        update_record,
        delete_record,
    },
    rpc::call_zome_method,
};
use hdk_semantic_indexes_client_lib::*;

use hc_zome_rea_fulfillment_storage_consts::*;
use hc_zome_rea_fulfillment_storage::*;
use hc_zome_rea_fulfillment_rpc::*;
use hc_zome_rea_fulfillment_integrity::*;
use hc_zome_rea_fulfillment_lib::construct_response;

// :SHONK: needed to re-export for zome `entry_defs()` where macro-assigned defs are overridden
pub use hdk_records::CAP_STORAGE_ENTRY_DEF_ID;

/// properties accessor for zome config
fn read_index_zome(conf: DnaConfigSlicePlanning) -> Option<String> {
    Some(conf.fulfillment.index_zome)
}

pub fn handle_create_fulfillment<S>(entry_def_id: S, fulfillment: CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str> + std::fmt::Display,
{
    let (meta, fulfillment_address, entry_resp): (_,_, EntryData) = create_record(read_index_zome, &entry_def_id, fulfillment.to_owned())?;

    // link entries in the local DNA
    let e = create_index!(fulfillment.fulfills(fulfillment.get_fulfills()), commitment.fulfilled_by(&fulfillment_address));
    hdk::prelude::debug!("handle_create_fulfillment::fulfills index (origin) {:?}", e);

    // :TODO: report any error
    // update in the associated foreign DNA as well
    let pingback: OtherCellResult<ResponseData> = call_zome_method(
        fulfillment.get_fulfilled_by(),
        &REPLICATE_CREATE_API_METHOD,
        CreateParams { fulfillment: CreateRequest {
            fulfilled_by: entry_resp.fulfilled_by.to_owned(),
            fulfills: entry_resp.fulfills.to_owned(),
            resource_quantity: entry_resp.resource_quantity.to_owned().into(),
            effort_quantity: entry_resp.effort_quantity.to_owned().into(),
            note: entry_resp.note.to_owned().into(),
            nonce: MaybeUndefined::Some(entry_resp._nonce.to_owned()),
        } },
    );
    hdk::prelude::debug!("handle_create_fulfillment::call_zome_method::{:?} {:?}", REPLICATE_CREATE_API_METHOD, pingback);

    construct_response(&fulfillment_address, &meta, &entry_resp)
}

pub fn handle_get_fulfillment(address: FulfillmentAddress) -> RecordAPIResult<ResponseData>
{
    let (meta, base_address, entry) = read_record_entry::<EntryData, EntryStorage, _,_,_>(address.as_ref())?;
    construct_response(&base_address, &meta, &entry)
}

pub fn handle_update_fulfillment(fulfillment: UpdateRequest) -> RecordAPIResult<ResponseData>
{
    let (meta, base_address, new_entry, prev_entry): (_, FulfillmentAddress, EntryData, EntryData) = update_record(&fulfillment.get_revision_id(), fulfillment.to_owned())?;

    // update commitment indexes in local DNA
    if new_entry.fulfills != prev_entry.fulfills {
        let e = update_index!(
            fulfillment
                .fulfills(&vec![new_entry.fulfills.clone()])
                .not(&vec![prev_entry.fulfills]),
            commitment.fulfilled_by(&base_address)
        );
        hdk::prelude::debug!("handle_update_fulfillment::fulfills index (origin) {:?}", e);
    }

    // update fulfillment records in remote DNA (and by proxy, event indexes in remote DNA)
    if new_entry.fulfilled_by != prev_entry.fulfilled_by {
        let pingback: OtherCellResult<ResponseData> = call_zome_method(
            // :TODO: update to intelligently call remote DNAs if new & old target record are not in same network
            &prev_entry.fulfilled_by,
            &REPLICATE_UPDATE_API_METHOD,
            UpdateParams { fulfillment: fulfillment.to_owned() },
        );
        // :TODO: report any error
        hdk::prelude::debug!("handle_update_fulfillment::call_zome_method::{:?} {:?}", REPLICATE_UPDATE_API_METHOD, pingback);
    }

    construct_response(&base_address, &meta, &new_entry)
}

pub fn handle_delete_fulfillment(revision_id: ActionHash) -> RecordAPIResult<bool>
{
    let (_meta, base_address, entry) = read_record_entry_by_action::<EntryData, EntryStorage, _>(&revision_id)?;

    // update commitment indexes in local DNA
    let e = update_index!(fulfillment.fulfills.not(&vec![entry.fulfills]), commitment.fulfilled_by(&base_address));
    hdk::prelude::debug!("handle_delete_fulfillment::fulfills index (origin) {:?}", e);

    // update fulfillment records in remote DNA (and by proxy, event indexes in remote DNA)
    let pingback: OtherCellResult<ResponseData> = call_zome_method(
        &entry.fulfilled_by,
        &REPLICATE_DELETE_API_METHOD,
        ByAction { address: revision_id.to_owned() },
    );
    // :TODO: report any error
    hdk::prelude::debug!("handle_delete_fulfillment::call_zome_method::{:?} {:?}", REPLICATE_DELETE_API_METHOD, pingback);

    delete_record::<EntryStorage>(&revision_id)
}

/// Properties accessor for zome config.
fn read_commitment_index_zome(conf: DnaConfigSlicePlanning) -> Option<String> {
    Some(conf.fulfillment.commitment_index_zome)
}

/// Properties accessor for zome config.
fn read_fulfillment_index_zome(conf: DnaConfigSlicePlanning) -> Option<String> {
    Some(conf.fulfillment.index_zome)
}
