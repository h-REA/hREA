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
    input_of: Local<process, observed_inputs>,
    output_of: Local<process, observed_outputs>,
    realization_of: Local<agreement, economic_events>,
    satisfies: Local<satisfaction, satisfied_by>,
    fulfills: Local<fulfillment, fulfilled_by>,

    // internal indexes (not part of REA spec)
    affects: Local<economic_resource, affected_by>,
    provider: Local<agent, economic_events_as_provider>,
    receiver: Local<agent, economic_events_as_receiver>,
}
