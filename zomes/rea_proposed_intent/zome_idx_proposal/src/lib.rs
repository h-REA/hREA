/**
 * ProposedIntent query indexes for proposal DNA
 *
 * @package Holo-REA
 * @since   2021-09-26
 */
use hdk_semantic_indexes_zome_derive::index_zome;
use hc_zome_rea_proposed_intent_rpc::*;
use hdk_semantic_indexes_zome_lib::ByAddress; // disambiguate from RPC query struct

#[index_zome]
struct ProposedIntent {
    published_in: Local<proposal, publishes>,
    publishes: Local<intent, proposed_in>,
}
