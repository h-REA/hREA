/**
 * Commitment query indexes for planning DNA
 *
 * @package Holo-REA
 * @since   2021-08-28
 */
use hdk_semantic_indexes_zome_derive::index_zome;
use hc_zome_rea_commitment_rpc::*;

#[index_zome]
struct Commitment {
    fulfilled_by: Local<fulfillment, fulfills>,
    satisfies: Local<satisfaction, satisfied_by>,
    input_of: Local<process, committed_inputs>,
    output_of: Local<process, committed_outputs>,
    clause_of: Local<agreement, commitments>,

    // internal indexes (not part of VF spec)
    provider: Local<agent, commitments_as_provider>,
    receiver: Local<agent, commitments_as_receiver>,
    independent_demand_of: Local<plan, independent_demands>,
    planned_within: Local<plan, non_process_commitments>,
    // in_scope_of: Local<agent, commitments>,
}
