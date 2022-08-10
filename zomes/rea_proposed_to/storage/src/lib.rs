/**
 * Holo-REA proposal addressees zome internal data structures
 *
 * Required by the zome itself, and for any DNA-local zomes interacting with its
 * storage API directly.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use hdk_records::{
    RecordAPIResult, DataIntegrityError,
    generate_record_entry,
};

use hc_zome_dna_auth_resolver_core::AvailableCapability;
use vf_attributes_hdk::{ProposedToAddress, AgentAddress, ProposalAddress};

use hc_zome_rea_proposed_to_rpc::CreateRequest;

//--------------- ZOME CONFIGURATION ATTRIBUTES ----------------

// :TODO: remove this, replace with reference to appropriate namespacing of zome config
#[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
pub struct DnaConfigSlice {
    pub proposed_to: ProposedToZomeConfig,
}

#[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
pub struct ProposedToZomeConfig {
    pub proposal_index_zome: String,
    pub index_zome: String,
}

//---------------- RECORD INTERNALS & VALIDATION ----------------

#[derive(Serialize, Deserialize, SerializedBytes, Debug, PartialEq, Clone)]
pub struct EntryData {
    pub proposed_to: AgentAddress,
    pub proposed: ProposalAddress,
}

generate_record_entry!(EntryData, ProposedToAddress, EntryStorage);

//---------------- Holochain App Entry And Link Types Setup ----------------

#[hdk_entry_defs(skip_hdk_extern = true)]
#[unit_enum(EntryTypesUnit)]
pub enum EntryTypes {
    ProposedTo(EntryStorage),
    #[entry_def(visibility = "private")]
    AvailableCapability(AvailableCapability)
}

impl From<EntryStorage> for EntryTypes
{
    fn from(e: EntryStorage) -> EntryTypes
    {
        EntryTypes::ProposedTo(e)
    }
}
impl TryFrom<AvailableCapability> for EntryTypes {
    type Error = DataIntegrityError;

    fn try_from(e: AvailableCapability) -> Result<EntryTypes, Self::Error>
    {
        Ok(EntryTypes::AvailableCapability(e))
    }
}

#[hdk_link_types(skip_no_mangle = true)]
pub enum LinkTypes {
    // relates to dna-auth-resolver mixin
    // and remote authorizations
    AvailableCapability
}

//---------------- CREATE ----------------

/// Pick relevant fields out of I/O record into underlying DHT entry
impl TryFrom<CreateRequest> for EntryData {
    type Error = DataIntegrityError;

    fn try_from(e: CreateRequest) -> RecordAPIResult<EntryData> {
        Ok(EntryData {
            proposed_to: e.proposed_to,
            proposed: e.proposed,
        })
    }
}
