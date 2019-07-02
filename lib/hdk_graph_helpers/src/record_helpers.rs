/**
 * Record-handling abstractions for Holochain apps
 *
 * Allows for data layers which behave like a traditional graph database, where
 * 'records' are the core abstraction, managed by complex arrangements of DHT
 * entries and links.
 *
 *
 * @package HoloREA
 * @since   2019-07-02
 */

use std::convert::TryFrom;
use hdk::{
    holochain_core_types::{
        cas::content::Address,
        entry::{
            Entry::App as AppEntry,
            entry_type::AppEntryType,
            AppEntryValue,
        },
        json::JsonString,
    },
    error::{ ZomeApiResult },
    commit_entry,
    remove_entry,
    link_entries,
    utils:: {
        get_as_type,    // :TODO: switch this method to one which doesn't consume the input
    },
};

use super::{
    LINK_TYPE_INITIAL_ENTRY,
    LINK_TAG_INITIAL_ENTRY,
};

/// Creates a new record in the DHT, assigns it a predictable base (static) `id`, and returns a tuple of
/// the static record `id` and initial record `entry` data.
/// It is recommended that you include a creation timestamp in newly created records, to avoid
/// them conflicting with previously entered entries that may be of the same content.
pub fn create_record<E, C, S>(
    base_entry_type: S,
    entry_type: S,
    create_payload: C,
) -> ZomeApiResult<(Address, E)>
    where E: Clone + Into<AppEntryValue>,
        C: Into<E>,
        S: Into<AppEntryType>,
{
    // convert the type's CREATE payload into internal struct via built-in conversion trait
    let entry_struct: E = create_payload.into();
    // clone entry for returning to caller
    // :TODO: should not need to do this if AppEntry stops consuming the value
    let entry_resp = entry_struct.clone();

    // write underlying entry and get initial address (which will become record ID)
    let entry = AppEntry(entry_type.into(), entry_struct.into());
    let address = commit_entry(&entry)?;

    // create a base entry pointer
    let base_address = create_base_entry(base_entry_type.into(), &address)?;
    // :NOTE: link is just for inference by external tools, it's not actually needed to query
    link_entries(&base_address, &address, LINK_TYPE_INITIAL_ENTRY, LINK_TAG_INITIAL_ENTRY)?;

    Ok((base_address, entry_resp))
}

/// Removes a record of the given `id` from the DHT by marking it as deleted.
/// Links are not affected so as to retain a link to the referencing information, which may now need to be updated.
pub fn delete_record<T>(address: &Address) -> ZomeApiResult<bool>
    where T: TryFrom<AppEntryValue>
{
    // read base entry to determine dereferenced entry address
    let entry_address = get_entry_address(&address);

    match entry_address {
        // note that we're relying on the deletions to be paired in using this as an existence check
        Ok(addr) => {
            let entry_data: ZomeApiResult<T> = get_as_type(addr.clone());
            match entry_data {
                Ok(_) => {
                    remove_entry(&address)?;
                    remove_entry(&addr)?;
                    Ok(true)
                },
                Err(e) => Err(e),
            }
        },
        Err(_) => Ok(false),
    }
}

/// Creates a `base` entry address- an entry consisting only of a pointer to some other referenced
/// `entry`. The address of the `base` entry (the alias the changing `entry` will be identified by
/// within this network) is returned.
pub fn create_base_entry(
    base_entry_type: AppEntryType,
    referenced_address: &Address,
) -> ZomeApiResult<Address> {
    let base_entry = AppEntry(base_entry_type.into(), referenced_address.into());
    commit_entry(&base_entry)
}

/// Query the `entry` address for a given `base` address.
fn get_entry_address(base_address: &Address) -> ZomeApiResult<Address> {
    get_as_type(base_address.clone())
}
