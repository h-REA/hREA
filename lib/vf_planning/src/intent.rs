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
        get_linked_remote_addresses_as_type,
        get_linked_addresses_as_type,
    },
};

use vf_core::{
    measurement::QuantityValue,
};
use vf_core::type_aliases::{
    IntentAddress,
    ActionId,
    Timestamp,
    ExternalURL,
    LocationAddress,
    AgentAddress,
    ResourceAddress,
    ProcessAddress,
    ResourceSpecificationAddress,
    SatisfactionAddress,
};
use super::identifiers::{
    INTENT_INPUT_OF_LINK_TYPE, INTENT_INPUT_OF_LINK_TAG,
    INTENT_OUTPUT_OF_LINK_TYPE, INTENT_OUTPUT_OF_LINK_TAG,
    INTENT_SATISFIEDBY_LINK_TYPE, INTENT_SATISFIEDBY_LINK_TAG,
};

use vf_knowledge::action::validate_flow_action;

//---------------- RECORD INTERNALS & VALIDATION ----------------

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Entry {
    pub action: ActionId,
    pub provider: Option<AgentAddress>,
    pub receiver: Option<AgentAddress>,
    pub input_of: Option<ProcessAddress>,   // :NOTE: shadows link, see https://github.com/holo-rea/holo-rea/issues/60#issuecomment-553756873
    pub output_of: Option<ProcessAddress>,
    pub resource_inventoried_as: Option<ResourceAddress>,
    pub resource_classified_as: Option<Vec<ExternalURL>>,
    pub resource_conforms_to: Option<ResourceSpecificationAddress>,
    pub resource_quantity: Option<QuantityValue>,
    pub effort_quantity: Option<QuantityValue>,
    pub available_quantity: Option<QuantityValue>,
    pub has_beginning: Option<Timestamp>,
    pub has_end: Option<Timestamp>,
    pub has_point_in_time: Option<Timestamp>,
    pub due: Option<Timestamp>,
    pub at_location: Option<LocationAddress>,
    pub agreed_in: Option<ExternalURL>,
    pub finished: bool,
    pub in_scope_of: Option<Vec<String>>,
    pub image: Option<ExternalURL>,
    pub note: Option<String>,
}

impl Entry {
    pub fn validate_action(self: Self) -> Result<(), String> {
        validate_flow_action(self.action, self.input_of, self.output_of)
    }

    pub fn validate_or_fields(&self) -> Result<(), String> {
        if !(self.provider.is_some() || self.receiver.is_some()) {
            return Err("Intent must have either a provider or a receiver".into());
        }
        Ok(())
    }
}

//---------------- CREATE ----------------

/// I/O struct to describe the complete input record, including all managed links
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    action: ActionId,
    #[serde(default)]
    note: MaybeUndefined<String>,
    #[serde(default)]
    image: MaybeUndefined<ExternalURL>,
    #[serde(default)]
    pub input_of: MaybeUndefined<ProcessAddress>,
    #[serde(default)]
    pub output_of: MaybeUndefined<ProcessAddress>,
    #[serde(default)]
    provider: MaybeUndefined<AgentAddress>,
    #[serde(default)]
    receiver: MaybeUndefined<AgentAddress>,
    #[serde(default)]
    resource_inventoried_as: MaybeUndefined<ResourceAddress>,
    #[serde(default)]
    resource_classified_as: MaybeUndefined<Vec<ExternalURL>>,
    #[serde(default)]
    resource_conforms_to: MaybeUndefined<ResourceSpecificationAddress>,
    #[serde(default)]
    resource_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    effort_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    available_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    has_beginning: MaybeUndefined<Timestamp>,
    #[serde(default)]
    has_end: MaybeUndefined<Timestamp>,
    #[serde(default)]
    has_point_in_time: MaybeUndefined<Timestamp>,
    #[serde(default)]
    due: MaybeUndefined<Timestamp>,
    #[serde(default)]
    at_location: MaybeUndefined<LocationAddress>,
    #[serde(default)]
    agreed_in: MaybeUndefined<ExternalURL>,
    #[serde(default = "default_false")]
    finished: MaybeUndefined<bool>,
    #[serde(default)]
    in_scope_of: MaybeUndefined<Vec<String>>,
}

impl<'a> CreateRequest {
    // :TODO: accessors for field data
}

