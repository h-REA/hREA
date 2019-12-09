use std::borrow::Cow;
use hdk::{
    holochain_json_api::{ json::JsonString, error::JsonError },
};
use holochain_json_derive::{ DefaultJson };

use hdk_graph_helpers::{
    MaybeUndefined,
    maybe_undefined::{ default_false },
    record_interface::Updateable,
    links::{
        get_linked_addresses_as_type,
        get_linked_remote_addresses_as_type,
    },
};

use super::identifiers::{
    PROCESS_EVENT_INPUTS_LINK_TYPE, PROCESS_EVENT_INPUTS_LINK_TAG,
    PROCESS_EVENT_OUTPUTS_LINK_TYPE, PROCESS_EVENT_OUTPUTS_LINK_TAG,
    PROCESS_COMMITMENT_INPUTS_LINK_TYPE, PROCESS_COMMITMENT_INPUTS_LINK_TAG,
    PROCESS_COMMITMENT_OUTPUTS_LINK_TYPE, PROCESS_COMMITMENT_OUTPUTS_LINK_TAG,
    PROCESS_INTENT_INPUTS_LINK_TYPE, PROCESS_INTENT_INPUTS_LINK_TAG,
    PROCESS_INTENT_OUTPUTS_LINK_TYPE, PROCESS_INTENT_OUTPUTS_LINK_TAG,
};
use vf_core::type_aliases::{
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

//---------------- RECORD INTERNALS & VALIDATION ----------------

#[derive(Serialize, Deserialize, Debug, DefaultJson, Default, Clone)]
pub struct Entry {
    name: String,
    has_beginning: Option<Timestamp>,
    has_end: Option<Timestamp>,
    before: Option<Timestamp>,
    after: Option<Timestamp>,
    classified_as: Option<Vec<ExternalURL>>,
    based_on: Option<ProcessSpecificationAddress>,
    planned_within: Option<PlanAddress>,
    finished: bool,
    in_scope_of: Option<Vec<String>>,
    note: Option<String>,
}

//---------------- CREATE ----------------

/// I/O struct to describe the complete input record, including all managed links
#[derive(Serialize, Deserialize, Debug, DefaultJson, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    name: String,
    #[serde(default)]
    has_beginning: MaybeUndefined<Timestamp>,
    #[serde(default)]
    has_end: MaybeUndefined<Timestamp>,
    #[serde(default)]
    before: MaybeUndefined<Timestamp>,
    #[serde(default)]
    after: MaybeUndefined<Timestamp>,
    #[serde(default)]
    classified_as: MaybeUndefined<Vec<ExternalURL>>,
    #[serde(default)]
    based_on: MaybeUndefined<ProcessSpecificationAddress>,
    #[serde(default)]
    planned_within: MaybeUndefined<PlanAddress>,
    #[serde(default = "default_false")]
    finished: MaybeUndefined<bool>,
    #[serde(default)]
    in_scope_of: MaybeUndefined<Vec<String>>,
    #[serde(default)]
    note: MaybeUndefined<String>,
}

impl<'a> CreateRequest {
    // :TODO: accessors for field data
}

/// Pick relevant fields out of I/O record into underlying DHT entry
impl From<CreateRequest> for Entry {
    fn from(e: CreateRequest) -> Entry {
        Entry {
            name: e.name.into(),
            has_beginning: e.has_beginning.into(),
            has_end: e.has_end.into(),
            before: e.before.into(),
            after: e.after.into(),
            classified_as: e.classified_as.into(),
            based_on: e.based_on.into(),
            planned_within: e.planned_within.into(),
            finished: e.finished.to_option().unwrap(),  // :NOTE: unsafe, would crash if not for "default_*" bindings via Serde
            in_scope_of: e.in_scope_of.into(),
            note: e.note.into(),
        }
    }
}

//---------------- UPDATE ----------------

/// I/O struct to describe the complete input record, including all managed links
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRequest {
    id: ProcessAddress,
    #[serde(default)]
    name: MaybeUndefined<String>,
    #[serde(default)]
    has_beginning: MaybeUndefined<Timestamp>,
    #[serde(default)]
    has_end: MaybeUndefined<Timestamp>,
    #[serde(default)]
    before: MaybeUndefined<Timestamp>,
    #[serde(default)]
    after: MaybeUndefined<Timestamp>,
    #[serde(default)]
    classified_as: MaybeUndefined<Vec<ExternalURL>>,
    #[serde(default)]
    based_on: MaybeUndefined<ProcessSpecificationAddress>,
    #[serde(default)]
    planned_within: MaybeUndefined<PlanAddress>,
    #[serde(default)]
    finished: MaybeUndefined<bool>,
    #[serde(default)]
    in_scope_of: MaybeUndefined<Vec<String>>,
    #[serde(default)]
    note: MaybeUndefined<String>,
}

