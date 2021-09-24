/**
 * Holo-REA proposal zome internal data structures
 *
 * Required by the zome itself, and for any DNA-local zomes interacting with its
 * storage API directly.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use hdk_records::{
    record_interface::Updateable, MaybeUndefined,
    generate_record_entry,
};

pub use vf_attributes_hdk::{ ProposalAddress, ProposedIntentAddress, ProposedToAddress, Timestamp };

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
    pub has_beginning: Option<Timestamp>,
    pub has_end: Option<Timestamp>,
    pub unit_based: Option<bool>,
    pub created: Option<Timestamp>,
    pub note: Option<String>,
    pub in_scope_of: Option<Vec<String>>,
    //[TODO]:
    //eligibleLocation: SpatialThing
    //publishes: [ProposedIntent!]
}

generate_record_entry!(EntryData, ProposalAddress, EntryStorage);

//---------------- CREATE ----------------

/// Pick relevant fields out of I/O record into underlying DHT entry
impl From<CreateRequest> for EntryData {
    fn from(e: CreateRequest) -> EntryData {
        EntryData {
            name: e.name.into(),
            has_beginning: e.has_beginning.into(),
            has_end: e.has_end.into(),
            unit_based: e.unit_based.into(),
            created: e.created.into(),
            note: e.note.into(),
            in_scope_of: e.in_scope_of.to_option(),
        }
    }
}

//---------------- UPDATE ----------------

/// Handles update operations by merging any newly provided fields
impl Updateable<UpdateRequest> for EntryData {
    fn update_with(&self, e: UpdateRequest) -> EntryData {
        EntryData {
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
        }
    }
}
