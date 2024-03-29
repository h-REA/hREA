/**
 * Process specification query indexes for observation DNA
 *
 * @package hREA
 * @since   2022-05-22
 */
use hdk_semantic_indexes_zome_derive::index_zome;
use hc_zome_rea_unit_rpc::*;

#[index_zome(record_read_fn_name="__internal_get_unit_by_hash")]
struct Unit {
    // :NOTE: blank means only the `read_all_` and `register_new_` APIs will be generated
}
