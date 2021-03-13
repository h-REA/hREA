/**
 * Holo-REA measurement unit zome internal data structures
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

use hc_zome_rea_unit_rpc::{ CreateRequest, UpdateRequest };

//---------------- RECORD INTERNALS & VALIDATION ----------------

#[derive(Serialize, Deserialize, Debug, DefaultJson, Default, Clone)]
pub struct Entry {
    pub label: String,
    pub symbol: String,
}

impl<'a> Entry {
    pub fn get_symbol(&'a self) -> String {
        self.symbol.to_owned()
    }
}

//---------------- CREATE ----------------

/// Pick relevant fields out of I/O record into underlying DHT entry
impl From<CreateRequest> for Entry {
    fn from(e: CreateRequest) -> Entry {
        Entry {
            label: e.label.into(),
            symbol: e.symbol.into(),
        }
    }
}

//---------------- UPDATE ----------------

/// Handles update operations by merging any newly provided fields
impl Updateable<UpdateRequest> for Entry {
    fn update_with(&self, e: &UpdateRequest) -> Entry {
        Entry {
            label:   if !e.label.is_some()   { self.label.to_owned()   } else { e.label.to_owned().unwrap() },
            symbol: if !e.symbol.is_some() { self.symbol.to_owned() } else { e.symbol.to_owned().unwrap() },
        }
    }
}
