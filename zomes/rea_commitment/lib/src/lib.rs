/**
 * Holo-REA commitment zome library API
 *
 * Contains helper methods that can be used to manipulate `Commitment` data
 * structures in either the local Holochain zome, or a separate DNA-local zome.
 *
 * @package Holo-REA
 */
use paste::paste;
use hdk_records::{
    RecordAPIResult, MaybeUndefined,
    records::{
        create_record,
        read_record_entry,
        read_record_entry_by_header,
        update_record,
        delete_record,
    },
};
use hdk_semantic_indexes_client_lib::*;

use hc_zome_rea_commitment_storage::*;
use hc_zome_rea_commitment_rpc::*;

// :SHONK: needed to re-export for zome `entry_defs()` where macro-assigned defs are overridden
pub use hdk_records::CAP_STORAGE_ENTRY_DEF_ID;

pub fn handle_create_commitment<S>(entry_def_id: S, commitment: CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let (header_addr, base_address, entry_resp): (_,_, EntryData) = create_record(&entry_def_id, commitment.to_owned())?;

    // handle link fields
    // :TODO: improve error handling
    let e1 = create_index!(commitment.provider(&commitment.provider), agent.commitments_as_provider(&base_address));
    hdk::prelude::debug!("handle_create_commitment::provider index {:?}", e1);
    let e2 = create_index!(commitment.receiver(&commitment.receiver), agent.commitments_as_receiver(&base_address));
    hdk::prelude::debug!("handle_create_commitment::receiver index {:?}", e2);

    if let CreateRequest { input_of: MaybeUndefined::Some(input_of), .. } = &commitment {
        let e = create_index!(commitment.input_of(input_of), process.committed_inputs(&base_address));
        hdk::prelude::debug!("handle_create_commitment::input_of index {:?}", e);
    };
    if let CreateRequest { output_of: MaybeUndefined::Some(output_of), .. } = &commitment {
        let e = create_index!(commitment.output_of(output_of), process.committed_outputs(&base_address));
        hdk::prelude::debug!("handle_create_commitment::output_of index {:?}", e);
    };
    if let CreateRequest { clause_of: MaybeUndefined::Some(clause_of), .. } = &commitment {
        let e = create_index!(commitment.clause_of(clause_of), agreement.commitments(&base_address));
        hdk::prelude::debug!("handle_create_commitment::clause_of index {:?}", e);
    };
    if let CreateRequest { independent_demand_of: MaybeUndefined::Some(independent_demand_of), .. } = &commitment {
        let e = create_index!(commitment.independent_demand_of(independent_demand_of), plan.independent_demands(&base_address));
        hdk::prelude::debug!("handle_create_commitment::independent_demand_of index {:?}", e);
    };
    if let CreateRequest { planned_within: MaybeUndefined::Some(planned_within), .. } = &commitment {
        let e = create_index!(commitment.planned_within(planned_within), plan.non_process_commitments(&base_address));
        hdk::prelude::debug!("handle_create_commitment::planned_within index {:?}", e);
    };
    // // TODO: because commitment.in_scope_of is a vec of ids rather than one id, make sure this is still handled properly
    // if let CreateRequest { in_scope_of: MaybeUndefined::Some(in_scope_of), .. } = &commitment {
    //     let e = create_index!(commitment.in_scope_of(in_scope_of), agent.commitments(&base_address));
    //     hdk::prelude::debug!("handle_create_commitment::in_scope_of index {:?}", e);
    // };

    // :TODO: pass results from link creation rather than re-reading
    construct_response(&base_address, &header_addr, &entry_resp, get_link_fields(&base_address)?)
}

pub fn handle_get_commitment<S>(entry_def_id: S, address: CommitmentAddress) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let (revision, base_address, entry) = read_record_entry::<EntryData, EntryStorage, _,_,_>(&entry_def_id, address.as_ref())?;
    construct_response(&base_address, &revision, &entry, get_link_fields(&address)?)
}

