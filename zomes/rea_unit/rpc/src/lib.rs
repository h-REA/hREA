/**
 * Holo-REA measurement unit zome I/O data structures
 *
 * Required by packages wishing to interact with the zome via its standard RPC interface,
 * and by the zome API handlers accepting these parameters.
 *
 * @package Holo-REA
 */
use holochain_serialized_bytes::prelude::*;

use hdk_records::{
    MaybeUndefined, RecordAPIResult,
    record_interface::{ UniquelyIdentifiable, UpdateableIdentifier },
};

//---------------- EXTERNAL RECORD STRUCTURE ----------------

// Export external type interface to allow consuming zomes to easily import & define zome API
pub use vf_attributes_hdk::{
    ActionHash,
    UnitId,
    UnitInternalAddress as UnitAddress,
    ByRevision, RevisionMeta,
};

/// I/O struct to describe the complete record, including all managed link fields
///
#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub id: UnitId,
    pub revision_id: ActionHash,
    pub meta: RevisionMeta,
    pub label: String,
    pub symbol: String,
}

impl<'a> Response {
    pub fn into_cursor(&'a self) -> Result<String, std::string::FromUtf8Error> {
        let s: &String = self.id.as_ref();
        Ok(s.to_owned())
    }
}

/// I/O struct to describe what is returned outside the gateway.
/// Responses are usually returned as named attributes in order to leave space
/// for future additional return values.
///
#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData {
    pub unit: Response,
}

//---------------- CREATE REQUEST ----------------

/// I/O struct to describe the complete input record, including all managed links
///
#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
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
    fn get_anchor_key(&self) -> RecordAPIResult<String> {
        Ok(self.get_symbol().to_string())
    }
}

//---------------- UPDATE REQUEST ----------------

/// I/O struct to describe the complete input record, including all managed links
///
#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRequest {
    pub revision_id: ActionHash,
    pub label: MaybeUndefined<String>,
    pub symbol: MaybeUndefined<String>,
}

impl<'a> UpdateRequest {
    pub fn get_revision_id(&'a self) -> &ActionHash {
        &self.revision_id
    }

    pub fn get_symbol(&'a self) -> Option<String> {
        self.symbol.to_owned().to_option()
    }
}

impl UpdateableIdentifier for UpdateRequest {
    fn get_new_anchor_key(&self) -> Option<String> {
        self.get_symbol()
    }
}

//---------------- QUERY FILTER REQUEST ----------------

#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    // :TODO:
}
