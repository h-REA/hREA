/**
 * Holo-REA proposed intents: maintains relationships between coordinated proposals and the individual intents that describe their planned enaction. zome I/O data structures
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

use holochain_json_api::{error::JsonError, json::JsonString};
use holochain_json_derive::DefaultJson;

use vf_attributes_hdk::{IntentAddress, ProposalAddress};

//---------------- EXTERNAL RECORD STRUCTURE ----------------

// Export external type interface to allow consuming zomes to easily import & define zome API
pub use vf_attributes_hdk::ProposedIntentAddress;

/// I/O struct to describe the complete record, including all managed link fields
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub id: ProposedIntentAddress,
    pub reciprocal: bool,
    pub published_in: ProposalAddress,
    pub publishes: IntentAddress,
}

/// I/O struct to describe what is returned outside the gateway.
/// Responses are usually returned as named attributes in order to leave space
/// for future additional return values.
///
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData {
    pub proposed_intent: Response,
}

//---------------- CREATE REQUEST ----------------

/// I/O struct to describe the complete input record, including all managed links
///
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
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
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct FwdCreateRequest {
    pub proposed_intent: CreateRequest,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct FwdDeleteRequest {
    pub address: ProposedIntentAddress,
}

//---------------- QUERY FILTER REQUEST ----------------

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    pub published_in: Option<ProposalAddress>,
    pub publishes: Option<IntentAddress>,
}
