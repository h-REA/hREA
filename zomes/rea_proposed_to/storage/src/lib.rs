/**
 * Holo-REA proposal addressees zome internal data structures
 *
 * Required by the zome itself, and for any DNA-local zomes interacting with its
 * storage API directly.
 *
 * @package Holo-REA
 */
use holochain_serialized_bytes::prelude::*;

use hdk_records::{
    record_interface::{Updateable},
    bind_identity,
};

use vf_attributes_hdk::{AgentAddress, ProposalAddress};

use hc_zome_rea_proposed_to_rpc::CreateRequest;

//---------------- RECORD INTERNALS & VALIDATION ----------------

#[derive(Serialize, Deserialize, SerializedBytes, Debug, PartialEq, Clone)]
pub struct EntryData {
    pub proposed_to: AgentAddress,
    pub proposed: ProposalAddress,
}
bind_identity!(EntryData, EntryStorage);

//---------------- CREATE ----------------

/// Pick relevant fields out of I/O record into underlying DHT entry
impl From<CreateRequest> for EntryData {
    fn from(e: CreateRequest) -> EntryData {
        EntryData {
            proposed_to: e.proposed_to,
            proposed: e.proposed,
        }
    }
}
