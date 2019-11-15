use std::borrow::Cow;
use hdk::{
    holochain_json_api::{ json::JsonString, error::JsonError },
};
use holochain_json_derive::{ DefaultJson };

use hdk_graph_helpers::{
    MaybeUndefined,
    record_interface::Updateable,
};

// use vf_knowledge::action::Action;

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
    PlanAddress,
    AgreementAddress,
    SatisfactionAddress,
};

// vfRecord! {
    #[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
    pub struct Entry {
        pub action: ActionId,
        pub provider: Option<AgentAddress>,
        pub receiver: Option<AgentAddress>,
        pub resource_inventoried_as: Option<ResourceAddress>,
        pub resource_classified_as: Option<Vec<ExternalURL>>,
        pub resource_conforms_to: Option<ResourceSpecificationAddress>,
        pub flow_quantity: Option<QuantityValue>,
        pub unit_quantity: Option<QuantityValue>,
        pub available_quantity: Option<QuantityValue>,
        pub has_beginning: Option<Timestamp>,
        pub has_end: Option<Timestamp>,
        pub has_point_in_time: Option<Timestamp>,
        pub before: Option<Timestamp>,
        pub after: Option<Timestamp>,
        pub at_location: Option<LocationAddress>,
        pub plan: Option<PlanAddress>,
        pub under: Option<AgreementAddress>,
        pub finished: bool,
        pub in_scope_of: Option<Vec<String>>,
        pub note: Option<String>,
    }
// }

/// Handles update operations by merging any newly provided fields into
impl Updateable<UpdateRequest> for Entry {
    fn update_with(&self, e: &UpdateRequest) -> Entry {
        Entry {
            action: if e.action == MaybeUndefined::Undefined && e.action.is_some() { self.action.to_owned() } else { e.action.to_owned().unwrap() },
            provider: if e.provider == MaybeUndefined::Undefined { self.provider.clone() } else { e.provider.clone().into() },
            receiver: if e.receiver == MaybeUndefined::Undefined { self.receiver.clone() } else { e.receiver.clone().into() },
            resource_inventoried_as: if e.resource_inventoried_as == MaybeUndefined::Undefined { self.resource_inventoried_as.clone() } else { e.resource_inventoried_as.clone().into() },
            resource_classified_as: if e.resource_classified_as== MaybeUndefined::Undefined { self.resource_classified_as.clone() } else { e.resource_classified_as.clone().into() },
            resource_conforms_to: if e.resource_conforms_to == MaybeUndefined::Undefined { self.resource_conforms_to.clone() } else { e.resource_conforms_to.clone().into() },
            flow_quantity: if e.flow_quantity== MaybeUndefined::Undefined { self.flow_quantity.clone() } else { e.flow_quantity.clone().into() },
            unit_quantity: if e.unit_quantity== MaybeUndefined::Undefined { self.unit_quantity.clone() } else { e.unit_quantity.clone().into() },
            available_quantity: if e.available_quantity== MaybeUndefined::Undefined { self.available_quantity.clone() } else { e.available_quantity.clone().into() },
            has_beginning: if e.has_beginning == MaybeUndefined::Undefined { self.has_beginning.clone() } else { e.has_beginning.clone().into() },
            has_end: if e.has_end == MaybeUndefined::Undefined { self.has_end.clone() } else { e.has_end.clone().into() },
            has_point_in_time: if e.has_point_in_time == MaybeUndefined::Undefined { self.has_point_in_time.clone() } else { e.has_point_in_time.clone().into() },
            before: if e.before == MaybeUndefined::Undefined { self.before.clone() } else { e.before.clone().into() },
            after: if e.after == MaybeUndefined::Undefined { self.after.clone() } else { e.after.clone().into() },
            at_location: if e.at_location == MaybeUndefined::Undefined { self.at_location.clone() } else { e.at_location.clone().into() },
            plan: if e.plan == MaybeUndefined::Undefined { self.plan.clone() } else { e.plan.clone().into() },
            under: if e.under == MaybeUndefined::Undefined { self.under.clone() } else { e.under.clone().into() },
            finished: if e.finished == MaybeUndefined::Undefined { self.finished.clone() } else { e.finished.clone().to_option().unwrap() },
            in_scope_of: if e.in_scope_of== MaybeUndefined::Undefined { self.in_scope_of.clone() } else { e.in_scope_of.clone().into() },
            note: if e.note== MaybeUndefined::Undefined { self.note.clone() } else { e.note.clone().into() },
        }
    }
}

