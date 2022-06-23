use chrono::{DateTime, Utc, NaiveDateTime};
use hdk::prelude::*;
use crate::{TimeIndexResult, TimeIndexingError};

/// Return a maximum of `limit` `Link`s, in order from most recent to oldest in `index_name`
/// starting at the given (already indexed) `entry_hash`.
///
/// :TODO: account for the possibility that an entry might be validly linked multiple times in the same index
/// :TODO: de-duplicate links to account for network partitions erroneously causing multiple writes of the same index
///
fn get_ordered_links_before<I>(entry_hash: EntryHash, index_name: &I, limit: usize) -> TimeIndexResult<Vec<Link>>
    where I: AsRef<str>,
{
    // inspect link from entry to index in order to determine indexed time & parent node in index tree
    let parents = get_links(
        entry_hash.to_owned(),
        Some(link_prefix_for_index(index_name)),
    )?;
    let parent_link = parents.first().ok_or(
        TimeIndexingError::NotIndexed(index_name.as_ref().to_string(), entry_hash)
    )?;
    let this_timestamp = decode_link_tag_timestamp(parent_link.tag.to_owned())?;

    // query children of parent node
    let parent_hash = EntryHash::from(parent_link.target.to_owned());
    let mut siblings = get_links(
        parent_hash,
        Some(link_prefix_for_index(index_name)),
    )?;

    // order them from newest to oldest
    siblings.sort_unstable_by(|a, b| b.tag.cmp(&a.tag));

    // filter out those newer
    let mut older_siblings: Vec<Link> = siblings.iter().cloned().filter(|sib| {
        match decode_link_tag_timestamp(sib.tag.to_owned()) {
            Ok(time) => time < this_timestamp,
            Err(_e) => false,
        }
    }).collect();

    // truncate to requested limit
    older_siblings.truncate(limit);

    if older_siblings.len() < limit {
        // :TODO: determine previous sibling of parent node based on tree depth, recurse backwards
    }

    Ok(older_siblings)
}

/// Determine the hash of the most recent `IndexSegment` written for the given `index_name`.
/// If the index has not been created yet, `None` is returned in the result.
///
fn get_latest_indexed_entry_hash<I>(index_name: &I) -> TimeIndexResult<Option<EntryHash>>
    where I: AsRef<str>,
{
    let root = Path::from(index_name.as_ref());
    if !root.exists()? {
        // bail early if the index hasn't been touched
        return Ok(None);
    }
    let root_hash = root.path_entry_hash()?;

    // recurse into the tree, returning leafmost node
    Ok(Some(get_newest_leafmost_hash(root_hash, index_name)?))
}

/// Find the most recent indexed hash in all children of `current_segment_hash`
/// by recursively performing a depth-first search of its children.
///
fn get_newest_leafmost_hash<I>(current_segment_hash: EntryHash, index_name: &I) -> TimeIndexResult<EntryHash>
    where I: AsRef<str>,
{
    let mut children = get_links(
        current_segment_hash.to_owned(),
        Some(link_prefix_for_index(index_name)),
    )?;

    children.sort_unstable_by(|a, b| a.tag.cmp(&b.tag));

    match children.last() {
        None => Ok(current_segment_hash),
        Some(l) => get_newest_leafmost_hash(EntryHash::from(l.target.to_owned()), index_name),
    }
}

/// Determines prefix `LinkTag` for locating nodes of the index `index_name`.
///
fn link_prefix_for_index<I>(index_name: &I) -> LinkTag
    where I: AsRef<str>,
{
    LinkTag::new([
        index_name.as_ref().as_bytes(), // prefix with index ID
        &[0x0 as u8],                   // null byte separator
    ].concat())
}
