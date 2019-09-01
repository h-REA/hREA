/**
 * A Satisfaction describes the relationship between an Intent and the
 * Commitment which (fully or partially) helps to complete the Intent.
 *
 * @package HoloREA
 */
use hdk::{
    holochain_json_api::{
        json::JsonString,
        error::JsonError,
    },
    holochain_persistence_api::{
        cas::content::Address,
    },
};
use holochain_json_derive::{ DefaultJson };

use hdk_graph_helpers::{
    MaybeUndefined,
    record_interface::Updateable,
};

use vf_core::{
    measurement::QuantityValue,
    type_aliases::{
        SatisfactionAddress,
        EventOrCommitmentAddress,
        IntentAddress,
    },
};

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Entry {
    pub satisfied_by: EventOrCommitmentAddress,
    pub satisfies: IntentAddress,
    pub resource_quantity: Option<QuantityValue>,
    pub effort_quantity: Option<QuantityValue>,
    pub note: Option<String>,
}

/// Handles update operations by merging any newly provided fields
impl Updateable<UpdateRequest> for Entry {
    fn update_with(&self, e: &UpdateRequest) -> Entry {
        Entry {
            satisfied_by: match &e.satisfied_by {
                MaybeUndefined::Some(satisfied_by) => satisfied_by.clone(),
                _ => self.satisfied_by.clone(),
            },
            satisfies: match &e.satisfies {
                MaybeUndefined::Some(satisfies) => satisfies.clone(),
                _ => self.satisfies.clone(),
            },
            resource_quantity: if e.resource_quantity== MaybeUndefined::Undefined { self.resource_quantity.clone() } else { e.resource_quantity.clone().into() },
            effort_quantity: if e.effort_quantity== MaybeUndefined::Undefined { self.effort_quantity.clone() } else { e.effort_quantity.clone().into() },
            note: if e.note== MaybeUndefined::Undefined { self.note.clone() } else { e.note.clone().into() },
        }
    }
}

/// I/O struct to describe the complete input record
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    satisfied_by: EventOrCommitmentAddress,
    satisfies: IntentAddress,
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    resource_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    effort_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    note: MaybeUndefined<String>,
}

impl<'a> CreateRequest {
    // :TODO: accessors for field data

    pub fn get_satisfied_by(&'a self) -> &EventOrCommitmentAddress {
        &self.satisfied_by
    }

    pub fn get_satisfies(&'a self) -> &IntentAddress {
        &self.satisfies
    }
}

/// I/O struct for forwarding records to other DNAs via zome API
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FwdCreateRequest {
    pub satisfaction: CreateRequest,
}

/// I/O struct for forwarding records to other DNAs via zome API
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CheckCommitmentRequest {
    pub address: Address,
}

/// I/O struct to describe the complete input record
#[derive(Serialize, Deserialize, Debug, DefaultJson, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRequest {
    id: Address,
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    satisfied_by: MaybeUndefined<EventOrCommitmentAddress>, // note this setup allows None to be passed but `update_with` ignores it
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    satisfies: MaybeUndefined<IntentAddress>,
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    resource_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    effort_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    note: MaybeUndefined<String>,
}

impl<'a> UpdateRequest {
    pub fn get_id(&'a self) -> &Address {
        &self.id
    }
}

/// I/O struct for forwarding records to other DNAs via zome API
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FwdUpdateRequest {
    pub satisfaction: UpdateRequest,
}

/// I/O struct to describe the complete output record, including all managed link fields
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    id: SatisfactionAddress,
    satisfied_by: EventOrCommitmentAddress,
    satisfies: IntentAddress,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_quantity: Option<QuantityValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effort_quantity: Option<QuantityValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<String>,
}

/// I/O struct to describe what is returned outside the gateway
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData {
    satisfaction: Response,
}

/// Pick relevant fields out of I/O record into underlying DHT entry
impl From<CreateRequest> for Entry {
    fn from(e: CreateRequest) -> Entry {
        Entry {
            satisfied_by: e.satisfied_by.into(),
            satisfies: e.satisfies.into(),
            resource_quantity: e.resource_quantity.into(),
            effort_quantity: e.effort_quantity.into(),
            note: e.note.into(),
        }
    }
}

/// Create response from input DHT primitives
pub fn construct_response(address: &Address, e: &Entry) -> ResponseData {
    ResponseData {
        satisfaction: Response {
            id: address.to_owned().into(),
            satisfied_by: e.satisfied_by.to_owned(),
            satisfies: e.satisfies.to_owned(),
            resource_quantity: e.resource_quantity.to_owned(),
            effort_quantity: e.effort_quantity.to_owned(),
            note: e.note.to_owned(),
        }
    }
}
