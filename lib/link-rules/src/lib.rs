/**
 * This module contains the LinkRules library, the spiritual descendent of LinkRepo.  LinkRules'
 * objects (should) encapsulate all activities that involve links.  If the LinkRules interface is
 * always used to add and remove links, the rules declared in that object will be followed without
 * any further boilerplate.
 *
 * - Reciprocal rules describe a pair of link tags that always come in `A tag B` and `B reciprocal A`.
 *  E.g. `A contains B` and `B inside A`.  If a `LinkRules` object has such a rule, there is no
 *  need to specify both tags.  Only one will suffice, and both links will be present.
 * - For any one entry, a tag with a Singular rule may only be one linked to one other unique object.
 *  E.g. if `A favorite_is B`, then there are no other links with the `favorite_is` tag from `A`.
 *  LinkRules will maintain this rule by removing the previous link before a second link is made.
 * - A Predicate rule describes a relationship that involves three entries with a pattern of links.
 *  E.g. `A child_of B` => `B child_of C ? A sibling_of C`.  A LinkRules with such a rule will
 *  complete that arrangement with just one call to `link()`.
 *
 * @author David Hand (sqykly@gmail.com)
 */

#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_core_types_derive;

use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    cas::content::Address,
    entry::Entry,
    dna::entry_types::Sharing,
    error::HolochainError,
    json::JsonString,
    validation::EntryValidationData
};
use std::collections;

#[macro_use]
mod link_set;
mod link_rules;

pub use link_set::{
    LinkSet,
    TakeOrLeave
};

pub use link_rules::LinkRules;
pub type Tag = String;
pub type HoloPtr = Address;

pub macro_rules! with_entry {
    (( $addr:expr ) {} -> {
        @null $if_null:block ;
        @exists ( $exists_name:ident ) $if_exists:block ;
        @fail ( $fail_name:ident ) $if_fail:block ;
    } ) => {
        match hdk::api::get_entry($addr) {
            Ok(entry) => {
                match entry {
                    Some($exists_name) => {
                        $if_exists
                    }
                    None => {
                        $if_null
                    }
                }
            }
            Err($fail_name) => {
                $if_fail
            }
        };
    };

    (($addr:expr) { $($section:tt)* }) => {
        with_entry!(($addr) {$($section)*} -> {
            @null { None } ;
            @exists (entry) { entry } ;
            @fail (error) { error } ;
        });
    };

    (($addr:expr) {
        hit ( $exists_name:ident ) $if_exists:block
        $($section:tt)*
    } -> {
        @null $if_null:block ;
        @exists ( $whatev:ident ) $dont_care:block ;
        @fail ( $fail_name:ident ) $if_fail:block ;
    }) => {
        with_entry!(($addr) { $($section)* } -> {
            @null $if_null ;
            @exists ($exists_name) $if_exists ;
            @fail ($fail_name) $if_fail ;
        })
    };

    (($addr:expr) {
        miss $if_null:block
        $($section:tt)*
    } -> {
        @null $dont_care:block ;
        @exists ( $exists_name:ident ) $if_exists:block ;
        @fail ( $fail_name:ident ) $if_fail:block ;
    }) => {
        with_entry!(($addr) { $($section)* } -> {
            @null $if_null ;
            @exists ($exists_name) $if_exists ;
            @fail ($fail_name) $if_fail ;
        })
    };

    (($addr:expr) {
        fail ( $fail_name:ident ) $if_fail:block
        $($sections:tt)*
    } -> {
        @null $if_null:block ;
        @exists ( $exists_name:ident ) $if_exists:block ;
        @fail ( $whatev:ident ) $dont_care:block ;
    }) => {
        with_entry!(($addr) {
            $($sections)*
        } -> {
            @null $if_null ;
            @exists ($exists_name) $if_exists ;
            @fail ($fail_name) $if_fail ;
        })
    };
};
