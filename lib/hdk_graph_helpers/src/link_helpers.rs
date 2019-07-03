/**
 * Link-handling abstractions for Holochain apps
 *
 * Handles common behaviours for linking between data in different hApps,
 * in a way that is predictable, semantically meaningful and easy to reason about.
 *
 * @package HoloREA
 * @since   2019-07-03
 */

use std::clone::Clone;
use hdk::{
    holochain_core_types::{
        cas::content::Address,
        json::JsonString,
        entry::{
            entry_type::AppEntryType,
        },
    },
    error::{ ZomeApiResult },
};

use super::{
    records::{
        create_base_entry,
    },
    link_entries_bidir,
    link_remote_entries,
};

/// Creates a 'remote' query index used for following a link from some external record
/// into records contained within the current DNA / zome.
///
/// This basically consists of a `base` address for the remote content and bidirectional
/// links between it and its target_base_addresses.
pub fn create_remote_query_index<T>(
    remote_base_entry_type: T,
    origin_relationship_link_type: &str,
    origin_relationship_link_tag: &str,
    destination_relationship_link_type: &str,
    destination_relationship_link_tag: &str,
    source_base_address: &Address,
    target_base_addresses: &Vec<Address>,
) -> ZomeApiResult<Vec<Address>>
    where T: Into<AppEntryType>,
{
    // create a base entry pointer for the referenced origin record
    let base_address = create_base_entry(&(remote_base_entry_type.into()), &source_base_address).unwrap();

    // link all referenced records to our pointer to the remote origin record
    let commitment_results = target_base_addresses.iter()
        .map(|target_address| {
            // link origin record to local records by specified edge
            link_entries_bidir(
                &base_address, &target_address,
                origin_relationship_link_type, origin_relationship_link_tag,
                destination_relationship_link_type, destination_relationship_link_tag
            );

            target_address.to_owned()
        })
        .collect();

    Ok(commitment_results)
}

/// Creates a 'local' query index used for fetching and querying pointers to other
/// records that are stored externally to this DNA / zome.
///
/// In the local DNA, this consists of `base` addresses for all referenced foreign
/// content, bidirectionally linked to the originating record for querying in either direction.
///
/// In the remote DNA, a corresponding remote query index is built via `create_remote_query_index`,
/// which is presumed to be linked to the other end of the specified `remote_zome_method`.
pub fn create_local_query_index(
    remote_dna_id: &str,
    remote_zome_id: &str,
    remote_zome_method: &str,
    remote_request_cap_token: Address,
    remote_base_entry_type: &str,
    origin_relationship_link_type: &str,
    origin_relationship_link_tag: &str,
    destination_relationship_link_type: &str,
    destination_relationship_link_tag: &str,
    source_base_address: &Address,
    target_base_addresses: &Vec<Address>,
) -> ZomeApiResult<Vec<Address>> {
    // abort if target_base_addresses are empty
    if target_base_addresses.len() == 0 { return Ok(vec![]) }

    // Build local index first (for reading linked record IDs from the `source_base_address`)
    let mut commitment_results: Vec<Address> = target_base_addresses.iter()
        .map(|base_entry_addr| {
            // create a base entry pointer for the referenced commitment
            let base_address = create_base_entry(&(remote_base_entry_type.to_string().into()), base_entry_addr).unwrap();
            // link event to commitment by `fulfilled`/`fulfilledBy` edge
            link_entries_bidir(
                &source_base_address, &base_address,
                origin_relationship_link_type, origin_relationship_link_tag,
                destination_relationship_link_type, destination_relationship_link_tag
            );

            base_entry_addr.to_owned()
        })
        .collect();

    // :TODO: implement bridge genesis callbacks & private chain entry to wire up cross-DNA link calls

    // Build query index in remote DNA (for retrieving linked `target` entries)
    // -> links to `Commitment`s in the associated Planning DNA from this `EconomicEvent.fulfills`,
    //    and back to this `EconomicEvent` via `Commitment.fulfilledBy`.
    // :TODO: resolve typecasting issue and propagate any errors in the response
    let mut _result: JsonString = link_remote_entries(
        remote_dna_id,
        remote_zome_id,
        &remote_request_cap_token,
        remote_zome_method,
        &source_base_address,
        target_base_addresses,
    )?;

    // :TODO: append the results properly once we're able to interpret them
    // result.append(&mut commitment_results);
    Ok(commitment_results)
}
