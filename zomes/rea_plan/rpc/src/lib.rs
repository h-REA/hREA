/**
 * hREA plan zome I/O data structures
 *
 * Required by packages wishing to interact with the zome via its standard RPC interface,
 * and by the zome API handlers accepting these parameters.
 *
 * @package hREA
 */
use holochain_serialized_bytes::prelude::*;

use serde_maybe_undefined::MaybeUndefined;
pub use vf_attributes_hdk::{
    PlanAddress,
    CommitmentAddress,
    ProcessAddress,
    EconomicEventAddress,
    DateTime,
    FixedOffset,
    ByAction, ActionHash, ByRevision, RecordMeta, RevisionMeta,
};

//---------------- EXTERNAL RECORD STRUCTURE ----------------

/// I/O struct utilized to read back a single record
/// specified by its hash/address.
///
#[derive(Debug, Serialize, Deserialize)]
struct ReadParams {
    pub address: PlanAddress,
}

/// I/O struct to describe the complete record, including all managed link fields
///
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub id: PlanAddress,
    pub revision_id: ActionHash,
    pub meta: RecordMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletable: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub processes: Vec<ProcessAddress>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub independent_demands: Vec<CommitmentAddress>,
}

/// I/O struct to describe what is returned outside the gateway.
/// Responses are usually returned as named attributes in order to leave space
/// for future additional return values.
///
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData {
    pub plan: Response,
}

//---------------- CREATE REQUEST ----------------

/// I/O struct to describe what is passed in from outside the gateway to create a Plan.
/// This is structured as-is with named attributes in order to leave space
/// for future additional input values.
///
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateParams {
    pub plan: CreateRequest,
}

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
    pub created: MaybeUndefined<DateTime<FixedOffset>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub due: MaybeUndefined<DateTime<FixedOffset>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub note: MaybeUndefined<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub deletable: MaybeUndefined<bool>,
    // exclude `refinementOf` because it relates to Scenario, which is out of MMR scope
    // #[serde(default)]
    // #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    // pub refinementOf: MaybeUndefined<ScenarioAddress>,
}

impl<'a> CreateRequest {
    // :TODO: accessors for field data
}

//---------------- UPDATE REQUEST ----------------

/// I/O struct to describe what is passed in from outside the gateway to update a Plan.
/// This is structured as-is with named attributes in order to leave space
/// for future additional input values.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateParams {
    pub plan: UpdateRequest,
}

/// I/O struct to describe the complete input record, including all managed links
///
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRequest {
    pub revision_id: ActionHash,
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub name: MaybeUndefined<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub created: MaybeUndefined<DateTime<FixedOffset>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub due: MaybeUndefined<DateTime<FixedOffset>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub note: MaybeUndefined<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    pub deletable: MaybeUndefined<bool>,
    // exclude `refinementOf` because it relates to Scenario, which is out of MMR scope
    // #[serde(default)]
    // #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    // pub refinementOf: MaybeUndefined<ScenarioAddress>,
}

impl<'a> UpdateRequest {
    pub fn get_revision_id(&self) -> ActionHash {
        self.revision_id.to_owned().into()
    }

    // :TODO: accessors for other field data
}

//---------------- QUERY FILTER REQUEST ----------------

#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    pub processes: Option<ProcessAddress>,
    pub non_process_commitments: Option<CommitmentAddress>,
    pub independent_demands: Option<CommitmentAddress>,
}