/// Pick relevant fields out of I/O record into underlying DHT entry
impl From<CreateRequest> for Entry {
    fn from(e: CreateRequest) -> Entry {
        Entry {
            action: e.action.to_owned(),
            note: e.note.to_owned().into(),
            image: e.image.to_owned().into(),
            provider: e.provider.to_owned().into(),
            receiver: e.receiver.to_owned().into(),
            input_of: e.input_of.into(),
            output_of: e.output_of.into(),
            resource_inventoried_as: e.resource_inventoried_as.to_owned().into(),
            resource_classified_as: e.resource_classified_as.to_owned().into(),
            resource_conforms_to: e.resource_conforms_to.to_owned().into(),
            resource_quantity: e.resource_quantity.to_owned().into(),
            effort_quantity: e.effort_quantity.to_owned().into(),
            available_quantity: e.available_quantity.to_owned().into(),
            has_beginning: e.has_beginning.to_owned().into(),
            has_end: e.has_end.to_owned().into(),
            has_point_in_time: e.has_point_in_time.to_owned().into(),
            due: e.due.to_owned().into(),
            at_location: e.at_location.to_owned().into(),
            agreed_in: e.agreed_in.to_owned().into(),
            finished: e.finished.to_option().unwrap(),  // :NOTE: unsafe, would crash if not for "default_false" binding via Serde
            in_scope_of: e.in_scope_of.to_owned().into(),
        }
    }
}

//---------------- UPDATE ----------------

/// I/O struct to describe the complete input record, including all managed links
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRequest {
    id: IntentAddress,
    #[serde(default)]
    action: MaybeUndefined<ActionId>,
    #[serde(default)]
    note: MaybeUndefined<String>,
    #[serde(default)]
    image: MaybeUndefined<ExternalURL>,
    #[serde(default)]
    pub input_of: MaybeUndefined<ProcessAddress>,
    #[serde(default)]
    pub output_of: MaybeUndefined<ProcessAddress>,
    #[serde(default)]
    provider: MaybeUndefined<AgentAddress>,
    #[serde(default)]
    receiver: MaybeUndefined<AgentAddress>,
    #[serde(default)]
    resource_inventoried_as: MaybeUndefined<ResourceAddress>,
    #[serde(default)]
    resource_classified_as: MaybeUndefined<Vec<ExternalURL>>,
    #[serde(default)]
    resource_conforms_to: MaybeUndefined<ResourceSpecificationAddress>,
    #[serde(default)]
    resource_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    effort_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    available_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    has_beginning: MaybeUndefined<Timestamp>,
    #[serde(default)]
    has_end: MaybeUndefined<Timestamp>,
    #[serde(default)]
    has_point_in_time: MaybeUndefined<Timestamp>,
    #[serde(default)]
    due: MaybeUndefined<Timestamp>,
    #[serde(default)]
    at_location: MaybeUndefined<LocationAddress>,
    #[serde(default)]
    agreed_in: MaybeUndefined<ExternalURL>,
    #[serde(default)]
    finished: MaybeUndefined<bool>,
    #[serde(default)]
    in_scope_of: MaybeUndefined<Vec<String>>,
}

impl<'a> UpdateRequest {
    pub fn get_id(&'a self) -> &IntentAddress {
        &self.id
    }

    // :TODO: accessors for other field data
}

/// Handles update operations by merging any newly provided fields into
impl Updateable<UpdateRequest> for Entry {
    fn update_with(&self, e: &UpdateRequest) -> Entry {
        Entry {
            action: if !e.action.is_some() { self.action.to_owned() } else { e.action.to_owned().unwrap() },
            provider: if e.provider == MaybeUndefined::Undefined { self.provider.to_owned() } else { e.provider.to_owned().into() },
            receiver: if e.receiver == MaybeUndefined::Undefined { self.receiver.to_owned() } else { e.receiver.to_owned().into() },
            input_of: if e.input_of == MaybeUndefined::Undefined { self.input_of.to_owned() } else { e.input_of.to_owned().into() },
            output_of: if e.output_of == MaybeUndefined::Undefined { self.output_of.to_owned() } else { e.output_of.to_owned().into() },
            resource_inventoried_as: if e.resource_inventoried_as == MaybeUndefined::Undefined { self.resource_inventoried_as.to_owned() } else { e.resource_inventoried_as.to_owned().into() },
            resource_classified_as: if e.resource_classified_as== MaybeUndefined::Undefined { self.resource_classified_as.to_owned() } else { e.resource_classified_as.to_owned().into() },
            resource_conforms_to: if e.resource_conforms_to == MaybeUndefined::Undefined { self.resource_conforms_to.to_owned() } else { e.resource_conforms_to.to_owned().into() },
            resource_quantity: if e.resource_quantity== MaybeUndefined::Undefined { self.resource_quantity.to_owned() } else { e.resource_quantity.to_owned().into() },
            effort_quantity: if e.effort_quantity== MaybeUndefined::Undefined { self.effort_quantity.to_owned() } else { e.effort_quantity.to_owned().into() },
            available_quantity: if e.available_quantity== MaybeUndefined::Undefined { self.available_quantity.to_owned() } else { e.available_quantity.to_owned().into() },
            has_beginning: if e.has_beginning == MaybeUndefined::Undefined { self.has_beginning.to_owned() } else { e.has_beginning.to_owned().into() },
            has_end: if e.has_end == MaybeUndefined::Undefined { self.has_end.to_owned() } else { e.has_end.to_owned().into() },
            has_point_in_time: if e.has_point_in_time == MaybeUndefined::Undefined { self.has_point_in_time.to_owned() } else { e.has_point_in_time.to_owned().into() },
            due: if e.due == MaybeUndefined::Undefined { self.due.to_owned() } else { e.due.to_owned().into() },
            at_location: if e.at_location == MaybeUndefined::Undefined { self.at_location.to_owned() } else { e.at_location.to_owned().into() },
            agreed_in: if e.agreed_in == MaybeUndefined::Undefined { self.agreed_in.to_owned() } else { e.agreed_in.to_owned().into() },
            finished: if e.finished == MaybeUndefined::Undefined { self.finished.to_owned() } else { e.finished.to_owned().to_option().unwrap() },
            in_scope_of: if e.in_scope_of== MaybeUndefined::Undefined { self.in_scope_of.to_owned() } else { e.in_scope_of.to_owned().into() },
            image: if e.image== MaybeUndefined::Undefined { self.image.to_owned() } else { e.image.to_owned().into() },
            note: if e.note== MaybeUndefined::Undefined { self.note.to_owned() } else { e.note.to_owned().into() },
        }
    }
}

