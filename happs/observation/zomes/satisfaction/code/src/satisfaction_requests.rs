/**
 * Handling for `Fulfillment` related behaviours as they relate to `EconomicEvent`s
 */

use hdk::{
    // PUBLIC_TOKEN,
    holochain_json_api::{json::JsonString, error::JsonError},
    error::ZomeApiResult,
    error::ZomeApiError,
    // call,
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

use vf_planning::type_aliases::{
    EventAddress,
    SatisfactionAddress,
};
use vf_observation::identifiers::{
    // BRIDGED_PLANNING_DHT,
    EVENT_SATISFIES_LINK_TYPE,
    EVENT_SATISFIES_LINK_TAG,
};
use vf_planning::identifiers::{
    SATISFACTION_BASE_ENTRY_TYPE,
    SATISFACTION_INITIAL_ENTRY_LINK_TYPE,
    SATISFACTION_ENTRY_TYPE,
    SATISFACTION_SATISFIEDBY_LINK_TYPE,
    SATISFACTION_SATISFIEDBY_LINK_TAG,
};

use vf_planning::satisfaction::{
    Entry,
    CreateRequest,
    UpdateRequest,
    ResponseData as Response,
    construct_response,
};

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    satisfied_by: Option<EventAddress>,
}

pub fn receive_create_satisfaction(satisfaction: CreateRequest) -> ZomeApiResult<Response> {
    handle_create_satisfaction(&satisfaction)
}

pub fn receive_get_satisfaction(address: SatisfactionAddress) -> ZomeApiResult<Response> {
    handle_get_satisfaction(&address)
}

pub fn receive_update_satisfaction(satisfaction: UpdateRequest) -> ZomeApiResult<Response> {
    handle_update_satisfaction(&satisfaction)
}

pub fn receive_delete_satisfaction(address: SatisfactionAddress) -> ZomeApiResult<bool> {
    delete_record::<Entry>(&address)
}

pub fn receive_query_satisfactions(params: QueryParams) -> ZomeApiResult<Vec<Response>> {
    handle_query_satisfactions(&params)
}

fn handle_create_satisfaction(satisfaction: &CreateRequest) -> ZomeApiResult<Response> {
    let (satisfaction_address, entry_resp): (SatisfactionAddress, Entry) = create_record(
        SATISFACTION_BASE_ENTRY_TYPE, SATISFACTION_ENTRY_TYPE,
        SATISFACTION_INITIAL_ENTRY_LINK_TYPE,
        satisfaction.to_owned()
    )?;

    // link entries in the local DNA
    let _results = link_entries_bidir(
        satisfaction_address.as_ref(),
        satisfaction.get_satisfied_by().as_ref(),
        SATISFACTION_SATISFIEDBY_LINK_TYPE, SATISFACTION_SATISFIEDBY_LINK_TAG,
        EVENT_SATISFIES_LINK_TYPE, EVENT_SATISFIES_LINK_TAG,
    );

    // register in the associated foreign DNA as well
    // :TODO: probably need to remove this and rethink to use a message broadcast / respond flow
    // let _pingback = call(
    //     BRIDGED_PLANNING_DHT,
    //     "fulfillment",
    //     Address::from(PUBLIC_TOKEN.to_string()),
    //     "fulfillment_created",
    //     fulfillment.into(),
    // );

    Ok(construct_response(&satisfaction_address, &entry_resp))
}

fn handle_update_satisfaction(satisfaction: &UpdateRequest) -> ZomeApiResult<Response> {
    let base_address = satisfaction.get_id();
    let new_entry = update_record(SATISFACTION_ENTRY_TYPE, &base_address, satisfaction)?;
    Ok(construct_response(&base_address, &new_entry))
}

/// Read an individual fulfillment's details
fn handle_get_satisfaction(base_address: &SatisfactionAddress) -> ZomeApiResult<Response> {
    let entry = read_record_entry(base_address)?;
    Ok(construct_response(&base_address, &entry))
}

fn handle_query_satisfactions(params: &QueryParams) -> ZomeApiResult<Vec<Response>> {
    let mut entries_result: ZomeApiResult<Vec<(SatisfactionAddress, Option<Entry>)>> = Err(ZomeApiError::Internal("No results found".to_string()));

    match &params.satisfied_by {
        Some(satisfied_by) => {
            entries_result = get_links_and_load_entry_data(
                satisfied_by, EVENT_SATISFIES_LINK_TYPE, EVENT_SATISFIES_LINK_TAG,
            );
        },
        _ => (),
    };

    match entries_result {
        Ok(entries) => Ok(
            entries.iter()
                .map(|(entry_base_address, maybe_entry)| {
                    // :TODO: avoid cloning entry
                    match maybe_entry {
                        Some(entry) => Ok(construct_response(&entry_base_address, &entry)),
                        None => Err(ZomeApiError::Internal("referenced entry not found".to_string()))
                    }
                })
                .filter_map(Result::ok)
                .collect()
        ),
        _ => Err(ZomeApiError::Internal("could not load linked addresses".to_string()))
    }
}
