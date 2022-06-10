/**
 * Holo-REA agent zome I/O data structures
 *
 * Required by packages wishing to interact with the zome via its standard RPC interface,
 * and by the zome API handlers accepting these parameters.
 *
 * @package Holo-REA
 */
use holochain_serialized_bytes::prelude::*;

// use serde_maybe_undefined::MaybeUndefined;
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<ExternalURL>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classified_as: Option<Vec<ExternalURL>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
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

// :TODO: CRUD structs

//---------------- QUERY FILTER REQUEST ----------------

#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    pub commitments: Option<CommitmentAddress>,
    pub intents: Option<IntentAddress>,
    pub economic_events: Option<EconomicEventAddress>,
    pub inventoried_economic_resources: Option<EconomicResourceAddress>,
    pub plans: Option<PlanAddress>,
    pub processes: Option<ProcessAddress>,
    pub proposals: Option<ProposalAddress>,
}
