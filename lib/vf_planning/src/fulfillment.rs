/**
 * A Fulfillment describes the relationship between a Commitment and the
 * EconomicEvent which (fully or partially) helps to complete the Commitment.
 *
 * @package HoloREA
 */
use hdk::{
    holochain_json_api::{ json::JsonString, error::JsonError },
};
use holochain_json_derive::{ DefaultJson };

use hdk_graph_helpers::{
    MaybeUndefined,
    record_interface::Updateable,
};

use vf_core::{
    measurement::QuantityValue,
};

use vf_core::type_aliases::{
    FulfillmentAddress,
    EventAddress,
    CommitmentAddress,
};

//---------------- RECORD INTERNALS & VALIDATION ----------------

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Entry {
    pub fulfilled_by: EventAddress,
    pub fulfills: CommitmentAddress,
    pub resource_quantity: Option<QuantityValue>,
    pub effort_quantity: Option<QuantityValue>,
    pub note: Option<String>,
}

//---------------- CREATE ----------------

/// I/O struct to describe the complete input record
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    fulfilled_by: EventAddress,
    fulfills: CommitmentAddress,
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

    pub fn get_fulfilled_by(&'a self) -> &EventAddress {
        &self.fulfilled_by
    }

    pub fn get_fulfills(&'a self) -> &CommitmentAddress {
        &self.fulfills
    }
}

/// I/O struct for forwarding records to other DNAs via zome API
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FwdCreateRequest {
    pub fulfillment: CreateRequest,
}

/// Pick relevant fields out of I/O record into underlying DHT entry
impl From<CreateRequest> for Entry {
    fn from(e: CreateRequest) -> Entry {
        Entry {
            fulfilled_by: e.fulfilled_by.into(),
            fulfills: e.fulfills.into(),
            resource_quantity: e.resource_quantity.into(),
            effort_quantity: e.effort_quantity.into(),
            note: e.note.into(),
        }
    }
}

//---------------- UPDATE ----------------

/// I/O struct to describe the complete input record
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRequest {
    id: FulfillmentAddress,
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    fulfilled_by: MaybeUndefined<EventAddress>, // note this setup allows None to be passed but `update_with` ignores it
    #[serde(default)]
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    fulfills: MaybeUndefined<CommitmentAddress>,
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
    pub fn get_id(&'a self) -> &FulfillmentAddress {
        &self.id
    }
}

/// Handles update operations by merging any newly provided fields
impl Updateable<UpdateRequest> for Entry {
    fn update_with(&self, e: &UpdateRequest) -> Entry {
        Entry {
            fulfilled_by: match &e.fulfilled_by {
                MaybeUndefined::Some(fulfilled_by) => fulfilled_by.clone(),
                _ => self.fulfilled_by.clone(),
            },
            fulfills: match &e.fulfills {
                MaybeUndefined::Some(fulfills) => fulfills.clone(),
                _ => self.fulfills.clone(),
            },
            resource_quantity: if e.resource_quantity.is_some() { e.resource_quantity.clone().into() } else { self.resource_quantity.clone() },
            effort_quantity: if e.effort_quantity.is_some() { e.effort_quantity.clone().into() } else { self.effort_quantity.clone() },
            note: if e.note.is_some() { e.note.clone().into() } else { self.note.clone() },
        }
    }
}

/// I/O struct for forwarding records to other DNAs via zome API
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FwdUpdateRequest {
    pub fulfillment: UpdateRequest,
}

//---------------- EXTERNAL API ----------------

/// I/O struct to describe the complete output record, including all managed link fields
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    id: FulfillmentAddress,
    fulfilled_by: EventAddress,
    fulfills: CommitmentAddress,
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
    fulfillment: Response,
}

/// Create response from input DHT primitives
pub fn construct_response(address: &FulfillmentAddress, e: &Entry) -> ResponseData {
    ResponseData {
        fulfillment: Response {
            id: address.to_owned(),
            fulfilled_by: e.fulfilled_by.to_owned(),
            fulfills: e.fulfills.to_owned(),
            resource_quantity: e.resource_quantity.to_owned(),
            effort_quantity: e.effort_quantity.to_owned(),
            note: e.note.to_owned(),
        }
    }
}
