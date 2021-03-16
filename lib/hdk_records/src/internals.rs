/**
 * Internal filter predicates & utility methods for link update logic
 *
 * @see     ../../README.md
 * @package HDK Graph Helpers
 * @since   2019-09-10
 */
use hdk::prelude::*;
use holo_hash::DnaHash;

use crate::{
    RecordAPIResult, DataIntegrityError,
    identity_helpers::create_entry_identity,
    local_indexes::{create_index, delete_index},
};

pub (crate) fn link_pair_matches<'a, A>(hashes: &'a [A]) -> Box<dyn for<'r> Fn(&'r &'a A) -> bool + 'a>
    where A: AsRef<DnaHash> + AsRef<EntryHash> + From<(DnaHash, EntryHash)>,
{
    Box::new(move |&existing_link| {
        let eh: &EntryHash = existing_link.as_ref();
        let dh: &DnaHash = existing_link.as_ref();
        hashes.iter().any(|h| {
            let heh: &EntryHash = h.as_ref();
            let hdh: &DnaHash = h.as_ref();
            *heh == *eh && *hdh == *dh
        })
    })
}

/// Set difference of two vectors, using HashSet
pub (crate) fn vect_difference<T>(v1: &Vec<T>, v2: &Vec<T>) -> Vec<T>
    where T: Clone + Eq + std::hash::Hash,
{
    let s1: HashSet<T> = v1.iter().cloned().collect();
    let s2: HashSet<T> = v2.iter().cloned().collect();
    (&s1 - &s2).iter().cloned().collect()
}

/// Returns the first error encountered (if any). Best used with the `?` operator.
pub (crate) fn throw_any_error<T>(mut errors: Vec<RecordAPIResult<T>>) -> RecordAPIResult<()> {
    if errors.len() == 0 {
        return Ok(());
    }
    let first_err = errors.pop().unwrap();
    Err(first_err.err().unwrap())
}

/// Convert internal zome errors into externally encodable type for response
pub (crate) fn convert_errors<E: Clone, F>(r: &Result<HeaderHash, E>) -> Result<HeaderHash, F>
    where F: From<E>,
{
    match r {
        Ok(header) => Ok(header.clone()),
        Err(e) => Err(F::from((*e).clone())),
    }
}

/// Helper for index update to add multiple destination links from some source.
pub (crate) fn create_dest_indexes<'a, A, S, I>(
    source_entry_type: &'a I,
    source: &'a A,
    dest_entry_type: &'a I,
    link_tag: &'a S,
    link_tag_reciprocal: &'a S,
) -> Box<dyn for<'r> Fn(&A) -> Vec<RecordAPIResult<HeaderHash>> + 'a>
    where I: AsRef<str>,
        S: 'a + AsRef<[u8]>,
        A: AsRef<DnaHash> + AsRef<EntryHash> + From<(DnaHash, EntryHash)>,
{
    Box::new(move |dest| {
        match create_index(source_entry_type, source, dest_entry_type, dest, link_tag, link_tag_reciprocal) {
            Ok(created) => created,
            Err(_) => {
                let h: &EntryHash = dest.as_ref();
                vec![Err(DataIntegrityError::IndexNotFound(h.clone()))]
            },
        }
    })
}

pub (crate) fn create_dest_identities_and_indexes<'a, A, S, I>(
    source_entry_type: &'a I,
    source: &'a A,
    dest_entry_type: &'a I,
    link_tag: &'a S,
    link_tag_reciprocal: &'a S,
) -> Box<dyn for<'r> Fn(&A) -> Vec<RecordAPIResult<HeaderHash>> + 'a>
    where I: AsRef<str>,
        S: 'a + AsRef<[u8]>,
        A: AsRef<DnaHash> + AsRef<EntryHash> + From<(DnaHash, EntryHash)>,
{
    let base_method = create_dest_indexes(source_entry_type, source, dest_entry_type, link_tag, link_tag_reciprocal);

    Box::new(move |dest| {
        match create_entry_identity(dest_entry_type, dest) {
            Ok(_id_hash) => {
                base_method(dest)
            },
            Err(e) => vec![Err(e)],
        }
    })
}

/// Helper for index update to remove multiple destination links from some source.
pub (crate) fn delete_dest_indexes<'a, A, S, I>(
    source_entry_type: &'a I,
    source: &'a A,
    dest_entry_type: &'a I,
    link_tag: &'a S,
    link_tag_reciprocal: &'a S,
) -> Box<dyn for<'r> Fn(&A) -> Vec<RecordAPIResult<HeaderHash>> + 'a>
    where I: AsRef<str>,
        S: 'a + AsRef<[u8]>,
        A: Clone + Eq + std::hash::Hash + AsRef<DnaHash> + AsRef<EntryHash> + From<(DnaHash, EntryHash)>,
{
    Box::new(move |dest_addr| {
        match delete_index(source_entry_type, source, dest_entry_type, dest_addr, link_tag, link_tag_reciprocal) {
            Ok(deleted) => deleted,
            Err(_) => {
                let dest_hash: &EntryHash = dest_addr.as_ref();
                vec![Err(DataIntegrityError::IndexNotFound(dest_hash.clone()))]
            },
        }
    })
}
