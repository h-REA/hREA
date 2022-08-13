/**
 * Agreement query indexes for agreement DNA
 *
 * @package Holo-REA
 * @since   2021-09-06
 */
use hdk_semantic_indexes_zome_derive::index_zome;
use hc_zome_rea_agreement_rpc::*;

#[index_zome]
struct Agreement {
    economic_events: Remote<economic_event, realization_of>,
    commitments: Remote<commitment, clause_of>,
}
