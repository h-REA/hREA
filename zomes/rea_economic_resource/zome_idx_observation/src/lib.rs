/**
 * Resource query indexes for observation DNA
 *
 * @package Holo-REA
 * @since   2021-08-29
 */
use hdk_semantic_indexes_zome_derive::index_zome;
use hc_zome_rea_economic_resource_rpc::*;
use hc_zome_rea_economic_event_rpc::{
    ResourceResponse as Response,
    ResourceResponseData as ResponseData,
};

// :TODO: remove this; should not be necessary since all these types are imported
// along with their entry_def! in dependent crates
#[hdk_extern]
fn entry_defs(_: ()) -> ExternResult<EntryDefsCallbackResult> {
    Ok(EntryDefsCallbackResult::from(vec![
        PathEntry::entry_def(),
        TimeIndex::entry_def(),
        EconomicResourceAddress::entry_def(),
        ResourceSpecificationAddress::entry_def(),
        EconomicEventAddress::entry_def(),
    ]))
}

#[index_zome]
struct EconomicResource {
    contains: Local<economic_resource, contained_in>,
    contained_in: Local<economic_resource, contains>,
    conforms_to: Local<resource_specification, conforming_resources>,

    // internal indexes (not part of REA spec)
    affected_by: Local<economic_event, affects>,
}
