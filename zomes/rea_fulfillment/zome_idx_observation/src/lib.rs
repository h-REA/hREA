/**
 * Fulfillment query indexes for observation DNA
 *
 * @package Holo-REA
 * @since   2021-08-29
 */
use hdk_semantic_indexes_zome_derive::index_zome;
use hc_zome_rea_fulfillment_rpc::*;
use hdk_semantic_indexes_zome_lib::ByAddress; // disambiguate from RPC query struct

#[index_zome]
struct Fulfillment {
    fulfilled_by: Local<economic_event, fulfills>,
}
