/**
 * Holo-REA fulfillment zome I/O data structures
 *
 * Required by packages wishing to interact with the zome via its standard RPC interface,
 * and by the zome API handlers accepting these parameters.
 *
 * @package Holo-REA
 */
use holochain_serialized_bytes::prelude::*;

use serde_bytes::ByteBuf;
use serde_maybe_undefined::{MaybeUndefined};
use vf_measurement::QuantityValue;
pub use vf_attributes_hdk::{
    HeaderHash, ByHeader, ByAddress, ByRevision, RevisionMeta,
    EconomicEventAddress,
    CommitmentAddress,
};

/// Toplevel I/O structs for WASM API

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateParams {
    pub fulfillment: CreateRequest,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateParams {
    pub fulfillment: UpdateRequest,
}

//---------------- EXTERNAL RECORD STRUCTURE ----------------

// Export external type interface to allow consuming zomes to easily import & define zome API
pub use vf_attributes_hdk::{ FulfillmentAddress };

/// I/O struct to describe the complete record, including all managed link fields
///
#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub id: FulfillmentAddress,
    pub revision_id: HeaderHash,
    pub meta: RevisionMeta,
    pub fulfilled_by: EconomicEventAddress,
    pub fulfills: CommitmentAddress,
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
    pub fulfillment: Response,
}

//---------------- CREATE REQUEST ----------------

/// I/O struct to describe the complete input record, including all managed links
///
#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    pub fulfilled_by: EconomicEventAddress,
    pub fulfills: CommitmentAddress,
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub resource_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub effort_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub note: MaybeUndefined<String>,
    // :TODO: nonce is sendable since records in split cells must have same hash.
    // This is attackable to force non-unique entries.
    // Eventually we will need more robust means of cross-cell record association.
    // @see https://github.com/h-REA/hREA/issues/266
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub nonce: MaybeUndefined<ByteBuf>,
}

impl<'a> CreateRequest {
    // :TODO: accessors for field data

    pub fn get_fulfilled_by(&'a self) -> &EconomicEventAddress {
        &self.fulfilled_by
    }

    pub fn get_fulfills(&'a self) -> &CommitmentAddress {
        &self.fulfills
    }
}

//---------------- UPDATE REQUEST ----------------

/// I/O struct to describe the complete input record, including all managed links
///
#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRequest {
    pub revision_id: HeaderHash,
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub fulfilled_by: MaybeUndefined<EconomicEventAddress>, // note this setup allows None to be passed but `update_with` ignores it
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub fulfills: MaybeUndefined<CommitmentAddress>,
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
    pub fn get_revision_id(&'a self) -> &HeaderHash {
        &self.revision_id
    }

    // :TODO: accessors for other field data
}

//---------------- QUERY FILTER REQUEST ----------------

#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    pub fulfills: Option<CommitmentAddress>,
    pub fulfilled_by: Option<EconomicEventAddress>,
}
