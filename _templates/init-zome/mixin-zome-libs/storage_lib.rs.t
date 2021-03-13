---
to: lib/<%= h.changeCase.snake(zome_name) %>/storage/src/lib.rs
---
/**
 * <%=zome_friendly_name%> zome internal data structures
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
    MaybeUndefined,
    record_interface::Updateable,
};

use vf_attributes_hdk::{
    // :TODO: import compile-time field type wrappers referenced by this record type
};

use hc_zome_<%= h.changeCase.snake(zome_name) %>_rpc::{ CreateRequest, UpdateRequest };

//---------------- RECORD INTERNALS & VALIDATION ----------------

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Entry {
    // :TODO:
}

//---------------- CREATE ----------------

/// Pick relevant fields out of I/O record into underlying DHT entry
impl From<CreateRequest> for Entry {
    fn from(e: CreateRequest) -> Entry {
        Entry {
            // :TODO:
        }
    }
}

//---------------- UPDATE ----------------

/// Handles update operations by merging any newly provided fields
impl Updateable<UpdateRequest> for Entry {
    fn update_with(&self, e: &UpdateRequest) -> Entry {
        Entry {
            // :TODO:
        }
    }
}
