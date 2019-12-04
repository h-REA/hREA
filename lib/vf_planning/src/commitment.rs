use std::borrow::Cow;
use hdk::{
    holochain_json_api::{ json::JsonString, error::JsonError },
};
use holochain_json_derive::{ DefaultJson };

use hdk_graph_helpers::{
    MaybeUndefined,
    record_interface::Updateable,
    maybe_undefined::{ default_false },
    links::{
        get_linked_addresses_as_type,
        get_linked_remote_addresses_as_type,
    },
};

use vf_core::{
    measurement::QuantityValue,
};
use vf_core::type_aliases::{
    CommitmentAddress,
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
    FulfillmentAddress,
    SatisfactionAddress,
};
use super::identifiers::{
    COMMITMENT_INPUT_OF_LINK_TYPE, COMMITMENT_INPUT_OF_LINK_TAG,
    COMMITMENT_OUTPUT_OF_LINK_TYPE, COMMITMENT_OUTPUT_OF_LINK_TAG,
    COMMITMENT_FULFILLEDBY_LINK_TYPE, COMMITMENT_FULFILLEDBY_LINK_TAG,
    COMMITMENT_SATISFIES_LINK_TYPE, COMMITMENT_SATISFIES_LINK_TAG,
};

use vf_knowledge::action::{ validate_flow_action, validate_move_inventories };

//---------------- RECORD INTERNALS & VALIDATION ----------------

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Entry {
    pub action: ActionId,
    pub provider: AgentAddress,
    pub receiver: AgentAddress,
    pub input_of: Option<ProcessAddress>,   // :NOTE: shadows link, see https://github.com/holo-rea/holo-rea/issues/60#issuecomment-553756873
    pub output_of: Option<ProcessAddress>,
    pub resource_inventoried_as: Option<ResourceAddress>,
    pub resource_classified_as: Option<Vec<ExternalURL>>,
    pub resource_conforms_to: Option<ResourceSpecificationAddress>,
    pub resource_quantity: Option<QuantityValue>,
    pub effort_quantity: Option<QuantityValue>,
    pub has_beginning: Option<Timestamp>,
    pub has_end: Option<Timestamp>,
    pub has_point_in_time: Option<Timestamp>,
    pub due: Option<Timestamp>,
    pub at_location: Option<LocationAddress>,
    pub agreed_in: Option<ExternalURL>,
    pub clause_of: Option<AgreementAddress>,
    pub independent_demand_of: Option<PlanAddress>,
    pub plan: Option<PlanAddress>,
    pub finished: bool,
    pub in_scope_of: Option<Vec<String>>,
    pub note: Option<String>,
}

impl Entry {
    pub fn validate_action(&self) -> Result<(), String> {
        let result = validate_flow_action(self.action.to_owned(), self.input_of.to_owned(), self.output_of.to_owned());
        if result.is_ok() && self.action.as_ref() == "move" {
            return validate_move_inventories(self.resource_inventoried_as.to_owned(), self.to_resource_inventoried_as.to_owned());
        }
        return result;
    }

    pub fn validate_or_fields(&self) -> Result<(), String> {
        if !(self.resource_inventoried_as.is_some() || self.resource_classified_as.is_some() || self.resource_conforms_to.is_some()) {
            return Err("Commitment must reference an inventoried resource, resource specification or resource classification".into());
        }
        if !(self.resource_quantity.is_some() || self.effort_quantity.is_some()) {
            return Err("Commmitment must include either a resource quantity or an effort quantity".into());
        }
        if !(self.has_beginning.is_some() || self.has_end.is_some() || self.has_point_in_time.is_some() || self.due.is_some()) {
            return Err("Commmitment must have a beginning, end, exact time or due date".into());
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
    pub input_of: MaybeUndefined<ProcessAddress>,
    #[serde(default)]
    pub output_of: MaybeUndefined<ProcessAddress>,
    provider: AgentAddress,
    receiver: AgentAddress,
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
    plan: MaybeUndefined<PlanAddress>,
    #[serde(default)]
    agreed_in: MaybeUndefined<ExternalURL>,
    #[serde(default)]
    clause_of: MaybeUndefined<AgreementAddress>,
    #[serde(default)]
    independent_demand_of: MaybeUndefined<PlanAddress>,
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
            note: e.note.into(),
            provider: e.provider.into(),
            receiver: e.receiver.into(),
            input_of: e.input_of.into(),
            output_of: e.output_of.into(),
            resource_inventoried_as: e.resource_inventoried_as.into(),
            resource_classified_as: e.resource_classified_as.into(),
            resource_conforms_to: e.resource_conforms_to.into(),
            resource_quantity: e.resource_quantity.into(),
            effort_quantity: e.effort_quantity.into(),
            has_beginning: e.has_beginning.into(),
            has_end: e.has_end.into(),
            has_point_in_time: e.has_point_in_time.into(),
            due: e.due.into(),
            at_location: e.at_location.into(),
            plan: e.plan.into(),
            agreed_in: e.agreed_in.into(),
            clause_of: e.clause_of.into(),
            independent_demand_of: e.independent_demand_of.into(),
            finished: e.finished.to_option().unwrap(),  // :NOTE: unsafe, would crash if not for "default_false" binding via Serde
            in_scope_of: e.in_scope_of.into(),
        }
    }
}

