/**
 * Handling for `Satisfaction`-related behaviours as they apply to `Intent`s
 */

use hdk::{
    PUBLIC_TOKEN,
    THIS_INSTANCE,
    holochain_json_api::{
        json::JsonString,
        error::JsonError,
    },
    holochain_persistence_api::{
        cas::content::Address,
    },
    error::{ ZomeApiResult, ZomeApiError },
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
    rpc::{
        read_from_zome,
    },
    local_indexes::{
        query_direct_index_with_foreign_key,
        create_direct_index,
    },
};

use vf_planning::type_aliases::{ SatisfactionAddress, IntentAddress, CommitmentAddress };
use vf_planning::identifiers::{
    BRIDGED_OBSERVATION_DHT,
    SATISFACTION_BASE_ENTRY_TYPE,
    SATISFACTION_INITIAL_ENTRY_LINK_TYPE,
    SATISFACTION_ENTRY_TYPE,
    SATISFACTION_SATISFIES_LINK_TYPE, SATISFACTION_SATISFIES_LINK_TAG,
    SATISFACTION_SATISFIEDBY_LINK_TYPE, SATISFACTION_SATISFIEDBY_LINK_TAG,
};
use hc_zome_rea_commitment_storage_consts::{
    COMMITMENT_SATISFIES_LINK_TYPE, COMMITMENT_SATISFIES_LINK_TAG,
};
use hc_zome_rea_intent_storage_consts::{
    INTENT_SATISFIEDBY_LINK_TYPE, INTENT_SATISFIEDBY_LINK_TAG,
};
use vf_planning::satisfaction::{
    Entry,
    CreateRequest,
    FwdCreateRequest,
    UpdateRequest,
    FwdUpdateRequest,
    CheckCommitmentRequest,
    ResponseData as Response,
    construct_response,
};
use hc_zome_rea_commitment_rpc::{
    ResponseData as CommitmentResponse,
};

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    satisfies: Option<IntentAddress>,
    satisfied_by: Option<CommitmentAddress>,
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
    handle_delete_satisfaction(&address)
}

pub fn receive_query_satisfactions(params: QueryParams) -> ZomeApiResult<Vec<Response>> {
    handle_query_satisfactions(&params)
}

fn handle_create_satisfaction(satisfaction: &CreateRequest) -> ZomeApiResult<Response> {
    let (satisfaction_address, entry_resp): (SatisfactionAddress, Entry) = create_record(
        SATISFACTION_BASE_ENTRY_TYPE, SATISFACTION_ENTRY_TYPE,
        SATISFACTION_INITIAL_ENTRY_LINK_TYPE,
        satisfaction.to_owned(),
    )?;

    // link entries in the local DNA
    let _results1 = create_direct_index(
        satisfaction_address.as_ref(),
        satisfaction.get_satisfies().as_ref(),
        SATISFACTION_SATISFIES_LINK_TYPE, SATISFACTION_SATISFIES_LINK_TAG,
        INTENT_SATISFIEDBY_LINK_TYPE, INTENT_SATISFIEDBY_LINK_TAG,
    );

    // link entries which may be local or remote
    // :TODO: Should not have to do this-
    //        One option is that linking to a nonexistent entry should autocreate the base.
    //        This would also make it safe to create things out of order at the expense of validation of external data.
    //        (Alternative: every link has to get a successful pingback from the destination object with its trait signature intact.)
    // :TODO: use of URIs and a Holochain protocol resolver would also make this type of logic entirely unnecessary
    let event_or_commitment = satisfaction.get_satisfied_by();
    let satisfying_commitment: ZomeApiResult<CommitmentResponse> = read_from_zome(
        THIS_INSTANCE,
        "commitment",
        Address::from(PUBLIC_TOKEN.to_string()),    // :TODO:
        "get_commitment",
        CheckCommitmentRequest { address: event_or_commitment.to_owned().into() }.into(),
    );

    match satisfying_commitment {
        // links to local commitment, create link index pair
        Ok(_) => {
            let _results2 = create_direct_index(
                satisfaction_address.as_ref(),
                event_or_commitment.as_ref().into(),
                SATISFACTION_SATISFIEDBY_LINK_TYPE, SATISFACTION_SATISFIEDBY_LINK_TAG,
                COMMITMENT_SATISFIES_LINK_TYPE, COMMITMENT_SATISFIES_LINK_TAG,
            );
        },
        // links to remote event, ping associated foreign DNA
        _ => {
            let _pingback = call(
                BRIDGED_OBSERVATION_DHT,
                "satisfaction",
                Address::from(PUBLIC_TOKEN.to_string()),    // :TODO:
                "satisfaction_created",
                FwdCreateRequest { satisfaction: satisfaction.to_owned() }.into()
            );
        },
    };

    Ok(construct_response(&satisfaction_address, &entry_resp))
}

/// Read an individual satisfaction's details
fn handle_get_satisfaction(base_address: &SatisfactionAddress) -> ZomeApiResult<Response> {
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

fn handle_delete_satisfaction(address: &SatisfactionAddress) -> ZomeApiResult<bool> {
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

fn handle_query_satisfactions(params: &QueryParams) -> ZomeApiResult<Vec<Response>> {
    let mut entries_result: ZomeApiResult<Vec<(SatisfactionAddress, Option<Entry>)>> = Err(ZomeApiError::Internal("No results found".to_string()));

    // :TODO: implement proper AND search rather than exclusive operations
    match &params.satisfies {
        Some(satisfies) => {
            entries_result = query_direct_index_with_foreign_key(satisfies, INTENT_SATISFIEDBY_LINK_TYPE, INTENT_SATISFIEDBY_LINK_TAG);
        },
        _ => (),
    };
    match &params.satisfied_by {
        Some(satisfied_by) => {
            entries_result = query_direct_index_with_foreign_key(satisfied_by, COMMITMENT_SATISFIES_LINK_TYPE, COMMITMENT_SATISFIES_LINK_TAG);
        },
        _ => (),
    };

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
