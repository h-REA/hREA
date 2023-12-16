/**
 * hREA recipe_process zome internal data structures
 *
 * Required by the zome itself, and for any DNA-local zomes interacting with its
 * storage API directly.
 *
 * @package hREA
 */
use hdk::prelude::*;

use hc_zome_dna_auth_resolver_core::AvailableCapability;
use hdk_records::{
    RecordAPIResult, DataIntegrityError,
    MaybeUndefined,
    record_interface::Updateable,
    generate_record_entry,
};
use vf_measurement::QuantityValue;

pub use vf_attributes_hdk::{
    // ActionId,
    DateTime, FixedOffset,
    ExternalURL,
    RecipeProcessAddress,
    LocationAddress,
    AgentAddress,
    EconomicResourceAddress,
    ProcessAddress,
    ResourceSpecificationAddress,
    ProcessSpecificationAddress
};

// use vf_actions::{ validate_flow_action };

use hc_zome_rea_recipe_process_rpc::{ CreateRequest, UpdateRequest };

// :SHONK: needed as re-export in zome logic to allow validation logic to parse entries
pub use hdk_records::record_interface::Identified;

//--------------- ZOME CONFIGURATION ATTRIBUTES ----------------

// :TODO: remove this, replace with reference to appropriate namespacing of zome config
#[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
pub struct DnaConfigSlice {
    pub recipe_process: RecipeProcessZomeConfig,
}

#[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
pub struct RecipeProcessZomeConfig {
    pub index_zome: String,
    pub process_index_zome: Option<String>,
    pub agent_index_zome: Option<String>,
}

//---------------- RECORD INTERNALS & VALIDATION ----------------

#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
pub struct EntryData {
    pub name: Option<String>,
    pub image: Option<String>,
    pub note: Option<String>,
    pub process_conforms_to: Option<ProcessSpecificationAddress>,
    pub _nonce: Bytes,
}

impl EntryData {
    pub fn validate_recipe_process(&self) -> Result<(), String> {
        Ok(())
    }
}

generate_record_entry!(EntryData, RecipeProcessAddress, EntryStorage);

//---------------- Holochain App Entry And Link Types Setup ----------------

#[hdk_entry_defs(skip_hdk_extern = true)]
#[unit_enum(EntryTypesUnit)]
pub enum EntryTypes {
    RecipeProcess(EntryStorage),
    #[entry_def(visibility = "private")]
    AvailableCapability(AvailableCapability)
}

impl From<EntryStorage> for EntryTypes
{
    fn from(e: EntryStorage) -> EntryTypes
    {
        EntryTypes::RecipeProcess(e)
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
            name: e.name.to_owned().into(),
            image: e.image.to_owned().into(),
            note: e.note.to_owned().into(),
            process_conforms_to: e.process_conforms_to.to_owned().into(),
            _nonce: random_bytes(32)?,
        })
    }
}

//---------------- UPDATE ----------------

/// Handles update operations by merging any newly provided fields
impl Updateable<UpdateRequest> for EntryData {
    fn update_with(&self, e: UpdateRequest) -> RecordAPIResult<EntryData> {
        Ok(EntryData {
            name: if e.name== MaybeUndefined::Undefined { self.name.to_owned() } else { e.name.to_owned().into() },
            image: if e.image== MaybeUndefined::Undefined { self.image.to_owned() } else { e.image.to_owned().into() },
            note: if e.note== MaybeUndefined::Undefined { self.note.to_owned() } else { e.note.to_owned().into() },
            process_conforms_to: if e.process_conforms_to== MaybeUndefined::Undefined { self.process_conforms_to.to_owned() } else { e.process_conforms_to.to_owned().into() },
           _nonce: self._nonce.to_owned(),
        })
    }
}
