/**
 * Holo-REA resource specification zome library API
 *
 * Contains helper methods that can be used to manipulate `ResourceSpecification` data
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

use vf_attributes_hdk::{
    EconomicResourceAddress,
};

use hc_zome_rea_resource_specification_storage::*;
use hc_zome_rea_resource_specification_rpc::*;

pub fn handle_create_resource_specification<S>(entry_def_id: S, resource_specification: CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>,
{
    let (revision_id, base_address, entry_resp): (_,_, EntryData) = create_record(&entry_def_id, resource_specification)?;

    Ok(construct_response(&base_address, &revision_id, &entry_resp, get_link_fields(&base_address)?))
}

pub fn handle_get_resource_specification<S>(entry_def_id: S, address: ResourceSpecificationAddress) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>,
{
    let (revision, base_address, entry) = read_record_entry::<EntryData, EntryStorage, _,_>(&entry_def_id, address.as_ref())?;
    Ok(construct_response(&address, &revision, &entry, get_link_fields(&base_address)?))
}

pub fn handle_update_resource_specification<S>(entry_def_id: S, resource_specification: UpdateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>,
{
    let old_revision = resource_specification.get_revision_id();
    let (revision_id, base_address, new_entry, _prev_entry): (_, ResourceSpecificationAddress, EntryData, EntryData) = update_record(&entry_def_id, old_revision, resource_specification.to_owned())?;
    Ok(construct_response(&base_address, &revision_id, &new_entry, get_link_fields(&base_address)?))
}

pub fn handle_delete_resource_specification(revision_id: RevisionHash) -> RecordAPIResult<bool>
{
    delete_record::<EntryStorage, _>(&revision_id)
}

/// Create response from input DHT primitives
fn construct_response<'a>(
    address: &ResourceSpecificationAddress,
    revision_id: &RevisionHash,
    e: &EntryData,
    // :TODO: link conforming resources in associated link registry DNA module
    (
        _conforming_resources,
    ): (
        Vec<EconomicResourceAddress>,
    )
) -> ResponseData {
    ResponseData {
        resource_specification: Response {
            // entry fields
            id: address.to_owned(),
            revision_id: revision_id.to_owned(),
            name: e.name.to_owned(),
            image: e.image.to_owned(),
            note: e.note.to_owned(),
            default_unit_of_effort: e.default_unit_of_effort.to_owned(),

            // conforming_resources: conforming_resources.map(Cow::into_owned),
        }
    }
}

// @see construct_response
fn get_link_fields(_address: &ResourceSpecificationAddress) -> RecordAPIResult<(
    Vec<EconomicResourceAddress>,
)> {
    Ok((
        vec![],   // :TODO:
    ))
}
