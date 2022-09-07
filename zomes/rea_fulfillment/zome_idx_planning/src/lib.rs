/**
 * Fulfillment query indexes for planning DNA
 *
 * @package hREA
 * @since   2021-08-29
 */
use hdk_semantic_indexes_zome_derive::index_zome;
use hc_zome_rea_fulfillment_rpc::*;
use hdk_semantic_indexes_zome_lib::ByAddress; // disambiguate from RPC query struct

#[index_zome]
struct Fulfillment {
    fulfills: Local<commitment, fulfilled_by>,
}
