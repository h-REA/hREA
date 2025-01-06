/**
 * hREA recipe_exchange zome I/O data structures
 *
 * Required by packages wishing to interact with the zome via its standard RPC interface,
 * and by the zome API handlers accepting these parameters.
 *
 * @package hREA
 */
use holochain_serialized_bytes::prelude::*;

use serde_maybe_undefined::{MaybeUndefined, default_false};
use vf_measurement::QuantityValue;
pub use vf_attributes_hdk::{
    ActionId,
    ExternalURL,
    DateTime, FixedOffset,
    ProcessAddress,
    AgentAddress,
    EconomicResourceAddress,
    ResourceSpecificationAddress,
    // RecipeExchangeAddress,
    ProcessSpecificationAddress,
    SatisfactionAddress,
    LocationAddress,
    ByAction, ByRevision, RecordMeta, RevisionMeta,
};
pub use holo_hash::ActionHash;

//---------------- EXTERNAL RECORD STRUCTURE ----------------

// Export external type interface to allow consuming zomes to easily import & define zome API
pub use vf_attributes_hdk::{ RecipeExchangeAddress, RecipeFlowAddress };

/// I/O struct to describe the complete record, including all managed link fields
///
#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub id: RecipeExchangeAddress,
    pub revision_id: ActionHash,
    pub meta: RecordMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub recipe_clauses: Vec<RecipeFlowAddress>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub recipe_reciprocal_clauses: Vec<RecipeFlowAddress>,
}

/// I/O struct to describe what is returned outside the gateway.
/// Responses are usually returned as named attributes in order to leave space
/// for future additional return values.
///
#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData {
    pub recipe_exchange: Response,
}

//---------------- CREATE REQUEST ----------------

/// I/O struct to describe the complete input record, including all managed links
///
#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    // pub action: ActionId,
    #[serde(default)]
    pub name: MaybeUndefined<String>,
    #[serde(default)]
    pub note: MaybeUndefined<String>,
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
    pub revision_id: ActionHash,
    #[serde(default)]
    pub name: MaybeUndefined<String>,
    #[serde(default)]
    pub note: MaybeUndefined<String>,
}

impl<'a> UpdateRequest {
    pub fn get_revision_id(&'a self) -> &ActionHash {
        &self.revision_id
    }

    // :TODO: accessors for other field data
}

//---------------- QUERY FILTER REQUEST ----------------

#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    pub recipe_clauses: Option<RecipeFlowAddress>,
    pub recipe_reciprocal_clauses: Option<RecipeFlowAddress>,
}
