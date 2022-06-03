/**
 * Holo-REA proposal zome I/O data structures
 *
 * Required by packages wishing to interact with the zome via its standard RPC interface,
 * and by the zome API handlers accepting these parameters.
 *
 * @package Holo-REA
 */
use holochain_serialized_bytes::prelude::*;
use serde_maybe_undefined::MaybeUndefined;
pub use vf_attributes_hdk::{
    HeaderHash, ByAddress, ByHeader,
    ProposalAddress, ProposedIntentAddress, ProposedToAddress,
    DateTime, FixedOffset,
};

/// Toplevel I/O structs for WASM API

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateParams {
    pub proposal: CreateRequest,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateParams {
    pub proposal: UpdateRequest,
}

//---------------- EXTERNAL RECORD STRUCTURE ----------------

/// I/O struct to describe the complete record, including all managed link fields
///
#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub id: ProposalAddress,
    pub revision_id: HeaderHash,
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_beginning: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_end: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_based: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_scope_of: Option<Vec<String>>,
    // links:
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub publishes: Vec<ProposedIntentAddress>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub published_to: Vec<ProposedToAddress>,
}

/// I/O struct to describe what is returned outside the gateway.
/// Responses are usually returned as named attributes in order to leave space
/// for future additional return values.
///
#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData {
    pub proposal: Response,
}

//---------------- CREATE REQUEST ----------------

/// I/O struct to describe the complete input record, including all managed links
///
#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    #[serde(default)]
    pub name: MaybeUndefined<String>,
    #[serde(default)]
    pub has_beginning: MaybeUndefined<DateTime<FixedOffset>>,
    #[serde(default)]
    pub has_end: MaybeUndefined<DateTime<FixedOffset>>,
    #[serde(default)]
    pub unit_based: MaybeUndefined<bool>,
    #[serde(default)]
    pub created: MaybeUndefined<DateTime<FixedOffset>>,
    #[serde(default)]
    pub note: MaybeUndefined<String>,
    #[serde(default)]
    pub in_scope_of: MaybeUndefined<Vec<String>>,
}

impl<'a> CreateRequest {
    // :TODO: accessors for field data
}

//---------------- UPDATE REQUEST ----------------

/// I/O struct to describe the complete input record, including all managed links
///
#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRequest {
    pub revision_id: HeaderHash,
    #[serde(default)]
    pub name: MaybeUndefined<String>,
    #[serde(default)]
    pub has_beginning: MaybeUndefined<DateTime<FixedOffset>>,
    #[serde(default)]
    pub has_end: MaybeUndefined<DateTime<FixedOffset>>,
    #[serde(default)]
    pub unit_based: MaybeUndefined<bool>,
    #[serde(default)]
    pub note: MaybeUndefined<String>,
    #[serde(default)]
    pub in_scope_of: MaybeUndefined<Vec<String>>,
}

impl<'a> UpdateRequest {
    pub fn get_revision_id(&'a self) -> &HeaderHash {
        &self.revision_id
    }

    // :TODO: accessors for other field data
}

//---------------- QUERY FILTER REQUEST ----------------

#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    pub publishes: Option<ProposedIntentAddress>,
    pub published_to: Option<ProposedToAddress>,
}