//---------------- UPDATE ----------------

/// I/O struct to describe the complete input record, including all managed links
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRequest {
    id: CommitmentAddress,
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
    resource_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    effort_quantity: MaybeUndefined<QuantityValue>,
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
    plan: MaybeUndefined<PlanAddress>,
    #[serde(default)]
    agreed_in: MaybeUndefined<ExternalURL>,
    #[serde(default)]
    clause_of: MaybeUndefined<AgreementAddress>,
    #[serde(default)]
    independent_demand_of: MaybeUndefined<PlanAddress>,
    #[serde(default)]
    finished: MaybeUndefined<bool>,
    #[serde(default)]
    in_scope_of: MaybeUndefined<Vec<String>>,
}

impl<'a> UpdateRequest {
    pub fn get_id(&'a self) -> &CommitmentAddress {
        &self.id
    }

    // :TODO: accessors for other field data
}

/// Handles update operations by merging any newly provided fields into
impl Updateable<UpdateRequest> for Entry {
    fn update_with(&self, e: &UpdateRequest) -> Entry {
        Entry {
            action: if !e.action.is_some() { self.action.to_owned() } else { e.action.to_owned().unwrap() },
            provider: if !e.provider.is_some() { self.provider.to_owned() } else { e.provider.to_owned().unwrap() },
            receiver: if !e.receiver.is_some() { self.receiver.to_owned() } else { e.receiver.to_owned().unwrap() },
            input_of: if e.input_of == MaybeUndefined::Undefined { self.input_of.to_owned() } else { e.input_of.to_owned().into() },
            output_of: if e.output_of == MaybeUndefined::Undefined { self.output_of.to_owned() } else { e.output_of.to_owned().into() },
            resource_inventoried_as: if e.resource_inventoried_as == MaybeUndefined::Undefined { self.resource_inventoried_as.clone() } else { e.resource_inventoried_as.clone().into() },
            resource_classified_as: if e.resource_classified_as== MaybeUndefined::Undefined { self.resource_classified_as.clone() } else { e.resource_classified_as.clone().into() },
            resource_conforms_to: if e.resource_conforms_to == MaybeUndefined::Undefined { self.resource_conforms_to.clone() } else { e.resource_conforms_to.clone().into() },
            resource_quantity: if e.resource_quantity== MaybeUndefined::Undefined { self.resource_quantity.to_owned() } else { e.resource_quantity.to_owned().into() },
            effort_quantity: if e.effort_quantity== MaybeUndefined::Undefined { self.effort_quantity.to_owned() } else { e.effort_quantity.to_owned().into() },
            has_beginning: if e.has_beginning == MaybeUndefined::Undefined { self.has_beginning.clone() } else { e.has_beginning.clone().into() },
            has_end: if e.has_end == MaybeUndefined::Undefined { self.has_end.clone() } else { e.has_end.clone().into() },
            has_point_in_time: if e.has_point_in_time == MaybeUndefined::Undefined { self.has_point_in_time.clone() } else { e.has_point_in_time.clone().into() },
            due: if e.due == MaybeUndefined::Undefined { self.due.clone() } else { e.due.clone().into() },
            at_location: if e.at_location == MaybeUndefined::Undefined { self.at_location.clone() } else { e.at_location.clone().into() },
            plan: if e.plan == MaybeUndefined::Undefined { self.plan.clone() } else { e.plan.clone().into() },
            agreed_in: if e.agreed_in == MaybeUndefined::Undefined { self.agreed_in.clone() } else { e.agreed_in.clone().into() },
            clause_of: if e.clause_of == MaybeUndefined::Undefined { self.clause_of.clone() } else { e.clause_of.clone().into() },
            independent_demand_of: if e.independent_demand_of == MaybeUndefined::Undefined { self.independent_demand_of.clone() } else { e.independent_demand_of.clone().into() },
            finished: if e.finished == MaybeUndefined::Undefined { self.finished.clone() } else { e.finished.clone().to_option().unwrap() },
            in_scope_of: if e.in_scope_of== MaybeUndefined::Undefined { self.in_scope_of.clone() } else { e.in_scope_of.clone().into() },
            note: if e.note== MaybeUndefined::Undefined { self.note.clone() } else { e.note.clone().into() },
        }
    }
}

