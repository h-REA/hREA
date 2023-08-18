use chrono::{ DateTime, Utc, NaiveDateTime };
use hdk::prelude::*;
use crate::{ RecordAPIResult, DataIntegrityError };

/// Metadata for a specific revision of a record, serializable for external transmission
///
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RevisionMeta {
    pub id: ActionHash,
    pub time: DateTime<Utc>,
    pub agent_pub_key: AgentPubKey,
}

/// Record metadata structure to enable iterating revisions of a record over time
///
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RecordMeta {
    // pub original_revision: RevisionMeta,
    pub previous_revision: Option<RevisionMeta>,
    // pub previous_revisions_count: u32,
    // pub latest_revision: RevisionMeta,
    // pub future_revisions_count: u32,
    pub retrieved_revision: RevisionMeta,
}

/// Retrieve minimal revision metadata for a record needed by UIs to retrieve version history
///
pub fn read_revision_metadata_abbreviated(header: &SignedActionHashed) -> RecordAPIResult<RecordMeta>
{
    let maybe_previous_element = get_previous_revision(header)?;

    Ok(RecordMeta {
        // original_revision: (&first).into(),
        previous_revision: maybe_previous_element.map(|e| e.into()),
        // previous_revisions_count,
        // future_revisions_count: 0,
        // latest_revision: e.clone().into(),
        retrieved_revision: header.into(),
    })
}

/**
 * Derive metadata for a record's full revision history by querying the DHT
 *
 * :TODO: handle conflicts @see https://github.com/h-REA/hREA/issues/196
 *
 * :TODO: think of some sensible way to differentiate a delete revision from
 * others if it is the one being requested
 */
pub fn read_revision_metadata_full(header: &SignedActionHashed) -> RecordAPIResult<RecordMeta>
{
    match get_details(get_action_hash(header), GetOptions { strategy: GetStrategy::Latest }) {
        Ok(Some(Details::Record(details))) => match details.validation_status {
            ValidationStatus::Valid => {
                // find previous Element first so we can reuse it to recurse backwards to original
                let maybe_previous_element = get_previous_revision(header)?;

                // recurse backwards from previous to determine original,
                // or indicate current as original if no previous Element exists
                let (_first, _previous_revisions_count) = match maybe_previous_element.clone() {
                    Some(previous_element) => find_earliest_revision(previous_element.signed_action(), 1)?,
                    None => (header.to_owned(), 0),
                };

                match details.updates.len() {
                    // no updates referencing this Element; therefore there are no future revisions and we are the latest
                    0 => {
                        Ok(RecordMeta {
                            // original_revision: (&first).into(),
                            previous_revision: maybe_previous_element.map(|e| e.into()),
                            // previous_revisions_count,
                            // future_revisions_count: 0,
                            // latest_revision: e.clone().into(),
                            retrieved_revision: header.into(),
                        })
                    },
                    // updates found, recurse to determine latest
                    _ => {
                        let (_latest, _future_revisions_count) = find_latest_revision(details.updates.as_slice(), 0)?;
                        Ok(RecordMeta {
                            // original_revision: (&first).into(),
                            previous_revision: maybe_previous_element.map(|e| e.into()),
                            // previous_revisions_count,
                            // future_revisions_count,
                            // latest_revision: (&latest).into(),
                            retrieved_revision: header.into(),
                        })
                    },
                }
            },
            _ => Err(DataIntegrityError::EntryNotFound),
        },
        _ => Err(DataIntegrityError::EntryNotFound),
    }
}

impl TryFrom<Record> for RecordMeta {
    type Error = DataIntegrityError;

    fn try_from(e: Record) -> Result<Self, Self::Error> {
        read_revision_metadata_full(e.signed_action())
    }
}

/// Pull relevant fields for a particular revision from any given DHT Record
///
impl From<Record> for RevisionMeta {
    fn from(e: Record) -> Self {
        e.signed_action().into()
    }
}

