/**
 * hREA measurement unit zome library API
 *
 * Contains helper methods that can be used to manipulate `Unit` data
 * structures in either the local Holochain zome, or a separate DNA-local zome.
 *
 * @package hREA
 */
use hdk::prelude::*;
use hdk_records::{
    RecordAPIResult,
    records_anchored::{
        create_anchored_record,
        read_anchored_record_entry,
        update_anchored_record,
        delete_anchored_record,
    },
    records::{
        read_record_entry,
        read_record_entry_by_action,
    },
    metadata::read_revision_metadata_abbreviated,
};

pub use vf_attributes_hdk::{
    ByAction, ByAddress,
    DnaIdentifiable,
};

pub use hc_zome_rea_unit_storage_consts::*;
use hc_zome_rea_unit_storage::*;
use hc_zome_rea_unit_rpc::*;


/// properties accessor for zome config
fn read_index_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.unit.index_zome)
}

pub fn handle_create_unit<S>(entry_def_id: S, unit: CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str> + std::fmt::Display,
{
    let (meta, entry_id, entry_resp): (_,UnitId,_) =
      create_anchored_record::<_, _, _, _, _, _, EntryTypes, _, _, _, _>(
        LinkTypes::UnitIdentifier,
        read_index_zome,
        &entry_def_id,
        unit.to_owned()
      )?;
    construct_response(&entry_id, &meta, &entry_resp)
}

pub fn handle_get_unit(id: UnitId) -> RecordAPIResult<ResponseData>
{
    let id_str: &String = id.as_ref();
    let (meta, entry_id, entry): (_,UnitId,_) =
      read_anchored_record_entry::<LinkTypes, EntryData, EntryStorage, UnitInternalAddress, _,_>(
        LinkTypes::UnitIdentifier,
        id_str
      )?;
    construct_response(&entry_id, &meta, &entry)
}

pub fn handle_get_unit_by_symbol<S>(symbol: S) -> RecordAPIResult<ResponseData>
    where S: AsRef<str> + std::fmt::Display,
{
    let (meta, entry_id, entry): (_,UnitId,_) =
      read_anchored_record_entry::<LinkTypes, EntryData, EntryStorage, UnitInternalAddress, _,_>(
        LinkTypes::UnitIdentifier,
        symbol.as_ref(),
      )?;
    construct_response(&entry_id, &meta, &entry)
}

// internal method used by index zomes to locate indexed unit record data
pub fn handle_get_unit_by_address(address: UnitInternalAddress) -> RecordAPIResult<ResponseData>
{
    let (meta, _base_address, entry) = read_record_entry::<EntryData, EntryStorage, _>(address.as_ref())?;
    construct_response(&UnitId::new(
        dna_info()?.hash,
        entry.symbol.to_owned(),
    ), &meta, &entry)
}

pub fn handle_get_revision(revision_id: ActionHash) -> RecordAPIResult<ResponseData>
{
    let (meta, _base_address, entry) = read_record_entry_by_action::<EntryData, EntryStorage, _>(&revision_id)?;
    construct_response(&UnitId::new(
        dna_info()?.hash,
        entry.symbol.to_owned(),
    ), &meta, &entry)
}

pub fn handle_update_unit(unit: UpdateRequest) -> RecordAPIResult<ResponseData>
{
    let revision_id = unit.get_revision_id().clone();
    let (meta, new_id, new_entry, _prev_entry): (_,UnitId,_,_) = update_anchored_record::<LinkTypes, EntryData, EntryStorage, UnitInternalAddress, _,_,_>(LinkTypes::UnitIdentifier, &revision_id, unit)?;
    construct_response(&new_id, &meta, &new_entry)
}

pub fn handle_delete_unit(revision_id: ActionHash) -> RecordAPIResult<bool> {
    delete_anchored_record::<EntryStorage>(&revision_id)
}

fn construct_response<'a>(
    id: &UnitId, meta: &SignedActionHashed, e: &EntryData
) -> RecordAPIResult<ResponseData> {
    Ok(ResponseData {
        unit: Response {
            id: id.to_owned(),
            revision_id: meta.as_hash().to_owned(),
            meta: read_revision_metadata_abbreviated(meta)?,
            label: e.label.to_owned(),
            symbol: e.symbol.to_owned(),
        }
    })
}
