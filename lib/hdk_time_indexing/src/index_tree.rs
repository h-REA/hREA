use std::time::Duration;
use chrono::{DateTime, NaiveDate, Timelike, Datelike, Utc};
use hdk::prelude::*;

use crate::{
    INDEX_DEPTH, CHUNK_INTERVAL,
    IndexType, TimeIndexResult,
};

/// An index segment stores a wrapped unsigned int representing the timestamp on the DHT
///
#[hdk_entry(id="time_index")]
pub struct IndexSegment(u64);

impl IndexSegment {
    /// Generate an index segment by truncating a timestamp (in ms)
    /// from the input `DateTime<Utc>` to the given `granularity`
    ///
    pub fn new(from: &DateTime<Utc>, granularity: &IndexType) -> Self {
        let truncated = match granularity {
            IndexType::Year => NaiveDate::from_ymd(from.year(), 1, 1).and_hms(0, 0, 0),
            IndexType::Month => NaiveDate::from_ymd(from.year(), from.month(), 1).and_hms(0, 0, 0),
            IndexType::Day => NaiveDate::from_ymd(from.year(), from.month(), from.day()).and_hms(0, 0, 0),
            IndexType::Hour => NaiveDate::from_ymd(from.year(), from.month(), from.day())
                .and_hms(from.hour(), 0, 0),
            IndexType::Minute => NaiveDate::from_ymd(from.year(), from.month(), from.day())
                .and_hms(from.hour(), from.minute(), 0),
            IndexType::Second => NaiveDate::from_ymd(from.year(), from.month(), from.day())
                .and_hms(from.hour(), from.minute(), from.second()),
        };

        Self(truncated.timestamp_millis() as u64)
    }

    /// Generate an index segment corresponding to the closest leaf chunk for the given timestamp
    ///
    pub fn new_chunk(from: &DateTime<Utc>) -> Self {
        let from_millis = from.timestamp_millis() as u64;
        let chunk_millis = CHUNK_INTERVAL.as_millis() as u64;
        Self(from_millis / chunk_millis)
    }

    /// Generate a virtual index segment for an exact time, to use with final referencing link tag
    ///
    pub fn leafmost_link(from: &DateTime<Utc>) -> Self {
        Self(from.timestamp_millis() as u64)
    }

    /// :SHONK: clone the `IndexSegment`. For some reason to_owned() is returning a ref?
    pub fn cloned(&self) -> Self {
        Self(self.0)
    }

    /// return the raw timestamp of this `IndexSegment`
    pub fn timestamp(&self) -> u64 {
        self.0
    }

    /// Generate a `LinkTag` with encoded time of this index, suitable for linking from
    /// other entries in the index tree rooted at `index_name`.
    ///
    pub fn tag_for_index<I>(&self, index_name: &I) -> LinkTag
        where I: AsRef<str>,
    {
        LinkTag::new([
            index_name.as_ref().as_bytes(), // prefix with index ID
            &[0x0 as u8],                   // null byte separator
            &self.timestamp().to_be_bytes() // raw timestamp bytes encoded for sorting
        ].concat())
    }

    /// What is the hash for the current [ `IndexSegment` ]?
    pub fn hash(&self) -> TimeIndexResult<EntryHash> {
        Ok(hash_entry(self.to_owned())?)
    }

    /// Does an entry exist at the hash we expect?
    pub fn exists(&self) -> TimeIndexResult<bool> {
        Ok(get(self.hash()?, GetOptions::content())?.is_some())
    }

    /// Ensure this [ `IndexSegment` ] has been written to the DHT
    pub fn ensure(&self) -> TimeIndexResult<()> {
        if !self.exists()? {
            create_entry(self)?;
        }
        Ok(())
    }
}

/// Generate a list of `IndexSegment` representing nodes in a radix trie for the given `time`.
/// The segments are returned in order of granularity, with least granular first.
///
pub (crate) fn get_index_segments(time: &DateTime<Utc>) -> Vec<IndexSegment> {
    let mut segments = vec![];

    // build main segments
    if INDEX_DEPTH.contains(&IndexType::Year) {
        segments.push(IndexSegment::new(&time, &IndexType::Year));
    }
    if INDEX_DEPTH.contains(&IndexType::Month) {
        segments.push(IndexSegment::new(&time, &IndexType::Month));
    }
    if INDEX_DEPTH.contains(&IndexType::Day) {
        segments.push(IndexSegment::new(&time, &IndexType::Day));
    }
    if INDEX_DEPTH.contains(&IndexType::Hour) {
        segments.push(IndexSegment::new(&time, &IndexType::Hour));
    }
    if INDEX_DEPTH.contains(&IndexType::Minute) {
        segments.push(IndexSegment::new(&time, &IndexType::Minute));
    }
    if INDEX_DEPTH.contains(&IndexType::Second) {
        segments.push(IndexSegment::new(&time, &IndexType::Second));
    }

    // build remainder chunk segment if it doesn't round evenly
    if *CHUNK_INTERVAL < Duration::from_secs(1)
        || (*CHUNK_INTERVAL > Duration::from_secs(1) && *CHUNK_INTERVAL < Duration::from_secs(60))
        || (*CHUNK_INTERVAL > Duration::from_secs(60) && *CHUNK_INTERVAL < Duration::from_secs(3600))
        || (*CHUNK_INTERVAL > Duration::from_secs(3600) && *CHUNK_INTERVAL < Duration::from_secs(86400))
    {
        segments.push(IndexSegment::new_chunk(&time));
    }

    segments
}
