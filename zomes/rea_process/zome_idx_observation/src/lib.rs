/**
 * Process query indexes for observation DNA
 *
 * @package Holo-REA
 * @since   2021-08-26
 */
use hdk_semantic_indexes_zome_derive::index_zome;
use hc_zome_rea_process_rpc::*;

// :TODO: remove this; should not be necessary since all these types are imported
// along with their entry_def! in dependent crates
#[hdk_extern]
fn entry_defs(_: ()) -> ExternResult<EntryDefsCallbackResult> {
    Ok(EntryDefsCallbackResult::from(vec![
        PathEntry::entry_def(),
        TimeIndex::entry_def(),
        IntentAddress::entry_def(),
        CommitmentAddress::entry_def(),
        ProcessAddress::entry_def(),
        EconomicEventAddress::entry_def(),
        PlanAddress::entry_def(),
    ]))
}

#[index_zome(query_fn_name="query_processes",read_all_fn_name="read_all_processes")]
struct Process {
    observed_inputs: Local<economic_event, input_of>,
    observed_outputs: Local<economic_event, output_of>,
    committed_inputs: Remote<commitment, input_of>,
    committed_outputs: Remote<commitment, output_of>,
    intended_inputs: Remote<intent, input_of>,
    intended_outputs: Remote<intent, output_of>,
    planned_within: Local<plan, processes>,
}
