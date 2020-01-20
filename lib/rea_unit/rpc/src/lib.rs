/**
 * Holo-REA measurement unit zome I/O data structures
 *
 * Required by packages wishing to interact with the zome via its standard RPC interface,
 * and by the zome API handlers accepting these parameters.
 *
 * @package Holo-REA
 */
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use holochain_json_api::{ json::JsonString, error::JsonError };
use holochain_json_derive::{ DefaultJson };

use hdk_graph_helpers::{
    MaybeUndefined,
    record_interface::{ Updateable, UniquelyIdentifiable, UpdateableIdentifier },
};
use vf_core::type_aliases::{
    // :TODO: import type IDs for entry `Address` reference fields
};

//---------------- EXTERNAL RECORD STRUCTURE ----------------

// Export external type interface to allow consuming zomes to easily import & define zome API
pub use vf_core::type_aliases::{ UnitAddress };

/// I/O struct to describe the complete record, including all managed link fields
///
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub id: UnitId,
    pub label: String,
    pub symbol: String,
}

/// I/O struct to describe what is returned outside the gateway.
/// Responses are usually returned as named attributes in order to leave space
/// for future additional return values.
///
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData {
    pub unit: Response,
}

//---------------- CREATE REQUEST ----------------

/// I/O struct to describe the complete input record, including all managed links
///
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    pub label: String,
    pub symbol: String,
}

impl<'a> CreateRequest {
    pub fn get_symbol(&'a self) -> &str {
        &self.symbol
    }
}

impl UniquelyIdentifiable for CreateRequest {
    fn get_anchor_key(&self) -> String {
        self.get_symbol().to_string()
    }
}

//---------------- UPDATE REQUEST ----------------

/// I/O struct to describe the complete input record, including all managed links
///
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRequest {
    pub id: UnitId,
    pub label: MaybeUndefined<String>,
    pub symbol: MaybeUndefined<String>,
}

impl<'a> UpdateRequest {
    pub fn get_id(&'a self) -> &UnitId {
        &self.id
    }

    pub fn get_symbol(&'a self) -> Option<String> {
        self.symbol.to_owned().to_option()
    }
}

impl UniquelyIdentifiable for UpdateRequest {
    fn get_anchor_key(&self) -> String {
        self.get_id().as_ref().to_string()
    }
}

impl UpdateableIdentifier for UpdateRequest {
    fn get_new_anchor_key(&self) -> Option<String> {
        self.get_symbol()
    }
}

//---------------- QUERY FILTER REQUEST ----------------

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    // :TODO:
}
