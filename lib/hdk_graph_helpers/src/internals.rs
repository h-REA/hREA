/**
 * Internal filter predicates & utility methods for link update logic
 *
 * @see     ../../README.md
 * @package HDK Graph Helpers
 * @since   2019-09-10
 */
use hdk::{
    holochain_persistence_api::cas::content::Address,
    error::{ ZomeApiResult },
};

use crate::{
    MaybeUndefined,
    keys::{
        get_key_index_address,
    },
    local_indexes::{
        delete_direct_index,
    },
};

/// Filter predicate to include all links which do not match the destination link
pub (crate) fn link_does_not_match<'a, B>(new_dest: &'a MaybeUndefined<B>) -> Box<dyn for<'r> Fn(&'r &'a B) -> bool + 'a>
    where B: AsRef<Address> + From<Address> + Clone + PartialEq,
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
    where B: AsRef<Address> + From<Address> + Clone + PartialEq,
{
    let inverse_filter = link_does_not_match(new_dest);
    Box::new(move |&existing_link| {
        !inverse_filter(&existing_link)
    })
}

/// Filter predicate to include all links which do not reference an entry containing the destination link
pub (crate) fn dereferenced_link_does_not_match<'a, B>(new_dest: &'a MaybeUndefined<B>) -> Box<dyn for<'r> Fn(&'r &'a B) -> bool + 'a>
    where B: AsRef<Address> + From<Address> + Clone + PartialEq,
{
    Box::new(move |&existing_link| {
        let existing_target = get_key_index_address(existing_link.as_ref());
        match &new_dest {
            &MaybeUndefined::Some(new_link) => {
                // ignore any current values which encounter errors
                if let Err(_) = existing_target {
                    return false;   // :NOTE: this should never happen if all write logic goes OK
                }
                // return comparison of existing (dereferenced) value & newly provided value
                *new_link != existing_target.unwrap().into()
            },
            // nothing matches if new value is None or Undefined
            _ => true,
        }
    })
}

/// Filter predicate to include all links which reference an entry containing the destination link
pub (crate) fn dereferenced_link_matches<'a, B>(new_dest: &'a MaybeUndefined<B>) -> Box<dyn for<'r> Fn(&'r &'a B) -> bool + 'a>
    where B: AsRef<Address> + From<Address> + Clone + PartialEq,
{
    let inverse_filter = dereferenced_link_does_not_match(new_dest);
    Box::new(move |&existing_link| {
        !inverse_filter(&existing_link)
    })
}


/// Iterator processor to wipe all links originating from a given `source` address
pub (crate) fn wipe_links_from_origin<'a, A, B>(
    link_type: &'a str,
    link_name: &'a str,
    link_type_reciprocal: &'a str,
    link_name_reciprocal: &'a str,
    source: &'a A,
) -> Box<dyn Fn(&'a B) -> Vec<ZomeApiResult<()>> + 'a>
    where A: AsRef<Address>,
        B: AsRef<Address> + From<Address> + Clone + PartialEq,
{
    Box::new(move |remove_link| {
        delete_direct_index(
            source.as_ref(), remove_link.as_ref(),
            link_type, link_name,
            link_type_reciprocal, link_name_reciprocal,
        )
    })
}
