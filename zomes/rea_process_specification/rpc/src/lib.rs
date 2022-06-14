/**
 * Holo-REA process specification zome I/O data structures
 *
 * Required by packages wishing to interact with the zome via its standard RPC interface,
 * and by the zome API handlers accepting these parameters.
 *
 * @package Holo-REA
 */
use holochain_serialized_bytes::prelude::*;
use serde_maybe_undefined::MaybeUndefined;
pub use vf_attributes_hdk::{
    HeaderHash, ByAddress, ByHeader, ByRevision,
    ProcessSpecificationAddress, TimedProcessSpecificationAddress,
};

// toplevel I/O structs for WASM API

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateParams {
    pub process_specification: CreateRequest,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateParams {
    pub process_specification: UpdateRequest,
}

//---------------- EXTERNAL RECORD STRUCTURE ----------------

/// I/O struct to describe the complete output record, including all managed link fields
#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub id: ProcessSpecificationAddress,
    pub revision_id: HeaderHash,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl<'a> Response {
    pub fn into_cursor(&'a self) -> Result<String, std::string::FromUtf8Error> {
        let bytes: Vec<u8> = self.id.to_owned().into();
        String::from_utf8(bytes)
    }
}

/// I/O struct to describe what is returned outside the gateway
#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData {
    pub process_specification: Response,
}

//---------------- CREATE REQUEST ----------------

/// I/O struct to describe the complete input record, including all managed links
#[derive(Serialize, Deserialize, Debug, SerializedBytes, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    pub name: String,
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
    pub revision_id: HeaderHash,
    #[serde(default)]
    pub name: MaybeUndefined<String>,
    #[serde(default)]
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
}
