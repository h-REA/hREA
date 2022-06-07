/**
 * Proposal query indexes for proposal DNA
 *
 * @package Holo-REA
 * @since   2021-09-21
 */
use hdk_semantic_indexes_zome_derive::index_zome;
use hc_zome_rea_proposal_rpc::*;
use hdk_semantic_indexes_zome_lib::ByAddress; // disambiguate from RPC query struct

// :TODO: remove this; should not be necessary since all these types are imported
// along with their entry_def! in dependent crates
#[hdk_extern]
fn entry_defs(_: ()) -> ExternResult<EntryDefsCallbackResult> {
    Ok(EntryDefsCallbackResult::from(vec![
        PathEntry::entry_def(),
        ProposalAddress::entry_def(),
        ProposedIntentAddress::entry_def(),
        ProposedToAddress::entry_def(),
    ]))
}

#[index_zome]
struct Proposal {
    publishes: Local<proposed_intent, published_in>,
    published_to: Local<proposed_to, proposed>,
}
