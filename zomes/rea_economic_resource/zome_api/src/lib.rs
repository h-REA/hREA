use hdk_records::{RecordAPIResult, SignedActionHashed};
use hc_zome_rea_economic_resource_rpc::*;
pub use hc_zome_rea_economic_event_rpc::{
    CreateRequest as EventCreateRequest,
    ResourceResponseData as ResponseData,
    ResourceResponseCollection as Collection,
};
use hc_zome_rea_economic_resource_storage::{EntryData};

/// API interface for the external zome gateway
pub trait API {
    type S: AsRef<str>;

    fn create_inventory_from_event(resource_entry_def_id: Self::S, params: CreationPayload) -> RecordAPIResult<(SignedActionHashed, EconomicResourceAddress, EntryData)>;
    fn update_inventory_from_event(
        event: EventCreateRequest,
    ) -> RecordAPIResult<Vec<(SignedActionHashed, EconomicResourceAddress, EntryData, EntryData)>>;
    fn get_economic_resource(address: EconomicResourceAddress) -> RecordAPIResult<ResponseData>;
    fn update_economic_resource(resource: UpdateRequest) -> RecordAPIResult<ResponseData>;
}

/// Macro to programatically and predictably bind an `API` implementation to a
/// ValueFlows-compatibile module storage API.
///
/// See the associated client-side bindings in `modules/vf-graphql-holochain`
/// as a reference for how this API signature binds to JavaScript application code.
///
#[macro_export]
macro_rules! declare_economic_resource_zome_api {
    ( $zome_api:ty ) => {
        // :TODO: The signature of this method, and its decoupling from the EconomicEvent zome, means that resources can be
        //        instantiated from the receiving inventory. Is this desirable? What are the repercussions?
        #[hdk_extern]
        fn _internal_create_inventory(params: CreationPayload) -> ExternResult<(SignedActionHashed, EconomicResourceAddress, EntryData)>
        {
            Ok(<$zome_api>::create_inventory_from_event(
                RESOURCE_ENTRY_TYPE,
                params,
            )?)
        }

        #[hdk_extern]
        fn _internal_update_inventory(event: EventCreateRequest) -> ExternResult<Vec<(SignedActionHashed, EconomicResourceAddress, EntryData, EntryData)>>
        {
            Ok(<$zome_api>::update_inventory_from_event(event)?)
        }

        #[hdk_extern]
        fn get_economic_resource(ByAddress { address }: ByAddress<EconomicResourceAddress>) -> ExternResult<$crate::ResponseData> {
            Ok(<$zome_api>::get_economic_resource(address)?)
        }

        #[hdk_extern]
        fn update_economic_resource(UpdateParams { resource }: UpdateParams) -> ExternResult<$crate::ResponseData> {
            Ok(<$zome_api>::update_economic_resource(resource)?)
        }
    };
}
