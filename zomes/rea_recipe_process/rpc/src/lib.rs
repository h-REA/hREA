/**
 * hREA recipe_process zome I/O data structures
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
    RecipeExchangeAddress,
    ProcessSpecificationAddress,
    SatisfactionAddress,
    LocationAddress,
    ActionHash, ByAction, ByRevision, RecordMeta, RevisionMeta,
};

//---------------- EXTERNAL RECORD STRUCTURE ----------------

// Export external type interface to allow consuming zomes to easily import & define zome API
pub use vf_attributes_hdk::{ RecipeProcessAddress };

/// I/O struct to describe the complete record, including all managed link fields
///
#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub id: RecipeProcessAddress,
    pub revision_id: ActionHash,
    pub meta: RecordMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_conforms_to: Option<ProcessSpecificationAddress>,
}

/// I/O struct to describe what is returned outside the gateway.
/// Responses are usually returned as named attributes in order to leave space
/// for future additional return values.
///
#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData {
    pub recipe_process: Response,
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
    pub image: MaybeUndefined<String>,
    #[serde(default)]
    pub note: MaybeUndefined<String>,
    #[serde(default)]
    pub process_conforms_to: MaybeUndefined<ProcessSpecificationAddress>,
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
    pub image: MaybeUndefined<String>,
    #[serde(default)]
    pub note: MaybeUndefined<String>,
    #[serde(default)]
    pub process_conforms_to: MaybeUndefined<ProcessSpecificationAddress>,
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
    pub process_conforms_to: Option<ProcessSpecificationAddress>,
}
