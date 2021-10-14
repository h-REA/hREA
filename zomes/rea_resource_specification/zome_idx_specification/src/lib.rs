/**
 * Resource query indexes for observation DNA
 *
 * @package Holo-REA
 * @since   2021-08-29
 */
use hdk_semantic_indexes_zome_derive::index_zome;
use hc_zome_rea_resource_specification_rpc::*;
use hdk_semantic_indexes_zome_lib::ByAddress; // disambiguate from record query struct

#[index_zome]
struct ResourceSpecification {
    conforming_resources: Local<economic_resource, conforms_to>,
}
