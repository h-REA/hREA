/**
 * RecipeProcess query indexes for planning DNA
 *
 * @package hREA
 * @since   2021-08-29
 */
use hdk_semantic_indexes_zome_derive::index_zome;
use hc_zome_rea_recipe_process_rpc::*;

#[index_zome]
struct RecipeProcess {
    satisfied_by: Local<satisfaction, satisfies>,
    input_of: Local<process, intended_inputs>,
    output_of: Local<process, intended_outputs>,
    proposed_in: Remote<proposed_recipe_process, publishes>,

    // internal indexes (not part of VF spec)
    provider: Local<agent, recipe_processs_as_provider>,
    receiver: Local<agent, recipe_processs_as_receiver>,
}
