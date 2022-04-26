/**
 * Intent query indexes for planning DNA
 *
 * @package Holo-REA
 * @since   2021-08-29
 */
use hdk_semantic_indexes_zome_derive::index_zome;
use hc_zome_rea_intent_rpc::*;

// :TODO: remove this; should not be necessary since all these types are imported
// along with their entry_def! in dependent crates
#[hdk_extern]
fn entry_defs(_: ()) -> ExternResult<EntryDefsCallbackResult> {
    Ok(EntryDefsCallbackResult::from(vec![
        PathEntry::entry_def(),
        IntentAddress::entry_def(),
        ProcessAddress::entry_def(),
    ]))
}

#[index_zome]
struct Intent {
    satisfied_by: Local<satisfaction, satisfies>,
    input_of: Local<process, intended_inputs>,
    output_of: Local<process, intended_outputs>,
    proposed_in: Remote<proposed_intent, publishes>,
}
