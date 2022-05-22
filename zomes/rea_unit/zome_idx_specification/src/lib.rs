/**
 * Process specification query indexes for observation DNA
 *
 * @package Holo-REA
 * @since   2022-05-22
 */
use hdk_semantic_indexes_zome_derive::index_zome;
use hc_zome_rea_unit_rpc::*;
use hdk_semantic_indexes_zome_lib::ByAddress; // disambiguate from record query struct

#[index_zome]
struct Unit {
    // :NOTE: blank means only the `read_all_` and `register_new_` APIs will be generated
}
