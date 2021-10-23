/**
 * satisfaction query indexes for observation DNA
 *
 * @package Holo-REA
 * @since   2021-08-29
 */
use hdk_semantic_indexes_zome_derive::index_zome;
use hc_zome_rea_satisfaction_rpc::*;
use hdk_semantic_indexes_zome_lib::ByAddress; // disambiguate from RPC query struct

#[index_zome]
struct Satisfaction {
    // :NOTE: this gets updated by shadowed local record storage zome, not the remote one in Planning DNA
    satisfied_by: Local<economic_event, satisfies>,
}
