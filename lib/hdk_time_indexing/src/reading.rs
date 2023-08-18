use chrono::{DateTime, Utc, Duration};
use hdk::prelude::*;
use crate::{
    INDEX_DEPTH, HAS_CHUNK_LEAVES,
    index_tree::IndexSegment,
    TimeIndexResult, TimeIndexingError,
};
use hdk_semantic_indexes_core::LinkTypes;

/**
 * Retrieve the complete set of linked `EntryHash`es referenced in the `index_name` index.
 * This method is highly inefficient and strongly discouraged for large datasets. Use
 * only with indexes which are known to be of a small size.
 */
pub fn read_all_entry_hashes<I>(index_name: &I) -> TimeIndexResult<Vec<EntryHash>>
    where I: AsRef<str>,
{
    let root_hash = get_root_hash(index_name)?;

    match root_hash {
        None => Ok(vec![]),
        Some(hash) => {
            Ok(collect_leaf_index_hashes(index_name, hash, (*INDEX_DEPTH).len() as isize)?)
        }
    }
}

/// Recursively performs a depth-first traversal of the specified time index tree, returning the `EntryHash`es
/// of all the leafmost nodes (i.e. indexed entries) present in the index.
///
fn collect_leaf_index_hashes<I>(index_name: &I, context_hash: EntryHash, context_depth: isize) -> TimeIndexResult<Vec<EntryHash>>
    where I: AsRef<str>,
{
    let children = get_ordered_child_links_of_node(
        index_name,
        context_hash.clone(),
    )?;

    // last hop outside the index tree links to the targeted nodes, so return them
    if (*HAS_CHUNK_LEAVES && context_depth == -1) || (!(*HAS_CHUNK_LEAVES) && context_depth == 0) {
        return Ok(children.iter()
            .map(|link|link.target.to_owned().into_entry_hash().unwrap())
            .collect());
    }

    // still recursing downwards- load descendent nodes for every child found
    let (descendents, errors): (Vec<TimeIndexResult<Vec<EntryHash>>>, Vec<TimeIndexResult<Vec<EntryHash>>>) = children.iter()
        .map(|link| {
            collect_leaf_index_hashes(index_name, link.target.to_owned().into_entry_hash().unwrap(), context_depth - 1)
        })
        .partition(Result::is_ok);

    // return any errors fetching children if encountered
    if errors.len() > 0 {
        return errors.first().unwrap().to_owned();
    }

    // flatten the descendent nodes into a single vector
    Ok(descendents.iter().cloned()
        .flat_map(Result::unwrap)
        .collect())
}

/**
 * Retrieve the most recent entry hashes stored in the `index_name` time-ordered index,
 * up to a maximum of `limit`.
 */
pub fn get_latest_entry_hashes<I>(index_name: &I, limit: usize) -> TimeIndexResult<Vec<EntryHash>>
    where I: AsRef<str>,
{
    // find the most recently indexed EntryHash as a starting point
    let most_recent = get_latest_indexed_entry_hash(index_name)?;

    match most_recent {
        None => Ok(vec![]),
        Some(latest) => {
            // load a page of links to entries immediately before the latest
            let mut earlier_page = get_older_entry_hashes(index_name, latest.to_owned(), limit)?;

            // remove the earliest linked entry
            let num_earlier = earlier_page.len();
            if num_earlier > 0 {
                earlier_page.truncate(num_earlier - 1);
            }

            // prepend the most recent one
            Ok([
                vec![latest.to_owned()],
                earlier_page,
            ].concat())
        },
    }
}

/**
 * Retrieve entry hashes indexed in the `index_name` time-ordered index immediately
 * before `before_entry` (not inclusive), up to a maximum of `limit`.
 *
 * This method is best used with cursor-based pagination, where the previously oldest
 * returned `EntryHash` is used as a cursor to return the next most recent page of entries.
 */
pub fn get_older_entry_hashes<I>(index_name: &I, before_entry: EntryHash, limit: usize) -> TimeIndexResult<Vec<EntryHash>>
    where I: AsRef<str>,
{
    Ok(get_ordered_links_before(index_name, before_entry, limit)?
        .iter()
        .filter_map(|link| { link.target.to_owned().into_entry_hash() })
        .collect())
}

