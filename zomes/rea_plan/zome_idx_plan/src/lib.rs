/**
 * Plan query indexes for plan DNA
 *
 * @package Holo-REA
 * @since   2021-09-06
 */
use hdk_semantic_indexes_zome_derive::index_zome;
use hc_zome_rea_plan_rpc::*;

#[index_zome]
struct Plan {
    processes: Remote<process, planned_within>,
    non_process_commitments: Remote<commitment, planned_within>,
    independent_demands: Remote<commitment, independent_demand_of>,
}
