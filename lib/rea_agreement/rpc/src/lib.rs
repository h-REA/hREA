/**
 * Holo-REA agreement zome I/O data structures
 *
 * Required by packages wishing to interact with the zome via its standard RPC interface,
 * and by the zome API handlers accepting these parameters.
 *
 * @package Holo-REA
 */
use holochain_serialized_bytes::prelude::*;

use hdk_records::MaybeUndefined;
pub use vf_attributes_hdk::{
    RevisionHash,
    AgreementAddress,
    CommitmentAddress,
    EventAddress,
    DateTime,
    Local,
};

//---------------- EXTERNAL RECORD STRUCTURE ----------------

/// I/O struct to describe the complete record, including all managed link fields
///
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub id: AgreementAddress,
    pub revision_id: RevisionHash,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Local>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub commitments: Vec<CommitmentAddress>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub economic_events: Vec<EventAddress>,
}

/// I/O struct to describe what is returned outside the gateway.
/// Responses are usually returned as named attributes in order to leave space
/// for future additional return values.
///
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData {
    pub agreement: Response,
}

//---------------- CREATE REQUEST ----------------

/// I/O struct to describe the complete input record, including all managed links
///
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub name: MaybeUndefined<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub created: MaybeUndefined<DateTime<Local>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub note: MaybeUndefined<String>,
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
    pub revision_id: RevisionHash,
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub name: MaybeUndefined<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub created: MaybeUndefined<DateTime<Local>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub note: MaybeUndefined<String>,
}

impl<'a> UpdateRequest {
    pub fn get_revision_id(&'a self) -> &RevisionHash {
        &self.revision_id
    }

    // :TODO: accessors for other field data
}

//---------------- QUERY FILTER REQUEST ----------------

#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    // :TODO:
}
