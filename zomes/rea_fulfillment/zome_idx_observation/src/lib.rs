/**
 * Fulfillment query indexes for observation DNA
 *
 * @package Holo-REA
 * @since   2021-08-29
 */
use hdk::prelude::*;

use hdk_semantic_indexes_zome_lib::{
    IndexingZomeConfig, RecordAPIResult, DataIntegrityError,
    RemoteEntryLinkRequest,
    RemoteEntryLinkResponse,
    query_index,
    sync_index,
};

use hc_zome_rea_fulfillment_rpc::*;
use hc_zome_rea_fulfillment_storage_consts::*;
use hc_zome_rea_economic_event_storage_consts::{ EVENT_ENTRY_TYPE, EVENT_FULFILLS_LINK_TAG };

entry_defs![Path::entry_def()];

// :TODO: obviate this with zome-specific configs
#[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
pub struct DnaConfigSlice {
    pub fulfillment_index: IndexingZomeConfig,
}

fn read_index_target_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.fulfillment_index.record_storage_zome)
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchInputs {
    pub params: QueryParams,
}

const READ_FN_NAME: &str = "get_fulfillment";

#[hdk_extern]
fn query_fulfillments(SearchInputs { params }: SearchInputs) -> ExternResult<Vec<ResponseData>>
{
    let mut entries_result: RecordAPIResult<Vec<RecordAPIResult<ResponseData>>> = Err(DataIntegrityError::EmptyQuery);

    match &params.fulfilled_by {
        Some(fulfilled_by) => {
            entries_result = query_index::<ResponseData, FulfillmentAddress, _,_,_,_,_,_>(
                &EVENT_ENTRY_TYPE,
                fulfilled_by, EVENT_FULFILLS_LINK_TAG,
                &read_index_target_zome, &READ_FN_NAME,
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

#[hdk_extern]
fn _internal_reindex_events(indexes: RemoteEntryLinkRequest<EconomicEventAddress, FulfillmentAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_index(
        &EVENT_ENTRY_TYPE, &remote_entry,
        &FULFILLMENT_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        &EVENT_FULFILLS_LINK_TAG, &FULFILLMENT_FULFILLEDBY_LINK_TAG,
    )?)
}