/// Return a maximum of `limit` `Link`s, in order from most recent to oldest in `index_name`
/// starting at the given (already indexed) `entry_hash`.
///
/// :TODO: account for the possibility that an entry might be validly linked multiple times in the same index
/// :TODO: de-duplicate links to account for network partitions erroneously causing multiple writes of the same index
///
fn get_ordered_links_before<I>(index_name: &I, entry_hash: EntryHash, limit: usize) -> TimeIndexResult<Vec<Link>>
    where I: AsRef<str>,
{
    // inspect link from entry to index in order to determine indexed time & parent node in index tree
    let parents = get_links(
        entry_hash.to_owned(),
        LinkTypes::TimeIndex,
        Some(link_prefix_for_index(index_name)),
    )?;
    let leaf_link = parents.first().ok_or(
        TimeIndexingError::NotIndexed(index_name.as_ref().to_string(), entry_hash)
    )?;
    let this_index: IndexSegment = leaf_link.tag.to_owned().try_into()?;
    let this_timestamp = this_index.into();

    let leaf_hash = leaf_link.target.to_owned().into_entry_hash().unwrap();

    // find all our older siblings
    let mut older_siblings = get_ordered_child_links_of_node_older_than(index_name, leaf_hash, this_timestamp)?;

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
                let more_older_siblings = get_ordered_child_links_of_node_older_than(index_name, prev.hash()?, this_timestamp)?;
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

/// Locate all the child links of the node with hash `leaf_hash`, ordered from newest to oldest.
///
fn get_ordered_child_links_of_node<I>(index_name: &I, leaf_hash: EntryHash) -> TimeIndexResult<Vec<Link>>
    where I: AsRef<str>,
{
    // query children of parent node
    let mut siblings = get_links(
        leaf_hash,
        LinkTypes::TimeIndex,
        Some(link_prefix_for_index(index_name)),
    )?;

    // order them from newest to oldest
    siblings.sort_unstable_by(|a, b| b.tag.cmp(&a.tag));

    Ok(siblings)
}

/// Locate all the child links of the node with hash `leaf_hash` which are older than `this_timestamp`,
/// ordered from newest to oldest.
///
fn get_ordered_child_links_of_node_older_than<I>(index_name: &I, leaf_hash: EntryHash, this_timestamp: DateTime<Utc>) -> TimeIndexResult<Vec<Link>>
    where I: AsRef<str>,
{
    // query children of parent node
    let siblings = get_ordered_child_links_of_node(
        index_name,
        leaf_hash,
    )?;

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
            Path::from(index_name.as_ref()).typed(LinkTypes::TimeIndex)?.path_entry_hash()?
        } else {
            // determine next parent upwards
            IndexSegment::new(from_timestamp, INDEX_DEPTH.get(this_depth).unwrap()).hash()?
        };

        // find all nodes in the tree at this depth that are older than the starting offset
        let older_siblings = get_ordered_child_links_of_node_older_than(index_name, try_parent_hash, from_timestamp.to_owned())?;

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
    match get_root_hash(index_name)? {
        None => Ok(None),
        // recurse into the tree, returning leafmost node
        Some(hash) => Ok(Some(get_newest_leafmost_hash(hash, index_name)?)),
    }
}

/// Find the most recent indexed hash in all children of `current_segment_hash`
/// by recursively performing a depth-first search of its children.
///
fn get_newest_leafmost_hash<I>(current_segment_hash: EntryHash, index_name: &I) -> TimeIndexResult<EntryHash>
    where I: AsRef<str>,
{
    let mut children = get_links(
        current_segment_hash.to_owned(),
        LinkTypes::TimeIndex,
        Some(link_prefix_for_index(index_name)),
    )?;

    children.sort_unstable_by(|a, b| a.tag.cmp(&b.tag));

    match children.last() {
        None => Ok(current_segment_hash),
        Some(l) => get_newest_leafmost_hash(l.target.to_owned().into_entry_hash().unwrap(), index_name),
    }
}

/// Determine the hash of the root node for the given index.
///
fn get_root_hash<I>(index_name: &I) -> TimeIndexResult<Option<EntryHash>>
    where I: AsRef<str>,
{
    let root = Path::from(index_name.as_ref()).typed(LinkTypes::TimeIndex)?;
    if !root.exists()? {
        // bail early if the index hasn't been touched
        return Ok(None);
    }
    Ok(Some(root.path_entry_hash()?))
}

/// Determines prefix `LinkTag` for locating nodes of the index `index_name`.
///
pub (crate) fn link_prefix_for_index<I>(index_name: &I) -> LinkTag
    where I: AsRef<str>,
{
    LinkTag::new([
        index_name.as_ref().as_bytes(), // prefix with index ID
        &[0x0 as u8],                   // null byte separator
    ].concat())
}
