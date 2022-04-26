/**
 * satisfaction query indexes for planning DNA
 *
 * @package Holo-REA
 * @since   2021-08-29
 */
use hdk_semantic_indexes_zome_derive::index_zome;
use hc_zome_rea_satisfaction_rpc::*;
use hdk_semantic_indexes_zome_lib::ByAddress; // disambiguate from RPC query struct

// :TODO: remove this; should not be necessary since all these types are imported
// along with their entry_def! in dependent crates
#[hdk_extern]
fn entry_defs(_: ()) -> ExternResult<EntryDefsCallbackResult> {
    Ok(EntryDefsCallbackResult::from(vec![
        PathEntry::entry_def(),
        IntentAddress::entry_def(),
        SatisfactionAddress::entry_def(),
        CommitmentAddress::entry_def(),
    ]))
}

#[index_zome]
struct Satisfaction {
    satisfies: Local<intent, satisfied_by>,
    satisfied_by: Local<commitment, satisfies>,
}
