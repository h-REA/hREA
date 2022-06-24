/**
 * Agent query indexes for agent DNA
 *
 * @package Holo-REA
 * @since   2022-04-25
 */
use hdk_semantic_indexes_zome_derive::index_zome;
use hc_zome_rea_agent_rpc::*;

// :TODO: remove this; should not be necessary since all these types are imported
// along with their entry_def! in dependent crates
#[hdk_extern]
fn entry_defs(_: ()) -> ExternResult<EntryDefsCallbackResult> {
    Ok(EntryDefsCallbackResult::from(vec![
        PathEntry::entry_def(),
        CommitmentAddress::entry_def(),
        IntentAddress::entry_def(),
        AgentAddress::entry_def(),
    ]))
}
#[index_zome]
struct Agent {
    // internal indexes (not part of VF spec)
    // commitments: Remote<commitment, in_scope_of>,
    // intents: Remote<intent, in_scope_of>,
    // economic_events: Remote<economic_event, in_scope_of>,
    // inventoried_economic_resources: Remote<economic_resource, primary_accountable>,
    // plans: Remote<plan, in_scope_of>,
    // processes: Remote<process, in_scope_of>,
    // proposals: Remote<proposal, in_scope_of>,
    commitments_as_provider: Remote<commitment, provider>,
    commitments_as_receiver: Remote<commitment, receiver>,
    intents_as_provider: Remote<intent, provider>,
    intents_as_receiver: Remote<intent, receiver>,
}
