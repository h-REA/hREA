/**
 * Holo-REA agreement index zome API definition
 *
 * Provides remote indexing capability for the Agreements of economic event records.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use hc_zome_rea_economic_event_storage_consts::{ EVENT_BASE_ENTRY_TYPE, EVENT_REALIZATION_OF_LINK_TYPE };
use hc_zome_rea_agreement_storage_consts::{ AGREEMENT_BASE_ENTRY_TYPE, AGREEMENT_EVENTS_LINK_TYPE };

entry_defs![ Path::entry_def() ];

// :TODO:
// Query EconomicEvents by event.realizationOf
// Will require some thinking as to how to compose query functionality between pluggable zomes.
// In the naive case we would handle event.realizationOf in this zome, commitment.clauseOf in a
// separate "hc_zome_rea_agreement_index_planning" zome. Though it makes composability simpler,
// this decomposition of indexes into separate zomes prevents any smart set-intersection logic
// from being able to efficiently handle compound queries across these fields.
//
// Suggest some pattern for a pluggable "indexing" zome or DNA with further configuration that
// can be used across various underlying `hdk::Path` indexing structures for combinatorial search.
