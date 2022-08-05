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
    ActionHash, ByAddress, RevisionMeta,
    EconomicResourceAddress,
    EconomicEventAddress,
    ExternalURL,
    LocationAddress,
    ResourceSpecificationAddress,
    UnitId,
    ProductBatchAddress,
    AgentAddress,
};

use hc_zome_rea_economic_event_rpc::{
    CreateRequest as EventCreateRequest,
    ResourceCreateRequest as CreateRequest,
    ResourceInventoryType,
};

//---------------- CREATE REQUEST ----------------

// differs from `hc_zome_rea_economic_event_rpc::CreateParams` in that resource is guaranteed.
// :TODO: is this needed, or can it be deleted for `event_rpc::CreateParams`?
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
    pub revision_id: ActionHash,
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
    pub fn get_revision_id(&'a self) -> &ActionHash {
        &self.revision_id
    }

    pub fn get_contained_in(&'a self) -> MaybeUndefined<EconomicResourceAddress> {
        self.contained_in.to_owned()
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateParams {
    pub resource: UpdateRequest,
}

//---------------- QUERY FILTER REQUEST ----------------

#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    pub contains: Option<EconomicResourceAddress>,
    pub contained_in: Option<EconomicResourceAddress>,
    pub conforms_to: Option<ResourceSpecificationAddress>,
    pub affected_by: Option<EconomicEventAddress>,
    pub primary_accountable: Option<AgentAddress>,
}
