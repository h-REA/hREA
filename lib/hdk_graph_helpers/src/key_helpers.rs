use hdk::{
    holochain_persistence_api::cas::content::Address,
    holochain_core_types::{
        entry::Entry::App as AppEntry,
        entry::entry_type::AppEntryType,
    },
    error::{ ZomeApiResult },
    commit_entry,
    utils:: {
        get_as_type,    // :TODO: switch this method to one which doesn't consume the input
    },
};

//--------------------------------[ READ ]--------------------------------------

/// Query the `entry` address for a given `key index` address and return the result in an Address
/// NewType wrapper of the expected type.
///
pub fn get_key_index_address_as_type<A>(key_address: &Address) -> ZomeApiResult<A>
    where A: AsRef<Address> + From<Address>,
{
    let result: ZomeApiResult<Address> = get_as_type(key_address.clone());

    match result {
        Ok(res) => Ok(res.into()),
        Err(e) => Err(e),
    }
}

/// Query the `entry` address for a given `key index` address and return as a raw Address
///
pub (crate) fn get_key_index_address(key_address: &Address) -> ZomeApiResult<Address> {
    get_as_type(key_address.clone())
}

//-------------------------------[ CREATE ]-------------------------------------

/// Creates a `key index`- an entry consisting only of a pointer to some other referenced
/// `entry`. The address of the `key index` entry (the alias the changing `entry` will be identified by
/// within this network) is returned.
pub (crate) fn create_key_index(
    base_entry_type: &AppEntryType,
    referenced_address: &Address,
) -> ZomeApiResult<Address> {
    let base_entry = AppEntry(base_entry_type.clone().into(), referenced_address.into());
    commit_entry(&base_entry)
}
