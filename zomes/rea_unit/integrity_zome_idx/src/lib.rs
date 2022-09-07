/**
 * hREA indexing integrity zome for API definition
 *
 * Defines the top-level zome configuration needed by Holochain's build system
 * to bundle the app. This basically involves wiring up the helper methods from the
 * related `_lib` module into a packaged zome WASM binary.
 *
 * @package hREA
 */

// Add the extern function that tells Holochain how many links this zome has.
// this is the equivalent of the thing that we prevent from happening in the 'core' itself
// when we specified #[hdk_link_types(skip_no_mangle = true)]
#[no_mangle]
pub fn __num_link_types() -> u8 {
    hdk_semantic_indexes_core::LinkTypes::len()
}