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
        EconomicEventAddress::entry_def(),
        CommitmentAddress::entry_def(),
    ]))
}

#[index_zome]
struct Plan {
    economic_events: Remote<economic_event, realization_of>,
    commitments: Remote<commitment, clause_of>,
}