pub fn handle_update_commitment<S>(entry_def_id: S, commitment: UpdateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let address = commitment.get_revision_id().to_owned();
    let (revision_id, base_address, new_entry, prev_entry): (_, CommitmentAddress, EntryData, EntryData) = update_record(&entry_def_id, &address, commitment.to_owned())?;

    if new_entry.input_of != prev_entry.input_of {
        let new_value = match &new_entry.input_of { Some(val) => vec![val.to_owned()], None => vec![] };
        let prev_value = match &prev_entry.input_of { Some(val) => vec![val.to_owned()], None => vec![] };
        let e = update_index!(
            commitment
                .input_of(new_value.as_slice())
                .not(prev_value.as_slice()),
            process.committed_inputs(&base_address)
        );
        hdk::prelude::debug!("handle_update_commitment::input_of index {:?}", e);
    }
    if new_entry.output_of != prev_entry.output_of {
        let new_value = match &new_entry.output_of { Some(val) => vec![val.to_owned()], None => vec![] };
        let prev_value = match &prev_entry.output_of { Some(val) => vec![val.to_owned()], None => vec![] };
        let e = update_index!(
            commitment
                .output_of(new_value.as_slice())
                .not(prev_value.as_slice()),
            process.committed_outputs(&base_address)
        );
        hdk::prelude::debug!("handle_update_commitment::output_of index {:?}", e);
    }
    if new_entry.clause_of != prev_entry.clause_of {
        let new_value = match &new_entry.clause_of { Some(val) => vec![val.to_owned()], None => vec![] };
        let prev_value = match &prev_entry.clause_of { Some(val) => vec![val.to_owned()], None => vec![] };
        let e = update_index!(
            commitment
                .clause_of(new_value.as_slice())
                .not(prev_value.as_slice()),
            agreement.commitments(&base_address)
        );
        hdk::prelude::debug!("handle_update_commitment::clause_of index {:?}", e);
    }
    if new_entry.independent_demand_of != prev_entry.independent_demand_of {
        let new_value = match &new_entry.independent_demand_of { Some(val) => vec![val.to_owned()], None => vec![] };
        let prev_value = match &prev_entry.independent_demand_of { Some(val) => vec![val.to_owned()], None => vec![] };
        let e = update_index!(
            commitment
                .independent_demand_of(new_value.as_slice())
                .not(prev_value.as_slice()),
            plan.independent_demands(&base_address)
        );
        hdk::prelude::debug!("handle_update_commitment::independent_demand_of index {:?}", e);
    }

    if new_entry.planned_within != prev_entry.planned_within {
        let new_value = match &new_entry.planned_within { Some(val) => vec![val.to_owned()], None => vec![] };
        let prev_value = match &prev_entry.planned_within { Some(val) => vec![val.to_owned()], None => vec![] };
        let e = update_index!(
            commitment
                .planned_within(new_value.as_slice())
                .not(prev_value.as_slice()),
            plan.non_process_commitments(&base_address)
        );
        hdk::prelude::debug!("handle_update_commitment::planned_within index {:?}", e);
    }
    // // TODO: ensure handling of vec of ids
    // if new_entry.in_scope_of != prev_entry.in_scope_of {
    //     let new_value = match &new_entry.in_scope_of { Some(val) => vec![val.to_owned()], None => vec![] };
    //     let prev_value = match &prev_entry.in_scope_of { Some(val) => vec![val.to_owned()], None => vec![] };
    //     let e = update_index!(
    //         commitment
    //             .in_scope_of(new_value.as_slice())
    //             .not(prev_value.as_slice()),
    //         agent.commitments(&base_address)
    //     );
    //     hdk::prelude::debug!("handle_update_commitment::in_scope_of index {:?}", e);
    // }
    if new_entry.provider != prev_entry.provider {
        let new_value = vec![new_entry.provider.to_owned()];
        let prev_value = vec![prev_entry.provider.to_owned()];
        let e = update_index!(
            commitment
                .provider(new_value.as_slice())
                .not(prev_value.as_slice()),
            agent.commitments_as_provider(&base_address)
        );
        hdk::prelude::debug!("handle_update_commitment::provider index {:?}", e);
    }
    if new_entry.receiver != prev_entry.receiver {
        let new_value = vec![new_entry.receiver.to_owned()];
        let prev_value = vec![prev_entry.receiver.to_owned()];
        let e = update_index!(
            commitment
                .receiver(new_value.as_slice())
                .not(prev_value.as_slice()),
            agent.commitments_as_receiver(&base_address)
        );
        hdk::prelude::debug!("handle_update_commitment::receiver index {:?}", e);
    }

    construct_response(&base_address, &revision_id, &new_entry, get_link_fields(&base_address)?)
}

