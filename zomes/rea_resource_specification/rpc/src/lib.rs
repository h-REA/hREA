/**
 * Holo-REA resource specification zome I/O data structures
 *
 * Required by packages wishing to interact with the zome via its standard RPC interface,
 * and by the zome API handlers accepting these parameters.
 *
 * @package Holo-REA
 */
use holochain_serialized_bytes::prelude::*;
use serde_maybe_undefined::MaybeUndefined;
pub use vf_attributes_hdk::{
    HeaderHash, ByAddress, ByHeader,
    ResourceSpecificationAddress,
    EconomicResourceAddress,
    ExternalURL,
    UnitId,
};

// toplevel I/O structs for WASM API

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateParams {
    pub resource_specification: CreateRequest,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateParams {
    pub resource_specification: UpdateRequest,
}

//---------------- EXTERNAL RECORD STRUCTURE ----------------

/// I/O struct to describe the complete record, including all managed link fields
///
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub id: ResourceSpecificationAddress,
    pub revision_id: HeaderHash,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<ExternalURL>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_unit_of_effort: Option<UnitId>,
}

impl<'a> Response {
    pub fn into_cursor(&'a self) -> Result<String, std::string::FromUtf8Error> {
        let bytes: Vec<u8> = self.id.to_owned().into();
        String::from_utf8(bytes)
    }
}

/// I/O struct to describe what is returned outside the gateway.
/// Responses are usually returned as named attributes in order to leave space
/// for future additional return values.
///
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData {
    pub resource_specification: Response,
}

//---------------- CREATE REQUEST ----------------

/// I/O struct to describe the complete input record, including all managed links
///
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    pub name: String,
    #[serde(default)]
    pub image: MaybeUndefined<ExternalURL>,
    #[serde(default)]
    pub note: MaybeUndefined<String>,
    #[serde(default)]
    pub default_unit_of_effort: MaybeUndefined<UnitId>,
}

impl<'a> CreateRequest {
    // :TODO: accessors for field data
}

//---------------- UPDATE REQUEST ----------------

/// I/O struct to describe the complete input record, including all managed links
///
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRequest {
    pub revision_id: HeaderHash,
    #[serde(default)]
    pub name: MaybeUndefined<String>,
    #[serde(default)]
    pub image: MaybeUndefined<ExternalURL>,
    #[serde(default)]
    pub note: MaybeUndefined<String>,
    #[serde(default)]
    pub default_unit_of_effort: MaybeUndefined<UnitId>,
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
    pub conforming_resources: Option<EconomicResourceAddress>,
}
