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

pub fn create_remote_query_index<T, S>(
    remote_base_entry_type: T,
    origin_relationship_link_type: S,
    origin_relationship_link_tag: S,
    destination_relationship_link_type: S,
    destination_relationship_link_tag: S,
    source_base_address: &Address,
    target_base_addresses: &Vec<Address>,
) -> ZomeApiResult<Vec<Address>>
    where S: Clone + Into<String>,
        T: Into<AppEntryType>,
{
    // create a base entry pointer for the referenced origin record
    let base_address = create_base_entry(remote_base_entry_type.into(), &source_base_address).unwrap();

    // link all referenced records to our pointer to the remote origin record
    let commitment_results = target_base_addresses.iter()
        .map(|target_address| {
            // link origin record to local records by specified edge
            link_entries_bidir(
                &base_address, &target_address,
                // :TODO: fix lazy borrow checker workaround
                origin_relationship_link_type.clone(), origin_relationship_link_tag.clone(),
                destination_relationship_link_type.clone(), destination_relationship_link_tag.clone()
            );

            target_address.to_owned()
        })
        .collect();

    Ok(commitment_results)
}
