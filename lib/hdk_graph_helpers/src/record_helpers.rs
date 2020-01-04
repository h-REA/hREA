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
    error::{ ZomeApiResult },
    link_entries,
    remove_entry,
    utils:: {
        get_as_type,    // :TODO: switch this method to one which doesn't consume the input
    },
};

use super::{
    identifiers::RECORD_INITIAL_ENTRY_LINK_TAG,
    type_wrappers::Addressable,
    record_interface::Updateable,
    entries::{
        create_entry,
        update_entry,
        delete_entry,
    },
    keys::{
        create_key_index,
        get_key_index_address,
        get_key_index_address_as_type,
    },
};



// CREATE

/// Creates a new record in the DHT, assigns it a predictable `base` (static) id, and returns a tuple of
/// the `base` id and initial record `entry` data.
/// It is recommended that you include a creation timestamp in newly created records, to avoid
/// them conflicting with previously entered entries that may be of the same content.
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




// READ

/// Read a record's entry data by its `base` (static) id.
pub fn read_record_entry<T: TryFrom<AppEntryValue>, A: AsRef<Address>>(
    address: &A,
) -> ZomeApiResult<T> {
    // read base entry to determine dereferenced entry address
    let data_address = get_key_index_address(address.as_ref());

    // return retrieval error or attempt underlying type fetch
    match data_address {
        Ok(addr) => get_as_type(addr),
        Err(e) => Err(e),
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
