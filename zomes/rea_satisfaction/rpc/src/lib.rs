/**
 * Holo-REA satisfaction zome I/O data structures
 *
 * Required by packages wishing to interact with the zome via its standard RPC interface,
 * and by the zome API handlers accepting these parameters.
 *
 * @package Holo-REA
 */
use holochain_serialized_bytes::prelude::*;

use serde_maybe_undefined::{MaybeUndefined};
use vf_measurement::QuantityValue;
pub use vf_attributes_hdk::{
    RevisionHash, ByHeader, ByAddress,
    SatisfactionAddress,
    EventOrCommitmentAddress,
    EconomicEventAddress,
    CommitmentAddress,
    IntentAddress,
};

/// Toplevel I/O structs for WASM API

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateParams {
    pub satisfaction: CreateRequest,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateParams {
    pub satisfaction: UpdateRequest,
}

//---------------- EXTERNAL RECORD STRUCTURE ----------------

/// I/O struct to describe the complete record, including all managed link fields
///
#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub id: SatisfactionAddress,
    pub revision_id: RevisionHash,
    pub satisfied_by: EventOrCommitmentAddress,
    pub satisfies: IntentAddress,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_quantity: Option<QuantityValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effort_quantity: Option<QuantityValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

/// I/O struct to describe what is returned outside the gateway.
/// Responses are usually returned as named attributes in order to leave space
/// for future additional return values.
///
#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData {
    pub satisfaction: Response,
}

//---------------- CREATE REQUEST ----------------

/// I/O struct to describe the complete input record, including all managed links
///
#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    pub satisfied_by: EventOrCommitmentAddress,
    pub satisfies: IntentAddress,
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub resource_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub effort_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub note: MaybeUndefined<String>,
}

impl<'a> CreateRequest {
    // :TODO: accessors for field data

    pub fn get_satisfied_by(&'a self) -> &EventOrCommitmentAddress {
        &self.satisfied_by
    }

    pub fn get_satisfies(&'a self) -> &IntentAddress {
        &self.satisfies
    }
}

//---------------- UPDATE REQUEST ----------------

/// I/O struct to describe the complete input record, including all managed links
///
#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRequest {
    pub revision_id: RevisionHash,
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub satisfied_by: MaybeUndefined<EventOrCommitmentAddress>, // note this setup allows None to be passed but `update_with` ignores it
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub satisfies: MaybeUndefined<IntentAddress>,
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub resource_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub effort_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub note: MaybeUndefined<String>,
}

impl<'a> UpdateRequest {
    pub fn get_revision_id(&'a self) -> &RevisionHash {
        &self.revision_id
    }

    // :TODO: accessors for other field data
}

//---------------- QUERY FILTER REQUEST ----------------

#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    pub satisfies: Option<IntentAddress>,
    pub satisfied_by: Option<CommitmentAddress>,
}
