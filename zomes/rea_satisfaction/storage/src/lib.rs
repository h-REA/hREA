/**
 * Holo-REA satisfaction zome internal data structures
 *
 * Required by the zome itself, and for any DNA-local zomes interacting with its
 * storage API directly.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use hdk_records::{
    MaybeUndefined,
    record_interface::Updateable,
    generate_record_entry,
};
use vf_measurement::QuantityValue;

pub use vf_attributes_hdk::{
    SatisfactionAddress,
    EventOrCommitmentAddress,
    IntentAddress,
};

use hc_zome_rea_satisfaction_rpc::{ CreateRequest, UpdateRequest };

//--------------- ZOME CONFIGURATION ATTRIBUTES ----------------

// :TODO: remove this, replace with reference to appropriate namespacing of zome config
#[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
pub struct DnaConfigSliceObservation {
    pub satisfaction: SatisfactionZomeConfigObservation,
}

#[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
pub struct SatisfactionZomeConfigObservation {
    pub economic_event_index_zome: String,
}

// :TODO: remove this, replace with reference to appropriate namespacing of zome config
#[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
pub struct DnaConfigSlicePlanning {
    pub satisfaction: SatisfactionZomeConfigPlanning,
}

#[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
pub struct SatisfactionZomeConfigPlanning {
    pub commitment_zome: Option<String>, // :TODO: deprecate this, now we have DnaHash-capable IDs we don't need to query related zome to check relevance
    pub commitment_index_zome: String,
    pub intent_index_zome: String,
}

//---------------- RECORD INTERNALS & VALIDATION ----------------

#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
pub struct EntryData {
    pub satisfied_by: EventOrCommitmentAddress,
    pub satisfies: IntentAddress,
    pub resource_quantity: Option<QuantityValue>,
    pub effort_quantity: Option<QuantityValue>,
    pub note: Option<String>,
}

generate_record_entry!(EntryData, SatisfactionAddress, EntryStorage);

//---------------- CREATE ----------------

/// Pick relevant fields out of I/O record into underlying DHT entry
impl From<CreateRequest> for EntryData {
    fn from(e: CreateRequest) -> EntryData {
        EntryData {
            satisfied_by: e.satisfied_by.into(),
            satisfies: e.satisfies.into(),
            resource_quantity: e.resource_quantity.into(),
            effort_quantity: e.effort_quantity.into(),
            note: e.note.into(),
        }
    }
}

//---------------- UPDATE ----------------

/// Handles update operations by merging any newly provided fields
impl Updateable<UpdateRequest> for EntryData {
    fn update_with(&self, e: UpdateRequest) -> EntryData {
        EntryData {
            satisfied_by: match &e.satisfied_by {
                MaybeUndefined::Some(satisfied_by) => satisfied_by.clone(),
                _ => self.satisfied_by.clone(),
            },
            satisfies: match &e.satisfies {
                MaybeUndefined::Some(satisfies) => satisfies.clone(),
                _ => self.satisfies.clone(),
            },
            resource_quantity: if e.resource_quantity== MaybeUndefined::Undefined { self.resource_quantity.clone() } else { e.resource_quantity.clone().into() },
            effort_quantity: if e.effort_quantity== MaybeUndefined::Undefined { self.effort_quantity.clone() } else { e.effort_quantity.clone().into() },
            note: if e.note== MaybeUndefined::Undefined { self.note.clone() } else { e.note.clone().into() },
        }
    }
}
