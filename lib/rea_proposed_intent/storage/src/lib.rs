/**
 * Holo-REA proposed intents: maintains relationships between coordinated proposals and the individual intents that describe their planned enaction. zome internal data structures
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

use vf_core::type_aliases::{
    IntentAddress,
};

use hc_zome_rea_proposed_intent_rpc::{ CreateRequest, UpdateRequest };

//---------------- RECORD INTERNALS & VALIDATION ----------------

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Entry {
    pub reciprocal: bool,
    pub publishes: Option<IntentAddress>,
}

//---------------- CREATE ----------------

/// Pick relevant fields out of I/O record into underlying DHT entry
impl From<CreateRequest> for Entry {
    fn from(e: CreateRequest) -> Entry {
        Entry {
            reciprocal: e.reciprocal,
            publishes: e.publishes.into(),
        }
    }
}

//---------------- UPDATE ----------------

/// Handles update operations by merging any newly provided fields
impl Updateable<UpdateRequest> for Entry {
    fn update_with(&self, e: &UpdateRequest) -> Entry {
        Entry {
            reciprocal: if e.reciprocal.is_undefined() { self.reciprocal } else { e.reciprocal.to_owned().unwrap() },
            publishes: if e.publishes == MaybeUndefined::Undefined { self.publishes.to_owned() } else { e.publishes.to_owned().into() },
        }
    }
}
