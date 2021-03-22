#![feature(proc_macro_hygiene)]
/**
 * Holo-REA agreement index zome API definition
 *
 * Provides remote indexing capability for the Agreements of Commitment records.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use hc_zome_rea_commitment_storage_consts::{ COMMITMENT_BASE_ENTRY_TYPE, COMMITMENT_CLAUSE_OF_LINK_TYPE };
use hc_zome_rea_agreement_storage_consts::{ AGREEMENT_BASE_ENTRY_TYPE, AGREEMENT_COMMITMENTS_LINK_TYPE };

entry_defs![ Path::entry_def() ];

// :TODO: decide if these types of indexing zomes are necessary for exposing any API endpoints
// @see ../zome_idx_observation for more
