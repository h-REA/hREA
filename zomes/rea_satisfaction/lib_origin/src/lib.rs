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
use hdk::prelude::*;
use hdk_records::{
    RecordAPIResult, OtherCellResult, DataIntegrityError,
    records::{
        create_record,
        read_record_entry,
        read_record_entry_by_header,
        update_record,
        delete_record,
    },
    rpc::{
        call_zome_method,
        call_local_zome_method,
    },
    local_indexes::{
        query_index,
    },
    foreign_indexes::{
        create_foreign_index,
        update_foreign_index,
    },
};

use hc_zome_rea_intent_storage_consts::{INTENT_SATISFIEDBY_LINK_TAG};
use hc_zome_rea_commitment_storage_consts::{COMMITMENT_SATISFIES_LINK_TAG};
use hc_zome_rea_commitment_rpc::{ResponseData as CommitmentResponse};
use hc_zome_rea_satisfaction_storage_consts::*;
use hc_zome_rea_satisfaction_storage::*;
use hc_zome_rea_satisfaction_rpc::*;
use hc_zome_rea_satisfaction_lib::construct_response;

pub fn receive_create_satisfaction<S>(entry_def_id: S, intent_entry_def_id: S, commitment_entry_def_id: S, satisfaction: CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    handle_create_satisfaction(entry_def_id, intent_entry_def_id, commitment_entry_def_id, &satisfaction)
}

pub fn receive_get_satisfaction<S>(entry_def_id: S, address: SatisfactionAddress) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    handle_get_satisfaction(entry_def_id, &address)
}

pub fn receive_update_satisfaction<S>(entry_def_id: S, intent_entry_def_id: S, commitment_entry_def_id: S, satisfaction: UpdateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    handle_update_satisfaction(entry_def_id, intent_entry_def_id, commitment_entry_def_id, &satisfaction)
}

pub fn receive_delete_satisfaction<S>(entry_def_id: S, intent_entry_def_id: S, commitment_entry_def_id: S, revision_id: RevisionHash) -> RecordAPIResult<bool>
    where S: AsRef<str>
{
    handle_delete_satisfaction(entry_def_id, intent_entry_def_id, commitment_entry_def_id, &revision_id)
}

fn is_satisfiedby_commitment(event_or_commitment: &EventOrCommitmentAddress) -> OtherCellResult<CommitmentResponse> {
    call_local_zome_method(
        |conf: DnaConfigSlicePlanning| { conf.satisfaction.commitment_zome },
        &CHECK_COMMITMENT_API_METHOD,
        CheckCommitmentRequest { address: event_or_commitment.to_owned().into() },
    )
}

/// Properties accessor for zome config.
fn read_foreign_intent_index_zome(conf: DnaConfigSlicePlanning) -> Option<String> {
    Some(conf.satisfaction.intent_index_zome)
}

/// Properties accessor for zome config.
fn read_foreign_commitment_index_zome(conf: DnaConfigSlicePlanning) -> Option<String> {
    Some(conf.satisfaction.commitment_index_zome)
}

fn handle_create_satisfaction<S>(entry_def_id: S, intent_entry_def_id: S, commitment_entry_def_id: S, satisfaction: &CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let (revision_id, satisfaction_address, entry_resp): (_,_, EntryData) = create_record(&entry_def_id, satisfaction.to_owned())?;

    // link entries in the local DNA
    let _results1 = create_foreign_index(
        read_foreign_intent_index_zome,
        &INTENT_INDEXING_API_METHOD,
        &entry_def_id, &satisfaction_address,
        &intent_entry_def_id, satisfaction.get_satisfies(),
        SATISFACTION_SATISFIES_LINK_TAG, INTENT_SATISFIEDBY_LINK_TAG,
    )?;

    // link entries which may be local or remote
    // :TODO: Should not have to do this-
    //        One option is that linking to a nonexistent entry should autocreate the base.
    //        This would also make it safe to create things out of order at the expense of validation of external data.
    //        (Alternative: every link has to get a successful pingback from the destination object with its trait signature intact.)
    //        Could also probably just run both ops and return OK if one succeeds; overhead would be similar to checking.
    // :TODO: use of URIs and a Holochain protocol resolver would also make this type of logic entirely unnecessary
    // :TODO: actually this will probably fail under the new model since may not have updated values if accessing via WASM
    let event_or_commitment = satisfaction.get_satisfied_by();
    let satisfying_commitment = is_satisfiedby_commitment(event_or_commitment);

    match satisfying_commitment {
        // links to local commitment, create link index pair
        Ok(_) => {
            let _results2 = create_foreign_index(
                read_foreign_commitment_index_zome,
                &COMMITMENT_INDEXING_API_METHOD,
                &entry_def_id, &satisfaction_address,
                &commitment_entry_def_id, event_or_commitment,
                SATISFACTION_SATISFIEDBY_LINK_TAG, COMMITMENT_SATISFIES_LINK_TAG,
            )?;
        },
        // links to remote event, ping associated foreign DNA
        _ => {
            let _pingback: OtherCellResult<ResponseData> = call_zome_method(
                event_or_commitment,
                &REPLICATE_CREATE_API_METHOD,
                FwdCreateRequest { satisfaction: satisfaction.to_owned() }
            );
        },
    };

    construct_response(&satisfaction_address, &revision_id, &entry_resp)
}

/// Read an individual satisfaction's details
fn handle_get_satisfaction<S>(entry_def_id: S, address: &SatisfactionAddress) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let (revision, base_address, entry) = read_record_entry::<EntryData, EntryStorage, _,_>(&entry_def_id, address.as_ref())?;
    construct_response(&base_address, &revision, &entry)
}

