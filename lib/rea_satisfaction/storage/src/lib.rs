/**
 * Holo-REA satisfaction zome internal data structures
 *
 * Required by the zome itself, and for any DNA-local zomes interacting with its
 * storage API directly.
 *
 * @package Holo-REA
 */
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use holochain_json_api::{ json::JsonString, error::JsonError };
use holochain_json_derive::{ DefaultJson };

use hdk_graph_helpers::{
    MaybeUndefined,
    record_interface::Updateable,
};

use vf_core::{
    measurement::QuantityValue,
    type_aliases::{
        EventOrCommitmentAddress,
        IntentAddress,
    },
};

use hc_zome_rea_satisfaction_rpc::{ CreateRequest, UpdateRequest };

//---------------- RECORD INTERNALS & VALIDATION ----------------

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Entry {
    pub satisfied_by: EventOrCommitmentAddress,
    pub satisfies: IntentAddress,
    pub resource_quantity: Option<QuantityValue>,
    pub effort_quantity: Option<QuantityValue>,
    pub note: Option<String>,
}

//---------------- CREATE ----------------

/// Pick relevant fields out of I/O record into underlying DHT entry
impl From<CreateRequest> for Entry {
    fn from(e: CreateRequest) -> Entry {
        Entry {
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
impl Updateable<UpdateRequest> for Entry {
    fn update_with(&self, e: &UpdateRequest) -> Entry {
        Entry {
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