//---------------- EXTERNAL API ----------------

/// I/O struct to describe the complete output record, including all managed link fields
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    id: IntentAddress,
    action: ActionId,
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<String>,
    #[serde(default)]
    image: Option<ExternalURL>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_of: Option<ProcessAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_of: Option<ProcessAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<AgentAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    receiver: Option<AgentAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_inventoried_as: Option<ResourceAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_classified_as: Option<Vec<ExternalURL>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_conforms_to: Option<ResourceSpecificationAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_quantity: Option<QuantityValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effort_quantity: Option<QuantityValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    available_quantity: Option<QuantityValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    has_beginning: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    has_end: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    has_point_in_time: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    due: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    at_location: Option<LocationAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    agreed_in: Option<ExternalURL>,
    #[serde(skip_serializing_if = "Option::is_none")]
    in_scope_of: Option<Vec<String>>,
    finished: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    satisfied_by: Option<Vec<SatisfactionAddress>>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // published_in: Option<Vec<ProposedIntentAddress>>,
}

/// I/O struct to describe what is returned outside the gateway
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData {
    intent: Response,
}

/// Create response from input DHT primitives
pub fn construct_response<'a>(
    address: &IntentAddress, e: &Entry, (
        input_process, output_process,
        satisfactions,
        // published_in,
    ): (
        Option<ProcessAddress>, Option<ProcessAddress>,
        Option<Cow<'a, Vec<SatisfactionAddress>>>,
        // Option<Cow<'a, Vec<ProposedIntentAddress>>>
    )
) -> ResponseData {
    ResponseData {
        intent: Response {
            id: address.to_owned(),
            action: e.action.to_owned(),
            note: e.note.to_owned(),
            image: e.image.to_owned(),
            input_of: input_process.to_owned(),
            output_of: output_process.to_owned(),
            provider: e.provider.to_owned(),
            receiver: e.receiver.to_owned(),
            resource_inventoried_as: e.resource_inventoried_as.to_owned(),
            resource_classified_as: e.resource_classified_as.to_owned(),
            resource_conforms_to: e.resource_conforms_to.to_owned(),
            resource_quantity: e.resource_quantity.to_owned(),
            effort_quantity: e.effort_quantity.to_owned(),
            available_quantity: e.available_quantity.to_owned(),
            has_beginning: e.has_beginning.to_owned(),
            has_end: e.has_end.to_owned(),
            has_point_in_time: e.has_point_in_time.to_owned(),
            due: e.due.to_owned(),
            at_location: e.at_location.to_owned(),
            agreed_in: e.agreed_in.to_owned(),
            finished: e.finished.to_owned(),
            in_scope_of: e.in_scope_of.to_owned(),
            satisfied_by: satisfactions.map(Cow::into_owned),
            // published_in: published_in.map(Cow::into_owned),
        }
    }
}

//---------------- READ ----------------

// @see construct_response
pub fn get_link_fields<'a>(intent: &IntentAddress) -> (
    Option<ProcessAddress>,
    Option<ProcessAddress>,
    Option<Cow<'a, Vec<SatisfactionAddress>>>,
) {
    (
        get_linked_remote_addresses_as_type(intent, INTENT_INPUT_OF_LINK_TYPE, INTENT_INPUT_OF_LINK_TAG).into_owned().pop(),
        get_linked_remote_addresses_as_type(intent, INTENT_OUTPUT_OF_LINK_TYPE, INTENT_OUTPUT_OF_LINK_TAG).into_owned().pop(),
        Some(get_linked_addresses_as_type(intent, INTENT_SATISFIEDBY_LINK_TYPE, INTENT_SATISFIEDBY_LINK_TAG)),
    )
}
