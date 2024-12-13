/**
 * Process specification query indexes for observation DNA
 *
 * @package hREA
 * @since   2023-12-11
 */
use hdk_semantic_indexes_zome_derive::index_zome;
use hc_zome_rea_recipe_process_rpc::*;
use hc_zome_rea_recipe_flow_rpc::{
    RecipeFlowAddress,
};

#[index_zome]
struct RecipeProcess {
    process_conforms_to: Local<process_specification, process_conforms_to>,
    recipe_inputs: Local<recipe_flow, recipe_input_of>,
    recipe_outputs: Local<recipe_flow, recipe_output_of>,
}