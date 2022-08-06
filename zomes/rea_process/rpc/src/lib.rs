/**
 *  Holo-REA 'economic resource' zome
 */
use holochain_serialized_bytes::prelude::*;

use serde_maybe_undefined::{
    MaybeUndefined,
    default_false,
};
pub use vf_attributes_hdk::{
    ActionHash, ByAction, ByRevision, RecordMeta, RevisionMeta,
    ProcessAddress,
    DateTime, FixedOffset,
    ExternalURL,
    ProcessSpecificationAddress,
    PlanAddress,
    EconomicEventAddress,
    CommitmentAddress,
    IntentAddress,
    AgentAddress,
};

//---------------- EXTERNAL RECORD STRUCTURE ----------------

/// I/O struct to describe the complete output record, including all managed link fields
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub id: ProcessAddress,
    pub revision_id: ActionHash,
    pub meta: RecordMeta,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_beginning: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_end: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classified_as: Option<Vec<ExternalURL>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub based_on: Option<ProcessSpecificationAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planned_within: Option<PlanAddress>,
    pub finished: bool,
    pub deletable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_scope_of: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,

    // query edges
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub observed_inputs: Vec<EconomicEventAddress>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub observed_outputs: Vec<EconomicEventAddress>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub unplanned_economic_events: Vec<EconomicEventAddress>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub committed_inputs: Vec<CommitmentAddress>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub committed_outputs: Vec<CommitmentAddress>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub intended_inputs: Vec<IntentAddress>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub intended_outputs: Vec<IntentAddress>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub next_processes: Vec<ProcessAddress>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub previous_processes: Vec<ProcessAddress>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub working_agents: Vec<AgentAddress>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub trace: Vec<EconomicEventAddress>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub track: Vec<EconomicEventAddress>,
}

/// I/O struct to describe what is returned outside the gateway
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData {
    pub process: Response,
}

//---------------- CREATE REQUEST ----------------

/// I/O struct to describe the complete input record, including all managed links
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    pub name: String,
    #[serde(default)]
    pub has_beginning: MaybeUndefined<DateTime<FixedOffset>>,
    #[serde(default)]
    pub has_end: MaybeUndefined<DateTime<FixedOffset>>,
    #[serde(default)]
    pub before: MaybeUndefined<DateTime<FixedOffset>>,
    #[serde(default)]
    pub after: MaybeUndefined<DateTime<FixedOffset>>,
    #[serde(default)]
    pub classified_as: MaybeUndefined<Vec<ExternalURL>>,
    #[serde(default)]
    pub based_on: MaybeUndefined<ProcessSpecificationAddress>,
    #[serde(default)]
    pub planned_within: MaybeUndefined<PlanAddress>,
    #[serde(default = "default_false")]
    pub finished: MaybeUndefined<bool>,
    #[serde(default)]
    pub in_scope_of: MaybeUndefined<Vec<String>>,
    #[serde(default)]
    pub note: MaybeUndefined<String>,
}

impl<'a> CreateRequest {
    // :TODO: accessors for field data
}

//---------------- UPDATE REQUEST ----------------

/// I/O struct to describe the complete input record, including all managed links
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRequest {
    pub revision_id: ActionHash,
    #[serde(default)]
    pub name: MaybeUndefined<String>,
    #[serde(default)]
    pub has_beginning: MaybeUndefined<DateTime<FixedOffset>>,
    #[serde(default)]
    pub has_end: MaybeUndefined<DateTime<FixedOffset>>,
    #[serde(default)]
    pub before: MaybeUndefined<DateTime<FixedOffset>>,
    #[serde(default)]
    pub after: MaybeUndefined<DateTime<FixedOffset>>,
    #[serde(default)]
    pub classified_as: MaybeUndefined<Vec<ExternalURL>>,
    #[serde(default)]
    pub based_on: MaybeUndefined<ProcessSpecificationAddress>,
    #[serde(default)]
    pub planned_within: MaybeUndefined<PlanAddress>,
    #[serde(default)]
    pub finished: MaybeUndefined<bool>,
    #[serde(default)]
    pub in_scope_of: MaybeUndefined<Vec<String>>,
    #[serde(default)]
    pub note: MaybeUndefined<String>,
}

impl<'a> UpdateRequest {
    pub fn get_revision_id(&'a self) -> &ActionHash {
        &self.revision_id
    }

    // :TODO: accessors for other field data
}

//---------------- QUERY FILTER REQUEST ----------------

#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    pub observed_inputs: Option<EconomicEventAddress>,
    pub observed_outputs: Option<EconomicEventAddress>,
    pub unplanned_economic_events: Option<EconomicEventAddress>,
    pub committed_inputs: Option<CommitmentAddress>,
    pub committed_outputs: Option<CommitmentAddress>,
    pub intended_inputs: Option<IntentAddress>,
    pub intended_outputs: Option<IntentAddress>,
    pub working_agents: Option<AgentAddress>,
    pub planned_within: Option<PlanAddress>,
}
