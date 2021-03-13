/**
 * Holo-REA resource specification zome internal data structures
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

use hdk_records::{
    record_interface::Updateable,
};

use vf_core::type_aliases::{
    ExternalURL,
    UnitId,
};

use hc_zome_rea_resource_specification_rpc::{ CreateRequest, UpdateRequest };

//---------------- RECORD INTERNALS & VALIDATION ----------------

#[derive(Serialize, Deserialize, Debug, DefaultJson, Default, Clone)]
pub struct Entry {
    pub name: String,
    pub image: Option<ExternalURL>,
    pub note: Option<String>,
    pub default_unit_of_effort: Option<UnitId>,
}

//---------------- CREATE ----------------

/// Pick relevant fields out of I/O record into underlying DHT entry
impl From<CreateRequest> for Entry {
    fn from(e: CreateRequest) -> Entry {
        Entry {
            name: e.name.into(),
            image: e.image.into(),
            note: e.note.into(),
            default_unit_of_effort: e.default_unit_of_effort.into(),
        }
    }
}

//---------------- UPDATE ----------------

/// Handles update operations by merging any newly provided fields
impl Updateable<UpdateRequest> for Entry {
    fn update_with(&self, e: &UpdateRequest) -> Entry {
        Entry {
            name: if !e.name.is_some() { self.name.to_owned() } else { e.name.to_owned().unwrap() },
            image: if e.image.is_undefined() { self.image.to_owned() } else { e.image.to_owned().into() },
            note: if e.note.is_undefined() { self.note.to_owned() } else { e.note.to_owned().into() },
            default_unit_of_effort: if e.default_unit_of_effort.is_undefined() { self.default_unit_of_effort.to_owned() } else { e.default_unit_of_effort.to_owned().into() },
        }
    }
}
