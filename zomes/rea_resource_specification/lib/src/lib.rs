/**
 * Holo-REA resource specification zome library API
 *
 * Contains helper methods that can be used to manipulate `ResourceSpecification` data
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

use vf_attributes_hdk::{
    EconomicResourceAddress,
};

use hc_zome_rea_resource_specification_storage::*;
use hc_zome_rea_resource_specification_rpc::*;

/// properties accessor for zome config
fn read_index_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.resource_specification.index_zome)
}

pub fn handle_create_resource_specification<S>(entry_def_id: S, resource_specification: CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str> + std::fmt::Display,
{
    let (meta, base_address, entry_resp): (_,_, EntryData) = create_record::<EntryTypes,_,_,_,_,_,_,_,_>(read_index_zome, &entry_def_id, resource_specification)?;

    construct_response(&base_address, &meta, &entry_resp, get_link_fields(&base_address)?)
}

pub fn handle_get_resource_specification(address: ResourceSpecificationAddress) -> RecordAPIResult<ResponseData>
{
    let (meta, base_address, entry) = read_record_entry::<EntryData, EntryStorage, _>(address.as_ref())?;
    construct_response(&address, &meta, &entry, get_link_fields(&base_address)?)
}

pub fn handle_update_resource_specification(resource_specification: UpdateRequest) -> RecordAPIResult<ResponseData>
{
    let old_revision = resource_specification.get_revision_id();
    let (meta, base_address, new_entry, _prev_entry): (_, ResourceSpecificationAddress, EntryData, EntryData) = update_record(old_revision, resource_specification.to_owned())?;
    construct_response(&base_address, &meta, &new_entry, get_link_fields(&base_address)?)
}

pub fn handle_delete_resource_specification(revision_id: ActionHash) -> RecordAPIResult<bool>
{
    delete_record::<EntryStorage>(&revision_id)
}

/// Create response from input DHT primitives
fn construct_response<'a>(
    address: &ResourceSpecificationAddress,
    meta: &SignedActionHashed,
    e: &EntryData,
    // :TODO: link conforming resources in associated link registry DNA module
    (
        _conforming_resources,
    ): (
        Vec<EconomicResourceAddress>,
    )
) -> RecordAPIResult<ResponseData> {
    Ok(ResponseData {
        resource_specification: Response {
            // entry fields
            id: address.to_owned(),
            revision_id: meta.as_hash().to_owned(),
            meta: read_revision_metadata_abbreviated(meta)?,
            name: e.name.to_owned(),
            image: e.image.to_owned(),
            note: e.note.to_owned(),
            default_unit_of_effort: e.default_unit_of_effort.to_owned(),

            // conforming_resources: conforming_resources.map(Cow::into_owned),
        }
    })
}

// @see construct_response
fn get_link_fields(_address: &ResourceSpecificationAddress) -> RecordAPIResult<(
    Vec<EconomicResourceAddress>,
)> {
    Ok((
        vec![],   // :TODO:
    ))
}
