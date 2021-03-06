/**
 * Holo-REA agreement zome internal data structures
 *
 * Required by the zome itself, and for any DNA-local zomes interacting with its
 * storage API directly.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use hdk_graph_helpers::{
    generate_record_entry,
    record_interface::{Updateable},
};

use vf_core::type_aliases::{
    Timestamp,
};

use hc_zome_rea_agreement_rpc::{ CreateRequest, UpdateRequest };

pub use vf_core::type_aliases::AgreementAddress;

//---------------- RECORD INTERNALS & VALIDATION ----------------

#[derive(Clone, PartialEq, Serialize, Deserialize, SerializedBytes, Debug)]
pub struct EntryData {
    pub name: Option<String>,
    pub created: Option<Timestamp>,
    pub note: Option<String>,
}

generate_record_entry!(EntryData, EntryStorage);

//---------------- CREATE ----------------

/// Pick relevant fields out of I/O record into underlying DHT entry
impl From<CreateRequest> for EntryData {
    fn from(e: CreateRequest) -> EntryData {
        EntryData {
            name: e.name.into(),
            created: e.created.into(),
            note: e.note.into(),
        }
    }
}

//---------------- UPDATE ----------------

/// Handles update operations by merging any newly provided fields
impl Updateable<UpdateRequest> for EntryData {
    fn update_with(&self, e: UpdateRequest) -> EntryData {
        EntryData {
            name: if !e.name.is_some() { self.name.to_owned() } else { e.name.to_owned().into() },
            created: if !e.created.is_some() { self.created.to_owned() } else { e.created.to_owned().into() },
            note: if !e.note.is_some() { self.note.to_owned() } else { e.note.to_owned().into() },
        }
    }
}