//---------------- EXTERNAL API ----------------

/// I/O struct to describe the complete output record, including all managed link fields
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    id: CommitmentAddress,
    action: ActionId,
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_of: Option<ProcessAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_of: Option<ProcessAddress>,
    provider: AgentAddress,
    receiver: AgentAddress,
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
    plan: Option<PlanAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    in_scope_of: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    agreed_in: Option<ExternalURL>,
    #[serde(skip_serializing_if = "Option::is_none")]
    clause_of: Option<AgreementAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    independent_demand_of: Option<PlanAddress>,

    finished: bool,

    // LINK FIELDS
    #[serde(skip_serializing_if = "Option::is_none")]
    fulfilled_by: Option<Vec<FulfillmentAddress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    satisfies: Option<Vec<SatisfactionAddress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    involved_agents: Option<Vec<AgentAddress>>,
}

/// I/O struct to describe what is returned outside the gateway
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData {
    commitment: Response,
}

/// Create response from input DHT primitives
pub fn construct_response<'a>(
    address: &CommitmentAddress, e: &Entry, (
        input_process, output_process,
        fulfillments,
        satisfactions,
        involved_agents,
    ): (
        Option<ProcessAddress>, Option<ProcessAddress>,
        Option<Cow<'a, Vec<FulfillmentAddress>>>,
        Option<Cow<'a, Vec<SatisfactionAddress>>>,
        Option<Cow<'a, Vec<AgentAddress>>>,
    )
) -> ResponseData {
    ResponseData {
        commitment: Response {
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
            resource_quantity: e.resource_quantity.to_owned(),
            effort_quantity: e.effort_quantity.to_owned(),
            has_beginning: e.has_beginning.to_owned(),
            has_end: e.has_end.to_owned(),
            has_point_in_time: e.has_point_in_time.to_owned(),
            due: e.due.to_owned(),
            at_location: e.at_location.to_owned(),
            plan: e.plan.to_owned(),
            agreed_in: e.agreed_in.to_owned(),
            clause_of: e.clause_of.to_owned(),
            independent_demand_of: e.independent_demand_of.to_owned(),
            finished: e.finished.to_owned(),
            in_scope_of: e.in_scope_of.to_owned(),
            fulfilled_by: fulfillments.map(Cow::into_owned),
            satisfies: satisfactions.map(Cow::into_owned),
            involved_agents: involved_agents.map(Cow::into_owned),
        }
    }
}

//---------------- READ ----------------

// @see construct_response
pub fn get_link_fields<'a>(commitment: &CommitmentAddress) -> (
    Option<ProcessAddress>,
    Option<ProcessAddress>,
    Option<Cow<'a, Vec<FulfillmentAddress>>>,
    Option<Cow<'a, Vec<SatisfactionAddress>>>,
    Option<Cow<'a, Vec<AgentAddress>>>,
) {
    (
        get_linked_remote_addresses_as_type(commitment, COMMITMENT_INPUT_OF_LINK_TYPE, COMMITMENT_INPUT_OF_LINK_TAG).into_owned().pop(),
        get_linked_remote_addresses_as_type(commitment, COMMITMENT_OUTPUT_OF_LINK_TYPE, COMMITMENT_OUTPUT_OF_LINK_TAG).into_owned().pop(),
        Some(get_linked_addresses_as_type(commitment, COMMITMENT_FULFILLEDBY_LINK_TYPE, COMMITMENT_FULFILLEDBY_LINK_TAG)),
        Some(get_linked_addresses_as_type(commitment, COMMITMENT_SATISFIES_LINK_TYPE, COMMITMENT_SATISFIES_LINK_TAG)),
        None,   // :TODO:
    )
}
