/**
 * Holo-REA intent zome I/O data structures
 *
 * Required by packages wishing to interact with the zome via its standard RPC interface,
 * and by the zome API handlers accepting these parameters.
 *
 * @package Holo-REA
 */
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use holochain_json_api::{ json::JsonString, error::JsonError };
use holochain_json_derive::{ DefaultJson };

use hdk_records::{
    MaybeUndefined,
    maybe_undefined::default_false,
};
use vf_core::{
    measurement::QuantityValue,
    type_aliases::{
        ActionId,
        ExternalURL,
        Timestamp,
        ProcessAddress,
        AgentAddress,
        ResourceAddress,
        ResourceSpecificationAddress,
        SatisfactionAddress,
        LocationAddress,
    },
};

//---------------- EXTERNAL RECORD STRUCTURE ----------------

// Export external type interface to allow consuming zomes to easily import & define zome API
pub use vf_attributes_hdk::{ IntentAddress };

/// I/O struct to describe the complete record, including all managed link fields
///
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub id: IntentAddress,
    pub action: ActionId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(default)]
    pub image: Option<ExternalURL>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_of: Option<ProcessAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_of: Option<ProcessAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<AgentAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver: Option<AgentAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_inventoried_as: Option<ResourceAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_classified_as: Option<Vec<ExternalURL>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_conforms_to: Option<ResourceSpecificationAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_quantity: Option<QuantityValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effort_quantity: Option<QuantityValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_quantity: Option<QuantityValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_beginning: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_end: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_point_in_time: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_location: Option<LocationAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agreed_in: Option<ExternalURL>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_scope_of: Option<Vec<String>>,
    pub finished: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub satisfied_by: Option<Vec<SatisfactionAddress>>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub published_in: Option<Vec<ProposedIntentAddress>>,
}

/// I/O struct to describe what is returned outside the gateway.
/// Responses are usually returned as named attributes in order to leave space
/// for future additional return values.
///
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData {
    pub intent: Response,
}

//---------------- CREATE REQUEST ----------------

/// I/O struct to describe the complete input record, including all managed links
///
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    pub action: ActionId,
    #[serde(default)]
    pub note: MaybeUndefined<String>,
    #[serde(default)]
    pub image: MaybeUndefined<ExternalURL>,
    #[serde(default)]
    pub input_of: MaybeUndefined<ProcessAddress>,
    #[serde(default)]
    pub output_of: MaybeUndefined<ProcessAddress>,
    #[serde(default)]
    pub provider: MaybeUndefined<AgentAddress>,
    #[serde(default)]
    pub receiver: MaybeUndefined<AgentAddress>,
    #[serde(default)]
    pub resource_inventoried_as: MaybeUndefined<ResourceAddress>,
    #[serde(default)]
    pub resource_classified_as: MaybeUndefined<Vec<ExternalURL>>,
    #[serde(default)]
    pub resource_conforms_to: MaybeUndefined<ResourceSpecificationAddress>,
    #[serde(default)]
    pub resource_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    pub effort_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    pub available_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    pub has_beginning: MaybeUndefined<Timestamp>,
    #[serde(default)]
    pub has_end: MaybeUndefined<Timestamp>,
    #[serde(default)]
    pub has_point_in_time: MaybeUndefined<Timestamp>,
    #[serde(default)]
    pub due: MaybeUndefined<Timestamp>,
    #[serde(default)]
    pub at_location: MaybeUndefined<LocationAddress>,
    #[serde(default)]
    pub agreed_in: MaybeUndefined<ExternalURL>,
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
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRequest {
    pub id: IntentAddress,
    #[serde(default)]
    pub action: MaybeUndefined<ActionId>,
    #[serde(default)]
    pub note: MaybeUndefined<String>,
    #[serde(default)]
    pub image: MaybeUndefined<ExternalURL>,
    #[serde(default)]
    pub input_of: MaybeUndefined<ProcessAddress>,
    #[serde(default)]
    pub output_of: MaybeUndefined<ProcessAddress>,
    #[serde(default)]
    pub provider: MaybeUndefined<AgentAddress>,
    #[serde(default)]
    pub receiver: MaybeUndefined<AgentAddress>,
    #[serde(default)]
    pub resource_inventoried_as: MaybeUndefined<ResourceAddress>,
    #[serde(default)]
    pub resource_classified_as: MaybeUndefined<Vec<ExternalURL>>,
    #[serde(default)]
    pub resource_conforms_to: MaybeUndefined<ResourceSpecificationAddress>,
    #[serde(default)]
    pub resource_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    pub effort_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    pub available_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    pub has_beginning: MaybeUndefined<Timestamp>,
    #[serde(default)]
    pub has_end: MaybeUndefined<Timestamp>,
    #[serde(default)]
    pub has_point_in_time: MaybeUndefined<Timestamp>,
    #[serde(default)]
    pub due: MaybeUndefined<Timestamp>,
    #[serde(default)]
    pub at_location: MaybeUndefined<LocationAddress>,
    #[serde(default)]
    pub agreed_in: MaybeUndefined<ExternalURL>,
    #[serde(default)]
    pub finished: MaybeUndefined<bool>,
    #[serde(default)]
    pub in_scope_of: MaybeUndefined<Vec<String>>,
}

impl<'a> UpdateRequest {
    pub fn get_id(&'a self) -> &IntentAddress {
        &self.id
    }

    // :TODO: accessors for other field data
}

//---------------- QUERY FILTER REQUEST ----------------

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    pub input_of: Option<ProcessAddress>,
    pub output_of: Option<ProcessAddress>,
    pub satisfied_by: Option<SatisfactionAddress>,
}