impl<'a> UpdateRequest {
    pub fn get_id(&'a self) -> &ProcessAddress {
        &self.id
    }

    // :TODO: accessors for other field data
}

/// Handles update operations by merging any newly provided fields
impl Updateable<UpdateRequest> for Entry {
    fn update_with(&self, e: &UpdateRequest) -> Entry {
        Entry {
            name: if !e.name.is_some() { self.name.to_owned() } else { e.name.to_owned().unwrap() },
            has_beginning: if e.has_beginning == MaybeUndefined::Undefined { self.has_beginning.to_owned() } else { e.has_beginning.to_owned().into() },
            has_end: if e.has_end == MaybeUndefined::Undefined { self.has_end.to_owned() } else { e.has_end.to_owned().into() },
            before: if e.before == MaybeUndefined::Undefined { self.before.to_owned() } else { e.before.to_owned().into() },
            after: if e.after == MaybeUndefined::Undefined { self.after.to_owned() } else { e.after.to_owned().into() },
            classified_as: if e.classified_as == MaybeUndefined::Undefined { self.classified_as.to_owned() } else { e.classified_as.to_owned().into() },
            based_on: if e.based_on == MaybeUndefined::Undefined { self.based_on.to_owned() } else { e.based_on.to_owned().into() },
            planned_within: if e.planned_within == MaybeUndefined::Undefined { self.planned_within.to_owned() } else { e.planned_within.to_owned().into() },
            finished: if e.finished == MaybeUndefined::Undefined { self.finished.to_owned() } else { e.finished.to_owned().to_option().unwrap() },
            in_scope_of: if e.in_scope_of== MaybeUndefined::Undefined { self.in_scope_of.to_owned() } else { e.in_scope_of.to_owned().into() },
            note: if e.note== MaybeUndefined::Undefined { self.note.to_owned() } else { e.note.to_owned().into() },
        }
    }
}

//---------------- EXTERNAL API ----------------

/// I/O struct to describe the complete output record, including all managed link fields
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    id: ProcessAddress,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    has_beginning: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    has_end: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    classified_as: Option<Vec<ExternalURL>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    based_on: Option<ProcessSpecificationAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    planned_within: Option<PlanAddress>,
    finished: bool,
    deletable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    in_scope_of: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<String>,

    // query edges
    #[serde(skip_serializing_if = "Option::is_none")]
    inputs: Option<Vec<EventAddress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    outputs: Option<Vec<EventAddress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unplanned_economic_events: Option<Vec<EventAddress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    committed_inputs: Option<Vec<CommitmentAddress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    committed_outputs: Option<Vec<CommitmentAddress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    intended_inputs: Option<Vec<IntentAddress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    intended_outputs: Option<Vec<IntentAddress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_processes: Option<Vec<ProcessAddress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    previous_processes: Option<Vec<ProcessAddress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    working_agents: Option<Vec<AgentAddress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trace: Option<Vec<EventAddress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    track: Option<Vec<EventAddress>>,
}

/// I/O struct to describe what is returned outside the gateway
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData {
    process: Response,
}

/// Create response from input DHT primitives
pub fn construct_response<'a>(
    address: &ProcessAddress, e: &Entry, (
        inputs, outputs,
        unplanned_economic_events,
        committed_inputs, committed_outputs,
        intended_inputs, intended_outputs,
        next_processes, previous_processes,
        working_agents,
        trace, track
     ): (
        Option<Cow<'a, Vec<EventAddress>>>, Option<Cow<'a, Vec<EventAddress>>>,
        Option<Cow<'a, Vec<EventAddress>>>,
        Option<Cow<'a, Vec<CommitmentAddress>>>, Option<Cow<'a, Vec<CommitmentAddress>>>,
        Option<Cow<'a, Vec<IntentAddress>>>, Option<Cow<'a, Vec<IntentAddress>>>,
        Option<Cow<'a, Vec<ProcessAddress>>>, Option<Cow<'a, Vec<ProcessAddress>>>,
        Option<Cow<'a, Vec<AgentAddress>>>,
        Option<Cow<'a, Vec<EventAddress>>>, Option<Cow<'a, Vec<EventAddress>>>,
    ),
) -> ResponseData {
    ResponseData {
        process: Response {
            // entry fields
            id: address.to_owned(),
            name: e.name.to_owned(),
            has_beginning: e.has_beginning.to_owned(),
            has_end: e.has_end.to_owned(),
            before: e.before.to_owned(),
            after: e.after.to_owned(),
            classified_as: e.classified_as.to_owned(),
            based_on: e.based_on.to_owned(),
            planned_within: e.planned_within.to_owned(),
            note: e.note.to_owned(),
            in_scope_of: e.in_scope_of.to_owned(),
            finished: e.finished.to_owned(),
            deletable: true,    // :TODO:

            // link fields
            inputs: inputs.map(Cow::into_owned),
            outputs: outputs.map(Cow::into_owned),
            unplanned_economic_events: unplanned_economic_events.map(Cow::into_owned),
            committed_inputs: committed_inputs.map(Cow::into_owned),
            committed_outputs: committed_outputs.map(Cow::into_owned),
            intended_inputs: intended_inputs.map(Cow::into_owned),
            intended_outputs: intended_outputs.map(Cow::into_owned),
            next_processes: next_processes.map(Cow::into_owned),
            previous_processes: previous_processes.map(Cow::into_owned),
            working_agents: working_agents.map(Cow::into_owned),
            trace: trace.map(Cow::into_owned),
            track: track.map(Cow::into_owned),
        }
    }
}

