/**
 * Intent query indexes for planning DNA
 *
 * @package hREA
 * @since   2021-08-29
 */
use hdk_semantic_indexes_zome_derive::index_zome;
use hc_zome_rea_intent_rpc::*;

#[index_zome]
struct Intent {
    satisfied_by: Local<satisfaction, satisfies>,
    input_of: Local<process, intended_inputs>,
    output_of: Local<process, intended_outputs>,
    proposed_in: Remote<proposed_intent, publishes>,

    // internal indexes (not part of VF spec)
    provider: Local<agent, intents_as_provider>,
    receiver: Local<agent, intents_as_receiver>,
}
