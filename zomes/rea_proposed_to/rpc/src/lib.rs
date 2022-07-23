/**
 * Holo-REA proposal addressees zome I/O data structures
 *
 * Required by packages wishing to interact with the zome via its standard RPC interface,
 * and by the zome API handlers accepting these parameters.
 *
 * @package Holo-REA
 */
use holochain_serialized_bytes::prelude::*;
pub use vf_attributes_hdk::{
    HeaderHash, ByAddress, ByHeader, ByRevision, RevisionMeta,
    ProposedToAddress, AgentAddress, ProposalAddress,
};

/// Toplevel I/O structs for WASM API

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateParams {
    pub proposed_to: CreateRequest,
}

//---------------- EXTERNAL RECORD STRUCTURE ----------------

/// I/O struct to describe the complete record, including all managed link fields
///
#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub id: ProposedToAddress,
    pub revision_id: HeaderHash,
    pub meta: RevisionMeta,
    pub proposed_to: AgentAddress,
    pub proposed: ProposalAddress,
}

/// I/O struct to describe what is returned outside the gateway.
/// Responses are usually returned as named attributes in order to leave space
/// for future additional return values.
///
#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData {
    pub proposed_to: Response,
}

//---------------- CREATE REQUEST ----------------

/// I/O struct to describe the complete input record, including all managed links
///
#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    pub proposed_to: AgentAddress,
    pub proposed: ProposalAddress,
}

impl<'a> CreateRequest {
    // :TODO: accessors for field data
}

//---------------- QUERY FILTER REQUEST ----------------

#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    pub proposed: Option<ProposalAddress>,
}
