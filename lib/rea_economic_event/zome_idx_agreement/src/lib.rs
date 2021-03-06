/**
 * Holo-REA economic event index zome API definition
 *
 * Provides remote indexing capability for the economic events of agreement records.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use hdk_graph_helpers::{
    remote_indexes::{
        RemoteEntryLinkRequest,
        RemoteEntryLinkResponse,
        sync_remote_index,
    },
};

use hc_zome_rea_agreement_storage_consts::{ AGREEMENT_ENTRY_TYPE, AGREEMENT_EVENTS_LINK_TAG };
use hc_zome_rea_economic_event_storage_consts::{ EVENT_ENTRY_TYPE, EVENT_REALIZATION_OF_LINK_TAG };

entry_defs![ Path::entry_def() ];

#[hdk_extern]
fn index_events(indexes: RemoteEntryLinkRequest) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_remote_index(
        &EVENT_ENTRY_TYPE, &remote_entry,
        &AGREEMENT_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        &AGREEMENT_EVENTS_LINK_TAG, &EVENT_REALIZATION_OF_LINK_TAG,
    )?)
}
