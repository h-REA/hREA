#![feature(try_from)]
/**
 * Helper methods for managing Holochain DHT links and entries as graphs of information
 */
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate hdk;

mod maybe_undefined;
pub mod record_helpers;
pub mod record_interface;

use std::convert::TryFrom;
use std::convert::TryInto;

use hdk::{
    holochain_core_types::{
        cas::content::Address,
        json::JsonString,
        entry::{
            AppEntryValue,
        },
        error::HolochainError,
    },
    error::{ ZomeApiError, ZomeApiResult },
    link_entries,
    call,
};
use holochain_core_types_derive::{ DefaultJson };

// submodule exports

pub use maybe_undefined::MaybeUndefined as MaybeUndefined;
pub use record_helpers as records;

// Holochain DHT record & link type names

pub const LINK_TYPE_INITIAL_ENTRY: &str = "record_initial_entry";
pub const LINK_TAG_INITIAL_ENTRY: &str = LINK_TYPE_INITIAL_ENTRY;

// :TODO: move link management logic into own submodule

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

// :TODO: move these out to shared lib for API gateway types
/// Common request format for linking remote entries in cooperating DNAs
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
struct RemoteEntryLinkRequest {
    base_entry: Address,
    target_entries: Vec<Address>,
}

/// Calls into a neighbouring DNA to link a "base entry" for the given entry ID
/// to multiple target entries, via a zome API request conforming to `RemoteEntryLinkRequest`.
/// This enables the DNA holding the target entries to setup data structures
/// for querying the associated remote entry records back out.
pub fn link_remote_entries<R, S>(
    target_dna_id: S,
    zome_name: S,
    cap_token: Address,
    fn_name: S,
    base_entry: &Address,
    target_entries: &Vec<Address>,
) -> ZomeApiResult<R>
  where S: Into<String>,
        R: TryFrom<AppEntryValue>,
{
    let result = call(target_dna_id, zome_name, cap_token, fn_name, RemoteEntryLinkRequest {
        base_entry: base_entry.clone().into(),
        target_entries: target_entries.clone().into(),
    }.into())?;

    result.try_into().map_err(|_| {
        ZomeApiError::Internal("Could not convert link_remote_entries result to requested type".to_string())
    })
}
