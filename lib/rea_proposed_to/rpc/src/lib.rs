/**
 * Holo-REA proposal addressees zome I/O data structures
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

use vf_core::type_aliases::{AgentAddress, ProposalAddress};

//---------------- EXTERNAL RECORD STRUCTURE ----------------

// Export external type interface to allow consuming zomes to easily import & define zome API
pub use vf_core::type_aliases::ProposedToAddress;

/// I/O struct to describe the complete record, including all managed link fields
///
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub id: ProposedToAddress,
    pub proposed_to: AgentAddress,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposed: Option<Vec<ProposalAddress>>,
}

/// I/O struct to describe what is returned outside the gateway.
/// Responses are usually returned as named attributes in order to leave space
/// for future additional return values.
///
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData {
    pub proposed_to: Response,
}

//---------------- CREATE REQUEST ----------------

/// I/O struct to describe the complete input record, including all managed links
///
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    pub proposed_to: AgentAddress,
    pub proposed: ProposalAddress,
}

impl<'a> CreateRequest {
    // :TODO: accessors for field data
}

//---------------- QUERY FILTER REQUEST ----------------

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    pub proposed: Option<ProposalAddress>,
}
