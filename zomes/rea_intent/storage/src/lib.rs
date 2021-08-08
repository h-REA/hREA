/**
 * Holo-REA intent zome internal data structures
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
    IntentAddress,
    LocationAddress,
    AgentAddress,
    ResourceAddress,
    ProcessAddress,
    ResourceSpecificationAddress,
};

use vf_actions::{ validate_flow_action };

use hc_zome_rea_intent_rpc::{ CreateRequest, UpdateRequest };

// :SHONK: needed as re-export in zome logic to allow validation logic to parse entries
pub use hdk_records::record_interface::Identified;

//---------------- RECORD INTERNALS & VALIDATION ----------------

#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
pub struct EntryData {
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

impl EntryData {
    pub fn validate_action(&self) -> Result<(), String> {
        validate_flow_action(self.action.to_owned(), self.input_of.to_owned(), self.output_of.to_owned())
    }

    pub fn validate_or_fields(&self) -> Result<(), String> {
        if !(self.provider.is_some() || self.receiver.is_some()) {
            return Err("Intent must have either a provider or a receiver".into());
        }
        Ok(())
    }
}

generate_record_entry!(EntryData, IntentAddress, EntryStorage);

//---------------- CREATE ----------------

/// Pick relevant fields out of I/O record into underlying DHT entry
impl From<CreateRequest> for EntryData {
    fn from(e: CreateRequest) -> EntryData {
        EntryData {
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

/// Handles update operations by merging any newly provided fields
impl Updateable<UpdateRequest> for EntryData {
    fn update_with(&self, e: UpdateRequest) -> EntryData {
        EntryData {
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
