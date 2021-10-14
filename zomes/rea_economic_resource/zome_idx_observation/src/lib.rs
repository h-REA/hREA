/**
 * Resource query indexes for observation DNA
 *
 * @package Holo-REA
 * @since   2021-08-29
 */
use hdk_semantic_indexes_zome_derive::index_zome;
use hc_zome_rea_economic_resource_rpc::*;
use hc_zome_rea_economic_event_rpc::ResourceResponseData as ResponseData;

#[index_zome]
struct EconomicResource {
    contains: Local<economic_resource, contained_in>,
    contained_in: Local<economic_resource, contains>,
    conforms_to: Remote<resource_specification, conforming_resources>,

    // internal indexes (not part of REA spec)
    affected_by: Local<economic_event, affects>,
}
