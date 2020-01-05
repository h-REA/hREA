/**
 * Record-handling abstractions for Holochain apps
 *
 * Allows for data layers which behave like a traditional graph database, where
 * 'records' are the core abstraction, managed by complex arrangements of DHT
 * entries and links.
 *
 * @package HoloREA
 * @since   2019-07-02
 */

use std::convert::TryFrom;
use hdk::{
    holochain_persistence_api::cas::content::Address,
    holochain_core_types::{
        entry::{
            entry_type::AppEntryType,
            AppEntryValue,
        },
    },
    error::{ ZomeApiResult, ZomeApiError },
    link_entries,
    get_entry,
    remove_entry,
};

use super::{
    identifiers::RECORD_INITIAL_ENTRY_LINK_TAG,
    type_wrappers::Addressable,
    record_interface::{ Updateable, UniquelyIdentifiable },
    entries::{
        create_entry,
        try_decode_entry,
        update_entry,
        delete_entry,
    },
    keys::{
        create_key_index,
        get_key_index_address,
        get_key_index_address_as_type,
    },
    anchors::{
        create_anchor_index,
        get_anchor_index_entry_address,
    },
};



// CREATE

/// Creates a new record in the DHT, assigns it a predictable `key index` (static id),
/// and returns a tuple of the `key index` address and initial record `entry` data.
/// The `key index` address then becomes the identifier by which this record should be
/// referred to hereafter.
///
/// It is recommended that you include a creation timestamp in newly created records, to avoid
/// them conflicting with previously entered entries that may be of the same content.
///
pub fn create_record<E, C, A, S>(
    base_entry_type: S,
    entry_type: S,
    initial_entry_link_type: &str,
    create_payload: C,
) -> ZomeApiResult<(A, E)>
    where E: Clone + Into<AppEntryValue>,
        C: Into<E>,
        S: Into<AppEntryType>,
        A: From<Address>,
{
    // write underlying entry
    let (address, entry_resp) = create_entry(entry_type, create_payload)?;

    // create a key index pointer
    let base_address = create_key_index(&(base_entry_type.into()), &address)?;
    // :NOTE: link is just for inference by external tools, it's not actually needed to query
    link_entries(&base_address, &address, initial_entry_link_type, RECORD_INITIAL_ENTRY_LINK_TAG)?;

    Ok((A::from(base_address), entry_resp))
}

/// Creates a new record in the DHT and assigns it a manually specified `anchor index`
/// that can be used like a primary key. The `create_payload` must also implement
/// `UniquelyIdentifiable` in order to derive the unique `anchor index` value.
///
/// It is recommended that you include a creation timestamp in newly created records, to avoid
/// them conflicting with previously entered entries that may be of the same content.
///
pub fn create_anchored_record<E, C, S>(
    base_entry_type: S,
    entry_type: S,
    initial_entry_link_type: &str,
    create_payload: C,
) -> ZomeApiResult<(String, E)>
    where E: Clone + Into<AppEntryValue>,
        C: Into<E> + UniquelyIdentifiable,
        S: Into<AppEntryType>,
{
    // determine unique anchor index key
    // :TODO: deal with collisions
    let entry_id = create_payload.get_anchor_key();

    // write underlying entry
    let (address, entry_resp) = create_entry(entry_type, create_payload)?;

    // write primary key index
    let _ = create_anchor_index(&base_entry_type.into(), initial_entry_link_type, &entry_id, &address)?;

    Ok((entry_id, entry_resp))
}




// READ

/// Read a record's entry data by its `key index` (static id).
///
pub fn read_record_entry<T: TryFrom<AppEntryValue>, A: AsRef<Address>>(
    address: &A,
) -> ZomeApiResult<T> {
    // read base entry to determine dereferenced entry address
    let data_address = get_key_index_address(address.as_ref());

    // return retrieval error or attempt underlying type fetch
    match data_address {
        Ok(addr) => {
            let entry = get_entry(&addr);
            let decoded = try_decode_entry(entry);
            match decoded {
                Ok(Some(entry)) => {
                    Ok(entry)
                },
                _ => Err(ZomeApiError::Internal("Could not locate entry".to_string())),
            }
        },
        Err(e) => Err(e),
    }
}

/// Reads an entry via its `anchor index`.
///
/// Follows an anchor identified by `id_entry_type`, `id_link_type` and
/// its well-known `id_string` to retrieve whichever entry of type `T` resides
/// at the anchored address.
///
/// @see anchor_helpers.rs
///
pub fn read_anchored_record_entry<T, E>(
    id_entry_type: &E,
    id_link_type: &str,
    id_string: &String,
) -> ZomeApiResult<T>
    where E: Into<AppEntryType> + Clone,
        T: TryFrom<AppEntryValue>,
{
    // determine ID anchor entry address
    let entry_address = get_anchor_index_entry_address(id_entry_type, id_link_type, id_string)?;
    match entry_address {
        Some(address) => {
            let entry = get_entry(&address);
            let decoded = try_decode_entry(entry);
            match decoded {
                Ok(Some(entry)) => {
                    Ok(entry)
                },
                _ => Err(ZomeApiError::Internal("Could not locate entry".to_string())),
            }
        },
        None => Err(ZomeApiError::Internal("Could not locate entry".to_string())),
    }
}





// UPDATE

/// Updates a record in the DHT by its `base` (static) id.
///
/// The way in which the input update payload is applied to the existing
/// entry data is up to the implementor of `Updateable<U>` for the entry type.
///
pub fn update_record<E, U, A, S>(
    entry_type: S,
    address: &A,
    update_payload: &U,
) -> ZomeApiResult<E>
    where E: Clone + TryFrom<AppEntryValue> + Into<AppEntryValue> + Updateable<U>,
        S: Into<AppEntryType> + Clone,
        A: AsRef<Address>,
{
    // read base entry to determine dereferenced entry address
    let data_address: Addressable = get_key_index_address_as_type(address.as_ref())?;

    // perform regular entry update using internal address
    let (_addr, updated_entry): (Address, E) = update_entry(entry_type, &data_address, update_payload)?;

    Ok(updated_entry)
}




// DELETE

/// Removes a record of the given `base` from the DHT by marking it as deleted.
/// Links are not affected so as to retain a link to the referencing information, which may now need to be updated.
///
pub fn delete_record<T>(address: &dyn AsRef<Address>) -> ZomeApiResult<bool>
    where T: TryFrom<AppEntryValue>
{
    // read base entry to determine dereferenced entry address
    let data_address = get_key_index_address(address.as_ref());

    match data_address {
        // note that we're relying on the deletions to be paired in using this as an existence check
        Ok(addr) => {
            remove_entry(address.as_ref())?;
            delete_entry::<T>(&addr)
        },
        Err(_) => Ok(false),
    }
}
