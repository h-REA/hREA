use std::borrow::Cow;
use hdk::{
    holochain_json_api::{ json::JsonString, error::JsonError },
};
use holochain_json_derive::{ DefaultJson };

use hdk_graph_helpers::{
    MaybeUndefined,
    record_interface::Updateable,
    links::{
        get_linked_addresses_with_foreign_key_as_type
    }
};

use vf_core::type_aliases::{
    ProposedIntentAddress,
    ProposalAddress,
};

use super::identifiers::{
    PROPOSED_INTENT_PUBLISHED_IN_LINK_TYPE, PROPOSED_INTENT_PUBLISHED_IN_LINK_TAG,
};

//---------------- RECORD INTERNALS & VALIDATION ----------------

#[derive(Serialize, Deserialize, Debug, DefaultJson, Default, Clone)]
pub struct Entry {
    reciprocal: bool,
}

//---------------- CREATE ----------------

/// I/O struct to describe the complete input record, including all managed links
#[derive(Serialize, Deserialize, Debug, DefaultJson, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    reciprocal: bool,
}

impl<'a> CreateRequest {
}

/// Pick relevant fields out of I/O record into underlying DHT entry
impl From<CreateRequest> for Entry {
    fn from(e: CreateRequest) -> Entry {
        Entry {
            reciprocal: e.reciprocal,
        }
    }
}

//---------------- UPDATE ----------------

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRequest {
    id: ProposedIntentAddress,
    #[serde(default)]
    reciprocal: MaybeUndefined<bool>,
}

impl<'a> UpdateRequest {
    pub fn get_id(&'a self) -> &ProposedIntentAddress {
        &self.id
    }

    // :TODO: accessors for other field data
}

/// Handles update operations by merging any newly provided fields
impl Updateable<UpdateRequest> for Entry {
    fn update_with(&self, e: &UpdateRequest) -> Entry {
        Entry {
            reciprocal: if e.reciprocal.is_undefined() { self.reciprocal } else { e.reciprocal.to_owned().unwrap() },
        }
    }
}

//---------------- EXTERNAL API ----------------

/// I/O struct to describe the complete output record, including all managed link fields
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    id: ProposedIntentAddress,
    reciprocal: bool,

    // query edges
    #[serde(skip_serializing_if = "Option::is_none")]
    published_in: Option<Vec<ProposalAddress>>,
    // TODO:
    // publishes: Intent!
}

/// I/O struct to describe what is returned outside the gateway
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData {
    proposed_intent: Response,
}

/// Create response from input DHT primitives
pub fn construct_response<'a>(
    address: &ProposedIntentAddress, e: &Entry,
    published_in: Option<Cow<'a, Vec<ProposalAddress>>>
) -> ResponseData {
    ResponseData {
        proposed_intent: Response {
            // entry fields
            id: address.to_owned(),
            reciprocal: e.reciprocal,
            // link field
            published_in: published_in.map(Cow::into_owned),
            // [ TODO ] publishes
        }
    }
}

pub fn get_link_fields <'a> ( p_in: &ProposedIntentAddress ) -> Option<Cow<'a, Vec<ProposalAddress>>> {
    Some(get_published_in_ids(p_in))
}


fn get_published_in_ids <'a> (p_to: &ProposedIntentAddress) -> Cow<'a, Vec<ProposalAddress>> {
    get_linked_addresses_with_foreign_key_as_type(p_to, PROPOSED_INTENT_PUBLISHED_IN_LINK_TYPE, PROPOSED_INTENT_PUBLISHED_IN_LINK_TAG)
}
