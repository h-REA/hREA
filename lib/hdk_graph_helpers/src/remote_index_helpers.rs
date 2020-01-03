/**
 * Helpers relating to `remote indexes`.
 *
 * A `remote index` is similar to a `local index`, except that it is composed of
 * two indexes which service queries on either side of the network boundary.
 *
 * On the `origin` side,
 *
 * @see     ../README.md
 * @package HDK Graph Helpers
 * @since   2019-05-16
 */
use hdk::{
    holochain_persistence_api::cas::content::Address,
    holochain_core_types::{
        entry::entry_type::AppEntryType,
    },
    error::{ ZomeApiResult },
};

use super::{
    keys::{
        create_key_index,
    },
    local_indexes::{
        create_direct_index,
    },
};

//--------------------------------[ READ ]--------------------------------------



//-------------------------------[ CREATE ]-------------------------------------

/// Creates a 'destination' query index used for following a link from some external record
/// into records contained within the current DNA / zome.
///
/// This basically consists of a `key index` for the remote content and bidirectional
/// links between it and its `target_base_addresses`.
///
/// :TODO: return any errors encountered in internal link creation
///
pub fn create_direct_remote_index_destination<'a, A, B>(
    remote_base_entry_type: &'a str,
    origin_relationship_link_type: &str,
    origin_relationship_link_tag: &str,
    destination_relationship_link_type: &str,
    destination_relationship_link_tag: &str,
    source_base_address: &A,
    target_base_addresses: Vec<B>,
) -> ZomeApiResult<Vec<ZomeApiResult<Address>>>
    where A: AsRef<Address> + From<Address> + Clone,
        B: AsRef<Address> + From<Address> + Clone + PartialEq,
{
    // create a base entry pointer for the referenced origin record
    let base_entry: AppEntryType = remote_base_entry_type.to_string().into();
    let base_resp = create_key_index(&base_entry, source_base_address.as_ref());
    if let Err(base_creation_failure) = base_resp {
        return Err(base_creation_failure);
    }
    let base_address = base_resp.unwrap();

    // link all referenced records to our pointer to the remote origin record
    Ok(target_base_addresses.iter()
        .map(|target_address| {
            // link origin record to local records by specified edge
            create_direct_index(
                &base_address, target_address.as_ref(),
                origin_relationship_link_type, origin_relationship_link_tag,
                destination_relationship_link_type, destination_relationship_link_tag
            );

            Ok(target_address.as_ref().clone())
        })
        .collect()
    )
}

/// Creates a 'origin' query index used for fetching and querying pointers to other
/// records that are stored externally to this DNA / zome.
///
/// In the local DNA, this consists of `key index` addresses for all referenced foreign
/// content, bidirectionally linked to the originating record for querying in either direction.
///
/// In the remote DNA, a corresponding remote query index is built via `create_direct_remote_index_destination`,
/// which is presumed to be linked to the other end of the specified `remote_zome_method`.
///
/// :TODO: return any errors encountered in internal link creation
///
pub (crate) fn create_direct_remote_index_origin(
    remote_base_entry_type: &str,
    origin_relationship_link_type: &str,
    origin_relationship_link_tag: &str,
    destination_relationship_link_type: &str,
    destination_relationship_link_tag: &str,
    source_base_address: &Address,
    target_base_addresses: Vec<Address>,
) -> Vec<ZomeApiResult<Address>> {
    // abort if target_base_addresses are empty
    if target_base_addresses.len() == 0 { return vec![] }

    // Build local index first (for reading linked record IDs from the `source_base_address`)
    let results: Vec<ZomeApiResult<Address>> = target_base_addresses.iter()
        .map(|base_entry_addr| {
            // create a base entry pointer for the referenced commitment
            let base_entry_result = create_key_index(&(remote_base_entry_type.to_string().into()), base_entry_addr);

            match &base_entry_result {
                Ok(base_address) => {
                    // link event to commitment by `fulfilled`/`fulfilledBy` edge
                    create_direct_index(
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
