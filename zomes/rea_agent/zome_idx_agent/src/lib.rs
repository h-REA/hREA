/**
 * Agent query indexes for agent DNA
 *
 * @package Holo-REA
 * @since   2022-04-25
 */
use hdk_semantic_indexes_zome_derive::index_zome;
use hc_zome_rea_agent_rpc::*;

#[index_zome]
struct Agent {
    // internal indexes (not part of VF spec)
    provider_of: Remote<economic_event, provider_of>,
    receiver_of: Remote<economic_event, receiver_of>,
    committed_providing: Remote<commitment, provider_of>,
    committed_receiving: Remote<commitment, receiver_of>,
    intended_providing: Remote<intent, provider_of>,
    intended_receiving: Remote<intent, receiver_of>,
}
