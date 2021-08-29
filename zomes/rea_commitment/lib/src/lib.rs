/**
 * Holo-REA commitment zome library API
 *
 * Contains helper methods that can be used to manipulate `Commitment` data
 * structures in either the local Holochain zome, or a separate DNA-local zome.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;
use hdk_records::{
    RecordAPIResult, DataIntegrityError,
    MaybeUndefined,
    records::{
        create_record,
        read_record_entry,
        read_record_entry_by_header,
        update_record,
        delete_record,
    },
    local_indexes::{
        read_index,
        query_index,
    },
    remote_indexes::{
        create_remote_index,
        update_remote_index,
    },
};

use vf_attributes_hdk::{
    AgentAddress,
    FulfillmentAddress,
    SatisfactionAddress,
};

use hc_zome_rea_commitment_storage_consts::*;
use hc_zome_rea_commitment_storage::*;
use hc_zome_rea_commitment_rpc::*;

use hc_zome_rea_process_storage_consts::{PROCESS_COMMITMENT_INPUTS_LINK_TAG, PROCESS_COMMITMENT_OUTPUTS_LINK_TAG};
use hc_zome_rea_fulfillment_storage_consts::{FULFILLMENT_FULFILLS_LINK_TAG};
use hc_zome_rea_satisfaction_storage_consts::{SATISFACTION_SATISFIEDBY_LINK_TAG};
use hc_zome_rea_agreement_storage_consts::{AGREEMENT_COMMITMENTS_LINK_TAG};

// :SHONK: needed to re-export for zome `entry_defs()` where macro-assigned defs are overridden
pub use hdk_records::CAP_STORAGE_ENTRY_DEF_ID;

pub fn receive_create_commitment<S>(entry_def_id: S, process_entry_def_id: S, agreement_entry_def_id: S, commitment: CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    handle_create_commitment(entry_def_id, process_entry_def_id, agreement_entry_def_id, &commitment)
}

pub fn receive_get_commitment<S>(entry_def_id: S, address: CommitmentAddress) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    handle_get_commitment(entry_def_id, &address)
}

pub fn receive_update_commitment<S>(entry_def_id: S, process_entry_def_id: S, commitment: UpdateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    handle_update_commitment(entry_def_id, process_entry_def_id, &commitment)
}

pub fn receive_delete_commitment<S>(entry_def_id: S, process_entry_def_id: S, revision_id: RevisionHash) -> RecordAPIResult<bool>
    where S: AsRef<str>
{
    handle_delete_commitment(entry_def_id, process_entry_def_id, revision_id)
}

fn handle_get_commitment<S>(entry_def_id: S, address: &CommitmentAddress) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let (revision, base_address, entry) = read_record_entry::<EntryData, EntryStorage, _,_>(&entry_def_id, address.as_ref())?;
    construct_response(&base_address, &revision, &entry, get_link_fields(&entry_def_id, &address)?)
}

fn handle_create_commitment<S>(
    entry_def_id: S, process_entry_def_id: S, agreement_entry_def_id: S,
    commitment: &CreateRequest,
) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let (header_addr, base_address, entry_resp): (_,_, EntryData) = create_record(&entry_def_id, commitment.to_owned())?;

    // handle link fields
    if let CreateRequest { input_of: MaybeUndefined::Some(input_of), .. } = commitment {
        let _results = create_remote_index(
            &PROCESS_INPUT_INDEXING_API_METHOD,
            &entry_def_id, &base_address,
            &process_entry_def_id,
            vec![input_of.clone()].as_slice(),
            COMMITMENT_INPUT_OF_LINK_TAG, PROCESS_COMMITMENT_INPUTS_LINK_TAG,
        )?;
    };
    if let CreateRequest { output_of: MaybeUndefined::Some(output_of), .. } = commitment {
        let _results = create_remote_index(
            &PROCESS_OUTPUT_INDEXING_API_METHOD,
            &entry_def_id, &base_address,
            &process_entry_def_id,
            vec![output_of.clone()].as_slice(),
            COMMITMENT_OUTPUT_OF_LINK_TAG, PROCESS_COMMITMENT_OUTPUTS_LINK_TAG,
        )?;
    };
    if let CreateRequest { clause_of: MaybeUndefined::Some(clause_of), .. } = commitment {
        let _results = create_remote_index(
            &AGREEMENT_CLAUSE_INDEXING_API_METHOD,
            &entry_def_id, &base_address,
            &agreement_entry_def_id,
            vec![clause_of.clone()].as_slice(),
            COMMITMENT_CLAUSE_OF_LINK_TAG, AGREEMENT_COMMITMENTS_LINK_TAG,
        )?;
    };

    // :TODO: pass results from link creation rather than re-reading
    construct_response(&base_address, &header_addr, &entry_resp, get_link_fields(&entry_def_id, &base_address)?)
}

fn handle_update_commitment<S>(
    entry_def_id: S, process_entry_def_id: S,
    commitment: &UpdateRequest,
) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let address = commitment.get_revision_id().to_owned();
    let (revision_id, base_address, new_entry, _prev_entry): (_, CommitmentAddress, EntryData, EntryData) = update_record(&entry_def_id, &address, commitment.to_owned())?;

    // handle link fields
    // :TODO: revise this logic; it creates dangling pointers. Need to check old record and ignore unchanged value, delete on removal.

    if let UpdateRequest { input_of: MaybeUndefined::Some(input_of), .. } = commitment {
        let _results = update_remote_index(
            &PROCESS_INPUT_INDEXING_API_METHOD,
            &entry_def_id, &base_address,
            &process_entry_def_id,
            vec![input_of.to_owned()].as_slice(),
            vec![].as_slice(),
            COMMITMENT_INPUT_OF_LINK_TAG, PROCESS_COMMITMENT_INPUTS_LINK_TAG,
        )?;
    }
    if let UpdateRequest { output_of: MaybeUndefined::Some(output_of), .. } = commitment {
        let _results = update_remote_index(
            &PROCESS_OUTPUT_INDEXING_API_METHOD,
            &entry_def_id, &base_address,
            &process_entry_def_id,
            vec![output_of.to_owned()].as_slice(),
            vec![].as_slice(),
            COMMITMENT_OUTPUT_OF_LINK_TAG, PROCESS_COMMITMENT_OUTPUTS_LINK_TAG,
        );
    }
    if let UpdateRequest { clause_of: MaybeUndefined::Some(clause_of), .. } = commitment {
        let _results = update_remote_index(
            &AGREEMENT_CLAUSE_INDEXING_API_METHOD,
            &entry_def_id, &base_address,
            &process_entry_def_id,
            vec![clause_of.to_owned()].as_slice(),
            vec![].as_slice(),
            COMMITMENT_CLAUSE_OF_LINK_TAG, AGREEMENT_COMMITMENTS_LINK_TAG,
        );
    }

    construct_response(&base_address, &revision_id, &new_entry, get_link_fields(&entry_def_id, &base_address)?)
}

fn handle_delete_commitment<S>(entry_def_id: S, process_entry_def_id: S, revision_id: RevisionHash) -> RecordAPIResult<bool>
    where S: AsRef<str>
{
    // load the record to ensure it is of the correct type
    let (base_address, entry) = read_record_entry_by_header::<EntryData, EntryStorage, _>(&revision_id)?;

    // handle link fields
    if let Some(process_address) = entry.input_of {
        let _results = update_remote_index(
            &PROCESS_INPUT_INDEXING_API_METHOD,
            &entry_def_id, &base_address,
            &process_entry_def_id,
            vec![].as_slice(),
            vec![process_address].as_slice(),
            COMMITMENT_INPUT_OF_LINK_TAG, PROCESS_COMMITMENT_INPUTS_LINK_TAG,
        );
    }
    if let Some(process_address) = entry.output_of {
        let _results = update_remote_index(
            &PROCESS_OUTPUT_INDEXING_API_METHOD,
            &entry_def_id, &base_address,
            &process_entry_def_id,
            vec![].as_slice(),
            vec![process_address].as_slice(),
            COMMITMENT_OUTPUT_OF_LINK_TAG, PROCESS_COMMITMENT_OUTPUTS_LINK_TAG,
        );
    }
    if let Some(agreement_address) = entry.clause_of {
        let _results = update_remote_index(
            &AGREEMENT_CLAUSE_INDEXING_API_METHOD,
            &entry_def_id, &base_address,
            &process_entry_def_id,
            vec![].as_slice(),
            vec![agreement_address].as_slice(),
            COMMITMENT_CLAUSE_OF_LINK_TAG, AGREEMENT_COMMITMENTS_LINK_TAG,
        );
    }

    // delete entry last, as it must be present in order for links to be removed
    delete_record::<EntryStorage, _>(&revision_id)
}

const READ_FN_NAME: &str = "get_commitment";

pub fn generate_query_handler<S, C, F>(
    foreign_zome_name_from_config: F,
    process_entry_def_id: S,
    fulfillment_entry_def_id: S,
    satisfaction_entry_def_id: S,
    agreement_entry_def_id: S,
) -> impl FnOnce(&QueryParams) -> RecordAPIResult<Vec<ResponseData>>
    where S: AsRef<str>,
        C: std::fmt::Debug,
        SerializedBytes: TryInto<C, Error = SerializedBytesError>,
        F: Fn(C) -> Option<String>,
{
    move |params| {
        let mut entries_result: RecordAPIResult<Vec<RecordAPIResult<ResponseData>>> = Err(DataIntegrityError::EmptyQuery);

        // :TODO: implement proper AND search rather than exclusive operations
        match &params.fulfilled_by {
            Some(fulfilled_by) => {
                entries_result = query_index::<ResponseData, CommitmentAddress, C,F,_,_,_,_>(
                    &fulfillment_entry_def_id,
                    fulfilled_by, FULFILLMENT_FULFILLS_LINK_TAG,
                    &foreign_zome_name_from_config, &READ_FN_NAME
                );
            },
            _ => (),
        };
        match &params.satisfies {
            Some(satisfies) => {
                entries_result = query_index::<ResponseData, CommitmentAddress, C,F,_,_,_,_>(
                    &satisfaction_entry_def_id,
                    satisfies, SATISFACTION_SATISFIEDBY_LINK_TAG,
                    &foreign_zome_name_from_config, &READ_FN_NAME
                );
            },
            _ => (),
        };
        match &params.input_of {
            Some(input_of) => {
                entries_result = query_index::<ResponseData, CommitmentAddress, C,F,_,_,_,_>(
                    &process_entry_def_id,
                    input_of, PROCESS_COMMITMENT_INPUTS_LINK_TAG,
                    &foreign_zome_name_from_config, &READ_FN_NAME
                );
            },
            _ => (),
        };
        match &params.output_of {
            Some(output_of) => {
                entries_result = query_index::<ResponseData, CommitmentAddress, C,F,_,_,_,_>(
                    &process_entry_def_id,
                    output_of, PROCESS_COMMITMENT_OUTPUTS_LINK_TAG,
                    &foreign_zome_name_from_config, &READ_FN_NAME
                );
            },
            _ => (),
        };
        match &params.clause_of {
            Some(clause_of) => {
                entries_result = query_index::<ResponseData, CommitmentAddress, C,F,_,_,_,_>(
                    &agreement_entry_def_id,
                    clause_of, AGREEMENT_COMMITMENTS_LINK_TAG,
                    &foreign_zome_name_from_config, &READ_FN_NAME
                );
            },
            _ => (),
        };

        // :TODO: return errors for UI, rather than filtering
        Ok(entries_result?.iter()
            .cloned()
            .filter_map(Result::ok)
            .collect())
    }
}

/// Create response from input DHT primitives
pub fn construct_response<'a>(
    address: &CommitmentAddress, revision_id: &RevisionHash, e: &EntryData, (
        fulfillments,
        satisfactions,
        involved_agents,
    ): (
        Vec<FulfillmentAddress>,
        Vec<SatisfactionAddress>,
        Vec<AgentAddress>,
    )
) -> RecordAPIResult<ResponseData> {
    Ok(ResponseData {
        commitment: Response {
            id: address.to_owned(),
            revision_id: revision_id.to_owned(),
            action: e.action.to_owned(),
            note: e.note.to_owned(),
            input_of: e.input_of.to_owned(),
            output_of: e.output_of.to_owned(),
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
            fulfilled_by: fulfillments.to_owned(),
            satisfies: satisfactions.to_owned(),
            involved_agents: involved_agents.to_owned(),
        }
    })
}

//---------------- READ ----------------

// @see construct_response
pub fn get_link_fields<'a, S>(entry_def_id: S, commitment: &CommitmentAddress) -> RecordAPIResult<(
    Vec<FulfillmentAddress>,
    Vec<SatisfactionAddress>,
    Vec<AgentAddress>,
)>
    where S: AsRef<str>,
{
    Ok((
        read_index(&entry_def_id, commitment, COMMITMENT_FULFILLEDBY_LINK_TAG)?,
        read_index(&entry_def_id, commitment, COMMITMENT_SATISFIES_LINK_TAG)?,
        vec![],   // :TODO:
    ))
}
