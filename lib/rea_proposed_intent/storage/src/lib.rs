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

use holochain_json_api::{error::JsonError, json::JsonString};
use holochain_json_derive::DefaultJson;

use vf_core::type_aliases::{IntentAddress,ProposalAddress};

use hc_zome_rea_proposed_intent_rpc::CreateRequest;

//---------------- RECORD INTERNALS & VALIDATION ----------------

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Entry {
    pub reciprocal: bool,
    pub publishes: IntentAddress,
    pub published_in: ProposalAddress,
}

//---------------- CREATE ----------------

/// Pick relevant fields out of I/O record into underlying DHT entry
impl From<CreateRequest> for Entry {
    fn from(e: CreateRequest) -> Entry {
        Entry {
            reciprocal: e.reciprocal,
            publishes: e.publishes.into(),
            published_in: e.published_in.into(),
        }
    }
}