/// I/O struct to describe the complete input record, including all managed links
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    action: ActionId,
    #[serde(default)]
    note: MaybeUndefined<String>,
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
    flow_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    unit_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    available_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    has_beginning: MaybeUndefined<Timestamp>,
    #[serde(default)]
    has_end: MaybeUndefined<Timestamp>,
    #[serde(default)]
    has_point_in_time: MaybeUndefined<Timestamp>,
    #[serde(default)]
    before: MaybeUndefined<Timestamp>,
    #[serde(default)]
    after: MaybeUndefined<Timestamp>,
    #[serde(default)]
    at_location: MaybeUndefined<LocationAddress>,
    #[serde(default)]
    plan: MaybeUndefined<PlanAddress>,
    #[serde(default)]
    under: MaybeUndefined<AgreementAddress>,
    #[serde(default = "default_false")]
    finished: MaybeUndefined<bool>,
    #[serde(default)]
    in_scope_of: MaybeUndefined<Vec<String>>,
}

// :TODO: refactor this out into shared code
fn default_false() -> MaybeUndefined<bool> {
    MaybeUndefined::Some(false)
}

impl<'a> CreateRequest {
    // :TODO: accessors for field data
}

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
    flow_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    unit_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    available_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    has_beginning: MaybeUndefined<Timestamp>,
    #[serde(default)]
    has_end: MaybeUndefined<Timestamp>,
    #[serde(default)]
    has_point_in_time: MaybeUndefined<Timestamp>,
    #[serde(default)]
    before: MaybeUndefined<Timestamp>,
    #[serde(default)]
    after: MaybeUndefined<Timestamp>,
    #[serde(default)]
    at_location: MaybeUndefined<LocationAddress>,
    #[serde(default)]
    plan: MaybeUndefined<PlanAddress>,
    #[serde(default)]
    under: MaybeUndefined<AgreementAddress>,
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

/// I/O struct to describe the complete output record, including all managed link fields
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    id: IntentAddress,
    action: ActionId,
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<String>,
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
    flow_quantity: Option<QuantityValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit_quantity: Option<QuantityValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    available_quantity: Option<QuantityValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    has_beginning: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    has_end: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    has_point_in_time: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    at_location: Option<LocationAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    plan: Option<PlanAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    under: Option<AgreementAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    in_scope_of: Option<Vec<String>>,
    finished: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    satisfied_by: Option<Vec<SatisfactionAddress>>,
}

/// I/O struct to describe what is returned outside the gateway
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData {
    intent: Response,
}

/// Pick relevant fields out of I/O record into underlying DHT entry
impl From<CreateRequest> for Entry {
    fn from(e: CreateRequest) -> Entry {
        Entry {
            action: e.action.to_owned(),
            note: e.note.to_owned().into(),
            provider: e.provider.to_owned().into(),
            receiver: e.receiver.to_owned().into(),
            resource_inventoried_as: e.resource_inventoried_as.to_owned().into(),
            resource_classified_as: e.resource_classified_as.to_owned().into(),
            resource_conforms_to: e.resource_conforms_to.to_owned().into(),
            flow_quantity: e.flow_quantity.to_owned().into(),
            unit_quantity: e.unit_quantity.to_owned().into(),
            available_quantity: e.available_quantity.to_owned().into(),
            has_beginning: e.has_beginning.to_owned().into(),
            has_end: e.has_end.to_owned().into(),
            has_point_in_time: e.has_point_in_time.to_owned().into(),
            before: e.before.to_owned().into(),
            after: e.after.to_owned().into(),
            at_location: e.at_location.to_owned().into(),
            plan: e.plan.to_owned().into(),
            under: e.under.to_owned().into(),
            finished: e.finished.to_option().unwrap(),  // :NOTE: unsafe, would crash if not for "default_false" binding via Serde
            in_scope_of: e.in_scope_of.to_owned().into(),
        }
    }
}

/// Create response from input DHT primitives
pub fn construct_response<'a>(
    address: &IntentAddress, e: &Entry, (
        input_process, output_process,
        satisfactions,
    ): (
        Option<ProcessAddress>, Option<ProcessAddress>,
        Option<Cow<'a, Vec<SatisfactionAddress>>>
    )
) -> ResponseData {
    ResponseData {
        intent: Response {
            id: address.to_owned(),
            action: e.action.to_owned(),
            note: e.note.to_owned(),
            input_of: input_process.to_owned(),
            output_of: output_process.to_owned(),
            provider: e.provider.to_owned(),
            receiver: e.receiver.to_owned(),
            resource_inventoried_as: e.resource_inventoried_as.to_owned(),
            resource_classified_as: e.resource_classified_as.to_owned(),
            resource_conforms_to: e.resource_conforms_to.to_owned(),
            flow_quantity: e.flow_quantity.to_owned(),
            unit_quantity: e.unit_quantity.to_owned(),
            available_quantity: e.available_quantity.to_owned(),
            has_beginning: e.has_beginning.to_owned(),
            has_end: e.has_end.to_owned(),
            has_point_in_time: e.has_point_in_time.to_owned(),
            before: e.before.to_owned(),
            after: e.after.to_owned(),
            at_location: e.at_location.to_owned(),
            plan: e.plan.to_owned(),
            under: e.under.to_owned(),
            finished: e.finished.to_owned(),
            in_scope_of: e.in_scope_of.to_owned(),
            satisfied_by: satisfactions.map(Cow::into_owned),
        }
    }
}
