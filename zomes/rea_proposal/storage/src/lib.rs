/**
 * hREA proposal zome internal data structures
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
    record_interface::Updateable, MaybeUndefined,
    generate_record_entry,
};

pub use vf_attributes_hdk::{ ProposalAddress, ProposedIntentAddress, ProposedToAddress, DateTime, FixedOffset };

use hc_zome_rea_proposal_rpc::{CreateRequest, UpdateRequest};

//--------------- ZOME CONFIGURATION ATTRIBUTES ----------------

// :TODO: remove this, replace with reference to appropriate namespacing of zome config
#[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
pub struct DnaConfigSlice {
    pub proposal: ProposalZomeConfig,
}

#[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
pub struct ProposalZomeConfig {
    pub index_zome: String,
}

//---------------- RECORD INTERNALS & VALIDATION ----------------

#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
pub struct EntryData {
    pub name: Option<String>,
    pub has_beginning: Option<DateTime<FixedOffset>>,
    pub has_end: Option<DateTime<FixedOffset>>,
    pub unit_based: Option<bool>,
    pub created: Option<DateTime<FixedOffset>>,
    pub note: Option<String>,
    pub in_scope_of: Option<Vec<String>>,
    //[TODO]:
    //eligibleLocation: SpatialThing
    //publishes: [ProposedIntent!]
    pub _nonce: Bytes,
}

generate_record_entry!(EntryData, ProposalAddress, EntryStorage);

//---------------- Holochain App Entry And Link Types Setup ----------------

#[hdk_entry_defs(skip_hdk_extern = true)]
#[unit_enum(EntryTypesUnit)]
pub enum EntryTypes {
    Proposal(EntryStorage),
    #[entry_def(visibility = "private")]
    AvailableCapability(AvailableCapability)
}

impl From<EntryStorage> for EntryTypes
{
    fn from(e: EntryStorage) -> EntryTypes
    {
        EntryTypes::Proposal(e)
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
            has_beginning: e.has_beginning.into(),
            has_end: e.has_end.into(),
            unit_based: e.unit_based.into(),
            created: e.created.into(),
            note: e.note.into(),
            in_scope_of: e.in_scope_of.to_option(),
            _nonce: random_bytes(32)?,
        })
    }
}

//---------------- UPDATE ----------------

/// Handles update operations by merging any newly provided fields
impl Updateable<UpdateRequest> for EntryData {
    fn update_with(&self, e: UpdateRequest) -> RecordAPIResult<EntryData> {
        Ok(EntryData {
            name: if !e.name.is_some() {
                self.name.to_owned()
            } else {
                e.name.to_owned().into()
            },
            has_beginning: if e.has_beginning == MaybeUndefined::Undefined {
                self.has_beginning.to_owned()
            } else {
                e.has_beginning.to_owned().into()
            },
            has_end: if e.has_end == MaybeUndefined::Undefined {
                self.has_end.to_owned()
            } else {
                e.has_end.to_owned().into()
            },
            unit_based: if e.unit_based == MaybeUndefined::Undefined {
                self.unit_based.to_owned()
            } else {
                e.unit_based.to_owned().into()
            },
            created: self.created.to_owned(),
            note: if e.note == MaybeUndefined::Undefined {
                self.note.to_owned()
            } else {
                e.note.to_owned().into()
            },
            in_scope_of: if e.in_scope_of == MaybeUndefined::Undefined {
                self.in_scope_of.to_owned()
            } else {
                e.in_scope_of.to_owned().to_option()
            },
            _nonce: self._nonce.to_owned(),
        })
    }
}
