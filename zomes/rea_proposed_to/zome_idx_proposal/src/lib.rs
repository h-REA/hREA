/**
 * ProposedTo query indexes for proposal DNA
 *
 * @package Holo-REA
 * @since   2021-09-26
 */
use hdk_semantic_indexes_zome_derive::index_zome;
use hc_zome_rea_proposed_to_rpc::*;
use hdk_semantic_indexes_zome_lib::ByAddress; // disambiguate from RPC query struct

// :TODO: remove this; should not be necessary since all these types are imported
// along with their entry_def! in dependent crates
#[hdk_extern]
fn entry_defs(_: ()) -> ExternResult<EntryDefsCallbackResult> {
    Ok(EntryDefsCallbackResult::from(vec![
        PathEntry::entry_def(),
        TimeIndex::entry_def(),
        ProposedToAddress::entry_def(),
        ProposalAddress::entry_def(),
        // AgentAddress::entry_def(), // :TODO:
    ]))
}

#[index_zome]
struct ProposedTo {
    proposed: Local<proposal, published_to>,
    // :TODO: figure out best approach for managing agent identifiers. Should there be a wrapper record to make treating them as records easier?
    // proposed_to: Local<agent, proposals>, // :TODO: finalise reciprocal query edge name
}
