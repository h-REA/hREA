use hdk::prelude::*;
use crate::{ RecordAPIResult, DataIntegrityError };

/// Metadata for a specific revision of a record, serializable for external transmission
///
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RevisionMeta {
    pub id: HeaderHash,
    pub time: Timestamp,
    pub author_id: AgentPubKey,
}

/// Record metadata structure to enable iterating revisions of a record over time
///
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RecordMeta {
    pub previous_revision: Option<RevisionMeta>,
    // pub previous_revision_count: u32,        :TODO:
    pub future_revisions_count: u32,
    pub latest_revision: RevisionMeta,
    pub current_revision: RevisionMeta,
}

/**
 * Derive metadata for a record's revision history by querying the DHT
 *
 * :TODO: handle conflicts @see https://github.com/h-REA/hREA/issues/196
 *
 * :TODO: iterate backwards to provide `previous_revision_count`
 *
 * :TODO: think of some sensible way to differentiate a delete revision from
 * others if it is the one being requested
 */
impl TryFrom<Element> for RecordMeta {
    type Error = DataIntegrityError;

    fn try_from(e: Element) -> Result<Self, Self::Error> {
        match get_details(get_header_hash(e.signed_header()), GetOptions { strategy: GetStrategy::Latest }) {
            Ok(Some(Details::Element(details))) => match details.validation_status {
                ValidationStatus::Valid => match details.updates.len() {
                    // no updates referencing this Element; therefore there are no future revisions and we are the latest
                    0 => {
                        Ok(Self {
                            previous_revision: get_previous_revision(details.element)?.map(|e| e.into()),
                            // previous_revision_count: 0,
                            future_revisions_count: 0,
                            latest_revision: e.clone().into(),
                            current_revision: e.into(),
                        })
                    },
                    // updates found, recurse to determine latest
                    _ => {
                        let (latest, future_revisions_count) = find_latest_revision(details.updates.as_slice(), 0)?;
                        Ok(Self {
                            previous_revision: get_previous_revision(details.element)?.map(|e| e.into()),
                            // previous_revision_count: 0,
                            future_revisions_count,
                            latest_revision: (&latest).into(),
                            current_revision: e.into(),
                        })
                    },
                },
                _ => Err(Self::Error::EntryNotFound),
            },
            _ => Err(Self::Error::EntryNotFound),
        }
    }
}

/// Pull relevant fields for a particular revision from any given DHT Element
///
impl From<Element> for RevisionMeta {
    fn from(e: Element) -> Self {
        e.signed_header().into()
    }
}

/// Pull relevant fields for a particular revision from a signed header
///
impl From<&SignedHeaderHashed> for RevisionMeta {
    fn from(e: &SignedHeaderHashed) -> Self {
        Self {
            id: get_header_hash(e),
            time: e.header().timestamp().to_owned(),
            author_id: e.header().author().to_owned(),
        }
    }
}

/// Step backwards to read the previous `Element` that was updated by the given `Element`
///
fn get_previous_revision(element: Element) -> RecordAPIResult<Option<Element>> {
    match element.signed_header() {
        // this is a Create, so there is no previous revision
        SignedHashed { hashed: HoloHashed { content: Header::Create(_), .. }, .. } => {
            Ok(None)
        },
        // this is an Update, so previous revision exists
        SignedHashed { hashed: HoloHashed { content: Header::Update(update), .. }, .. } => {
            let previous_element = get(update.original_header_address.clone(), GetOptions { strategy: GetStrategy::Latest })?;
            match previous_element {
                None => Ok(None),
                Some(el) => Ok(Some(el)),
            }
        },
        // this is a Delete, so previous revision is what was deleted
        SignedHashed { hashed: HoloHashed { content: Header::Delete(delete), .. }, .. } => {
            let previous_element = get(delete.deletes_address.clone(), GetOptions { strategy: GetStrategy::Latest })?;
            match previous_element {
                None => Ok(None),
                Some(el) => Ok(Some(el)),
            }
        },
        _ => Err(DataIntegrityError::EntryWrongType)?,
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
 * We could/should probably be able to count revisions in both directions as part of
 * the correct algorithm in future; since we will have to go backwards as well in order
 * to resolve any parallel branches which merge later.
 */
fn find_latest_revision(updates: &[SignedHeaderHashed], revisions_until: u32) -> RecordAPIResult<(SignedHeaderHashed, u32)> {
    let mut sortlist = updates.to_vec();
    sortlist.sort_by_key(by_header_time);
    let most_recent = sortlist.last().unwrap().to_owned();

    match get_details(get_header_hash(&most_recent), GetOptions { strategy: GetStrategy::Latest }) {
        Ok(Some(Details::Element(details))) => match details.validation_status {
            ValidationStatus::Valid => match details.updates.len() {
                // found latest revision
                0 => Ok((details.element.signed_header().to_owned(), revisions_until + 1)),
                // still more updates to crawl, keep going
                _ => find_latest_revision(details.updates.as_slice(), revisions_until + 1),
            },
            _ => Err(DataIntegrityError::EntryNotFound),
        },
        // :TODO: should we account for `None` being returned from the DHT?
        _ => Err(DataIntegrityError::EntryNotFound),
    }
}

/// Helper to retrieve the HeaderHash for an Element
pub (crate) fn get_header_hash(shh: &element::SignedHeaderHashed) -> HeaderHash {
    shh.as_hash().to_owned()
}

/// helper for sorting headers by creation time
fn by_header_time(h: &SignedHeaderHashed) -> i64 {
    h.header().timestamp().as_micros()
}
