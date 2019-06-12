#![feature(try_from)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_core_types_derive;

#[macro_use]
pub mod vf_record;
pub mod type_aliases;
pub mod measurement;
pub mod location;
