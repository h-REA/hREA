/**
 * Link-handling abstractions for Holochain apps
 *
 * Handles common behaviours for linking between data in different hApps,
 * in a way that is predictable, semantically meaningful and easy to reason about.
 *
 * @package HoloREA
 * @since   2019-07-03
 */

use hdk::prelude::*;
use hdk::prelude::link::Link;

use crate::{
    GraphAPIResult,
};

// HDK re-exports
pub use hdk::prelude::create_link;
pub use hdk::prelude::delete_link;

//--------------------------------[ READ ]--------------------------------------

/// Load any set of linked `EntryHash`es being referenced from the
/// provided `base_address` with the given `link_tag`.
///
pub (crate) fn get_linked_addresses(
    base_address: &EntryHash,
    link_tag: LinkTag,
) -> GraphAPIResult<Vec<EntryHash>> {
    pull_links_data(base_address, link_tag, get_link_target_entry)
}

/// Load any set of linked `HeaderHash`es being referenced from the
/// provided `base_address` with the given `link_tag`.
///
/// Required to retrieve link headers for executing deletions.
///
pub (crate) fn get_linked_headers(
    base_address: &EntryHash,
    link_tag: LinkTag,
) -> GraphAPIResult<Vec<HeaderHash>> {
    pull_links_data(base_address, link_tag, get_link_target_header)
}

//-----------------------------------------------------

// :TODO: ensure ordering is latest-first
fn pull_links_data<T, F>(
    base_address: &EntryHash,
    link_tag: LinkTag,
    link_map: F,
) -> GraphAPIResult<Vec<T>>
    where F: Fn(&Link) -> T,
{
    let links_result = get_links((*base_address).clone(), Some(link_tag))?;

    Ok(links_result
        .into_inner()
        .iter()
        .map(link_map)
        .collect()
    )
}

fn get_link_target_entry(l: &Link) -> EntryHash {
    l.target.clone()
}

fn get_link_target_header(l: &Link) -> HeaderHash {
    l.create_link_hash.clone()
}
