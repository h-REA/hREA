/**
 * Holo-REA process specification zome library API
 *
 * Contains helper methods that can be used to manipulate `ProcessSpecification` data
 * structures in either the local Holochain zome, or a separate DNA-local zome.
 *
 * @package Holo-REA
 */
use hdk_records::{
    RecordAPIResult, SignedActionHashed,
    records::{
        create_record,
        read_record_entry,
        update_record,
        delete_record,
    },
    metadata::read_revision_metadata_abbreviated,
};

use hc_zome_rea_process_specification_storage::*;
use hc_zome_rea_process_specification_rpc::*;
use hc_zome_rea_process_specification_integrity::*;

/// properties accessor for zome config
fn read_index_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.process_specification.index_zome)
}

pub fn handle_create_process_specification<S>(entry_def_id: S, process_specification: CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str> + std::fmt::Display,
{
    let (meta, base_address, entry_resp): (_,_, EntryData) = create_record(read_index_zome, &entry_def_id, process_specification)?;

    construct_response(&base_address, &meta, &entry_resp)
}

pub fn handle_get_process_specification(address: ProcessSpecificationAddress) -> RecordAPIResult<ResponseData>
{
    let (meta, base_address, entry) = read_record_entry::<EntryData, EntryStorage, _,_,_>(address.as_ref())?;
    construct_response(&base_address, &meta, &entry)
}

pub fn handle_update_process_specification(process_specification: UpdateRequest) -> RecordAPIResult<ResponseData>
{
    let old_revision = process_specification.get_revision_id();
    let (meta, base_address, new_entry, _prev_entry): (_, ProcessSpecificationAddress, EntryData, EntryData) = update_record(old_revision, process_specification.to_owned())?;
    construct_response(&base_address, &meta, &new_entry)
}

pub fn handle_delete_process_specification(revision_id: ActionHash) -> RecordAPIResult<bool>
{
    delete_record::<EntryStorage>(&revision_id)
}

/// Create response from input DHT primitives
fn construct_response<'a>(
    address: &ProcessSpecificationAddress, meta: &SignedActionHashed, e: &EntryData,
) -> RecordAPIResult<ResponseData> {
    Ok(ResponseData {
        process_specification: Response {
            id: address.to_owned(),
            revision_id: meta.as_hash().to_owned(),
            meta: read_revision_metadata_abbreviated(meta)?,
            name: e.name.to_owned(),
            note: e.note.to_owned(),
        }
    })
}
