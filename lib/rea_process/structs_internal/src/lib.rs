/**
 * Holo-REA 'process' zome internal data structures
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
    Timestamp,
    ExternalURL,
    ProcessSpecificationAddress,
    PlanAddress,
};

use hc_zome_rea_process_structs_rpc::{ CreateRequest, UpdateRequest };

//---------------- RECORD INTERNALS & VALIDATION ----------------

#[derive(Serialize, Deserialize, Debug, DefaultJson, Default, Clone)]
pub struct Entry {
    pub name: String,
    pub has_beginning: Option<Timestamp>,
    pub has_end: Option<Timestamp>,
    pub before: Option<Timestamp>,
    pub after: Option<Timestamp>,
    pub classified_as: Option<Vec<ExternalURL>>,
    pub based_on: Option<ProcessSpecificationAddress>,
    pub planned_within: Option<PlanAddress>,
    pub finished: bool,
    pub in_scope_of: Option<Vec<String>>,
    pub note: Option<String>,
}

//---------------- CREATE ----------------

/// Pick relevant fields out of I/O record into underlying DHT entry
impl From<CreateRequest> for Entry {
    fn from(e: CreateRequest) -> Entry {
        Entry {
            name: e.name.into(),
            has_beginning: e.has_beginning.into(),
            has_end: e.has_end.into(),
            before: e.before.into(),
            after: e.after.into(),
            classified_as: e.classified_as.into(),
            based_on: e.based_on.into(),
            planned_within: e.planned_within.into(),
            finished: e.finished.to_option().unwrap(),  // :NOTE: unsafe, would crash if not for "default_*" bindings via Serde
            in_scope_of: e.in_scope_of.into(),
            note: e.note.into(),
        }
    }
}

//---------------- UPDATE ----------------

/// Handles update operations by merging any newly provided fields
impl Updateable<UpdateRequest> for Entry {
    fn update_with(&self, e: &UpdateRequest) -> Entry {
        Entry {
            name: if !e.name.is_some() { self.name.to_owned() } else { e.name.to_owned().unwrap() },
            has_beginning: if e.has_beginning == MaybeUndefined::Undefined { self.has_beginning.to_owned() } else { e.has_beginning.to_owned().into() },
            has_end: if e.has_end == MaybeUndefined::Undefined { self.has_end.to_owned() } else { e.has_end.to_owned().into() },
            before: if e.before == MaybeUndefined::Undefined { self.before.to_owned() } else { e.before.to_owned().into() },
            after: if e.after == MaybeUndefined::Undefined { self.after.to_owned() } else { e.after.to_owned().into() },
            classified_as: if e.classified_as == MaybeUndefined::Undefined { self.classified_as.to_owned() } else { e.classified_as.to_owned().into() },
            based_on: if e.based_on == MaybeUndefined::Undefined { self.based_on.to_owned() } else { e.based_on.to_owned().into() },
            planned_within: if e.planned_within == MaybeUndefined::Undefined { self.planned_within.to_owned() } else { e.planned_within.to_owned().into() },
            finished: if e.finished == MaybeUndefined::Undefined { self.finished.to_owned() } else { e.finished.to_owned().to_option().unwrap() },
            in_scope_of: if e.in_scope_of == MaybeUndefined::Undefined { self.in_scope_of.to_owned() } else { e.in_scope_of.to_owned().into() },
            note: if e.note == MaybeUndefined::Undefined { self.note.to_owned() } else { e.note.to_owned().into() },
        }
    }
}
