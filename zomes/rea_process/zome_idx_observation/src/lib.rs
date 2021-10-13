/**
 * Process query indexes for observation DNA
 *
 * @package Holo-REA
 * @since   2021-08-26
 */
use hdk_semantic_indexes_zome_derive::index_zome;
use hc_zome_rea_process_rpc::*;

#[index_zome(query_fn_name="query_processes")]
struct Process {
    inputs: Local<economic_event, input_of>,
    outputs: Local<economic_event, output_of>,
    committed_inputs: Remote<commitment, input_of>,
    committed_outputs: Remote<commitment, output_of>,
    intended_inputs: Remote<intent, input_of>,
    intended_outputs: Remote<intent, output_of>
}
