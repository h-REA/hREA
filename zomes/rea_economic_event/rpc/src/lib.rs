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
use hdk_relay_pagination::PageInfo;
pub use vf_attributes_hdk::{
    HeaderHash, ByAddress, ByHeader, ByRevision, RecordMeta, RevisionMeta,
    EconomicEventAddress,
    EconomicResourceAddress,
    ActionId,
    ExternalURL,
    LocationAddress,
    AgentAddress,
    ProcessAddress,
    ResourceSpecificationAddress,
    ProcessSpecificationAddress,
    IntentAddress,
    CommitmentAddress,
    FulfillmentAddress,
    SatisfactionAddress,
    AgreementAddress,
    ProductBatchAddress,
    UnitId,
    DateTime, FixedOffset,
};

//---------------- EXTERNAL RECORD STRUCTURE ----------------

/// I/O struct to describe EconomicEvents, including all managed link fields
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub id: EconomicEventAddress,
    pub revision_id: HeaderHash,
    pub meta: RecordMeta,
    pub action: ActionId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_of: Option<ProcessAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_of: Option<ProcessAddress>,
    pub provider: AgentAddress,
    pub receiver: AgentAddress,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_inventoried_as: Option<EconomicResourceAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_resource_inventoried_as: Option<EconomicResourceAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_classified_as: Option<Vec<ExternalURL>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_conforms_to: Option<ResourceSpecificationAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_quantity: Option<QuantityValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effort_quantity: Option<QuantityValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_beginning: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_end: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_point_in_time: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_location: Option<LocationAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agreed_in: Option<ExternalURL>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realization_of: Option<AgreementAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggered_by: Option<EconomicEventAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_scope_of: Option<Vec<String>>,

    // LINK FIELDS
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fulfills: Vec<FulfillmentAddress>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub satisfies: Vec<SatisfactionAddress>,
}

/// I/O struct to describe EconomicResources, including all managed link fields
/// Defined here since EconomicEvent responses may contain EconomicResource data
///
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResourceResponse {
    pub id: EconomicResourceAddress,
    pub revision_id: HeaderHash,
    pub meta: RecordMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conforms_to: Option<ResourceSpecificationAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classified_as: Option<Vec<ExternalURL>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lot: Option<ProductBatchAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<ExternalURL>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_quantity: Option<QuantityValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onhand_quantity: Option<QuantityValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_of_effort: Option<UnitId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contained_in: Option<EconomicResourceAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<ProcessSpecificationAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ActionId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_location: Option<LocationAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_accountable: Option<AgentAddress>,

    // query edges
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub contains: Vec<EconomicResourceAddress>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // trace: Option<Vec<EconomicEventAddress>>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // track: Option<Vec<EconomicEventAddress>>,
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
/// Conforms to Relay connections spec
/// :TODO: parameterize type and abstract declarations
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EventResponseCollection {
    pub edges: Vec<EventResponseEdge>,
    pub page_info: PageInfo,
}

#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EventResponseEdge {
    pub node: Response,
    pub cursor: String,
}

/// I/O struct to describe what is returned outside the gateway
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResourceResponseData {
    pub economic_resource: ResourceResponse,
}

/// I/O struct to describe what is returned outside the gateway
/// Conforms to Relay connections spec
/// :TODO: parameterize type and abstract declarations
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResourceResponseCollection {
    pub edges: Vec<ResourceResponseEdge>,
    pub page_info: PageInfo,
}

#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResourceResponseEdge {
    pub node: ResourceResponse,
    pub cursor: String,
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
    pub input_of: MaybeUndefined<ProcessAddress>,
    #[serde(default)]
    pub output_of: MaybeUndefined<ProcessAddress>,
    pub provider: AgentAddress,
    pub receiver: AgentAddress,
    #[serde(default)]
    pub resource_inventoried_as: MaybeUndefined<EconomicResourceAddress>,
    #[serde(default)]
    pub to_resource_inventoried_as: MaybeUndefined<EconomicResourceAddress>,
    #[serde(default)]
    pub resource_classified_as: MaybeUndefined<Vec<ExternalURL>>,
    #[serde(default)]
    pub resource_conforms_to: MaybeUndefined<ResourceSpecificationAddress>,
    #[serde(default)]
    pub resource_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    pub effort_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    pub has_beginning: MaybeUndefined<DateTime<FixedOffset>>,
    #[serde(default)]
    pub has_end: MaybeUndefined<DateTime<FixedOffset>>,
    #[serde(default)]
    pub has_point_in_time: MaybeUndefined<DateTime<FixedOffset>>,
    #[serde(default)]
    pub at_location: MaybeUndefined<LocationAddress>,
    #[serde(default)]
    pub agreed_in: MaybeUndefined<ExternalURL>,
    #[serde(default)]
    pub realization_of: MaybeUndefined<AgreementAddress>,
    #[serde(default)]
    pub triggered_by: MaybeUndefined<EconomicEventAddress>,
    #[serde(default)]
    pub in_scope_of: MaybeUndefined<Vec<String>>,

    // :SHONK: internal field used in updating linked resource quantities
    #[serde(default)]
    pub target_inventory_type: Option<ResourceInventoryType>,
}

impl<'a> CreateRequest {
    pub fn with_inventoried_resource(&self, resource_address: &EconomicResourceAddress) -> Self {
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

    pub fn get_location(&'a self) -> MaybeUndefined<LocationAddress> {
        self.at_location.to_owned()
    }

    pub fn get_realization_of(&'a self) -> MaybeUndefined<AgreementAddress> {
        self.realization_of.to_owned()
    }
}

// used in EconomicResource API
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResourceCreateRequest {
    #[serde(default)]
    pub name: MaybeUndefined<String>,
    #[serde(default)]
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

impl<'a> ResourceCreateRequest {
    pub fn get_contained_in(&'a self) -> Option<EconomicResourceAddress> {
        self.contained_in.to_owned().to_option()
    }
}

#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateParams {
    pub event: CreateRequest,
    pub new_inventoried_resource: Option<ResourceCreateRequest>,
}

//---------------- UPDATE REQUEST ----------------

/// I/O struct to describe the complete input record, including all managed links
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRequest {
    pub revision_id: HeaderHash,
    #[serde(default)]
    pub note: MaybeUndefined<String>,
    #[serde(default)]
    pub agreed_in: MaybeUndefined<ExternalURL>,
    #[serde(default)]
    pub realization_of: MaybeUndefined<AgreementAddress>,
    #[serde(default)]
    pub triggered_by: MaybeUndefined<EconomicEventAddress>,
    #[serde(default)]
    pub in_scope_of: MaybeUndefined<Vec<String>>,
}

impl<'a> UpdateRequest {
    pub fn get_revision_id(&'a self) -> &HeaderHash {
        &self.revision_id
    }

    // :TODO: accessors for other field data
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateParams {
    pub event: UpdateRequest,
}

//---------------- QUERY FILTER REQUEST ----------------

#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    pub input_of: Option<ProcessAddress>,
    pub output_of: Option<ProcessAddress>,
    pub satisfies: Option<IntentAddress>,
    pub fulfills: Option<CommitmentAddress>,
    pub realization_of: Option<AgreementAddress>,
    pub affects: Option<EconomicResourceAddress>,
    pub provider: Option<AgentAddress>,
    pub receiver: Option<AgentAddress>,
}
