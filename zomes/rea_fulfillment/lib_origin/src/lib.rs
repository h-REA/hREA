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

use hc_zome_rea_fulfillment_storage_consts::*;
use hc_zome_rea_fulfillment_storage::*;
use hc_zome_rea_fulfillment_rpc::*;
use hc_zome_rea_fulfillment_lib::construct_response;

// :SHONK: needed to re-export for zome `entry_defs()` where macro-assigned defs are overridden
pub use hdk_records::CAP_STORAGE_ENTRY_DEF_ID;

pub fn handle_create_fulfillment<S>(entry_def_id: S, fulfillment: CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let (revision_id, fulfillment_address, entry_resp): (_,_, EntryData) = create_record(&entry_def_id, fulfillment.to_owned())?;

    // link entries in the local DNA
    let create_index_results = create_index!(fulfillment.fulfills(fulfillment.get_fulfills()), commitment.fulfilled_by(&fulfillment_address))?;
    hdk::prelude::debug!("handle_create_fulfillment::fulfills::create_index!: {:?}", create_index_results);
    
    // :TODO: report any error
    // update in the associated foreign DNA as well
    let pingback: OtherCellResult<ResponseData> = call_zome_method(
      fulfillment.get_fulfilled_by(),
      &REPLICATE_CREATE_API_METHOD,
      CreateParams { fulfillment: fulfillment.to_owned() },
    );
    hdk::prelude::debug!("handle_create_fulfillment::call_zome_method::{:?}: {:?}", REPLICATE_CREATE_API_METHOD, pingback);

    construct_response(&fulfillment_address, &revision_id, &entry_resp)
}

pub fn handle_get_fulfillment<S>(entry_def_id: S, address: FulfillmentAddress) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let (revision, base_address, entry) = read_record_entry::<EntryData, EntryStorage, _,_,_>(&entry_def_id, address.as_ref())?;
    construct_response(&base_address, &revision, &entry)
}

pub fn handle_update_fulfillment<S>(entry_def_id: S, fulfillment: UpdateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let (revision_id, base_address, new_entry, prev_entry): (_, FulfillmentAddress, EntryData, EntryData) = update_record(&entry_def_id, &fulfillment.get_revision_id(), fulfillment.to_owned())?;

    // update commitment indexes in local DNA
    if new_entry.fulfills != prev_entry.fulfills {
        update_index!(
            fulfillment
                .fulfills(&vec![new_entry.fulfills.clone()])
                .not(&vec![prev_entry.fulfills]),
            commitment.fulfilled_by(&base_address)
        )?;
    }

    // update fulfillment records in remote DNA (and by proxy, event indexes in remote DNA)
    if new_entry.fulfilled_by != prev_entry.fulfilled_by {
        let _pingback: OtherCellResult<ResponseData> = call_zome_method(
            // :TODO: update to intelligently call remote DNAs if new & old target record are not in same network
            &prev_entry.fulfilled_by,
            &REPLICATE_UPDATE_API_METHOD,
            UpdateParams { fulfillment: fulfillment.to_owned() },
        );
        // :TODO: report any error
    }

    construct_response(&base_address, &revision_id, &new_entry)
}

pub fn handle_delete_fulfillment(revision_id: HeaderHash) -> RecordAPIResult<bool>
{
    let (base_address, entry) = read_record_entry_by_header::<EntryData, EntryStorage, _>(&revision_id)?;

    // update commitment indexes in local DNA
    update_index!(fulfillment.fulfills.not(&vec![entry.fulfills]), commitment.fulfilled_by(&base_address))?;

    // update fulfillment records in remote DNA (and by proxy, event indexes in remote DNA)
    let _pingback: OtherCellResult<ResponseData> = call_zome_method(
        &entry.fulfilled_by,
        &REPLICATE_DELETE_API_METHOD,
        ByHeader { address: revision_id.to_owned() },
    );
    // :TODO: report any error

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
