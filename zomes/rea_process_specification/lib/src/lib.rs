/**
 * Holo-REA process specification zome library API
 *
 * Contains helper methods that can be used to manipulate `ProcessSpecification` data
 * structures in either the local Holochain zome, or a separate DNA-local zome.
 *
 * @package Holo-REA
 */
use hdk_records::{
    RecordAPIResult,
    records::{
        create_record,
        read_record_entry,
        update_record,
        delete_record,
    },
};

use hc_zome_rea_process_specification_storage::*;
use hc_zome_rea_process_specification_rpc::*;

pub fn receive_create_process_specification<S>(entry_def_id: S, process_specification: CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>,
{
    let (revision_id, base_address, entry_resp): (_,_, EntryData) = create_record(&entry_def_id, process_specification)?;

    Ok(construct_response(&base_address, &revision_id, &entry_resp))
}

pub fn receive_get_process_specification<S>(entry_def_id: S, address: ProcessSpecificationAddress) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>,
{
    let (revision, base_address, entry) = read_record_entry::<EntryData, EntryStorage, _,_>(&entry_def_id, address.as_ref())?;
    Ok(construct_response(&base_address, &revision, &entry))
}

pub fn receive_update_process_specification<S>(entry_def_id: S, process_specification: UpdateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>,
{
    let old_revision = process_specification.get_revision_id();
    let (revision_id, base_address, new_entry, _prev_entry): (_, ProcessSpecificationAddress, EntryData, EntryData) = update_record(&entry_def_id, old_revision, process_specification.to_owned())?;
    Ok(construct_response(&base_address, &revision_id, &new_entry))
}

pub fn receive_delete_process_specification(revision_id: RevisionHash) -> RecordAPIResult<bool>
{
    delete_record::<EntryStorage, _>(&revision_id)
}

/// Create response from input DHT primitives
fn construct_response<'a>(
    address: &ProcessSpecificationAddress, revision_id: &RevisionHash, e: &EntryData,
) -> ResponseData {
    ResponseData {
        process_specification: Response {
            id: address.to_owned(),
            revision_id: revision_id.to_owned(),
            name: e.name.to_owned(),
            note: e.note.to_owned(),
        }
    }
}
