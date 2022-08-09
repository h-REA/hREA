use hdi::prelude::*;


// Here, we use skip_no_mangle so that
// this code is better suited for use as a library, than as a
// zome directly itself as a crate. Elsewhere, in your zomes
// you export the necessary extern, like so

// Add the extern function that says how many links this zome has.
// #[no_mangle]
// pub fn __num_link_types() -> u8 {
//     hdk_semantic_indexes_core::LinkTypes::len()
// }

#[hdk_link_types(skip_no_mangle = true)]
pub enum LinkTypes {
    IdentityAnchor,
    SemanticIndex,
    TimeIndex,
}
