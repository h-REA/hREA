/**
 * Process specification query indexes for observation DNA
 *
 * @package hREA
 * @since   2023-12-11
 */
use hdk_semantic_indexes_zome_derive::index_zome;
use hc_zome_rea_recipe_process_rpc::*;

#[index_zome]
struct RecipeProcess {
    process_conforms_to: Remote<process_specification, process_conforms_to>,
}