/**
 * Holo-REA 'economic resource' zome I/O data structures
 *
 * Required by packages wishing to interact with the zome via its standard RPC interface,
 * and by the zome API handlers accepting these parameters.
 *
 * @package Holo-REA
 */
use holochain_serialized_bytes::prelude::*;

use serde_maybe_undefined::MaybeUndefined;
pub use vf_attributes_hdk::{
    RevisionHash,
    EconomicResourceAddress,
    ExternalURL,
    LocationAddress,
    ResourceSpecificationAddress,
    UnitId,
    ProductBatchAddress,
};

use hc_zome_rea_economic_event_rpc::{
    CreateRequest as EventCreateRequest,
    ResourceInventoryType,
};

//---------------- CREATE REQUEST ----------------

// used in EconomicEvent API
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    pub conforms_to: MaybeUndefined<ResourceSpecificationAddress>,
    #[serde(default)]
    pub tracking_identifier: MaybeUndefined<String>,
    #[serde(default)]
    pub lot: MaybeUndefined<ProductBatchAddress>,
    #[serde(default)]
    pub image: MaybeUndefined<ExternalURL>,
    #[serde(default)]
    pub contained_in: MaybeUndefined<EconomicResourceAddress>,
    #[serde(default)]
    pub current_location: MaybeUndefined<LocationAddress>,
    #[serde(default)]
    pub note: MaybeUndefined<String>,
}

impl<'a> CreateRequest {
    pub fn get_contained_in(&'a self) -> Option<EconomicResourceAddress> {
        self.contained_in.to_owned().to_option()
    }
}

#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
pub struct CreationPayload {
    pub event: EventCreateRequest,
    pub resource: CreateRequest,
}

impl<'a> CreationPayload {
    pub fn get_event_params(&'a self) -> &EventCreateRequest {
        &self.event
    }

    pub fn get_resource_params(&'a self) -> &CreateRequest {
        &self.resource
    }

    pub fn get_resource_specification_id(&'a self) -> Option<ResourceSpecificationAddress> {
        if self.resource.conforms_to.is_some() { self.resource.conforms_to.to_owned().to_option() } else { self.event.resource_conforms_to.to_owned().to_option() }
    }

    pub fn with_inventory_type(self, i: ResourceInventoryType) -> CreationPayload {
        CreationPayload {
            event: self.event.with_inventory_type(i),
            resource: self.resource,
        }
    }
}

//---------------- UPDATE REQUEST ----------------

// used in EconomicResource API
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRequest {
    pub revision_id: RevisionHash,
    #[serde(default)]
    pub classified_as: MaybeUndefined<Vec<ExternalURL>>,
    #[serde(default)]
    pub image: MaybeUndefined<ExternalURL>,
    #[serde(default)]
    pub contained_in: MaybeUndefined<EconomicResourceAddress>,
    #[serde(default)]
    pub unit_of_effort: MaybeUndefined<UnitId>,
    #[serde(default)]
    pub note: MaybeUndefined<String>,
}

impl<'a> UpdateRequest {
    pub fn get_revision_id(&'a self) -> &RevisionHash {
        &self.revision_id
    }

    pub fn get_contained_in(&'a self) -> MaybeUndefined<EconomicResourceAddress> {
        self.contained_in.to_owned()
    }
}

//---------------- QUERY FILTER REQUEST ----------------

#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    pub contains: Option<EconomicResourceAddress>,
    pub contained_in: Option<EconomicResourceAddress>,
    pub conforms_to: Option<ResourceSpecificationAddress>,
}
