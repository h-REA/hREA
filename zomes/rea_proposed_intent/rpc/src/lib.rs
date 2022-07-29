/**
 * Holo-REA proposed intents: maintains relationships between coordinated proposals and the individual intents that describe their planned enaction. zome I/O data structures
 *
 * Required by packages wishing to interact with the zome via its standard RPC interface,
 * and by the zome API handlers accepting these parameters.
 *
 * @package Holo-REA
 */
use holochain_serialized_bytes::prelude::*;
pub use vf_attributes_hdk::{
    HeaderHash, ByAddress, ByHeader, ByRevision, RecordMeta, RevisionMeta,
    ProposedIntentAddress, IntentAddress, ProposalAddress,
};

//---------------- EXTERNAL RECORD STRUCTURE ----------------

/// I/O struct to describe the complete record, including all managed link fields
#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub id: ProposedIntentAddress,
    pub revision_id: HeaderHash,
    pub meta: RecordMeta,
    pub reciprocal: bool,
    pub published_in: ProposalAddress,
    pub publishes: IntentAddress,
}

/// I/O struct to describe what is returned outside the gateway.
/// Responses are usually returned as named attributes in order to leave space
/// for future additional return values.
///
#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData {
    pub proposed_intent: Response,
}

/// Toplevel I/O structs for WASM API

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateParams {
    pub proposed_intent: CreateRequest,
}

//---------------- CREATE REQUEST ----------------

/// I/O struct to describe the complete input record, including all managed links
///
#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    pub reciprocal: bool,
    pub published_in: ProposalAddress,
    pub publishes: IntentAddress,
}

impl<'a> CreateRequest {
    // :TODO: accessors for field data
}

/// I/O struct for forwarding records to other DNAs via zome API
#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
pub struct FwdCreateRequest {
    pub proposed_intent: CreateRequest,
}

#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
pub struct FwdDeleteRequest {
    pub address: ProposedIntentAddress,
}

//---------------- QUERY FILTER REQUEST ----------------

#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    pub published_in: Option<ProposalAddress>,
    pub publishes: Option<IntentAddress>,
}
