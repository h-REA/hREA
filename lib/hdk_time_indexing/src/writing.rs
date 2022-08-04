use chrono::{DateTime, Utc};
use hdk::prelude::*;
use crate::{
    TimeIndexResult, TimeIndexingError,
    index_tree::*,
};
use index_integrity::LinkTypes;

/// Index an entry with hash `entry_hash` into the time-ordered index
/// identified by `index_entry` at the given time point.
///
/// The entry must already exist and have been written to the local DHT.
///
pub fn index_entry<I>(index_name: &I, entry_hash: EntryHash, time: DateTime<Utc>) -> TimeIndexResult<()>
    where I: AsRef<str>,
{
    // write the time index tree
    let leafmost_segment = ensure_time_index(index_name, time)?;
    let leafmost_hash = leafmost_segment.hash()?;

    // create a virtual segment for determining the final link tag data
    let target_entry_segment = IndexSegment::leafmost_link(&time);
    let encoded_link_tag = target_entry_segment.tag_for_index(&index_name);

    // ensure link from the leaf index to the target entry
    link_if_not_linked(leafmost_hash.to_owned(), entry_hash.to_owned(), encoded_link_tag.to_owned())?;

    // ensure a reciprocal link from the target entry back to the leaf index node
    link_if_not_linked(entry_hash, leafmost_hash, encoded_link_tag)?;

    Ok(())
}

/// Returns the leaf-most `IndexSegment` in the time tree, so that target entries can be
/// linked from it.
///
fn ensure_time_index<I>(index_name: &I, time: DateTime<Utc>) -> TimeIndexResult<IndexSegment>
    where I: AsRef<str>,
{
    // create a root anchor for the path based on the index name
    let root = Path::from(index_name.as_ref()).typed(LinkTypes::TimeIndex)?;
    root.ensure()?;
    let root_hash = root.path_entry_hash()?;

    let segments = get_index_segments(&time);

    for (idx, segment) in segments.iter().enumerate() {
        segment.ensure()?;

        if idx == 0 {
            // link the first segment to the root
            if !segment_links_exist(index_name, &root_hash, segment)? {
                create_link(
                    root_hash.to_owned(),
                    segment.hash()?,
                    LinkTypes::TimeIndex,
                    segment.tag_for_index(&index_name),
                )?;
            }
        } else {
            // link subsequent segments to the previous one
            let prev_segment_hash = segments.get(idx - 1).unwrap().hash()?;

            if !segment_links_exist(index_name, &prev_segment_hash, segment)? {
                create_link(
                    prev_segment_hash,
                    segment.hash()?,
                    LinkTypes::TimeIndex,
                    segment.tag_for_index(&index_name),
                )?;
            }
        }
    }

    Ok(segments.last().unwrap().cloned())
}

fn segment_links_exist<I>(index_name: &I, base_hash: &EntryHash, target_segment: &IndexSegment) -> TimeIndexResult<bool>
    where I: AsRef<str>,
{
    Ok(get_links(base_hash.to_owned(), LinkTypes::TimeIndex, Some(target_segment.tag_for_index(&index_name)))?
        .len() > 0)
}

fn link_if_not_linked(origin_hash: EntryHash, dest_hash: EntryHash, link_tag: LinkTag) -> TimeIndexResult<()> {
    if false == get_links(origin_hash.to_owned(), LinkTypes::TimeIndex, Some(link_tag.to_owned()))?
        .iter().any(|l| { EntryHash::from(l.target.to_owned()) == dest_hash })
    {
        create_link(
            origin_hash.to_owned(),
            dest_hash.to_owned(),
            LinkTypes::TimeIndex,
            link_tag,
        ).map_err(|_e| { TimeIndexingError::EntryNotFound(origin_hash.to_owned()) })?;
    }

    Ok(())
}
