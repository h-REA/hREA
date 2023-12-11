/**
 * Process specification query indexes for observation DNA
 *
 * @package hREA
 * @since   2023-12-11
 */
use hdk_semantic_indexes_zome_derive::index_zome;
use hc_zome_rea_recipe_flow_rpc::*;

#[index_zome]
struct RecipeFlow {
    resource_conforms_to: Remote<resource_specification, resource_conforms_to>,
    stage: Remote<process_specification, process_conforms_to>,
    recipe_input_of: Local<recipe_process, recipe_input_of>,
    recipe_output_of: Local<recipe_process, recipe_output_of>,
}