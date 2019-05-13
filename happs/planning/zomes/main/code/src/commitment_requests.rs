/**
 * Handling for `Commitment`-related requests
 */

use hdk::{
    holochain_core_types::{
        cas::content::Address,
        error::HolochainError,
        json::JsonString,
        entry::Entry,
    },
    error::ZomeApiResult,
    commit_entry,
};
use holochain_core_types_derive::{ DefaultJson };

use vf_planning::type_aliases::{
    Timestamp,
    ExternalURL,
    LocationAddress,
    AgentAddress,
    ResourceAddress,
    ProcessOrTransferAddress,
    ResourceSpecificationAddress,
    PlanAddress,
};
use vf_planning::{
    commitment::{
        Entry as CommitmentEntry,
    },
    measurement::QuantityValue,
};

// Entry types

pub const COMMITMENT_ENTRY_TYPE: &str = "vf_commitment";

/**
 * I/O struct to describe the complete record, including all managed links
 */
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct CommitmentCreateFields {
    // ENTRY FIELDS
    note: Option<String>,
    input_of: ProcessOrTransferAddress,
    output_of: ProcessOrTransferAddress,
    provider: AgentAddress,
    receiver: AgentAddress,
    resource_inventoried_as: ResourceAddress,
    resource_classified_as: Option<Vec<ExternalURL>>,
    resource_conforms_to: ResourceSpecificationAddress,
    quantified_as: Option<QuantityValue>,
    has_beginning: Timestamp,
    has_end: Timestamp,
    has_point_in_time: Timestamp,
    before: Timestamp,
    after: Timestamp,
    plan: PlanAddress,
    at_location: LocationAddress,
    in_scope_of: Option<Vec<String>>,
}

/**
 * I/O struct to describe the complete externally-facing commitment record
 */
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct CommitmentResponse {
    // ENTRY FIELDS
    note: Option<String>,
    input_of: ProcessOrTransferAddress,
    output_of: ProcessOrTransferAddress,
    provider: AgentAddress,
    receiver: AgentAddress,
    resource_inventoried_as: ResourceAddress,
    resource_classified_as: Option<Vec<ExternalURL>>,
    resource_conforms_to: ResourceSpecificationAddress,
    quantified_as: Option<QuantityValue>,
    has_beginning: Timestamp,
    has_end: Timestamp,
    has_point_in_time: Timestamp,
    before: Timestamp,
    after: Timestamp,
    plan: PlanAddress,
    at_location: LocationAddress,
    in_scope_of: Option<Vec<String>>,
    finished: bool,
}

/**
 * Pick relevant fields out of I/O record into underlying DHT entry
 */
impl From<CommitmentCreateFields> for CommitmentEntry {
    fn from(e: CommitmentCreateFields) -> CommitmentEntry {
        CommitmentEntry {
            note: e.note.into(),
            input_of: e.input_of.into(),
            output_of: e.output_of.into(),
            provider: e.provider.into(),
            receiver: e.receiver.into(),
            resource_inventoried_as: e.resource_inventoried_as.into(),
            resource_classified_as: e.resource_classified_as.into(),
            resource_conforms_to: e.resource_conforms_to.into(),
            quantified_as: e.quantified_as.into(),
            has_beginning: e.has_beginning.into(),
            has_end: e.has_end.into(),
            has_point_in_time: e.has_point_in_time.into(),
            before: e.before.into(),
            after: e.after.into(),
            plan: e.plan.into(),
            at_location: e.at_location.into(),
            in_scope_of: e.in_scope_of.into(),

            // default values
            finished: false,
        }
    }
}

pub fn handle_create_commitment(commitment: CommitmentCreateFields) -> ZomeApiResult<Address> {
    // handle core entry fields
    let entry_struct: CommitmentEntry = commitment.into();
    let entry = Entry::App(COMMITMENT_ENTRY_TYPE.into(), entry_struct.into());
    let address = commit_entry(&entry)?;

    // return address
    // :TODO: return entire record structure
    Ok(address)
}
