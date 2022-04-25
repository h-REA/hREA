/**
 * Event query indexes for observation DNA
 *
 * @package Holo-REA
 * @since   2021-08-29
 */
use hdk_semantic_indexes_zome_derive::index_zome;
use hc_zome_rea_economic_event_rpc::*;

// :TODO: remove this; should not be necessary since all these types are imported
// along with their entry_def! in dependent crates
#[hdk_extern]
fn entry_defs(_: ()) -> ExternResult<EntryDefsCallbackResult> {
    Ok(EntryDefsCallbackResult::from(vec![
        PathEntry::entry_def(),
        AgreementAddress::entry_def(),
        EconomicEventAddress::entry_def(),
    ]))
}

#[index_zome]
struct EconomicEvent {
    input_of: Local<process, inputs>,
    output_of: Local<process, outputs>,
    realization_of: Local<agreement, economic_events>,
    satisfies: Local<satisfaction, satisfied_by>,
    fulfills: Local<fulfillment, fulfilled_by>,

    // internal indexes (not part of REA spec)
    affects: Local<economic_resource, affected_by>,
}
