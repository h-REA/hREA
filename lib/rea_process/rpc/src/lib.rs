/**
 *  Holo-REA 'economic resource' zome
 */
use holochain_serialized_bytes::prelude::*;

use hdk_records::{
    MaybeUndefined,
    maybe_undefined::default_false,
};
pub use vf_core::type_aliases::{
    RevisionHash,
    ProcessAddress,
    Timestamp,
    ExternalURL,
    ProcessSpecificationAddress,
    PlanAddress,
    EventAddress,
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
    pub revision_id: RevisionHash,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_beginning: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_end: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<Timestamp>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<EventAddress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<EventAddress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unplanned_economic_events: Option<Vec<EventAddress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub committed_inputs: Option<Vec<CommitmentAddress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub committed_outputs: Option<Vec<CommitmentAddress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intended_inputs: Option<Vec<IntentAddress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intended_outputs: Option<Vec<IntentAddress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_processes: Option<Vec<ProcessAddress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_processes: Option<Vec<ProcessAddress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_agents: Option<Vec<AgentAddress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace: Option<Vec<EventAddress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track: Option<Vec<EventAddress>>,
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
    pub has_beginning: MaybeUndefined<Timestamp>,
    #[serde(default)]
    pub has_end: MaybeUndefined<Timestamp>,
    #[serde(default)]
    pub before: MaybeUndefined<Timestamp>,
    #[serde(default)]
    pub after: MaybeUndefined<Timestamp>,
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
    pub revision_id: RevisionHash,
    #[serde(default)]
    pub name: MaybeUndefined<String>,
    #[serde(default)]
    pub has_beginning: MaybeUndefined<Timestamp>,
    #[serde(default)]
    pub has_end: MaybeUndefined<Timestamp>,
    #[serde(default)]
    pub before: MaybeUndefined<Timestamp>,
    #[serde(default)]
    pub after: MaybeUndefined<Timestamp>,
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
    pub fn get_revision_id(&'a self) -> &RevisionHash {
        &self.revision_id
    }

    // :TODO: accessors for other field data
}

//---------------- QUERY FILTER REQUEST ----------------

#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    pub inputs: Option<EventAddress>,
    pub outputs: Option<EventAddress>,
    pub unplanned_economic_events: Option<EventAddress>,
    pub committed_inputs: Option<CommitmentAddress>,
    pub committed_outputs: Option<CommitmentAddress>,
    pub intended_inputs: Option<IntentAddress>,
    pub intended_outputs: Option<IntentAddress>,
    pub working_agents: Option<AgentAddress>,
}
