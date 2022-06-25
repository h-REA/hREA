/**
 * Holo-REA agent zome I/O data structures
 *
 * Required by packages wishing to interact with the zome via its standard RPC interface,
 * and by the zome API handlers accepting these parameters.
 *
 * @package Holo-REA
 */
use holochain_serialized_bytes::prelude::*;

use serde_maybe_undefined::MaybeUndefined;
pub use vf_attributes_hdk::{
    AgentAddress,
    ProcessAddress,
    EconomicEventAddress,
    CommitmentAddress,
    IntentAddress,
    HeaderHash,
    ExternalURL,
    EconomicResourceAddress,
    PlanAddress,
    ProposalAddress,
    ByRevision,

};

//---------------- EXTERNAL RECORD STRUCTURE ----------------

/// I/O struct to describe the complete record, including all managed link fields
///
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub id: AgentAddress,
    pub revision_id: HeaderHash,
    pub name: String,
    pub agent_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<ExternalURL>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classified_as: Option<Vec<ExternalURL>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub commitments_as_provider: Vec<CommitmentAddress>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub commitments_as_receiver: Vec<CommitmentAddress>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub intents_as_provider: Vec<IntentAddress>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub intents_as_receiver: Vec<IntentAddress>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub economic_events_as_provider: Vec<EconomicEventAddress>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub economic_events_as_receiver: Vec<EconomicEventAddress>,
}

/// I/O struct to describe what is returned outside the gateway.
/// Responses are usually returned as named attributes in order to leave space
/// for future additional return values.
///
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData {
    pub agent: Response,
}

//---------------- CREATE REQUEST ----------------

/// I/O struct to describe the complete input record, including all managed links
///
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    #[serde(default)]
    pub name: String,
    #[serde()]
    pub agent_type: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub image: MaybeUndefined<ExternalURL>,
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub classified_as: MaybeUndefined<Vec<ExternalURL>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub note: MaybeUndefined<String>,
}

impl<'a> CreateRequest {
    // :TODO: accessors for field data
}

//---------------- UPDATE REQUEST ----------------

/// I/O struct to describe the complete input record, including all managed links
///
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRequest {
    pub revision_id: HeaderHash,
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub name: MaybeUndefined<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub image: MaybeUndefined<ExternalURL>,
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub classified_as: MaybeUndefined<Vec<ExternalURL>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub note: MaybeUndefined<String>,
}

impl<'a> UpdateRequest {
    pub fn get_revision_id(&self) -> HeaderHash {
        self.revision_id.to_owned().into()
    }

    // :TODO: accessors for other field data
}

//---------------- QUERY FILTER REQUEST ----------------

#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    // pub commitments: Option<CommitmentAddress>,
    // pub intents: Option<IntentAddress>,
    // pub economic_events: Option<EconomicEventAddress>,
    // pub inventoried_economic_resources: Option<EconomicResourceAddress>,
    // pub plans: Option<PlanAddress>,
    // pub processes: Option<ProcessAddress>,
    // pub proposals: Option<ProposalAddress>,
    pub commitments_as_provider: Option<CommitmentAddress>,
    pub commitments_as_receiver: Option<CommitmentAddress>,
    pub intents_as_provider: Option<IntentAddress>,
    pub intents_as_receiver: Option<IntentAddress>,
    pub economic_events_as_provider: Option<EconomicEventAddress>,
    pub economic_events_as_receiver: Option<EconomicEventAddress>,
}
