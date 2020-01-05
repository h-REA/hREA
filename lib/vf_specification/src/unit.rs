use hdk::{
    holochain_json_api::{ json::JsonString, error::JsonError },
};
use holochain_json_derive::{ DefaultJson };

use hdk_graph_helpers::{
    MaybeUndefined,
    record_interface::{ Updateable, UniquelyIdentifiable, UpdateableIdentifier },
};

use vf_core::type_aliases::{
    UnitId,
};

//---------------- RECORD INTERNALS & VALIDATION ----------------

#[derive(Serialize, Deserialize, Debug, DefaultJson, Default, Clone)]
pub struct Entry {
    label: String,
    symbol: String,
}

impl<'a> Entry {
    pub fn get_symbol(&'a self) -> String {
        self.symbol.to_owned()
    }
}

//---------------- CREATE ----------------

/// I/O struct to describe the complete input record, including all managed links
#[derive(Serialize, Deserialize, Debug, DefaultJson, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    label: String,
    symbol: String,
}

impl<'a> CreateRequest {
    pub fn get_symbol(&'a self) -> &str {
        &self.symbol
    }

    // :TODO: accessors for field data
}

impl UniquelyIdentifiable for CreateRequest {
    fn get_anchor_key(&self) -> String {
        self.get_symbol().to_string()
    }
}

/// Pick relevant fields out of I/O record into underlying DHT entry
impl From<CreateRequest> for Entry {
    fn from(e: CreateRequest) -> Entry {
        Entry {
            label: e.label.into(),
            symbol: e.symbol.into(),
        }
    }
}

//---------------- UPDATE ----------------

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRequest {
    id: UnitId,
    label: MaybeUndefined<String>,
    symbol: MaybeUndefined<String>,
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

/// Handles update operations by merging any newly provided fields
impl Updateable<UpdateRequest> for Entry {
    fn update_with(&self, e: &UpdateRequest) -> Entry {
        Entry {
            label:   if !e.label.is_some()   { self.label.to_owned()   } else { e.label.to_owned().unwrap() },
            symbol: if !e.symbol.is_some() { self.symbol.to_owned() } else { e.symbol.to_owned().unwrap() },
        }
    }
}

//---------------- EXTERNAL API ----------------

/// I/O struct to describe the complete output record, including all managed link fields
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    id: UnitId,
    label: String,
    symbol: String,
}

/// I/O struct to describe what is returned outside the gateway
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData {
    unit: Response,
}

pub fn construct_response<'a>(
    id: &UnitId, e: &Entry
) -> ResponseData {
    ResponseData {
        unit: Response {
            // entry fields
            id: id.to_owned(),
            label: e.label.to_owned(),
            symbol: e.symbol.to_owned(),
        }
    }
}
