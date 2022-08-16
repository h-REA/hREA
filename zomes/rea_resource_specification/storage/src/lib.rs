/**
 * Holo-REA resource specification zome internal data structures
 *
 * Required by the zome itself, and for any DNA-local zomes interacting with its
 * storage API directly.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use hc_zome_dna_auth_resolver_core::AvailableCapability;
use hdk_records::{
    RecordAPIResult, DataIntegrityError,
    record_interface::Updateable,
    generate_record_entry,
};

use vf_attributes_hdk::{
    ExternalURL,
    UnitId,
};

use hc_zome_rea_resource_specification_rpc::{CreateRequest, ResourceSpecificationAddress, UpdateRequest};

//--------------- ZOME CONFIGURATION ATTRIBUTES ----------------

// :TODO: remove this, replace with reference to appropriate namespacing of zome config
#[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
pub struct DnaConfigSlice {
    pub resource_specification: ResourceSpecificationZomeConfig,
}

#[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
pub struct ResourceSpecificationZomeConfig {
    pub index_zome: String,
}

//---------------- RECORD INTERNALS & VALIDATION ----------------

#[derive(Serialize, Deserialize, Debug, SerializedBytes, Default, Clone)]
pub struct EntryData {
    pub name: String,
    pub image: Option<ExternalURL>,
    pub note: Option<String>,
    pub default_unit_of_effort: Option<UnitId>,
    pub default_unit_of_resource: Option<UnitId>,
    pub _nonce: Bytes,
}

generate_record_entry!(EntryData, ResourceSpecificationAddress, EntryStorage);

//---------------- Holochain App Entry And Link Types Setup ----------------

#[hdk_entry_defs(skip_hdk_extern = true)]
#[unit_enum(EntryTypesUnit)]
pub enum EntryTypes {
    ResourceSpecification(EntryStorage),
    #[entry_def(visibility = "private")]
    AvailableCapability(AvailableCapability)
}

impl From<EntryStorage> for EntryTypes
{
    fn from(e: EntryStorage) -> EntryTypes
    {
        EntryTypes::ResourceSpecification(e)
    }
}

impl TryFrom<AvailableCapability> for EntryTypes {
    type Error = WasmError;

    fn try_from(e: AvailableCapability) -> Result<EntryTypes, Self::Error>
    {
        Ok(EntryTypes::AvailableCapability(e))
    }
}

#[hdk_link_types(skip_no_mangle = true)]
pub enum LinkTypes {
    // relates to dna-auth-resolver mixin
    // and remote authorizations
    AvailableCapability
}

//---------------- CREATE ----------------

/// Pick relevant fields out of I/O record into underlying DHT entry
impl TryFrom<CreateRequest> for EntryData {
    type Error = DataIntegrityError;

    fn try_from(e: CreateRequest) -> RecordAPIResult<EntryData> {
        Ok(EntryData {
            name: e.name.into(),
            image: e.image.into(),
            note: e.note.into(),
            default_unit_of_effort: e.default_unit_of_effort.into(),
            default_unit_of_resource: e.default_unit_of_resource.into(),
            _nonce: random_bytes(32)?,
        })
    }
}

//---------------- UPDATE ----------------

/// Handles update operations by merging any newly provided fields
impl Updateable<UpdateRequest> for EntryData {
    fn update_with(&self, e: UpdateRequest) -> EntryData {
        EntryData {
            name: if !e.name.is_some() { self.name.to_owned() } else { e.name.to_owned().unwrap() },
            image: if e.image.is_undefined() { self.image.to_owned() } else { e.image.to_owned().into() },
            note: if e.note.is_undefined() { self.note.to_owned() } else { e.note.to_owned().into() },
            default_unit_of_effort: if e.default_unit_of_effort.is_undefined() { self.default_unit_of_effort.to_owned() } else { e.default_unit_of_effort.to_owned().into() },
            default_unit_of_resource: if e.default_unit_of_resource.is_undefined() { self.default_unit_of_resource.to_owned() } else { e.default_unit_of_resource.to_owned().into() },
            _nonce: self._nonce.to_owned(),
        }
    }
}
