use std::borrow::Cow;
use hdk::{
    holochain_json_api::{ json::JsonString, error::JsonError },
};
use holochain_json_derive::{ DefaultJson };

use hdk_graph_helpers::{
    MaybeUndefined,
    record_interface::Updateable,
};

use vf_core::type_aliases::{
    ExternalURL,
    ResourceSpecificationAddress,
    ResourceAddress,
};

//---------------- RECORD INTERNALS & VALIDATION ----------------

#[derive(Serialize, Deserialize, Debug, DefaultJson, Default, Clone)]
pub struct Entry {
    name: String,
    image: Option<ExternalURL>,
    note: Option<String>,
}

//---------------- CREATE ----------------

/// I/O struct to describe the complete input record, including all managed links
#[derive(Serialize, Deserialize, Debug, DefaultJson, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    name: String,
    #[serde(default)]
    image: MaybeUndefined<ExternalURL>,
    #[serde(default)]
    note: MaybeUndefined<String>,
}

impl<'a> CreateRequest {
}

/// Pick relevant fields out of I/O record into underlying DHT entry
impl From<CreateRequest> for Entry {
    fn from(e: CreateRequest) -> Entry {
        Entry {
            name: e.name.into(),
            image: e.image.into(),
            note: e.note.into(),
        }
    }
}

//---------------- UPDATE ----------------

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRequest {
    id: ResourceSpecificationAddress,
    #[serde(default)]
    name: MaybeUndefined<String>,
    #[serde(default)]
    image: MaybeUndefined<ExternalURL>,
    #[serde(default)]
    note: MaybeUndefined<String>,
}

impl<'a> UpdateRequest {
    pub fn get_id(&'a self) -> &ResourceSpecificationAddress {
        &self.id
    }
    // :TODO: accessors for other field data
}

/// Handles update operations by merging any newly provided fields
impl Updateable<UpdateRequest> for Entry {
    fn update_with(&self, e: &UpdateRequest) -> Entry {
        Entry {
            name: if !e.name.is_some() { self.name.to_owned() } else { e.name.to_owned().unwrap() },
            image: if e.image.is_undefined() { self.image.to_owned() } else { e.image.to_owned().into() },
            note: if e.note.is_undefined() { self.note.to_owned() } else { e.note.to_owned().into() },
        }
    }
}

//---------------- EXTERNAL API ----------------

/// I/O struct to describe the complete output record, including all managed link fields
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    id: ResourceSpecificationAddress,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<ExternalURL>,
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<String>,
}

/// I/O struct to describe what is returned outside the gateway
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData {
    resource_specification: Response,
}

/// Create response from input DHT primitives
pub fn construct_response<'a>(
    address: &ResourceSpecificationAddress,
    e: &Entry,
    // :TODO: link conforming resources in associated link registry DNA module
    _conforming_resources : Option<Cow<'a, Vec<ResourceAddress>>>
) -> ResponseData {
    ResponseData {
        resource_specification: Response {
            // entry fields
            id: address.to_owned(),
            name: e.name.to_owned(),
            image: e.image.to_owned(),
            note: e.note.to_owned(),

            // conforming_resources: conforming_resources.map(Cow::into_owned),
        }
    }
}
