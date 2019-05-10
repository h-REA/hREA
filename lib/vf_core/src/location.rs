use hdk::holochain_core_types::{
    json::JsonString,
    cas::content::Address,
    error::HolochainError,
};
use hdk::holochain_core_types_derive::{ DefaultJson };

vfRecord! {
    pub struct Location {
        name: String,
        address: Option<String>,
        latitude: Option<f32>,
        longitude: Option<f32>,
    }
}
