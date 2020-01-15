/**
 * Holo-REA 'economic event' zome I/O data structures
 *
 * Required by packages wishing to interact with the zome via its standard RPC interface,
 * and by the zome API handlers accepting these parameters.
 *
 * @package Holo-REA
 */
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use holochain_json_api::{ json::JsonString, error::JsonError };
use holochain_json_derive::{ DefaultJson };

use hdk_graph_helpers::MaybeUndefined;
use vf_core::measurement::QuantityValue;
use vf_core::type_aliases::{
    ActionId,
    Timestamp,
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
};

//---------------- EXTERNAL RECORD STRUCTURE ----------------

// Export external type interface to allow consuming zomes to easily import & define zome API
pub use vf_core::type_aliases::{ EventAddress, ResourceAddress };

/// I/O struct to describe EconomicEvents, including all managed link fields
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub id: EventAddress,
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
    pub resource_inventoried_as: Option<ResourceAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_resource_inventoried_as: Option<ResourceAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_classified_as: Option<Vec<ExternalURL>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_conforms_to: Option<ResourceSpecificationAddress>,
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
    pub at_location: Option<LocationAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agreed_in: Option<ExternalURL>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realization_of: Option<AgreementAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggered_by: Option<EventAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_scope_of: Option<Vec<String>>,

    // LINK FIELDS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfills: Option<Vec<FulfillmentAddress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub satisfies: Option<Vec<SatisfactionAddress>>,
}

/// I/O struct to describe EconomicResources, including all managed link fields
/// Defined here since EconomicEvent responses may contain EconomicResource data
///
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResourceResponse {
    pub id: ResourceAddress,
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
    pub contained_in: Option<ResourceAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<ProcessSpecificationAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ActionId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_location: Option<LocationAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,

    // query edges
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains: Option<Vec<ResourceAddress>>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // trace: Option<Vec<EventAddress>>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // track: Option<Vec<EventAddress>>,
}

/// I/O struct to describe what is returned outside the gateway
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData {
    pub economic_event: Response,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub economic_resource: Option<ResourceResponse>,
}

/// I/O struct to describe what is returned outside the gateway
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResourceResponseData {
    pub economic_resource: ResourceResponse,
}

//---------------- CREATE REQUEST ----------------

#[derive(Serialize, Deserialize, DefaultJson, Clone, Debug)]
pub enum ResourceInventoryType {
    ProvidingInventory,
    ReceivingInventory,
}

/// I/O struct to describe the complete input record, including all managed links
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
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
    pub resource_inventoried_as: MaybeUndefined<ResourceAddress>,
    #[serde(default)]
    pub to_resource_inventoried_as: MaybeUndefined<ResourceAddress>,
    #[serde(default)]
    pub resource_classified_as: MaybeUndefined<Vec<ExternalURL>>,
    #[serde(default)]
    pub resource_conforms_to: MaybeUndefined<ResourceSpecificationAddress>,
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
    pub at_location: MaybeUndefined<LocationAddress>,
    #[serde(default)]
    pub agreed_in: MaybeUndefined<ExternalURL>,
    #[serde(default)]
    pub realization_of: MaybeUndefined<AgreementAddress>,
    #[serde(default)]
    pub triggered_by: MaybeUndefined<EventAddress>,
    #[serde(default)]
    pub in_scope_of: MaybeUndefined<Vec<String>>,

    // :SHONK: internal field used in updating linked resource quantities
    #[serde(default)]
    pub target_inventory_type: Option<ResourceInventoryType>,
}

impl<'a> CreateRequest {
    pub fn with_inventoried_resource(&self, resource_address: &ResourceAddress) -> Self {
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
}

//---------------- UPDATE REQUEST ----------------

/// I/O struct to describe the complete input record, including all managed links
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRequest {
    pub id: EventAddress,
    #[serde(default)]
    pub note: MaybeUndefined<String>,
    #[serde(default)]
    pub agreed_in: MaybeUndefined<ExternalURL>,
    #[serde(default)]
    pub realization_of: MaybeUndefined<AgreementAddress>,
    #[serde(default)]
    pub triggered_by: MaybeUndefined<EventAddress>,
    #[serde(default)]
    pub in_scope_of: MaybeUndefined<Vec<String>>,
}

impl<'a> UpdateRequest {
    pub fn get_id(&'a self) -> &EventAddress {
        &self.id
    }

    // :TODO: accessors for other field data
}

//---------------- QUERY FILTER REQUEST ----------------

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    pub input_of: Option<ProcessAddress>,
    pub output_of: Option<ProcessAddress>,
    pub satisfies: Option<IntentAddress>,
    pub fulfills: Option<CommitmentAddress>,
}
