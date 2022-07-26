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
    RecordAPIResult,
};

// HDK re-exports
pub use hdk::prelude::create_link;
pub use hdk::prelude::delete_link;

//--------------------------------[ READ ]--------------------------------------

/// Load any set of linked `EntryHash`es being referenced from the
/// provided `base_address` with the given `link_tag`.
///
pub fn get_linked_addresses(
    base_address: &EntryHash,
    link_tag: LinkTag,
) -> RecordAPIResult<Vec<EntryHash>> {
    pull_links_data(base_address, link_tag, get_link_target_entry)
}

/// Load any set of linked `ActionHash`es being referenced from the
/// provided `base_address` with the given `link_tag`.
///
/// Required to retrieve link actions for executing deletions.
///
pub fn get_linked_actions(
    base_address: &EntryHash,
    link_tag: LinkTag,
) -> RecordAPIResult<Vec<ActionHash>> {
    pull_links_data(base_address, link_tag, get_link_target_action)
}

/// Load any set of `LinkTag`s being referenced from the
/// provided `base_address` with the given `link_tag` prefix.
///
pub fn get_linked_tags(
    base_address: &EntryHash,
    link_tag: LinkTag,
) -> RecordAPIResult<Vec<LinkTag>> {
    pull_links_data(base_address, link_tag, get_link_target_tag)
}

/// Execute the provided `link_map` function against the set of links
/// between a `base_address` and `target_address` via the given `link_tag`.
///
/// If you have a bidirectional link between two `EntryHash`es, you must
/// run this method twice (once to remove each direction of the paired link).
///
pub fn walk_links_matching_entry<T, F>(
    base_address: &EntryHash,
    target_address: &EntryHash,
    link_tag: LinkTag,
    link_map: F,
) -> RecordAPIResult<Vec<T>>
    where F: Fn(&Link) -> T,
{
    let links_result = get_links(base_address.to_owned(), Some(link_tag))?;

    Ok(links_result
        .iter()
        .filter(|l| { EntryHash::from(l.target.clone()) == *target_address })
        .map(link_map)
        .collect()
    )
}

//-----------------------------------------------------

// :TODO: ensure ordering is latest-first
fn pull_links_data<T, F>(
    base_address: &EntryHash,
    link_tag: LinkTag,
    link_map: F,
) -> RecordAPIResult<Vec<T>>
    where F: Fn(&Link) -> T,
{
    let links_result = get_links((*base_address).clone(), Some(link_tag))?;

    Ok(links_result
        .iter()
        .map(link_map)
        .collect()
    )
}

fn get_link_target_entry(l: &Link) -> EntryHash {
    l.target.to_owned().into()
}

fn get_link_target_action(l: &Link) -> ActionHash {
    l.create_link_hash.clone()
}

fn get_link_target_tag(l: &Link) -> LinkTag {
    l.tag.clone()
}
