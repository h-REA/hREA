use chrono::{DateTime, Utc, Duration};
use hdk::prelude::*;
use crate::{
    INDEX_DEPTH, HAS_CHUNK_LEAVES,
    index_tree::IndexSegment,
    TimeIndexResult, TimeIndexingError,
};

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
    let leaf_link = parents.first().ok_or(
        TimeIndexingError::NotIndexed(index_name.as_ref().to_string(), entry_hash)
    )?;
    let this_index: IndexSegment = leaf_link.tag.to_owned().try_into()?;
    let this_timestamp = this_index.into();

    let leaf_hash = EntryHash::from(leaf_link.target.to_owned());

    // find all our older siblings
    let mut older_siblings = get_child_links_of_node_older_than(index_name, leaf_hash, this_timestamp)?;

    // truncate to requested limit
    older_siblings.truncate(limit);

    // continue fetching links from previous leaf nodes in the index until we have reached the limit
    let mut context_timestamp = this_timestamp;
    while older_siblings.len() < limit {
        let previous_leaf_link = get_previous_leaf(
            index_name,
            &context_timestamp,
            // :NOTE: if there are chunk nodes in the tree, immediate parent will be at
            //        first INDEX_DEPTH, otherwise it will be second tier.
            if *HAS_CHUNK_LEAVES { 0 } else { 1 },
            if *HAS_CHUNK_LEAVES { 0 } else { 1 },
        )?;

        match previous_leaf_link {
            // stop looking if there are no more leaves to query
            None => { break; },
            Some(prev) => {
                // append any found link targets from the previous leaf
                let more_older_siblings = get_child_links_of_node_older_than(index_name, prev.hash()?, this_timestamp)?;
                older_siblings = [older_siblings, more_older_siblings].concat();
                older_siblings.truncate(limit);

                // look further backward from previous leaf if we still need to find more
                let prev_time: DateTime<Utc> = prev.into();
                context_timestamp = prev_time + Duration::milliseconds(-1);
            },
        }
    }

    Ok(older_siblings)
}

/// Locate all the child links of the node with hash `leaf_hash` which are older than `this_timestamp`,
/// ordered from newest to oldest.
///
fn get_child_links_of_node_older_than<I>(index_name: &I, leaf_hash: EntryHash, this_timestamp: DateTime<Utc>) -> TimeIndexResult<Vec<Link>>
    where I: AsRef<str>,
{
    // query children of parent node
    let mut siblings = get_links(
        leaf_hash,
        Some(link_prefix_for_index(index_name)),
    )?;

    // order them from newest to oldest
    siblings.sort_unstable_by(|a, b| b.tag.cmp(&a.tag));

    // filter out those newer
    Ok(siblings.iter().cloned().filter(|sib| {
        let this_index: Result<IndexSegment, _> = sib.tag.to_owned().try_into();
        match this_index {
            Ok(time) => {
                let dt: DateTime<Utc> = time.into();
                dt < this_timestamp
            },
            Err(_e) => false,
        }
    }).collect())
}

/// Find the previous leafmost node in the time index tree
/// :TODO: make this work
///
fn get_previous_leaf<I>(index_name: &I, from_timestamp: &DateTime<Utc>, try_depth: usize, starting_depth: usize) -> TimeIndexResult<Option<IndexSegment>>
    where I: AsRef<str>,
{
    let mut this_depth = try_depth;
    while this_depth >= starting_depth {
        let try_parent_hash = if this_depth >= INDEX_DEPTH.len() {
            // can't go upwards anymore, so go to the root
            Path::from(index_name.as_ref()).path_entry_hash()?
        } else {
            // determine next parent upwards
            IndexSegment::new(from_timestamp, INDEX_DEPTH.get(this_depth).unwrap()).hash()?
        };

        // find all nodes in the tree at this depth that are older than the starting offset
        let older_siblings = get_child_links_of_node_older_than(index_name, try_parent_hash, from_timestamp.to_owned())?;

        match older_siblings.first() {
            // there are no older siblings in this list
            None => {
                if this_depth >= INDEX_DEPTH.len() {
                    // if we are at the root, there are no more index links to try
                    return Ok(None);
                } else {
                    // if we are not yet at the root, go up a level and try reading more links
                    this_depth += 1;
                    continue;
                }
            },
            // if there was an older sibling, come back down to the earliest leaf node
            Some(link) => {
                let _found_at_this_depth: IndexSegment = link.tag.to_owned().try_into()?;
                while this_depth >= starting_depth {
                    // :TODO: finish this, or scrap it and start again

                    this_depth -= 1;
                }
                // this isn't what we want...
                return Ok(Some(IndexSegment::new(from_timestamp, INDEX_DEPTH.get(starting_depth).unwrap())))
            },
        }
    }

    Ok(None)    // also probably wrong
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
