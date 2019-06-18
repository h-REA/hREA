#![feature(try_from)]
/**
 * Helper methods for managing Holochain DHT links and entries as graphs of information
 */
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate hdk;

use std::convert::TryFrom;

use hdk::{
    holochain_core_types::{
        cas::content::Address,
        json::JsonString,
        entry::{
            Entry,
            AppEntryValue,
            entry_type::AppEntryType,
        },
        error::HolochainError,
    },
    error::{ ZomeApiResult, ZomeApiError },
    commit_entry,
    call,
};
use holochain_core_types_derive::{ DefaultJson };

/// Type alias for dealing with entry fields that are not provided separately to nulls.
/// Used for update behaviour- null erases fields, undefined leaves them untouched.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MaybeUndefined<T> {
    None,
    Some(T),
    Undefined,
}

// helper method for pulling values out to regular Option
impl<T> MaybeUndefined<T> {
    pub fn as_option(&self) -> Option<&T> {
        match self {
            MaybeUndefined::Some(val) => Option::Some(val),
            _ => None,
        }
    }
}

// default to undefined, not null
// used by Serde to provide default values via `#[serde(default)]`
impl<T> Default for MaybeUndefined<T> {
    fn default() -> MaybeUndefined<T> {
        MaybeUndefined::Undefined
    }
}

/// Creates a "base" entry- and entry consisting only of a pointer to some other external
/// entry. The address of this entry (the alias it will be identified by within this network) is returned.
pub fn create_base_entry(
    base_entry_type: AppEntryType,
    referenced_address: &Address,
) -> Address {
    let base_entry = Entry::App(base_entry_type.into(), referenced_address.into());
    commit_entry(&base_entry).unwrap()
}

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
        hdk::link_entries(source, dest, link_type, link_name).unwrap(),
        hdk::link_entries(dest, source, link_type_reciprocal, link_name_reciprocal).unwrap(),
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
pub fn link_remote_entries<S, R>(
    target_dna_id: S,
    zome_name: S,
    cap_token: Address,
    fn_name: S,
    base_entry: &Address,
    target_entries: &Vec<Address>,
) -> R
  where S: Into<String>, R: TryFrom<AppEntryValue>
{
    let result: JsonString = call(target_dna_id, zome_name, cap_token, fn_name, RemoteEntryLinkRequest {
        base_entry: base_entry.clone().into(),
        target_entries: target_entries.clone().into(),
    }.into()).unwrap();

    let typed_entry = R::try_from(result.to_owned()).map_err(|_| {
        ZomeApiError::Internal("Could not convert link_remote_entries result to requested type".to_string())
    });

    typed_entry.unwrap()
}