/// Pull relevant fields for a particular revision from a signed action
///
/// :TODO: update this method to handle date out of range errors more gracefully
/// (will currently panic due to unwrapping a `None` value)
///
impl From<&SignedActionHashed> for RevisionMeta {
    fn from(e: &SignedActionHashed) -> Self {
        let (secs, nsecs) = e.action().timestamp().as_seconds_and_nanos();
        Self {
            id: get_action_hash(e),
            time: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp_opt(secs, nsecs).unwrap(), Utc),
            agent_pub_key: e.action().author().to_owned(),
        }
    }
}

/// Step backwards to read the previous `Record` that was updated by the given `Record`
///
fn get_previous_revision(signed_action: &SignedActionHashed) -> RecordAPIResult<Option<Record>> {
    match signed_action {
        // this is a Create, so there is no previous revision
        SignedHashed { hashed: HoloHashed { content: Action::Create(_), .. }, .. } => {
            Ok(None)
        },
        // this is an Update, so previous revision exists
        SignedHashed { hashed: HoloHashed { content: Action::Update(update), .. }, .. } => {
            let previous_record = get(update.original_action_address.clone(), GetOptions { strategy: GetStrategy::Latest })?;
            match previous_record {
                None => Ok(None),
                Some(el) => Ok(Some(el)),
            }
        },
        // this is a Delete, so previous revision is what was deleted
        SignedHashed { hashed: HoloHashed { content: Action::Delete(delete), .. }, .. } => {
            let previous_record = get(delete.deletes_address.clone(), GetOptions { strategy: GetStrategy::Latest })?;
            match previous_record {
                None => Ok(None),
                Some(el) => Ok(Some(el)),
            }
        },
        _ => Err(DataIntegrityError::EntryWrongType)?,
    }
}

/**
 * Recursive helper for determining earliest revision in chain, and count of prior revisions.
 */
fn find_earliest_revision(signed_action: &SignedActionHashed, revisions_before: u32) -> RecordAPIResult<(SignedActionHashed, u32)> {
    let prev_record = get_previous_revision(signed_action)?;

    match prev_record {
        None => Ok((signed_action.to_owned(), revisions_before)),
        Some(e) => find_earliest_revision(e.signed_action(), revisions_before + 1),
    }
}

/**
 * Recursive helper for determining latest revision in chain, and count of subsequent revisions.
 *
 * :TODO: currently we assume multiple updates to the same entry were non-conflicting
 * and perceive the most recent as pointing to the next revision. We should not.
 * Instead every update path needs to be checked, all diverging leaf nodes resolved;
 * and if any remain then a DataIntegrityError::UpdateConflict should be thrown
 * with all the conflicting branch heads.
 *
 * :TODO: decide whether to return a delete as the latest revision for deleted entries
 */
fn find_latest_revision(updates: &[SignedActionHashed], revisions_until: u32) -> RecordAPIResult<(SignedActionHashed, u32)> {
    let mut sortlist = updates.to_vec();
    sortlist.sort_by_key(by_action_time);
    let most_recent = sortlist.last().unwrap().to_owned();

    match get_details(get_action_hash(&most_recent), GetOptions { strategy: GetStrategy::Latest }) {
        Ok(Some(Details::Record(details))) => match details.validation_status {
            ValidationStatus::Valid => match details.updates.len() {
                // found latest revision
                0 => Ok((details.record.signed_action().to_owned(), revisions_until + 1)),
                // still more updates to crawl, keep going
                _ => find_latest_revision(details.updates.as_slice(), revisions_until + 1),
            },
            // :TODO: how to handle abandoned validations?
            _ => Err(DataIntegrityError::EntryNotFound),
        },
        // :TODO: should we account for `None` being returned from the DHT?
        _ => Err(DataIntegrityError::EntryNotFound),
    }
}

/// Helper to retrieve the ActionHash for an Record
pub (crate) fn get_action_hash(shh: &SignedActionHashed) -> ActionHash {
    shh.as_hash().to_owned()
}

/// helper for sorting actions by creation time
fn by_action_time(h: &SignedActionHashed) -> i64 {
    h.action().timestamp().as_micros()
}