//---------------- READ ----------------

// @see construct_response
pub fn get_link_fields<'a>(process: &ProcessAddress) -> (
    Option<Cow<'a, Vec<EventAddress>>>,
    Option<Cow<'a, Vec<EventAddress>>>,
    Option<Cow<'a, Vec<EventAddress>>>,
    Option<Cow<'a, Vec<CommitmentAddress>>>,
    Option<Cow<'a, Vec<CommitmentAddress>>>,
    Option<Cow<'a, Vec<IntentAddress>>>,
    Option<Cow<'a, Vec<IntentAddress>>>,
    Option<Cow<'a, Vec<ProcessAddress>>>,
    Option<Cow<'a, Vec<ProcessAddress>>>,
    Option<Cow<'a, Vec<AgentAddress>>>,
    Option<Cow<'a, Vec<EventAddress>>>,
    Option<Cow<'a, Vec<EventAddress>>>,
) {
    (
        Some(get_input_event_ids(process)),
        Some(get_output_event_ids(process)),
        None,  // :TODO: unplanned_economic_events
        Some(get_input_commitment_ids(process)),
        Some(get_output_commitment_ids(process)),
        Some(get_input_intent_ids(process)),
        Some(get_output_intent_ids(process)),
        None, // :TODO: next_processes
        None, // :TODO: previous_processes
        None, // :TODO: working_agents
        None, // :TODO: trace
        None, // :TODO: track
    )
}

fn get_input_event_ids<'a>(process: &ProcessAddress) -> Cow<'a, Vec<EventAddress>> {
    get_linked_addresses_as_type(process, PROCESS_EVENT_INPUTS_LINK_TYPE, PROCESS_EVENT_INPUTS_LINK_TAG)
}

fn get_output_event_ids<'a>(process: &ProcessAddress) -> Cow<'a, Vec<EventAddress>> {
    get_linked_addresses_as_type(process, PROCESS_EVENT_OUTPUTS_LINK_TYPE, PROCESS_EVENT_OUTPUTS_LINK_TAG)
}

fn get_input_commitment_ids<'a>(process: &ProcessAddress) -> Cow<'a, Vec<CommitmentAddress>> {
    get_linked_remote_addresses_as_type(process, PROCESS_COMMITMENT_INPUTS_LINK_TYPE, PROCESS_COMMITMENT_INPUTS_LINK_TAG)
}

fn get_output_commitment_ids<'a>(process: &ProcessAddress) -> Cow<'a, Vec<CommitmentAddress>> {
    get_linked_remote_addresses_as_type(process, PROCESS_COMMITMENT_OUTPUTS_LINK_TYPE, PROCESS_COMMITMENT_OUTPUTS_LINK_TAG)
}

fn get_input_intent_ids<'a>(process: &ProcessAddress) -> Cow<'a, Vec<IntentAddress>> {
    get_linked_remote_addresses_as_type(process, PROCESS_INTENT_INPUTS_LINK_TYPE, PROCESS_INTENT_INPUTS_LINK_TAG)
}

fn get_output_intent_ids<'a>(process: &ProcessAddress) -> Cow<'a, Vec<IntentAddress>> {
    get_linked_remote_addresses_as_type(process, PROCESS_INTENT_OUTPUTS_LINK_TYPE, PROCESS_INTENT_OUTPUTS_LINK_TAG)
}
