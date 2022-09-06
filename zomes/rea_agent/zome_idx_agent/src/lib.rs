/**
 * Agent query indexes for agent DNA
 *
 * @package hREA
 * @since   2022-04-25
 */
use hdk_semantic_indexes_zome_derive::index_zome;
use hc_zome_rea_agent_rpc::*;

#[index_zome]
struct Agent {
    // internal indexes (not part of VF spec)
    // commitments: Remote<commitment, in_scope_of>,
    // intents: Remote<intent, in_scope_of>,
    // economic_events: Remote<economic_event, in_scope_of>,
    // inventoried_economic_resources: Remote<economic_resource, primary_accountable>,
    // plans: Remote<plan, in_scope_of>,
    // processes: Remote<process, in_scope_of>,
    // proposals: Remote<proposal, in_scope_of>,
    commitments_as_provider: Remote<commitment, provider>,
    commitments_as_receiver: Remote<commitment, receiver>,
    intents_as_provider: Remote<intent, provider>,
    intents_as_receiver: Remote<intent, receiver>,
    economic_events_as_provider: Remote<economic_event, provider>,
    economic_events_as_receiver: Remote<economic_event, receiver>,
    inventoried_economic_resources: Remote<economic_resource, primary_accountable>,

    // query agents by type
    agent_type: Local<agent, agent_type_internal>::String,
    // :SHONK: redundant loopback index, required for internals of bidirectional index link management.
    // Aside from better support for such edge-cases, the other benefit to obviating this workaround is DHT bloat.
    agent_type_internal: Local<agent, agent_type>,
}
