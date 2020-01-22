/**
 * Holo-REA fulfillment zome library API
 *
 * Contains helper methods that can be used to manipulate `Fulfillment` data
 * structures in either the local Holochain zome, or a separate DNA-local zome.
 *
 * Contains functionality for the "origin" side of an "indirect remote index" pair
 * (@see `hdk_graph_helpers` README).

 * @package Holo-REA
 */
use hdk::prelude::*;
use hdk::{
    PUBLIC_TOKEN,
    call,
};

use hdk_graph_helpers::{
    records::{
        create_record,
        read_record_entry,
        update_record,
        delete_record,
    },
    local_indexes::{
        create_direct_index,
        query_direct_index_with_foreign_key,
    },
};

use hc_zome_rea_commitment_storage_consts::{COMMITMENT_FULFILLEDBY_LINK_TYPE, COMMITMENT_FULFILLEDBY_LINK_TAG};
use hc_zome_rea_fulfillment_storage_consts::*;
use hc_zome_rea_fulfillment_storage::Entry;
use hc_zome_rea_fulfillment_rpc::*;
use hc_zome_rea_fulfillment_lib::construct_response;

pub fn receive_create_fulfillment(fulfillment: CreateRequest) -> ZomeApiResult<ResponseData> {
    handle_create_fulfillment(&fulfillment)
}

pub fn receive_get_fulfillment(address: FulfillmentAddress) -> ZomeApiResult<ResponseData> {
    handle_get_fulfillment(&address)
}

pub fn receive_update_fulfillment(fulfillment: UpdateRequest) -> ZomeApiResult<ResponseData> {
    handle_update_fulfillment(&fulfillment)
}

pub fn receive_delete_fulfillment(address: FulfillmentAddress) -> ZomeApiResult<bool> {
    handle_delete_fulfillment(&address)
}

pub fn receive_query_fulfillments(params: QueryParams) -> ZomeApiResult<Vec<ResponseData>> {
    handle_query_fulfillments(&params)
}

fn handle_create_fulfillment(fulfillment: &CreateRequest) -> ZomeApiResult<ResponseData> {
    let (fulfillment_address, entry_resp): (FulfillmentAddress, Entry) = create_record(
        FULFILLMENT_BASE_ENTRY_TYPE, FULFILLMENT_ENTRY_TYPE,
        FULFILLMENT_INITIAL_ENTRY_LINK_TYPE,
        fulfillment.to_owned(),
    )?;

    // link entries in the local DNA
    let _results = create_direct_index(
        fulfillment_address.as_ref(),
        fulfillment.get_fulfills().as_ref(),
        FULFILLMENT_FULFILLS_LINK_TYPE, FULFILLMENT_FULFILLS_LINK_TAG,
        COMMITMENT_FULFILLEDBY_LINK_TYPE, COMMITMENT_FULFILLEDBY_LINK_TAG,
    );

    // update in the associated foreign DNA as well
    let _pingback = call(
        BRIDGED_OBSERVATION_DHT,
        "fulfillment",
        Address::from(PUBLIC_TOKEN.to_string()),
        "fulfillment_created",
        FwdCreateRequest { fulfillment: fulfillment.to_owned() }.into()
    );

    Ok(construct_response(&fulfillment_address, &entry_resp))
}

/// Read an individual fulfillment's details
fn handle_get_fulfillment(base_address: &FulfillmentAddress) -> ZomeApiResult<ResponseData> {
    let entry = read_record_entry(base_address)?;
    Ok(construct_response(&base_address, &entry))
}

fn handle_update_fulfillment(fulfillment: &UpdateRequest) -> ZomeApiResult<ResponseData> {
    let base_address = fulfillment.get_id();
    let new_entry = update_record(FULFILLMENT_ENTRY_TYPE, &base_address, fulfillment)?;

    // update in the associated foreign DNA as well
    let _pingback = call(
        BRIDGED_OBSERVATION_DHT,
        "fulfillment",
        Address::from(PUBLIC_TOKEN.to_string()),
        "fulfillment_updated",
        FwdUpdateRequest { fulfillment: fulfillment.clone() }.into()
    );

    Ok(construct_response(base_address, &new_entry))
}

fn handle_delete_fulfillment(address: &FulfillmentAddress) -> ZomeApiResult<bool> {
    let result = delete_record::<Entry>(address);

    // update in the associated foreign DNA as well
    let _pingback = call(
        BRIDGED_OBSERVATION_DHT,
        "fulfillment",
        Address::from(PUBLIC_TOKEN.to_string()),
        "fulfillment_deleted",
        address.into(),
    );

    result
}

fn handle_query_fulfillments(params: &QueryParams) -> ZomeApiResult<Vec<ResponseData>> {
    let mut entries_result: ZomeApiResult<Vec<(FulfillmentAddress, Option<Entry>)>> = Err(ZomeApiError::Internal("No results found".to_string()));

    // :TODO: proper search logic, not mutually exclusive ID filters
    match &params.fulfills {
        Some(fulfills) => {
            entries_result = query_direct_index_with_foreign_key(fulfills, COMMITMENT_FULFILLEDBY_LINK_TYPE, COMMITMENT_FULFILLEDBY_LINK_TAG);
        },
        _ => (),
    };
    // :TODO: observation DNA handles this. Should queries be possible in planning DNA, too?
    // match &params.fulfilled_by {
    //     Some(fulfilled_by) => {
    //         entries_result = query_direct_index_with_foreign_key(fulfilled_by, EVENT_FULFILLS_LINK_TYPE, EVENT_FULFILLS_LINK_TAG);
    //     },
    //     _ => (),
    // };

    match entries_result {
        Ok(entries) => Ok(
            entries.iter()
                .map(|(entry_base_address, maybe_entry)| {
                    // :TODO: avoid cloning entry
                    match maybe_entry {
                        Some(entry) => Ok(construct_response(entry_base_address, entry)),
                        None => Err(ZomeApiError::Internal("referenced entry not found".to_string()))
                    }
                })
                .filter_map(Result::ok)
                .collect()
        ),
        _ => Err(ZomeApiError::Internal("could not load linked addresses".to_string()))
    }
}
