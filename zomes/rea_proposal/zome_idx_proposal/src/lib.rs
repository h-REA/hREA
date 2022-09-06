/**
 * Proposal query indexes for proposal DNA
 *
 * @package hREA
 * @since   2021-09-21
 */
use hdk_semantic_indexes_zome_derive::index_zome;
use hc_zome_rea_proposal_rpc::*;
use hdk_semantic_indexes_zome_lib::ByAddress; // disambiguate from RPC query struct

#[index_zome]
struct Proposal {
    publishes: Local<proposed_intent, published_in>,
    published_to: Local<proposed_to, proposed>,
}
