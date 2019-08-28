/**
 * A Fulfillment describes the relationship between a Commitment and the
 * EconomicEvent which (fully or partially) helps to complete the Commitment.
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
};

use vf_core::type_aliases::{
    FulfillmentAddress,
    EventAddress,
    CommitmentAddress,
};

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Entry {
    pub fulfilled_by: EventAddress,
    pub fulfills: CommitmentAddress,
    pub resource_quantity: Option<QuantityValue>,
    pub effort_quantity: Option<QuantityValue>,
    pub note: Option<String>,
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
            resource_quantity: if e.resource_quantity== MaybeUndefined::Undefined { self.resource_quantity.clone() } else { e.resource_quantity.clone().into() },
            effort_quantity: if e.effort_quantity== MaybeUndefined::Undefined { self.effort_quantity.clone() } else { e.effort_quantity.clone().into() },
            note: if e.note== MaybeUndefined::Undefined { self.note.clone() } else { e.note.clone().into() },
        }
    }
}

/// I/O struct to describe the complete input record
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct CreateRequest {
    fulfilled_by: EventAddress,
    fulfills: CommitmentAddress,
    #[serde(default)]
    resource_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    effort_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
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

/// I/O struct to describe the complete input record
#[derive(Serialize, Deserialize, Debug, DefaultJson, Default, Clone)]
pub struct UpdateRequest {
    id: Address,
    #[serde(default)]
    fulfilled_by: MaybeUndefined<EventAddress>, // note this setup allows None to be passed but `update_with` ignores it
    #[serde(default)]
    fulfills: MaybeUndefined<CommitmentAddress>,
    #[serde(default)]
    resource_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    effort_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    note: MaybeUndefined<String>,
}

impl<'a> UpdateRequest {
    pub fn get_id(&'a self) -> &Address {
        &self.id
    }
}

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

/// Create response from input DHT primitives
pub fn construct_response(address: &Address, e: Entry) -> ResponseData {
    ResponseData {
        fulfillment: Response {
            id: address.to_owned().into(),
            fulfilled_by: e.fulfilled_by,
            fulfills: e.fulfills,
            resource_quantity: e.resource_quantity,
            effort_quantity: e.effort_quantity,
            note: e.note,
        }
    }
}
