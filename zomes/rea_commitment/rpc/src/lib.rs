/**
 * Holo-REA commitment zome I/O data structures
 *
 * Required by packages wishing to interact with the zome via its standard RPC interface,
 * and by the zome API handlers accepting these parameters.
 *
 * @package Holo-REA
 */
use holochain_serialized_bytes::prelude::*;

use serde_maybe_undefined::{MaybeUndefined, default_false};
use vf_measurement::QuantityValue;
pub use vf_attributes_hdk::{
    HeaderHash, ByHeader, ByRevision,
    ActionId,
    DateTime, FixedOffset,
    ExternalURL,
    LocationAddress,
    AgentAddress,
    EconomicResourceAddress,
    ProcessAddress,
    ResourceSpecificationAddress,
    PlanAddress,
    AgreementAddress,
    FulfillmentAddress,
    SatisfactionAddress,
};

//---------------- EXTERNAL RECORD STRUCTURE ----------------

// Export external type interface to allow consuming zomes to easily import & define zome API
pub use vf_attributes_hdk::{ CommitmentAddress };

/// I/O struct to describe the complete record, including all managed link fields
///
#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub id: CommitmentAddress,
    pub revision_id: HeaderHash,
    pub action: ActionId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_of: Option<ProcessAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_of: Option<ProcessAddress>,
    pub provider: AgentAddress,
    pub receiver: AgentAddress,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_inventoried_as: Option<EconomicResourceAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_classified_as: Option<Vec<ExternalURL>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_conforms_to: Option<ResourceSpecificationAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_quantity: Option<QuantityValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effort_quantity: Option<QuantityValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_beginning: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_end: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_point_in_time: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_location: Option<LocationAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<PlanAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_scope_of: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agreed_in: Option<ExternalURL>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clause_of: Option<AgreementAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub independent_demand_of: Option<PlanAddress>,

    pub finished: bool,

    // LINK FIELDS
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fulfilled_by: Vec<FulfillmentAddress>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub satisfies: Vec<SatisfactionAddress>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub involved_agents: Vec<AgentAddress>,
}

/// I/O struct to describe what is returned outside the gateway.
/// Responses are usually returned as named attributes in order to leave space
/// for future additional return values.
///
#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData {
    pub commitment: Response,
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
    pub input_of: MaybeUndefined<ProcessAddress>,
    #[serde(default)]
    pub output_of: MaybeUndefined<ProcessAddress>,
    pub provider: AgentAddress,
    pub receiver: AgentAddress,
    #[serde(default)]
    pub resource_inventoried_as: MaybeUndefined<EconomicResourceAddress>,
    #[serde(default)]
    pub resource_classified_as: MaybeUndefined<Vec<ExternalURL>>,
    #[serde(default)]
    pub resource_conforms_to: MaybeUndefined<ResourceSpecificationAddress>,
    #[serde(default)]
    pub resource_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    pub effort_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    pub has_beginning: MaybeUndefined<DateTime<FixedOffset>>,
    #[serde(default)]
    pub has_end: MaybeUndefined<DateTime<FixedOffset>>,
    #[serde(default)]
    pub has_point_in_time: MaybeUndefined<DateTime<FixedOffset>>,
    #[serde(default)]
    pub due: MaybeUndefined<DateTime<FixedOffset>>,
    #[serde(default)]
    pub at_location: MaybeUndefined<LocationAddress>,
    #[serde(default)]
    pub plan: MaybeUndefined<PlanAddress>,
    #[serde(default)]
    pub agreed_in: MaybeUndefined<ExternalURL>,
    #[serde(default)]
    pub clause_of: MaybeUndefined<AgreementAddress>,
    #[serde(default)]
    pub independent_demand_of: MaybeUndefined<PlanAddress>,
    #[serde(default = "default_false")]
    pub finished: MaybeUndefined<bool>,
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
    pub action: MaybeUndefined<ActionId>,
    #[serde(default)]
    pub note: MaybeUndefined<String>,
    #[serde(default)]
    pub input_of: MaybeUndefined<ProcessAddress>,
    #[serde(default)]
    pub output_of: MaybeUndefined<ProcessAddress>,
    #[serde(default)]
    pub provider: MaybeUndefined<AgentAddress>,
    #[serde(default)]
    pub receiver: MaybeUndefined<AgentAddress>,
    #[serde(default)]
    pub resource_inventoried_as: MaybeUndefined<EconomicResourceAddress>,
    #[serde(default)]
    pub resource_classified_as: MaybeUndefined<Vec<ExternalURL>>,
    #[serde(default)]
    pub resource_conforms_to: MaybeUndefined<ResourceSpecificationAddress>,
    #[serde(default)]
    pub resource_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    pub effort_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    pub has_beginning: MaybeUndefined<DateTime<FixedOffset>>,
    #[serde(default)]
    pub has_end: MaybeUndefined<DateTime<FixedOffset>>,
    #[serde(default)]
    pub has_point_in_time: MaybeUndefined<DateTime<FixedOffset>>,
    #[serde(default)]
    pub due: MaybeUndefined<DateTime<FixedOffset>>,
    #[serde(default)]
    pub at_location: MaybeUndefined<LocationAddress>,
    #[serde(default)]
    pub plan: MaybeUndefined<PlanAddress>,
    #[serde(default)]
    pub agreed_in: MaybeUndefined<ExternalURL>,
    #[serde(default)]
    pub clause_of: MaybeUndefined<AgreementAddress>,
    #[serde(default)]
    pub independent_demand_of: MaybeUndefined<PlanAddress>,
    #[serde(default)]
    pub finished: MaybeUndefined<bool>,
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
    pub input_of: Option<ProcessAddress>,
    pub output_of: Option<ProcessAddress>,
    pub fulfilled_by: Option<FulfillmentAddress>,
    pub satisfies: Option<SatisfactionAddress>,
    pub clause_of: Option<AgreementAddress>,
    pub provider: Option<AgentAddress>,
    pub receiver: Option<AgentAddress>,
    pub independent_demand_of: Option<PlanAddress>,
    pub planned_within: Option<PlanAddress>,
}
