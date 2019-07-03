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
            AppEntryValue,
        },
    },
    error::{ ZomeApiResult },
};

use super::{
    records::{
        create_base_entry,
    },
    link_entries_bidir,
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
    let base_address = create_base_entry(remote_base_entry_type.into(), &source_base_address).unwrap();

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