pub fn handle_delete_commitment(revision_id: HeaderHash) -> RecordAPIResult<bool>
{
    // load the record to ensure it is of the correct type
    let (base_address, entry) = read_record_entry_by_header::<EntryData, EntryStorage, _>(&revision_id)?;

    // handle link fields
    if let Some(process_address) = entry.input_of {
        let e = update_index!(commitment.input_of.not(&vec![process_address]), process.committed_inputs(&base_address));
        hdk::prelude::debug!("handle_delete_commitment::input_of index {:?}", e);
    }
    if let Some(process_address) = entry.output_of {
        let e = update_index!(commitment.output_of.not(&vec![process_address]), process.committed_outputs(&base_address));
        hdk::prelude::debug!("handle_delete_commitment::output_of index {:?}", e);
    }
    if let Some(agreement_address) = entry.clause_of {
        let e = update_index!(commitment.clause_of.not(&vec![agreement_address]), agreement.commitments(&base_address));
        hdk::prelude::debug!("handle_delete_commitment::clause_of index {:?}", e);
    }
    if let Some(plan_address) = entry.independent_demand_of {
        let e = update_index!(commitment.independent_demand_of.not(&vec![plan_address]), plan.independent_demands(&base_address));
        hdk::prelude::debug!("handle_delete_commitment::independent_demand_of index {:?}", e);
    }
    // if let Some(agent_address) = entry.in_scope_of {
    //     let e = update_index!(commitment.in_scope_of.not(&vec![agent_address]), agent.commitments(&base_address));
    //     hdk::prelude::debug!("handle_delete_commitment::in_scope_of index {:?}", e);
    // }
    let e1 = update_index!(commitment.provider.not(&vec![entry.provider]), agent.commitments_as_provider(&base_address));
    hdk::prelude::debug!("handle_delete_commitment::provider index {:?}", e1);
    let e2 = update_index!(commitment.receiver.not(&vec![entry.receiver]), agent.commitments_as_receiver(&base_address));
    hdk::prelude::debug!("handle_delete_commitment::receiver index {:?}", e2);

    // delete entry last, as it must be present in order for links to be removed
    delete_record::<EntryStorage>(&revision_id)
}

/// Create response from input DHT primitives
fn construct_response<'a>(
    address: &CommitmentAddress, revision_id: &HeaderHash, e: &EntryData, (
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
            planned_within: e.planned_within.to_owned(),
            agreed_in: e.agreed_in.to_owned(),
            clause_of: e.clause_of.to_owned(),
            independent_demand_of: e.independent_demand_of.to_owned(),
            finished: e.finished.to_owned(),
            in_scope_of: e.in_scope_of.to_owned(),
            fulfilled_by: fulfillments.to_owned(),
            satisfies: satisfactions.to_owned(),
            involved_agents: involved_agents.to_owned(),
            // TODO: does this need to be updated to reflect current vf-graphql? For example, shouldn't the field `in_scope_of` not be part of `EntryData` and instead passed in?
        }
    })
}

//---------------- READ ----------------

/// Properties accessor for zome config
fn read_commitment_index_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.commitment.index_zome)
}

/// Properties accessor for zome config
fn read_process_index_zome(conf: DnaConfigSlice) -> Option<String> {
    conf.commitment.process_index_zome
}

/// Properties accessor for zome config
fn read_agreement_index_zome(conf: DnaConfigSlice) -> Option<String> {
    conf.commitment.agreement_index_zome
}

/// Properties accessor for zome config
fn read_agent_index_zome(conf: DnaConfigSlice) -> Option<String> {
    conf.commitment.agent_index_zome
}

/// Properties accessor for zome config
fn read_plan_index_zome(conf: DnaConfigSlice) -> Option<String> {
    conf.commitment.plan_index_zome
}

// @see construct_response
fn get_link_fields(commitment: &CommitmentAddress) -> RecordAPIResult<(
    Vec<FulfillmentAddress>,
    Vec<SatisfactionAddress>,
    Vec<AgentAddress>, // is this for `involved_agents` or `in_scope_of` or both?
)> {
    Ok((
        read_index!(commitment(commitment).fulfilled_by)?,
        read_index!(commitment(commitment).satisfies)?,
        vec![],   // :TODO:
    ))
}
