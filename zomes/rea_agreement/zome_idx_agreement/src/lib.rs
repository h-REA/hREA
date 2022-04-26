/**
 * Agreement query indexes for agreement DNA
 *
 * @package Holo-REA
 * @since   2021-09-06
 */
use hdk_semantic_indexes_zome_derive::index_zome;
use hc_zome_rea_agreement_rpc::*;

// :TODO: remove this; should not be necessary since all these types are imported
// along with their entry_def! in dependent crates
#[hdk_extern]
fn entry_defs(_: ()) -> ExternResult<EntryDefsCallbackResult> {
    Ok(EntryDefsCallbackResult::from(vec![
        PathEntry::entry_def(),
        AgreementAddress::entry_def(),
        EconomicEventAddress::entry_def(),
        CommitmentAddress::entry_def(),
    ]))
}

#[index_zome]
struct Agreement {
    economic_events: Remote<economic_event, realization_of>,
    commitments: Remote<commitment, clause_of>,
}
