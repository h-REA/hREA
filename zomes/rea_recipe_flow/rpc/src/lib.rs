/**
 * hREA recipe_flow zome I/O data structures
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
    RecipeProcessAddress,
    ProcessSpecificationAddress,
    SatisfactionAddress,
    LocationAddress,
    ProposedRecipeFlowAddress,
    ActionHash, ByAction, ByRevision, RecordMeta, RevisionMeta,
};

//---------------- EXTERNAL RECORD STRUCTURE ----------------

// Export external type interface to allow consuming zomes to easily import & define zome API
pub use vf_attributes_hdk::{ RecipeFlowAddress };

/// I/O struct to describe the complete record, including all managed link fields
///
#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub id: RecipeFlowAddress,
    pub revision_id: ActionHash,
    pub meta: RecordMeta,
    pub action: ActionId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_quantity: Option<QuantityValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effort_quantity: Option<QuantityValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_conforms_to: Option<ResourceSpecificationAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_clause_of: Option<RecipeExchangeAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<ProcessSpecificationAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_input_of: Option<RecipeProcessAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_output_of: Option<RecipeProcessAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
}

/// I/O struct to describe what is returned outside the gateway.
/// Responses are usually returned as named attributes in order to leave space
/// for future additional return values.
///
#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData {
    pub recipe_flow: Response,
}

//---------------- CREATE REQUEST ----------------

/// I/O struct to describe the complete input record, including all managed links
///
#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    pub action: ActionId,
    #[serde(default)]
    pub note: MaybeUndefined<String>,
    #[serde(default)]
    pub state: MaybeUndefined<String>,
    #[serde(default)]
    pub resource_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    pub effort_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    pub resource_conforms_to: MaybeUndefined<ResourceSpecificationAddress>,
    #[serde(default)]
    pub recipe_clause_of: MaybeUndefined<RecipeExchangeAddress>,
    #[serde(default)]
    pub stage: MaybeUndefined<ProcessSpecificationAddress>,
    #[serde(default)]
    pub recipe_input_of: MaybeUndefined<RecipeProcessAddress>,
    #[serde(default)]
    pub recipe_output_of: MaybeUndefined<RecipeProcessAddress>,
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
    pub note: MaybeUndefined<String>,
    #[serde(default)]
    pub state: MaybeUndefined<String>,
    #[serde(default)]
    pub resource_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    pub effort_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    pub resource_conforms_to: MaybeUndefined<ResourceSpecificationAddress>,
    #[serde(default)]
    pub recipe_clause_of: MaybeUndefined<RecipeExchangeAddress>,
    #[serde(default)]
    pub stage: MaybeUndefined<ProcessSpecificationAddress>,
    #[serde(default)]
    pub recipe_input_of: MaybeUndefined<RecipeProcessAddress>,
    #[serde(default)]
    pub recipe_output_of: MaybeUndefined<RecipeProcessAddress>,
}

impl<'a> UpdateRequest {
    pub fn get_revision_id(&'a self) -> &ActionHash {
        &self.revision_id
    }

    // :TODO: accessors for other field data
}

//---------------- QUERY FILTER REQUEST ----------------

// #[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct QueryParams {
//     pub input_of: Option<ProcessAddress>,
//     pub output_of: Option<ProcessAddress>,
//     pub satisfied_by: Option<SatisfactionAddress>,
//     pub proposed_in: Option<ProposedRecipeFlowAddress>,
//     pub provider: Option<AgentAddress>,
//     pub receiver: Option<AgentAddress>,
// }
