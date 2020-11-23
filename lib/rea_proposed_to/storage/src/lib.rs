/**
 * Holo-REA proposal addressees zome internal data structures
 *
 * Required by the zome itself, and for any DNA-local zomes interacting with its
 * storage API directly.
 *
 * @package Holo-REA
 */
use holochain_serialized_bytes::prelude::*;

use hdk_graph_helpers::{
    record_interface::{Updateable},
    bind_identity,
};

use vf_core::type_aliases::{AgentAddress, ProposalAddress};

use hc_zome_rea_proposed_to_rpc::CreateRequest;

//---------------- RECORD INTERNALS & VALIDATION ----------------

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Entry {
    pub proposed_to: AgentAddress,
    pub proposed: ProposalAddress,
}
bind_identity!(Entry: id="rea_proposed_to");

//---------------- CREATE ----------------

/// Pick relevant fields out of I/O record into underlying DHT entry
impl From<CreateRequest> for Entry {
    fn from(e: CreateRequest) -> Entry {
        Entry {
            proposed_to: e.proposed_to,
            proposed: e.proposed,
        }
    }
}
