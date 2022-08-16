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
    ActionHash, ByAddress, ByAction, RecordMeta, RevisionMeta,
    ResourceSpecificationAddress,
    EconomicResourceAddress,
    ExternalURL,
    UnitId,
    ByRevision,
};

// toplevel I/O structs for WASM API

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateParams {
    pub resource_specification: CreateRequest,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
    pub revision_id: ActionHash,
    pub meta: RecordMeta,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<ExternalURL>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_unit_of_effort: Option<UnitId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_unit_of_resource: Option<UnitId>,
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
    #[serde(default)]
    pub default_unit_of_resource: MaybeUndefined<UnitId>,
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
    pub revision_id: ActionHash,
    #[serde(default)]
    pub name: MaybeUndefined<String>,
    #[serde(default)]
    pub image: MaybeUndefined<ExternalURL>,
    #[serde(default)]
    pub note: MaybeUndefined<String>,
    #[serde(default)]
    pub default_unit_of_effort: MaybeUndefined<UnitId>,
    #[serde(default)]
    pub default_unit_of_resource: MaybeUndefined<UnitId>,
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
    pub conforming_resources: Option<EconomicResourceAddress>,
}
