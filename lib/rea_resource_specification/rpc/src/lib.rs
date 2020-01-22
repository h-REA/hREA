/**
 * Holo-REA resource specification zome I/O data structures
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

use hdk_graph_helpers::MaybeUndefined;
use vf_core::type_aliases::ExternalURL;

//---------------- EXTERNAL RECORD STRUCTURE ----------------

// Export external type interface to allow consuming zomes to easily import & define zome API
pub use vf_core::type_aliases::{ ResourceSpecificationAddress };

/// I/O struct to describe the complete record, including all managed link fields
///
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub id: ResourceSpecificationAddress,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<ExternalURL>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

/// I/O struct to describe what is returned outside the gateway.
/// Responses are usually returned as named attributes in order to leave space
/// for future additional return values.
///
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData {
    pub resource_specification: Response,
}

//---------------- CREATE REQUEST ----------------

/// I/O struct to describe the complete input record, including all managed links
///
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    pub name: String,
    #[serde(default)]
    pub image: MaybeUndefined<ExternalURL>,
    #[serde(default)]
    pub note: MaybeUndefined<String>,
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
    pub id: ResourceSpecificationAddress,
    #[serde(default)]
    pub name: MaybeUndefined<String>,
    #[serde(default)]
    pub image: MaybeUndefined<ExternalURL>,
    #[serde(default)]
    pub note: MaybeUndefined<String>,
}

impl<'a> UpdateRequest {
    pub fn get_id(&'a self) -> &ResourceSpecificationAddress {
        &self.id
    }

    // :TODO: accessors for other field data
}

//---------------- QUERY FILTER REQUEST ----------------

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    // :TODO:
}
