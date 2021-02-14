/**
 * Holo-REA economic event index zome API definition
 *
 * Provides remote indexing capability for the economic events of agreement records.
 *
 * @package Holo-REA
 */

use hdk3::prelude::*;

use hdk_graph_helpers::{
    remote_indexes::{
        RemoteEntryLinkResponse,
        sync_remote_index,
    },
};

use vf_core::type_aliases::{ AgreementAddress, EventAddress };
use hc_zome_rea_agreement_storage_consts::{ AGREEMENT_ENTRY_TYPE, AGREEMENT_EVENTS_LINK_TAG };
use hc_zome_rea_economic_event_storage_consts::{ EVENT_ENTRY_TYPE, EVENT_REALIZATION_OF_LINK_TAG };

entry_defs![ Path::entry_def() ];

#[hdk_extern]
fn index_events(base_entry: EventAddress, target_entries: Vec<AgreementAddress>, removed_entries: Vec<AgreementAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    Ok(sync_remote_index(
        &EVENT_ENTRY_TYPE, &base_entry,
        &AGREEMENT_ENTRY_TYPE,
        target_entries.iter().map(|e| *e.as_ref()).collect::<Vec<EntryHash>>().as_slice(),
        removed_entries.iter().map(|e| *e.as_ref()).collect::<Vec<EntryHash>>().as_slice(),
        &AGREEMENT_EVENTS_LINK_TAG. &EVENT_REALIZATION_OF_LINK_TAG,
    )?)
}
