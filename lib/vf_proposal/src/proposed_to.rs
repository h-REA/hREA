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
    ProposedToAddress,
    AgentAddress,
    ProposalAddress,
};

use super::identifiers::{
    PROPOSED_TO_PROPOSED_LINK_TYPE, PROPOSED_TO_PROPOSED_LINK_TAG,
};

//---------------- RECORD INTERNALS & VALIDATION ----------------

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Entry {
    proposed_to: AgentAddress,
}

//---------------- CREATE ----------------

/// I/O struct to describe the complete input record, including all managed links
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    proposed_to: AgentAddress,
}

impl<'a> CreateRequest {
}

/// Pick relevant fields out of I/O record into underlying DHT entry
impl From<CreateRequest> for Entry {
    fn from(e: CreateRequest) -> Entry {
        Entry {
            proposed_to: e.proposed_to,
        }
    }
}

//---------------- UPDATE ----------------

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRequest {
    id: ProposedToAddress,
    #[serde(default)]
    proposed_to: MaybeUndefined<AgentAddress>,
}

impl<'a> UpdateRequest {
    pub fn get_id(&'a self) -> &ProposedToAddress {
        &self.id
    }

    // :TODO: accessors for other field data
}

/// Handles update operations by merging any newly provided fields
impl Updateable<UpdateRequest> for Entry {
    fn update_with(&self, e: &UpdateRequest) -> Entry {
        Entry {
            proposed_to: if e.proposed_to.is_some() { e.proposed_to.to_owned().unwrap() } else { self.proposed_to.to_owned() },
        }
    }
}

//---------------- EXTERNAL API ----------------

/// I/O struct to describe the complete output record, including all managed link fields
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    id: ProposedToAddress,
    proposed_to: AgentAddress,

    #[serde(skip_serializing_if = "Option::is_none")]
    proposed: Option<Vec<ProposalAddress>>,
}

/// I/O struct to describe what is returned outside the gateway
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData {
    proposed_to: Response,
}

/// Create response from input DHT primitives
pub fn construct_response<'a>(
    address: &ProposedToAddress,
    e: &Entry,
    proposed: Option<Cow<'a, Vec<ProposalAddress>>>
) -> ResponseData {
    ResponseData {
        proposed_to: Response {
            // entry fields
            id: address.to_owned(),
            proposed_to: e.proposed_to.to_owned(),
            // link field
            proposed: proposed.map(Cow::into_owned),
        }
    }
}

pub fn get_link_fields <'a> ( p_to: &ProposedToAddress ) -> Option<Cow<'a, Vec<ProposalAddress>>> {
    Some(get_proposed_ids(p_to))
}


fn get_proposed_ids <'a> (p_to: &ProposedToAddress) -> Cow<'a, Vec<ProposalAddress>> {
    get_linked_addresses_with_foreign_key_as_type(p_to, PROPOSED_TO_PROPOSED_LINK_TYPE, PROPOSED_TO_PROPOSED_LINK_TAG)
}
