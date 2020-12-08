/**
 * Internal filter predicates & utility methods for link update logic
 *
 * @see     ../../README.md
 * @package HDK Graph Helpers
 * @since   2019-09-10
 */
use hdk3::prelude::*;

use crate::{
    GraphAPIResult, DataIntegrityError,
    local_indexes::{create_index, delete_index},
};

/// Filter predicate to include any link present in provided destination list
pub (crate) fn link_matches<'a>(hashes: &'a [EntryHash]) -> Box<dyn for<'r> Fn(&'r &'a EntryHash) -> bool + 'a>
{
    Box::new(move |&existing_link| {
        hashes.iter().cloned().any(|h| h == *existing_link)
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

/// handles separation of errors from successful results in functions which process lists of things
pub (crate) fn result_partitioner<T>(i: &GraphAPIResult<T>) -> bool
    where T: Into<AnyDhtHash>,
{
    i.is_ok()
}

/// Returns the first error encountered (if any). Best used with the `?` operator.
pub (crate) fn throw_any_error<T>(mut errors: Vec<GraphAPIResult<T>>) -> GraphAPIResult<()> {
    if errors.len() == 0 {
        return Ok(());
    }
    let first_err = errors.pop().unwrap();
    Err(first_err.err().unwrap())
}
