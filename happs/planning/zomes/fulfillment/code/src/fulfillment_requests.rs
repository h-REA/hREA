/**
 * Handling for `Fulfillment` related behaviours as they relate to `Commitment`s
 */

use hdk::{
    PUBLIC_TOKEN,
    holochain_json_api::{ json::JsonString, error::JsonError },
    holochain_persistence_api::cas::content::Address,
    error::ZomeApiResult,
    error::ZomeApiError,
    call,
};
use holochain_json_derive::{ DefaultJson };
use hdk_graph_helpers::{
    records::{
        create_record,
        read_record_entry,
        update_record,
        delete_record,
    },
    links::{
        link_entries_bidir,
        get_links_and_load_entry_data,
    },
};

use vf_planning::type_aliases::{ FulfillmentAddress, CommitmentAddress, EventAddress };
// use vf_observation::identifiers::{
//     EVENT_FULFILLS_LINK_TYPE,
//     EVENT_FULFILLS_LINK_TAG,
// };
use vf_planning::identifiers::{
    BRIDGED_OBSERVATION_DHT,
    FULFILLMENT_BASE_ENTRY_TYPE,
    FULFILLMENT_INITIAL_ENTRY_LINK_TYPE,
    FULFILLMENT_ENTRY_TYPE,
    FULFILLMENT_FULFILLS_LINK_TYPE,
    FULFILLMENT_FULFILLS_LINK_TAG,
    COMMITMENT_FULFILLEDBY_LINK_TYPE,
    COMMITMENT_FULFILLEDBY_LINK_TAG,
};
use vf_planning::fulfillment::{
    Entry,
    CreateRequest,
    FwdCreateRequest,
    UpdateRequest,
    FwdUpdateRequest,
    ResponseData as Response,
    construct_response,
};

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    fulfills: Option<CommitmentAddress>,
    fulfilled_by: Option<EventAddress>,
}

pub fn receive_create_fulfillment(fulfillment: CreateRequest) -> ZomeApiResult<Response> {
    handle_create_fulfillment(&fulfillment)
}

pub fn receive_get_fulfillment(address: FulfillmentAddress) -> ZomeApiResult<Response> {
    handle_get_fulfillment(&address)
}

pub fn receive_update_fulfillment(fulfillment: UpdateRequest) -> ZomeApiResult<Response> {
    handle_update_fulfillment(&fulfillment)
}

pub fn receive_delete_fulfillment(address: FulfillmentAddress) -> ZomeApiResult<bool> {
    handle_delete_fulfillment(&address)
}

pub fn receive_query_fulfillments(params: QueryParams) -> ZomeApiResult<Vec<Response>> {
    handle_query_fulfillments(&params)
}

fn handle_create_fulfillment(fulfillment: &CreateRequest) -> ZomeApiResult<Response> {
    let (fulfillment_address, entry_resp): (FulfillmentAddress, Entry) = create_record(
        FULFILLMENT_BASE_ENTRY_TYPE, FULFILLMENT_ENTRY_TYPE,
        FULFILLMENT_INITIAL_ENTRY_LINK_TYPE,
        fulfillment.to_owned(),
    )?;

    // link entries in the local DNA
    let _results = link_entries_bidir(
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
fn handle_get_fulfillment(base_address: &FulfillmentAddress) -> ZomeApiResult<Response> {
    let entry = read_record_entry(base_address)?;
    Ok(construct_response(&base_address, &entry))
}

fn handle_update_fulfillment(fulfillment: &UpdateRequest) -> ZomeApiResult<Response> {
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

fn handle_query_fulfillments(params: &QueryParams) -> ZomeApiResult<Vec<Response>> {
    let mut entries_result: ZomeApiResult<Vec<(FulfillmentAddress, Option<Entry>)>> = Err(ZomeApiError::Internal("No results found".to_string()));

    // :TODO: proper search logic, not mutually exclusive ID filters
    match &params.fulfills {
        Some(fulfills) => {
            entries_result = get_links_and_load_entry_data(fulfills, COMMITMENT_FULFILLEDBY_LINK_TYPE, COMMITMENT_FULFILLEDBY_LINK_TAG);
        },
        _ => (),
    };
    // :TODO: observation DNA handles this. Should queries be possible in planning DNA, too?
    // match &params.fulfilled_by {
    //     Some(fulfilled_by) => {
    //         entries_result = get_links_and_load_entry_data(fulfilled_by, EVENT_FULFILLS_LINK_TYPE, EVENT_FULFILLS_LINK_TAG);
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
