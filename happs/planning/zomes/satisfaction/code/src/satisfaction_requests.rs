/**
 * Handling for `Satisfaction`-related behaviours as they apply to `Intent`s
 */

use hdk::{
    PUBLIC_TOKEN,
    holochain_persistence_api::{
        cas::content::Address,
    },
    error::{ ZomeApiResult, ZomeApiError },
    call,
};
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

use vf_planning::identifiers::{
    BRIDGED_OBSERVATION_DHT,
    SATISFACTION_BASE_ENTRY_TYPE,
    SATISFACTION_INITIAL_ENTRY_LINK_TYPE,
    SATISFACTION_ENTRY_TYPE,
    SATISFACTION_SATISFIES_LINK_TYPE, SATISFACTION_SATISFIES_LINK_TAG,
    SATISFACTION_SATISFIEDBY_LINK_TYPE, SATISFACTION_SATISFIEDBY_LINK_TAG,
    INTENT_SATISFIEDBY_LINK_TYPE, INTENT_SATISFIEDBY_LINK_TAG,
    COMMITMENT_SATISFIES_LINK_TYPE, COMMITMENT_SATISFIES_LINK_TAG,
};

use vf_planning::satisfaction::{
    Entry,
    CreateRequest,
    FwdCreateRequest,
    UpdateRequest,
    FwdUpdateRequest,
    ResponseData as Response,
    construct_response,
};

pub fn receive_create_satisfaction(satisfaction: CreateRequest) -> ZomeApiResult<Response> {
    handle_create_satisfaction(&satisfaction)
}

pub fn receive_get_satisfaction(address: Address) -> ZomeApiResult<Response> {
    handle_get_satisfaction(&address)
}

pub fn receive_update_satisfaction(satisfaction: UpdateRequest) -> ZomeApiResult<Response> {
    handle_update_satisfaction(&satisfaction)
}

pub fn receive_delete_satisfaction(address: Address) -> ZomeApiResult<bool> {
    handle_delete_satisfaction(&address)
}

pub fn receive_query_satisfactions(economic_event: Address) -> ZomeApiResult<Vec<Response>> {
    handle_query_satisfactions(&economic_event)
}

fn handle_create_satisfaction(satisfaction: &CreateRequest) -> ZomeApiResult<Response> {
    let (satisfaction_address, entry_resp): (Address, Entry) = create_record(
        SATISFACTION_BASE_ENTRY_TYPE, SATISFACTION_ENTRY_TYPE,
        SATISFACTION_INITIAL_ENTRY_LINK_TYPE,
        satisfaction.to_owned(),
    )?;

    // link entries in the local DNA
    let _results = link_entries_bidir(
        &satisfaction_address,
        satisfaction.get_satisfies().as_ref(),
        SATISFACTION_SATISFIES_LINK_TYPE, SATISFACTION_SATISFIES_LINK_TAG,
        INTENT_SATISFIEDBY_LINK_TYPE, INTENT_SATISFIEDBY_LINK_TAG,
    );
    let _results = link_entries_bidir(
        &satisfaction_address,
        satisfaction.get_satisfied_by().as_ref(),
        SATISFACTION_SATISFIEDBY_LINK_TYPE, SATISFACTION_SATISFIEDBY_LINK_TAG,
        COMMITMENT_SATISFIES_LINK_TYPE, COMMITMENT_SATISFIES_LINK_TAG,
    );

    // update in the associated foreign DNA as well, in case it's an event being linked
    let _pingback = call(
        BRIDGED_OBSERVATION_DHT,
        "satisfaction",
        Address::from(PUBLIC_TOKEN.to_string()),
        "satisfaction_created",
        FwdCreateRequest { satisfaction: satisfaction.to_owned() }.into()
    );

    Ok(construct_response(&satisfaction_address, &entry_resp))
}

/// Read an individual satisfaction's details
fn handle_get_satisfaction(base_address: &Address) -> ZomeApiResult<Response> {
    let entry = read_record_entry(base_address)?;
    Ok(construct_response(&base_address, &entry))
}

fn handle_update_satisfaction(satisfaction: &UpdateRequest) -> ZomeApiResult<Response> {
    let base_address = satisfaction.get_id();
    let new_entry = update_record(SATISFACTION_ENTRY_TYPE, &base_address, satisfaction)?;

    // update in the associated foreign DNA as well
    let _pingback = call(
        BRIDGED_OBSERVATION_DHT,
        "satisfaction",
        Address::from(PUBLIC_TOKEN.to_string()),
        "satisfaction_updated",
        FwdUpdateRequest { satisfaction: satisfaction.clone() }.into()
    );

    Ok(construct_response(base_address, &new_entry))
}

fn handle_delete_satisfaction(address: &Address) -> ZomeApiResult<bool> {
    let result = delete_record::<Entry>(address);

    // update in the associated foreign DNA as well
    let _pingback = call(
        BRIDGED_OBSERVATION_DHT,
        "satisfaction",
        Address::from(PUBLIC_TOKEN.to_string()),
        "satisfaction_deleted",
        address.into(),
    );

    result
}

// :TODO: implement satisfied_by filter
fn handle_query_satisfactions(satisfies: &Address) -> ZomeApiResult<Vec<Response>> {
    let entries_result: ZomeApiResult<Vec<(Address, Option<Entry>)>> = get_links_and_load_entry_data(
        satisfies,
        SATISFACTION_SATISFIEDBY_LINK_TYPE,
        SATISFACTION_SATISFIEDBY_LINK_TAG
    );

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
