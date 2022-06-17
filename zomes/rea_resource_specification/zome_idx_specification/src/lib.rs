/**
 * Resource query indexes for observation DNA
 *
 * @package Holo-REA
 * @since   2021-08-29
 */
use hdk_semantic_indexes_zome_derive::index_zome;
use hc_zome_rea_resource_specification_rpc::*;
use hdk_semantic_indexes_zome_lib::ByAddress; // disambiguate from record query struct

// :TODO: remove this; should not be necessary since all these types are imported
// along with their entry_def! in dependent crates
#[hdk_extern]
fn entry_defs(_: ()) -> ExternResult<EntryDefsCallbackResult> {
    Ok(EntryDefsCallbackResult::from(vec![
        PathEntry::entry_def(),
        TimeIndex::entry_def(),
        ResourceSpecificationAddress::entry_def(),
        EconomicResourceAddress::entry_def(),
    ]))
}

#[index_zome]
struct ResourceSpecification {
    conforming_resources: Remote<economic_resource, conforms_to>,
}
