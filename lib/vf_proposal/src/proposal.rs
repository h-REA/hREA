use std::borrow::Cow;
use hdk::{
    holochain_json_api::{ json::JsonString, error::JsonError },
};
use holochain_json_derive::{ DefaultJson };

use hdk_graph_helpers::{
    MaybeUndefined,
    record_interface::Updateable,
    links::{
        get_linked_addresses_with_foreign_key_as_type,
    }
};

use vf_core::type_aliases::{
    ProposedIntentAddress,
    ProposalAddress,
    Timestamp,
};

use super::identifiers::{
    PROPOSAL_PUBLISHES_LINK_TYPE,
    PROPOSAL_PUBLISHES_LINK_TAG,
};

//---------------- RECORD INTERNALS & VALIDATION ----------------

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Entry {
    name: Option<String>,
    has_beginning: Option<Timestamp>,
    has_end: Option<Timestamp>,
    unit_based: Option<bool>,
    created: Option<Timestamp>,
    note: Option<String>,
    //[TODO]:
    //eligibleLocation: SpatialThing
    //inScopeOf: [AnyType!]
    //publishes: [ProposedIntent!]
}

//---------------- CREATE ----------------

/// I/O struct to describe the complete input record, including all managed links
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    #[serde(default)]
    name: MaybeUndefined<String>,
    #[serde(default)]
    has_beginning: MaybeUndefined<Timestamp>,
    #[serde(default)]
    has_end: MaybeUndefined<Timestamp>,
    #[serde(default)]
    unit_based: MaybeUndefined<bool>,
    #[serde(default)]
    created: MaybeUndefined<Timestamp>,
    #[serde(default)]
    note: MaybeUndefined<String>,
}

// impl<'a> CreateRequest {
//     // :TODO: accessors for field data
// }

/// Pick relevant fields out of I/O record into underlying DHT entry
impl From<CreateRequest> for Entry {
    fn from(e: CreateRequest) -> Entry {
        Entry {
            name: e.name.into(),
            has_beginning: e.has_beginning.into(),
            has_end: e.has_end.into(),
            unit_based: e.unit_based.into(),
            created: e.created.into(),
            note: e.note.into(),
        }
    }
}

//---------------- UPDATE ----------------

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRequest {
    id: ProposalAddress,
    #[serde(default)]
    name: MaybeUndefined<String>,
    #[serde(default)]
    has_beginning: MaybeUndefined<Timestamp>,
    #[serde(default)]
    has_end: MaybeUndefined<Timestamp>,
    #[serde(default)]
    unit_based: MaybeUndefined<bool>,
    #[serde(default)]
    created: MaybeUndefined<Timestamp>,
    #[serde(default)]
    note: MaybeUndefined<String>,
}

impl<'a> UpdateRequest {
    pub fn get_id(&'a self) -> &ProposalAddress {
        &self.id
    }
}

/// Handles update operations by merging any newly provided fields
impl Updateable<UpdateRequest> for Entry {
    fn update_with(&self, e: &UpdateRequest) -> Entry {
        Entry {
            name: if !e.name.is_some() { self.name.to_owned() } else { e.name.to_owned().into() },
            has_beginning: if e.has_beginning == MaybeUndefined::Undefined { self.has_beginning.to_owned() } else { e.has_beginning.to_owned().into() },
            has_end: if e.has_end == MaybeUndefined::Undefined { self.has_end.to_owned() } else { e.has_end.to_owned().into() },
            unit_based: if e.unit_based == MaybeUndefined::Undefined { self.unit_based.to_owned() } else { e.unit_based.to_owned().into() },
            created: if e.created == MaybeUndefined::Undefined { self.created.to_owned() } else { e.created.to_owned().into() },
            note: if e.note== MaybeUndefined::Undefined { self.note.to_owned() } else { e.note.to_owned().into() },
        }
    }
}

//---------------- EXTERNAL API ----------------

/// I/O struct to describe the complete output record, including all managed link fields
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    id: ProposalAddress,
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    has_beginning: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    has_end: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit_based: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<String>,
    // links:
    #[serde(skip_serializing_if = "Option::is_none")]
    publishes: Option<Vec<ProposedIntentAddress>>,
}

/// I/O struct to describe what is returned outside the gateway
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData {
    proposal: Response,
}

/// Create response from input DHT primitives
pub fn construct_response<'a>(
    address: &ProposalAddress,
    e: &Entry,
    publishes: Option<Cow<'a, Vec<ProposedIntentAddress>>>
) -> ResponseData {
    ResponseData {
        proposal: Response {
            // entry fields
            id: address.to_owned(),
            name: e.name.to_owned(),
            has_beginning: e.has_beginning.to_owned(),
            has_end: e.has_end.to_owned(),
            unit_based: e.unit_based.to_owned(),
            created: e.created.to_owned(),
            note: e.note.to_owned(),
            // link fields
            publishes: publishes.map(Cow::into_owned),
        }
    }
}

pub fn get_link_fields <'a> ( proposal: &ProposalAddress ) -> Option<Cow<'a, Vec<ProposedIntentAddress>>> {
    Some(get_published_ids(proposal))
}


fn get_published_ids <'a> (p_to: &ProposalAddress) -> Cow<'a, Vec<ProposedIntentAddress>> {
    get_linked_addresses_with_foreign_key_as_type(p_to, PROPOSAL_PUBLISHES_LINK_TYPE, PROPOSAL_PUBLISHES_LINK_TAG)
}