fn handle_update_satisfaction<S>(entry_def_id: S, intent_entry_def_id: S, commitment_entry_def_id: S, satisfaction: &UpdateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let (revision_id, base_address, new_entry, prev_entry): (_, SatisfactionAddress, EntryData, EntryData) = update_record(&entry_def_id, &satisfaction.get_revision_id(), satisfaction.to_owned())?;

    // update intent indexes in local DNA
    if new_entry.satisfies != prev_entry.satisfies {
        let _results = update_foreign_index(
            read_foreign_intent_index_zome,
            &INTENT_INDEXING_API_METHOD,
            &entry_def_id, &base_address,
            &intent_entry_def_id,
            vec![new_entry.satisfies.clone()].as_slice(), vec![prev_entry.satisfies].as_slice(),
            SATISFACTION_SATISFIES_LINK_TAG, INTENT_SATISFIEDBY_LINK_TAG,
        )?;
    }

    // update commitment / event indexes in local and/or remote DNA
    if new_entry.satisfied_by != prev_entry.satisfied_by {
        let _results = update_foreign_index(
            read_foreign_commitment_index_zome,
            &COMMITMENT_INDEXING_API_METHOD,
            &entry_def_id, &base_address,
            &commitment_entry_def_id,
            vec![new_entry.satisfied_by.clone()].as_slice(), vec![prev_entry.satisfied_by.clone()].as_slice(),
            SATISFACTION_SATISFIEDBY_LINK_TAG, COMMITMENT_SATISFIES_LINK_TAG,
        )?;

        // update satisfaction records in remote DNA (and by proxy, indexes held there)
        let _pingback: OtherCellResult<ResponseData> = call_zome_method(
            // :TODO: update to intelligently call remote DNAs if new & old target record are not in same network
            &prev_entry.satisfied_by,
            &REPLICATE_UPDATE_API_METHOD,
            UpdateParams { satisfaction: satisfaction.clone() },
        );

        // :TODO: ensure exactly 1 operation succeeded
    }

    construct_response(&base_address, &revision_id, &new_entry)
}

fn handle_delete_satisfaction<S>(entry_def_id: S, intent_entry_def_id: S, commitment_entry_def_id: S, revision_id: &RevisionHash) -> RecordAPIResult<bool>
    where S: AsRef<str>
{
    let (base_address, entry) = read_record_entry_by_header::<EntryData, EntryStorage, _>(&revision_id)?;

    // update intent indexes in local DNA
    let _results = update_foreign_index(
        read_foreign_intent_index_zome,
        &INTENT_INDEXING_API_METHOD,
        &entry_def_id, &base_address,
        &intent_entry_def_id,
        vec![].as_slice(), vec![entry.satisfies].as_slice(),
        SATISFACTION_SATISFIES_LINK_TAG, INTENT_SATISFIEDBY_LINK_TAG,
    )?;

    // :TODO: implement URI resolving logic so as to not have to make this check
    let event_or_commitment = entry.satisfied_by.clone();
    let satisfying_commitment = is_satisfiedby_commitment(&event_or_commitment);

    match satisfying_commitment {
        // links to local commitment, create link index pair
        Ok(_) => {
            let _results2 = update_foreign_index(
                read_foreign_commitment_index_zome,
                &COMMITMENT_INDEXING_API_METHOD,
                &entry_def_id, &base_address,
                &commitment_entry_def_id,
                vec![].as_slice(), vec![entry.satisfied_by].as_slice(),
                SATISFACTION_SATISFIEDBY_LINK_TAG, COMMITMENT_SATISFIES_LINK_TAG,
            )?;
        },
        // links to remote event, ping associated foreign DNA to replicate deletion there
        _ => {
            let _pingback: OtherCellResult<ResponseData> = call_zome_method(
                &event_or_commitment,
                &REPLICATE_DELETE_API_METHOD,
                ByHeader { address: revision_id.to_owned() },
            );
        },
    };

    delete_record::<EntryStorage, _>(&revision_id)
}

const READ_FN_NAME: &str = "get_satisfaction";

pub fn generate_query_handler<S, C, F>(
    foreign_zome_name_from_config: F,
    intent_entry_def_id: S,
    commitment_entry_def_id: S,
) -> impl FnOnce(&QueryParams) -> RecordAPIResult<Vec<ResponseData>>
    where S: AsRef<str>,
        C: std::fmt::Debug,
        SerializedBytes: TryInto<C, Error = SerializedBytesError>,
        F: Fn(C) -> Option<String>,
{
    move |params| {
        let mut entries_result: RecordAPIResult<Vec<RecordAPIResult<ResponseData>>> = Err(DataIntegrityError::EmptyQuery);

        // :TODO: implement proper AND search rather than exclusive operations
        match &params.satisfies {
            Some(satisfies) => {
                entries_result = query_index::<ResponseData, SatisfactionAddress, C,F,_,_,_,_>(
                    &intent_entry_def_id,
                    satisfies, INTENT_SATISFIEDBY_LINK_TAG,
                    &foreign_zome_name_from_config, &READ_FN_NAME,
                );
            },
            _ => (),
        };
        match &params.satisfied_by {
            Some(satisfied_by) => {
                entries_result = query_index::<ResponseData, SatisfactionAddress, C,F,_,_,_,_>(
                    &commitment_entry_def_id,
                    satisfied_by, COMMITMENT_SATISFIES_LINK_TAG,
                    &foreign_zome_name_from_config, &READ_FN_NAME,
                );
            },
            _ => (),
        };

        // :TODO: return errors for UI, rather than filtering
        Ok(entries_result?.iter()
            .cloned()
            .filter_map(Result::ok)
            .collect())
    }
}
