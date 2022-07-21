/**
 * Holo-REA plan zome internal data structures
 *
 * Required by the zome itself, and for any DNA-local zomes interacting with its
 * storage API directly.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use hdk_records::{
    RecordAPIResult, DataIntegrityError,
    generate_record_entry,
    record_interface::{Updateable},
};

use vf_attributes_hdk::{
    DateTime,
    FixedOffset,
};

use hc_zome_rea_plan_rpc::{ CreateRequest, UpdateRequest };

pub use vf_attributes_hdk::PlanAddress;
pub use hc_zome_rea_plan_storage_consts::PLAN_ENTRY_TYPE;

// :SHONK: needed as re-export in zome logic to allow validation logic to parse entries
pub use hdk_records::record_interface::Identified;

//--------------- ZOME CONFIGURATION ATTRIBUTES ----------------

// :TODO: remove this, replace with reference to appropriate namespacing of zome config
#[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
pub struct DnaConfigSlice {
    pub plan: PlanZomeConfig,
}

#[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
pub struct PlanZomeConfig {
    pub index_zome: String,
}

//---------------- RECORD INTERNALS & VALIDATION ----------------

#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
pub struct EntryData {
    pub name: Option<String>,
    pub created: Option<DateTime<FixedOffset>>,
    pub due: Option<DateTime<FixedOffset>>,
    pub note: Option<String>,
    pub deletable: Option<bool>,
    pub _nonce: Bytes,
}

generate_record_entry!(EntryData, PlanAddress, EntryStorage);

//---------------- CREATE ----------------

/// Pick relevant fields out of I/O record into underlying DHT entry
impl TryFrom<CreateRequest> for EntryData {
    type Error = DataIntegrityError;

    fn try_from(e: CreateRequest) -> RecordAPIResult<EntryData> {
        Ok(EntryData {
            name: e.name.into(),
            created: e.created.into(),
            due: e.due.into(),
            note: e.note.into(),
            deletable: e.deletable.into(),
            _nonce: random_bytes(32)?,
        })
    }
}

//---------------- UPDATE ----------------

/// Handles update operations by merging any newly provided fields
impl Updateable<UpdateRequest> for EntryData {
    fn update_with(&self, e: UpdateRequest) -> EntryData {
        EntryData {
            name: if !e.name.is_some() { self.name.to_owned() } else { e.name.to_owned().into() },
            created: if !e.created.is_some() { self.created.to_owned() } else { e.created.to_owned().into() },
            due: if !e.due.is_some() { self.due.to_owned() } else { e.due.to_owned().into() },
            note: if !e.note.is_some() { self.note.to_owned() } else { e.note.to_owned().into() },
            deletable: if !e.deletable.is_some() { self.deletable.to_owned() } else { e.deletable.to_owned().into() },
            _nonce: self._nonce.to_owned(),
        }
    }
}
