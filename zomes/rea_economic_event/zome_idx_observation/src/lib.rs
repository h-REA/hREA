/**
 * Event query indexes for observation DNA
 *
 * @package Holo-REA
 * @since   2021-08-29
 */
use hdk_semantic_indexes_zome_derive::index_zome;
use hc_zome_rea_economic_event_rpc::*;

#[index_zome]
struct EconomicEvent {
    input_of: Local<process, inputs>,
    output_of: Local<process, outputs>,
    realization_of: Local<agreement, economic_events>,
    satisfies: Remote<satisfaction, satisfied_by>,
    fulfills: Remote<fulfillment, fulfilled_by>,

    // internal indexes (not part of REA spec)
    affects: Local<economic_resource, affected_by>,
}
