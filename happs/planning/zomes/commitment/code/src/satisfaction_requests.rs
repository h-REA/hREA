/**
 * Handling for `Satisfaction`-related behaviours as they apply to `Commitment`s
 */

use hdk::{
    holochain_core_types::{
        cas::content::Address,
    },
};

use hdk_graph_helpers::{
    link_entries_bidir,
};

pub const INTENT_BASE_ENTRY_TYPE: &str = "vf_intent_baseurl";
pub const COMMITMENT_SATISFIES_LINK_TYPE: &str = "vf_commitment_satisfies";
pub const COMMITMENT_SATISFIES_LINK_TAG: &str = "satisfies";
pub const INTENT_SATISFIEDBY_LINK_TYPE: &str = "vf_intent_satisfied_by";
pub const INTENT_SATISFIEDBY_LINK_TAG: &str = "satisfied_by";

pub fn link_satisfactions(base_address: &Address, targets: &Vec<Address>) -> Vec<Address> {
    // link all referenced satisfactions to the commitment
    let commitment_results = targets.iter()
        .flat_map(|target_address| {
            // link event to commitment by `fulfilled`/`fulfilledBy` edge
            link_entries_bidir(
                &base_address, &target_address,
                COMMITMENT_SATISFIES_LINK_TYPE, COMMITMENT_SATISFIES_LINK_TAG,
                INTENT_SATISFIEDBY_LINK_TYPE, INTENT_SATISFIEDBY_LINK_TAG
            )
        })
        .collect();

    commitment_results
}
