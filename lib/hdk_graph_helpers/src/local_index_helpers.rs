use hdk::{
    holochain_persistence_api::cas::content::Address,
    error::{ ZomeApiResult },
    link_entries,
    remove_link,
};

//--------------------------------[ READ ]--------------------------------------



//-------------------------------[ CREATE ]-------------------------------------

/// Creates a bidirectional link between two entry addresses, and returns a vector
/// of the addresses of the (respectively) forward & reciprocal links created.
pub fn create_direct_index<S: Into<String>>(
    source: &Address,
    dest: &Address,
    link_type: S,
    link_name: S,
    link_type_reciprocal: S,
    link_name_reciprocal: S,
) -> Vec<ZomeApiResult<Address>> {
    vec! [
        link_entries(source, dest, link_type, link_name),
        link_entries(dest, source, link_type_reciprocal, link_name_reciprocal),
    ]
}

//-------------------------------[ DELETE ]-------------------------------------

/// Deletes a bidirectional link between two entry addresses, and returns any errors encountered
/// to the caller.
///
/// :TODO: filter empty success tuples from results and return as flattened error array
///
pub fn delete_direct_index<S: Into<String>>(
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
