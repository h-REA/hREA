/**
 * Holo-REA commitment zome internal data structures
 *
 * Required by the zome itself, and for any DNA-local zomes interacting with its
 * storage API directly.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use hdk_records::{
    MaybeUndefined,
    record_interface::Updateable,
    generate_record_entry,
};
use vf_measurement::QuantityValue;

pub use vf_attributes_hdk::{
    RevisionHash,
    ActionId,
    Timestamp,
    ExternalURL,
    CommitmentAddress,
    LocationAddress,
    AgentAddress,
    ResourceAddress,
    ProcessAddress,
    ResourceSpecificationAddress,
    AgreementAddress,
    PlanAddress,
};

use vf_actions::{ validate_flow_action };

use hc_zome_rea_commitment_rpc::{ CreateRequest, UpdateRequest };

// :SHONK: needed as re-export in zome logic to allow validation logic to parse entries
pub use hdk_records::record_interface::Identified;

//---------------- RECORD INTERNALS & VALIDATION ----------------

#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
pub struct EntryData {
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

impl EntryData {
    pub fn validate_action(&self) -> Result<(), String> {
        validate_flow_action(self.action.to_owned(), self.input_of.to_owned(), self.output_of.to_owned())
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

generate_record_entry!(EntryData, CommitmentAddress, EntryStorage);

//---------------- CREATE ----------------

/// Pick relevant fields out of I/O record into underlying DHT entry
impl From<CreateRequest> for EntryData {
    fn from(e: CreateRequest) -> EntryData {
        EntryData {
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

/// Handles update operations by merging any newly provided fields
impl Updateable<UpdateRequest> for EntryData {
    fn update_with(&self, e: UpdateRequest) -> EntryData {
        EntryData {
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
