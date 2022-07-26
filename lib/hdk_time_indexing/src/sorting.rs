use std::cmp::Ordering;
use chrono::{DateTime, Utc, NaiveDateTime};
use hdk::prelude::*;
use crate::{
    TimeIndexResult,
    reading::link_prefix_for_index,
    index_tree::IndexSegment,
};
use integrity_types::LinkTypes;

/**
 * Higher-order function to generate a comparator callback for use in
 * sorting sets of `EntryHash`es that have been indexed into the
 * `index_name` time-ordered index.
 *
 * It is presumed that the entries are validly linked in the index. If
 * not, they will be placed at the end of the list in undefined order.
 *
 * Usage:
 *  let mut my_entry_hashes: Vec<EntryHash> = some_list_of_hashes;
 *  my_entry_hashes.sort_by(sort_entries_by_time_index("my_time_index_name"));
 */
pub fn sort_entries_by_time_index<'a, I>(index_name: &'a I) -> Box<dyn for<'r> Fn(&EntryHash, &EntryHash) -> Ordering + 'a>
    where I: AsRef<str>,
{
    let prefix = link_prefix_for_index(index_name);

    Box::new(move |a, b| {
        let a_timestamp = get_time_for_entry_hash(prefix.to_owned(), a);
        let b_timestamp = get_time_for_entry_hash(prefix.to_owned(), b);
        b_timestamp.cmp(&a_timestamp)
    })
}

/// Determine the timestamp for an `entry_hash` indexed into a time index.
///
/// `index_link_prefix` is the leading bytes shared by all `LinkTag`s relevant to the index, used
/// with Holochain's native link filtering. Pre-generated via `link_prefix_for_index` for minor performance gains.
///
/// :TODO: consider an appropriate way of handling errors
/// :TODO: should we adjust for possibility of entries being validly indexed multiple times in the same index?
///
fn get_time_for_entry_hash(index_link_prefix: LinkTag, entry_hash: &EntryHash) -> DateTime<Utc>
{
    let links = get_links(
        entry_hash.to_owned(),
        LinkTypes::Any,
        Some(index_link_prefix),
    );
    match links {
        Err(_e) => null_time(),
        Ok(links) => {
            if links.len() != 1 {
                return null_time();
            }
            let try_segment: TimeIndexResult<IndexSegment> = links.first().unwrap().tag.to_owned().try_into();
            match try_segment {
                Ok(segment) => match segment.try_into() {
                    Ok(date) => date,
                    Err(_e) => null_time(),
                },
                Err(_e) => null_time(),
            }
        }
    }
}

fn null_time() -> DateTime<Utc> {
    DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc)
}
