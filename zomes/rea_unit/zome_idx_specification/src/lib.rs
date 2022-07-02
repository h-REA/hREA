/**
 * Process specification query indexes for observation DNA
 *
 * @package Holo-REA
 * @since   2022-05-22
 */
use hdk_semantic_indexes_zome_derive::index_zome;
use hc_zome_rea_unit_rpc::*;

// :TODO: remove this; should not be necessary since all these types are imported
// along with their entry_def! in dependent crates
#[hdk_extern]
fn entry_defs(_: ()) -> ExternResult<EntryDefsCallbackResult> {
    Ok(EntryDefsCallbackResult::from(vec![
        PathEntry::entry_def(),
        TimeIndex::entry_def(),
        UnitAddress::entry_def(),
    ]))
}

#[index_zome]
struct Unit {
    // :NOTE: blank means only the `read_all_` and `register_new_` APIs will be generated
}
