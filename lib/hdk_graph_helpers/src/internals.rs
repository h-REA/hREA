/**
 * Internal filter predicates & utility methods for link update logic
 *
 * @see     ../../README.md
 * @package HDK Graph Helpers
 * @since   2019-09-10
 */
use hdk3::prelude::*;

use crate::MaybeUndefined;

/// Filter predicate to include all links which do not match the destination link
pub (crate) fn link_does_not_match<'a, B>(new_dest: &'a MaybeUndefined<B>) -> Box<dyn for<'r> Fn(&'r &'a B) -> bool + 'a>
    where B: AsRef<EntryHash> + From<EntryHash> + Clone + PartialEq,
{
    Box::new(move |&existing_link| {
        match &new_dest {
            // return comparison of existing value & newly provided value
            &MaybeUndefined::Some(new_link) => {
                *new_link != *existing_link
            },
            // nothing matches if new value is None or Undefined
            _ => true,
        }
    })
}

/// Filter predicate to include all links which match the destination link
pub (crate) fn link_matches<'a, B>(new_dest: &'a MaybeUndefined<B>) -> Box<dyn for<'r> Fn(&'r &'a B) -> bool + 'a>
    where B: AsRef<EntryHash> + From<EntryHash> + Clone + PartialEq,
{
    let inverse_filter = link_does_not_match(new_dest);
    Box::new(move |&existing_link| {
        !inverse_filter(&existing_link)
    })
}
