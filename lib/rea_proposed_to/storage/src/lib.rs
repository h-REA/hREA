/**
 * Holo-REA proposal addressees zome internal data structures
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

use holochain_json_api::{error::JsonError, json::JsonString};
use holochain_json_derive::DefaultJson;

use hdk_graph_helpers::record_interface::Updateable;

use vf_core::type_aliases::AgentAddress;

use hc_zome_rea_proposed_to_rpc::{CreateRequest, UpdateRequest};

//---------------- RECORD INTERNALS & VALIDATION ----------------

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Entry {
    pub proposed_to: AgentAddress,
}

//---------------- CREATE ----------------

/// Pick relevant fields out of I/O record into underlying DHT entry
impl From<CreateRequest> for Entry {
    fn from(e: CreateRequest) -> Entry {
        Entry {
            proposed_to: e.proposed_to,
        }
    }
}

//---------------- UPDATE ----------------

/// Handles update operations by merging any newly provided fields
impl Updateable<UpdateRequest> for Entry {
    fn update_with(&self, e: &UpdateRequest) -> Entry {
        Entry {
            proposed_to: if e.proposed_to.is_some() {
                e.proposed_to.to_owned().unwrap()
            } else {
                self.proposed_to.to_owned()
            },
        }
    }
}
