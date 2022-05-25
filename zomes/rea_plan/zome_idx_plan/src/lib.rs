/**
 * Plan query indexes for plan DNA
 *
 * @package Holo-REA
 * @since   2021-09-06
 */
use hdk_semantic_indexes_zome_derive::index_zome;
use hc_zome_rea_plan_rpc::*;

// :TODO: remove this; should not be necessary since all these types are imported
// along with their entry_def! in dependent crates
#[hdk_extern]
fn entry_defs(_: ()) -> ExternResult<EntryDefsCallbackResult> {
    Ok(EntryDefsCallbackResult::from(vec![
        PathEntry::entry_def(),
        PlanAddress::entry_def(),
        ProcessAddress::entry_def(),
        CommitmentAddress::entry_def(),
    ]))
}

#[index_zome]
struct Plan {
    processes: Remote<process, planned_within>,
    independent_demands: Remote<commitment, planned_within>,
    // NOTE: not sure how to handle the fact that there are two kinds of relationships between Commitment and Plan
    independent_demands: Remote<commitment, independent_demand_of>,
}
