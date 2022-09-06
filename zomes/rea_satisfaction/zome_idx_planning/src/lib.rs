/**
 * satisfaction query indexes for planning DNA
 *
 * @package hREA
 * @since   2021-08-29
 */
use hdk_semantic_indexes_zome_derive::index_zome;
use hc_zome_rea_satisfaction_rpc::*;
use hdk_semantic_indexes_zome_lib::ByAddress; // disambiguate from RPC query struct

#[index_zome]
struct Satisfaction {
    satisfies: Local<intent, satisfied_by>,
    satisfied_by: Local<commitment, satisfies>,
}
