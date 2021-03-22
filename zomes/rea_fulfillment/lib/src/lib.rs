/**
 * Holo-REA fulfillment zome library API
 *
 * Contains helper methods that can be used to manipulate `Fulfillment` data
 * structures in either the local Holochain zome, or a separate DNA-local zome.
 *
 * Contains functionality for the "origin" side of an "indirect remote index" pair
 * (@see `hdk_records` README).
 *
 * @package Holo-REA
 */
use vf_attributes_hdk::FulfillmentAddress;
use hc_zome_rea_fulfillment_storage::Entry;
use hc_zome_rea_fulfillment_rpc::*;

/// Create response from input DHT primitives
pub fn construct_response(address: &FulfillmentAddress, e: &Entry) -> ResponseData {
    ResponseData {
        fulfillment: Response {
            id: address.to_owned(),
            fulfilled_by: e.fulfilled_by.to_owned(),
            fulfills: e.fulfills.to_owned(),
            resource_quantity: e.resource_quantity.to_owned(),
            effort_quantity: e.effort_quantity.to_owned(),
            note: e.note.to_owned(),
        }
    }
}
