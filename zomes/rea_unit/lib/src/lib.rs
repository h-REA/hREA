/**
 * Holo-REA measurement unit zome library API
 *
 * Contains helper methods that can be used to manipulate `Unit` data
 * structures in either the local Holochain zome, or a separate DNA-local zome.
 *
 * @package Holo-REA
 */
use hdk_records::{
    RecordAPIResult,
    records_anchored::{
        create_anchored_record,
        read_anchored_record_entry,
        update_anchored_record,
        delete_anchored_record,
    },
};

pub use vf_attributes_hdk::{
    ByHeader, ByAddress,
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
    let (revision_id, entry_id, entry_resp): (_,UnitId,_) = create_anchored_record(read_index_zome, &entry_def_id, unit.to_owned())?;
    Ok(construct_response(&entry_id, &revision_id, &entry_resp))
}

pub fn handle_get_unit<S>(entry_def_id: S, id: UnitId) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>,
{
    let id_str: &String = id.as_ref();
    let (revision_id, entry_id, entry): (_,UnitId,_) = read_anchored_record_entry::<EntryData, EntryStorage, UnitInternalAddress, _,_,_>(&entry_def_id, id_str)?;
    Ok(construct_response(&entry_id, &revision_id, &entry))
}

pub fn handle_update_unit<S>(entry_def_id: S, unit: UpdateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>,
{
    let revision_id = unit.get_revision_id().clone();
    let (new_revision, new_id, new_entry, _prev_entry): (_,UnitId,_,_) = update_anchored_record::<EntryData, EntryStorage, UnitInternalAddress, _,_,_,_>(&entry_def_id, &revision_id, unit)?;
    Ok(construct_response(&new_id, &new_revision, &new_entry))
}

pub fn handle_delete_unit(revision_id: HeaderHash) -> RecordAPIResult<bool> {
    delete_anchored_record::<EntryStorage>(&revision_id)
}

fn construct_response<'a>(
    id: &UnitId, revision_id: &HeaderHash, e: &EntryData
) -> ResponseData {
    ResponseData {
        unit: Response {
            id: id.to_owned(),
            revision_id: revision_id.to_owned(),
            label: e.label.to_owned(),
            symbol: e.symbol.to_owned(),
        }
    }
}
