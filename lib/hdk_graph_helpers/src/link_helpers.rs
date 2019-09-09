/**
 * Link-handling abstractions for Holochain apps
 *
 * Handles common behaviours for linking between data in different hApps,
 * in a way that is predictable, semantically meaningful and easy to reason about.
 *
 * @package HoloREA
 * @since   2019-07-03
 */
use std::borrow::Cow;
use std::convert::{ TryFrom };
use hdk::{
    holochain_persistence_api::cas::content::Address,
    holochain_core_types::{
        entry::Entry::App as AppEntry,
        entry::AppEntryValue,
        entry::entry_type::AppEntryType,
        link::LinkMatch,
        link::LinkMatch::Exactly,
    },
    holochain_wasm_utils::api_serialization::get_links::GetLinksOptions,
    error::{ ZomeApiError, ZomeApiResult },
    link_entries,
    get_entry,
    get_links,
    get_links_with_options,
    remove_link,
};

use super::{
    MaybeUndefined,
    records::{
        create_base_entry,
    },
};



// CREATE

/// Creates a 'remote' query index used for following a link from some external record
/// into records contained within the current DNA / zome.
///
/// This basically consists of a `base` address for the remote content and bidirectional
/// links between it and its target_base_addresses.
pub fn create_remote_query_index<T>(
    remote_base_entry_type: T,
    origin_relationship_link_type: &str,
    origin_relationship_link_tag: &str,
    destination_relationship_link_type: &str,
    destination_relationship_link_tag: &str,
    source_base_address: &Address,
    target_base_addresses: &Vec<Address>,
) -> ZomeApiResult<Vec<Address>>
    where T: Into<AppEntryType>,
{
    // create a base entry pointer for the referenced origin record
    let base_address = create_base_entry(&(remote_base_entry_type.into()), &source_base_address).unwrap();

    // link all referenced records to our pointer to the remote origin record
    let results = target_base_addresses.iter()
        .map(|target_address| {
            // link origin record to local records by specified edge
            link_entries_bidir(
                &base_address, &target_address,
                origin_relationship_link_type, origin_relationship_link_tag,
                destination_relationship_link_type, destination_relationship_link_tag
            );

            target_address.to_owned()
        })
        .collect();

    Ok(results)
}

/// Creates a 'local' query index used for fetching and querying pointers to other
/// records that are stored externally to this DNA / zome.
///
/// In the local DNA, this consists of `base` addresses for all referenced foreign
/// content, bidirectionally linked to the originating record for querying in either direction.
///
/// In the remote DNA, a corresponding remote query index is built via `create_remote_query_index`,
/// which is presumed to be linked to the other end of the specified `remote_zome_method`.
pub fn create_local_query_index(
    remote_base_entry_type: &str,
    origin_relationship_link_type: &str,
    origin_relationship_link_tag: &str,
    destination_relationship_link_type: &str,
    destination_relationship_link_tag: &str,
    source_base_address: &Address,
    target_base_addresses: &Vec<Address>,
) -> Vec<ZomeApiResult<Address>> {
    // abort if target_base_addresses are empty
    if target_base_addresses.len() == 0 { return vec![] }

    // Build local index first (for reading linked record IDs from the `source_base_address`)
    let results: Vec<ZomeApiResult<Address>> = target_base_addresses.iter()
        .map(|base_entry_addr| {
            // create a base entry pointer for the referenced commitment
            let base_entry_result = create_base_entry(&(remote_base_entry_type.to_string().into()), base_entry_addr);

            match &base_entry_result {
                Ok(base_address) => {
                    // link event to commitment by `fulfilled`/`fulfilledBy` edge
                    link_entries_bidir(
                        &source_base_address, base_address,
                        origin_relationship_link_type, origin_relationship_link_tag,
                        destination_relationship_link_type, destination_relationship_link_tag
                    );
                },
                _ => (),
            }

            base_entry_result
        })
        .collect();

    results
}

/// Creates a bidirectional link between two entry addresses, and returns a vector
/// of the addresses of the (respectively) forward & reciprocal links created.
pub fn link_entries_bidir<S: Into<String>>(
    source: &Address,
    dest: &Address,
    link_type: S,
    link_name: S,
    link_type_reciprocal: S,
    link_name_reciprocal: S,
) -> Vec<Address> {
    vec! [
        link_entries(source, dest, link_type, link_name).unwrap(),
        link_entries(dest, source, link_type_reciprocal, link_name_reciprocal).unwrap(),
    ]
}



// READ

