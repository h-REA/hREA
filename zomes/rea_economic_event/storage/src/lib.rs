/**
 * Holo-REA 'economic event' zome internal data structures
 *
 * Required by the zome itself, and for any DNA-local zomes interacting with its
 * storage API directly.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use hdk_records::{
    generate_record_entry,
    MaybeUndefined,
    record_interface::Updateable,
};

use vf_measurement::QuantityValue;
use vf_attributes_hdk::{
    EconomicEventAddress,
    ActionId,
    ExternalURL,
    LocationAddress,
    AgentAddress,
    EconomicResourceAddress,
    ProcessAddress,
    ResourceSpecificationAddress,
    AgreementAddress,
    DateTime, FixedOffset,
};
use vf_actions::{ validate_flow_action, validate_move_inventories };
use hc_zome_rea_economic_event_rpc::*;

// :SHONK: needed as re-export in zome logic to allow validation logic to parse entries
pub use hdk_records::record_interface::Identified;

//--------------- ZOME CONFIGURATION ATTRIBUTES ----------------

// :TODO: remove this, replace with reference to appropriate namespacing of zome config
#[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
pub struct DnaConfigSlice {
    pub economic_event: EconomicEventZomeConfig,
}

#[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
pub struct EconomicEventZomeConfig {
    pub index_zome: String,
    // zome ID (defined in `dna.yaml`) of a ValueFlows `EconomicResource`-compatible zome where inventory state for these `EconomicEvents` can be managed.
    pub economic_resource_zome: Option<String>,
    pub economic_resource_index_zome: Option<String>,
    pub process_index_zome: Option<String>,
    pub agreement_index_zome: Option<String>,
    pub agent_index_zome: Option<String>,
}

//---------------- RECORD INTERNALS & VALIDATION ----------------

#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
pub struct EntryData {
    pub action: ActionId,
    pub provider: AgentAddress,
    pub receiver: AgentAddress,
    pub input_of: Option<ProcessAddress>,   // :NOTE: shadows link, see https://github.com/holo-rea/holo-rea/issues/60#issuecomment-553756873
    pub output_of: Option<ProcessAddress>,
    pub resource_inventoried_as: Option<EconomicResourceAddress>,
    pub to_resource_inventoried_as: Option<EconomicResourceAddress>,
    pub resource_classified_as: Option<Vec<ExternalURL>>,
    pub resource_conforms_to: Option<ResourceSpecificationAddress>,
    pub resource_quantity: Option<QuantityValue>,
    pub effort_quantity: Option<QuantityValue>,
    pub has_beginning: Option<DateTime<FixedOffset>>,
    pub has_end: Option<DateTime<FixedOffset>>,
    pub has_point_in_time: Option<DateTime<FixedOffset>>,
    pub at_location: Option<LocationAddress>,
    pub agreed_in: Option<ExternalURL>,
    pub realization_of: Option<AgreementAddress>,
    pub triggered_by: Option<EconomicEventAddress>,
    pub in_scope_of: Option<Vec<String>>,
    pub note: Option<String>,
}

impl EntryData {
    pub fn validate_action(&self) -> Result<(), String> {
        let result = validate_flow_action(self.action.to_owned(), self.input_of.to_owned(), self.output_of.to_owned());
        if result.is_ok() && self.action.as_ref() == "move" {
            return validate_move_inventories(self.resource_inventoried_as.to_owned(), self.to_resource_inventoried_as.to_owned());
        }
        return result;
    }

    pub fn validate_or_fields(&self) -> Result<(), String> {
        if !(self.resource_inventoried_as.is_some() || self.resource_classified_as.is_some() || self.resource_conforms_to.is_some()) {
            return Err("EconomicEvent must reference an inventoried resource, resource specification or resource classification".into());
        }
        if !(self.resource_quantity.is_some() || self.effort_quantity.is_some()) {
            return Err("EconomicEvent must include either a resource quantity or an effort quantity".into());
        }
        if !(self.has_beginning.is_some() || self.has_end.is_some() || self.has_point_in_time.is_some()) {
            return Err("EconomicEvent must have a beginning, end or exact time".into());
        }
        Ok(())
    }
}

generate_record_entry!(EntryData, EconomicEventAddress, EntryStorage);

//---------------- CREATE ----------------

/**
 * Pick relevant fields out of I/O record into underlying DHT entry
 */
impl From<CreateRequest> for EntryData {
    fn from(e: CreateRequest) -> EntryData {
        EntryData {
            action: e.action.into(),
            note: e.note.into(),
            provider: e.provider.into(),
            receiver: e.receiver.into(),
            input_of: e.input_of.into(),
            output_of: e.output_of.into(),
            resource_inventoried_as: e.resource_inventoried_as.into(),
            to_resource_inventoried_as: e.to_resource_inventoried_as.into(),
            resource_classified_as: e.resource_classified_as.into(),
            resource_conforms_to: e.resource_conforms_to.into(),
            resource_quantity: e.resource_quantity.into(),
            effort_quantity: e.effort_quantity.into(),
            has_beginning: e.has_beginning.into(),
            has_end: e.has_end.into(),
            has_point_in_time: e.has_point_in_time.into(),
            agreed_in: e.agreed_in.into(),
            realization_of: e.realization_of.into(),
            triggered_by: e.triggered_by.into(),
            at_location: e.at_location.into(),
            in_scope_of: e.in_scope_of.into(),
        }
    }
}

//---------------- UPDATE ----------------

/// Handles update operations by merging any newly provided fields into
impl Updateable<UpdateRequest> for EntryData {
    fn update_with(&self, e: UpdateRequest) -> EntryData {
        EntryData {
            action: self.action.to_owned(),
            provider: self.provider.to_owned(),
            receiver: self.receiver.to_owned(),
            input_of: self.input_of.to_owned(),
            output_of: self.output_of.to_owned(),
            resource_inventoried_as: self.resource_inventoried_as.to_owned(),
            to_resource_inventoried_as: self.to_resource_inventoried_as.to_owned(),
            resource_classified_as: self.resource_classified_as.to_owned(),
            resource_conforms_to: self.resource_conforms_to.to_owned(),
            resource_quantity: self.resource_quantity.to_owned(),
            effort_quantity: self.effort_quantity.to_owned(),
            has_beginning: self.has_beginning.to_owned(),
            has_end: self.has_end.to_owned(),
            has_point_in_time: self.has_point_in_time.to_owned(),
            agreed_in: self.agreed_in.to_owned(),
            triggered_by: self.triggered_by.to_owned(),
            realization_of: self.realization_of.to_owned(),
            at_location: self.at_location.to_owned(),
            in_scope_of: if e.in_scope_of== MaybeUndefined::Undefined { self.in_scope_of.to_owned() } else { e.in_scope_of.to_owned().into() },
            note: if e.note== MaybeUndefined::Undefined { self.note.to_owned() } else { e.note.to_owned().into() },
        }
    }
}
