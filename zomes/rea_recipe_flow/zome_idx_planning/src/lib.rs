/**
 * RecipeFlow query indexes for planning DNA
 *
 * @package hREA
 * @since   2021-08-29
 */
use hdk_semantic_indexes_zome_derive::index_zome;
use hc_zome_rea_recipe_flow_rpc::*;

#[index_zome]
struct RecipeFlow {
    satisfied_by: Local<satisfaction, satisfies>,
    input_of: Local<process, intended_inputs>,
    output_of: Local<process, intended_outputs>,
    proposed_in: Remote<proposed_recipe_flow, publishes>,

    // internal indexes (not part of VF spec)
    provider: Local<agent, recipe_flows_as_provider>,
    receiver: Local<agent, recipe_flows_as_receiver>,
}
