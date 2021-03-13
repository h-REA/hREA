/**
 * Holo-REA satisfaction zome library API
 *
 * Contains helper methods that can be used to manipulate `Satisfaction` data
 * structures in either the local Holochain zome, or a separate DNA-local zome.
 *
 * Contains functionality for the "origin" side of an "indirect remote index" pair
 * (@see `hdk_records` README).
 *
 * @package Holo-REA
 */
use vf_core::type_aliases::SatisfactionAddress;
use hc_zome_rea_satisfaction_storage::Entry;
use hc_zome_rea_satisfaction_rpc::*;

/// Create response from input DHT primitives
pub fn construct_response(address: &SatisfactionAddress, e: &Entry) -> ResponseData {
    ResponseData {
        satisfaction: Response {
            id: address.to_owned().into(),
            satisfied_by: e.satisfied_by.to_owned(),
            satisfies: e.satisfies.to_owned(),
            resource_quantity: e.resource_quantity.to_owned(),
            effort_quantity: e.effort_quantity.to_owned(),
            note: e.note.to_owned(),
        }
    }
}