/// Load any set of records of type `R` that are linked from the
/// `base_address` entry via `link_type` and `link_name`.
///
/// Results are automatically deserialized into `R` as they are retrieved from the DHT.
/// Any entries that either fail to load or cannot be converted to the type will be dropped.
///
/// :TODO: return errors, improve error handling
///
pub fn get_links_and_load_entry_data<R, F, A>(
    base_address: &F,
    link_type: &str,
    link_name: &str,
) -> ZomeApiResult<Vec<(A, Option<R>)>>
    where R: Clone + TryFrom<AppEntryValue>,
        A: From<Address>,
        F: AsRef<Address>,
{
    let get_links_result = get_links_with_options(
        base_address.as_ref(),
        LinkMatch::Exactly(link_type),
        LinkMatch::Exactly(link_name),
        GetLinksOptions::default(),
    )?;

    let addresses = get_links_result.addresses();

    let entries: Vec<Option<R>> = addresses.iter()
        .map(|address| {
            let entry_address = get_entry(&address)?;
            let entry = match entry_address {
                Some(AppEntry(_, entry_address_value)) => {
                    get_entry(&Address::try_from(entry_address_value)?)
                },
                _ => Err(ZomeApiError::Internal("Could not locate entry".to_string())),
            };
            match entry {
                Ok(Some(AppEntry(_, entry_value))) => {
                    match R::try_from(entry_value.to_owned()) {
                        Ok(val) => Ok(Some(val)),
                        Err(_) => Err(ZomeApiError::Internal("Could not convert get_links result to requested type".to_string())),
                    }
                },
                _ => Err(ZomeApiError::Internal("Could not locate entry".to_string())),
            }
        })
        .filter_map(Result::ok)
        .collect();

    Ok(addresses.iter()
        .map(|address| {
            address.to_owned().into()
        })
        .zip(entries)
        .collect()
    )
}

/// Load any set of addresses of type `T` that are linked from the
/// `base_address` entry via `link_type` and `link_name`.
///
/// The `Address` array is returned wrapped in a copy-on-write smart pointer
/// such that it can be cheaply passed by reference into Serde output
/// structs & functions via `Cow::into_owned`.
///
/// :TODO: propagate errors in link retrieval
///
/// @see `type_aliases.rs`
///
pub fn get_linked_addresses_as_type<'a, T, I>(
    base_address: I,
    link_type: &str,
    link_tag: &str,
) -> Cow<'a, Vec<T>>
    where T: From<Address> + Clone, I: AsRef<Address>
{
    Cow::Owned(get_links(base_address.as_ref(), Exactly(link_type), Exactly(link_tag))
        .unwrap()   // :TODO: handle errors gracefully
        .addresses()
        .iter()
        .map(|addr| { T::from(addr.to_owned()) })
        .collect())
}



// UPDATE

/// Remove any links of `link_type`/`link_name` and their reciprocal links of
/// `link_type_reciprocal`/`link_name_reciprocal` that might be present on `source`;
/// then add new links of the given types & names that instead point from `source`
/// to `new_dest`.
///
/// If `new_dest` is `MaybeUndefined::None`, the links are simply removed.
/// If `new_dest` is `MaybeUndefined::Undefined`, this is a no-op.
///
/// Returns the addresses of the newly inserted link entries, if any.
///
/// :TODO: propagate errors to allow non-critical failures in calling code
///
pub fn replace_entry_link_set<A, B>(
    source: &A,
    new_dest: &MaybeUndefined<B>,
    link_type: &str,
    link_name: &str,
    link_type_reciprocal: &str,
    link_name_reciprocal: &str,
) -> Vec<Address>
    where A: AsRef<Address> + From<Address> + Clone,
        B: AsRef<Address> + From<Address> + Clone + PartialEq,
{
    // if not updating, skip operation
    if let MaybeUndefined::Undefined = new_dest {
        return vec![];
    }

    // load any existing linked entries from the originating address
    let existing_links: Vec<B> = get_linked_addresses_as_type(
        source, link_type, link_name,
    ).into_owned();

    let _ = existing_links.iter()
        // determine links to erase
        .filter(|&existing_link| {
            match &new_dest {
                // erase if new value differs from current value
                &MaybeUndefined::Some(new_link) => {
                    *new_link != *existing_link
                },
                // erase if new value is None (note we abort on Undefined)
                _ => true,
            }
        })
        // wipe stale links
        .for_each(|remove_link| {
            remove_entries_bidir(
                source.as_ref(), remove_link.as_ref(),
                link_type, link_name,
                link_type_reciprocal, link_name_reciprocal,
            );
        });

    // run insert if needed
    match new_dest {
        MaybeUndefined::Some(new_link) => {
            let already_present = existing_links.iter().filter(|&preexisting| {
                *new_link == *preexisting
            }).count() > 0;

            if already_present {
                vec![]    // return empty vector for no-op
            } else {
                link_entries_bidir(
                    source.as_ref(), new_link.as_ref(),
                    link_type, link_name,
                    link_type_reciprocal, link_name_reciprocal
                )
            }
        },
        _ => vec![],    // return empty vector for no-op
    }
}



// DELETE

/// Deletes a bidirectional link between two entry addresses, and returns any errors encountered
/// to the caller.
///
/// :TODO: filter empty success tuples from results and return as flattened error array
///
pub fn remove_entries_bidir<S: Into<String>>(
    source: &Address,
    dest: &Address,
    link_type: S,
    link_name: S,
    link_type_reciprocal: S,
    link_name_reciprocal: S,
) -> Vec<ZomeApiResult<()>> {
    vec! [
        remove_link(source, dest, link_type, link_name),
        remove_link(dest, source, link_type_reciprocal, link_name_reciprocal),
    ]
}
