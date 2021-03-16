/**
 * Holo-REA 'economic event' zome I/O data structures
 *
 * Required by packages wishing to interact with the zome via its standard RPC interface,
 * and by the zome API handlers accepting these parameters.
 *
 * @package Holo-REA
 */
use holochain_serialized_bytes::prelude::*;

use serde_maybe_undefined::MaybeUndefined;
use vf_measurement::QuantityValue;
pub use vf_attributes_hdk::{
    RevisionHash,
    EventRef,
    ResourceRef,
    ActionId,
    Timestamp,
    ExternalURL,
    LocationRef,
    AgentRef,
    ProcessRef,
    ResourceSpecificationRef,
    ProcessSpecificationRef,
    IntentRef,
    CommitmentRef,
    FulfillmentRef,
    SatisfactionRef,
    AgreementRef,
    ProductBatchRef,
    UnitId,
};

//---------------- EXTERNAL RECORD STRUCTURE ----------------

/// I/O struct to describe EconomicEvents, including all managed link fields
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub id: EventRef,
    pub revision_id: RevisionHash,
    pub action: ActionId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_of: Option<ProcessRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_of: Option<ProcessRef>,
    pub provider: AgentRef,
    pub receiver: AgentRef,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_inventoried_as: Option<ResourceRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_resource_inventoried_as: Option<ResourceRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_classified_as: Option<Vec<ExternalURL>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_conforms_to: Option<ResourceSpecificationRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_quantity: Option<QuantityValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effort_quantity: Option<QuantityValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_beginning: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_end: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_point_in_time: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_location: Option<LocationRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agreed_in: Option<ExternalURL>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realization_of: Option<AgreementRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggered_by: Option<EventRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_scope_of: Option<Vec<String>>,

    // LINK FIELDS
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fulfills: Vec<FulfillmentRef>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub satisfies: Vec<SatisfactionRef>,
}

/// I/O struct to describe EconomicResources, including all managed link fields
/// Defined here since EconomicEvent responses may contain EconomicResource data
///
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResourceResponse {
    pub id: ResourceRef,
    pub revision_id: RevisionHash,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conforms_to: Option<ResourceSpecificationRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classified_as: Option<Vec<ExternalURL>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lot: Option<ProductBatchRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<ExternalURL>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_quantity: Option<QuantityValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onhand_quantity: Option<QuantityValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_of_effort: Option<UnitId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contained_in: Option<ResourceRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<ProcessSpecificationRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ActionId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_location: Option<LocationRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,

    // query edges
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub contains: Vec<ResourceRef>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // trace: Option<Vec<EventRef>>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // track: Option<Vec<EventRef>>,
}

/// I/O struct to describe what is returned outside the gateway
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData {
    pub economic_event: Response,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub economic_resource: Option<ResourceResponse>,
}

/// I/O struct to describe what is returned outside the gateway
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResourceResponseData {
    pub economic_resource: ResourceResponse,
}

//---------------- CREATE REQUEST ----------------

#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
pub enum ResourceInventoryType {
    ProvidingInventory,
    ReceivingInventory,
}

/// I/O struct to describe the complete input record, including all managed links
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    pub action: ActionId,
    #[serde(default)]
    pub note: MaybeUndefined<String>,
    #[serde(default)]
    pub input_of: MaybeUndefined<ProcessRef>,
    #[serde(default)]
    pub output_of: MaybeUndefined<ProcessRef>,
    pub provider: AgentRef,
    pub receiver: AgentRef,
    #[serde(default)]
    pub resource_inventoried_as: MaybeUndefined<ResourceRef>,
    #[serde(default)]
    pub to_resource_inventoried_as: MaybeUndefined<ResourceRef>,
    #[serde(default)]
    pub resource_classified_as: MaybeUndefined<Vec<ExternalURL>>,
    #[serde(default)]
    pub resource_conforms_to: MaybeUndefined<ResourceSpecificationRef>,
    #[serde(default)]
    pub resource_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    pub effort_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    pub has_beginning: MaybeUndefined<Timestamp>,
    #[serde(default)]
    pub has_end: MaybeUndefined<Timestamp>,
    #[serde(default)]
    pub has_point_in_time: MaybeUndefined<Timestamp>,
    #[serde(default)]
    pub at_location: MaybeUndefined<LocationRef>,
    #[serde(default)]
    pub agreed_in: MaybeUndefined<ExternalURL>,
    #[serde(default)]
    pub realization_of: MaybeUndefined<AgreementRef>,
    #[serde(default)]
    pub triggered_by: MaybeUndefined<EventRef>,
    #[serde(default)]
    pub in_scope_of: MaybeUndefined<Vec<String>>,

    // :SHONK: internal field used in updating linked resource quantities
    #[serde(default)]
    pub target_inventory_type: Option<ResourceInventoryType>,
}

impl<'a> CreateRequest {
    pub fn with_inventoried_resource(&self, resource_address: &ResourceRef) -> Self {
        CreateRequest {
            resource_inventoried_as: MaybeUndefined::Some(resource_address.to_owned()),
            ..self.to_owned()
        }
    }

    pub fn with_inventory_type(&self, t: ResourceInventoryType) -> Self {
        CreateRequest {
            target_inventory_type: Some(t),
            ..self.to_owned()
        }
    }

    // accessors for field data

    pub fn get_action(&'a self) -> &str {
        &(self.action.as_ref())[..]
    }

    pub fn get_location(&'a self) -> MaybeUndefined<LocationRef> {
        self.at_location.to_owned()
    }

    pub fn get_realization_of(&'a self) -> MaybeUndefined<AgreementRef> {
        self.realization_of.to_owned()
    }
}

//---------------- UPDATE REQUEST ----------------

/// I/O struct to describe the complete input record, including all managed links
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRequest {
    pub revision_id: RevisionHash,
    #[serde(default)]
    pub note: MaybeUndefined<String>,
    #[serde(default)]
    pub agreed_in: MaybeUndefined<ExternalURL>,
    #[serde(default)]
    pub realization_of: MaybeUndefined<AgreementRef>,
    #[serde(default)]
    pub triggered_by: MaybeUndefined<EventRef>,
    #[serde(default)]
    pub in_scope_of: MaybeUndefined<Vec<String>>,
}

impl<'a> UpdateRequest {
    pub fn get_revision_id(&'a self) -> &RevisionHash {
        &self.revision_id
    }

    // :TODO: accessors for other field data
}

//---------------- QUERY FILTER REQUEST ----------------

#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    pub input_of: Option<ProcessRef>,
    pub output_of: Option<ProcessRef>,
    pub satisfies: Option<IntentRef>,
    pub fulfills: Option<CommitmentRef>,
    pub realization_of: Option<AgreementRef>,
}
