/**
 * Holo-REA proposed intents: maintains relationships between coordinated proposals and the individual intents that describe their planned enaction. zome internal data structures
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
use vf_attributes_hdk::{ProposedIntentAddress, IntentAddress, ProposalAddress};

use hc_zome_rea_proposed_intent_rpc::CreateRequest;

//--------------- ZOME CONFIGURATION ATTRIBUTES ----------------

// :TODO: remove this, replace with reference to appropriate namespacing of zome config
#[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
pub struct DnaConfigSlice {
    pub proposed_intent: ProposedIntentZomeConfig,
}

#[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
pub struct ProposedIntentZomeConfig {
    pub index_zome: String,
    pub proposal_index_zome: String,
    pub intent_index_zome: Option<String>,
}

//---------------- RECORD INTERNALS & VALIDATION ----------------

#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
pub struct EntryData {
    pub reciprocal: bool,
    pub publishes: IntentAddress,
    pub published_in: ProposalAddress,
}

generate_record_entry!(EntryData, ProposedIntentAddress, EntryStorage);

//---------------- Holochain App Entry And Link Types Setup ----------------

#[hdk_entry_defs(skip_hdk_extern = true)]
#[unit_enum(EntryTypesUnit)]
pub enum EntryTypes {
    ProposedIntent(EntryStorage),
    #[entry_def(visibility = "private")]
    AvailableCapability(AvailableCapability)
}

impl From<EntryStorage> for EntryTypes
{
    fn from(e: EntryStorage) -> EntryTypes
    {
        EntryTypes::ProposedIntent(e)
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
            reciprocal: e.reciprocal,
            publishes: e.publishes.into(),
            published_in: e.published_in.into(),
        })
    }
}
