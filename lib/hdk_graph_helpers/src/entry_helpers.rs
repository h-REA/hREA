/**
 * Helpers related to management of low-level Holochain entries.
 *
 * @see     ../README.md
 * @package HDK Graph Helpers
 * @since   2019-05-16
 */
use std::convert::{ TryFrom };
use hdk::{
    holochain_persistence_api::cas::content::Address,
    holochain_core_types::{
        entry::Entry,
        entry::Entry::App as AppEntry,
        entry::entry_type::AppEntryType,
        entry::AppEntryValue,
    },
    error::{ ZomeApiError, ZomeApiResult },
    get_entry,
    commit_entry,
};

//--------------------------------[ READ ]--------------------------------------

/// Loads up all entry data for the input list of `Addresses` and returns a vector
/// of tuples corresponding to the entry address and deserialized entry data.
///
pub fn get_entries_by_address<R, A>(addresses: Vec<Address>) -> ZomeApiResult<Vec<(A, Option<R>)>>
    where R: Clone + TryFrom<AppEntryValue>,
        A: From<Address>,
{
    let entries: Vec<Option<R>> = addresses.iter()
        .map(|address| {
            let entry = get_entry(&address);
            try_decode_entry(entry)
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

/// Loads up all entry data for the input list of `key indexes` and returns a vector
/// of tuples corresponding to the entry key's address and deserialized entry data.
///
pub fn get_entries_by_key_index<R, A>(addresses: Vec<Address>) -> ZomeApiResult<Vec<(A, Option<R>)>>
    where R: Clone + TryFrom<AppEntryValue>,
        A: From<Address>,
{
    let entries: Vec<Option<R>> = addresses.iter()
        .map(|address| {
            let entry_address = get_entry(&address)?;
            let entry = match entry_address {
                Some(AppEntry(_, entry_address_value)) => {
                    get_entry(&Address::try_from(entry_address_value)?)
                },
                _ => Err(ZomeApiError::Internal("Could not locate entry".to_string())),
            };

            try_decode_entry(entry)
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

/// Helper for handling decoding of entry data to requested entry struct type
///
pub fn try_decode_entry<R>(entry: ZomeApiResult<Option<Entry>>) -> ZomeApiResult<Option<R>>
    where R: TryFrom<AppEntryValue>,
{
    match entry {
        Ok(Some(AppEntry(_, entry_value))) => {
            match R::try_from(entry_value.to_owned()) {
                Ok(val) => Ok(Some(val)),
                Err(_) => Err(ZomeApiError::Internal("Could not convert entry to requested type".to_string())),
            }
        },
        _ => Err(ZomeApiError::Internal("Could not locate entry".to_string())),
    }
}

//-------------------------------[ CREATE ]-------------------------------------

/// Creates a new entry in the DHT and returns a tuple of
/// the `entry address` and initial record `entry` data.
///
/// It is recommended that you include a creation timestamp in newly created records, to avoid
/// them conflicting with previously entered entries that may be of the same content.
///
pub fn create_entry<E, C, S>(
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

    // write underlying entry and get initial address
    let entry = AppEntry(entry_type.into(), entry_struct.into());
    let address = commit_entry(&entry)?;

    Ok((address, entry_resp))
}
