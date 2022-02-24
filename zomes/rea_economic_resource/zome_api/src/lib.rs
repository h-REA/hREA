use hdk_records::{RecordAPIResult};
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

    fn create_inventory_from_event(resource_entry_def_id: Self::S, params: CreationPayload) -> RecordAPIResult<(HeaderHash, EconomicResourceAddress, EntryData)>;
    fn update_inventory_from_event(
        resource_entry_def_id: Self::S,
        event: EventCreateRequest,
    ) -> RecordAPIResult<Vec<(HeaderHash, EconomicResourceAddress, EntryData, EntryData)>>;
    fn get_economic_resource(entry_def_id: Self::S, event_entry_def_id: Self::S, process_entry_def_id: Self::S, address: EconomicResourceAddress) -> RecordAPIResult<ResponseData>;
    fn update_economic_resource(entry_def_id: Self::S, event_entry_def_id: Self::S, process_entry_def_id: Self::S, resource: UpdateRequest) -> RecordAPIResult<ResponseData>;
    fn get_all_economic_resources(entry_def_id: Self::S, event_entry_def_id: Self::S, process_entry_def_id: Self::S) -> RecordAPIResult<Collection>;
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
        fn _internal_create_inventory(params: CreationPayload) -> ExternResult<(HeaderHash, EconomicResourceAddress, EntryData)>
        {
            Ok(<$zome_api>::create_inventory_from_event(
                RESOURCE_ENTRY_TYPE,
                params,
            )?)
        }

        #[hdk_extern]
        fn _internal_update_inventory(event: EventCreateRequest) -> ExternResult<Vec<(HeaderHash, EconomicResourceAddress, EntryData, EntryData)>>
        {
            Ok(<$zome_api>::update_inventory_from_event(RESOURCE_ENTRY_TYPE, event)?)
        }

        #[hdk_extern]
        fn get_economic_resource(ByAddress { address }: ByAddress<EconomicResourceAddress>) -> ExternResult<$crate::ResponseData> {
            Ok(<$zome_api>::get_economic_resource(
                RESOURCE_ENTRY_TYPE, EVENT_ENTRY_TYPE, PROCESS_ENTRY_TYPE,
                address,
            )?)
        }

        #[hdk_extern]
        fn update_economic_resource(UpdateParams { resource }: UpdateParams) -> ExternResult<$crate::ResponseData> {
            Ok(<$zome_api>::update_economic_resource(
                RESOURCE_ENTRY_TYPE, EVENT_ENTRY_TYPE, PROCESS_ENTRY_TYPE,
                resource
            )?)
        }

        #[hdk_extern]
        fn get_all_economic_resources(_: ()) -> ExternResult<$crate::Collection> {
            Ok(<$zome_api>::get_all_economic_resources(RESOURCE_ENTRY_TYPE, EVENT_ENTRY_TYPE, PROCESS_ENTRY_TYPE)?)
        }
    };
}

/// Macro to generate a default (permissable) validation function for EconomicResource
/// records in the local zome (local field checks only).
/// This is the minimum validation required by any zome, and regardless of other
/// validation rules being implemented it is critical that `record.validate()`
/// be peformed upon `EntryStorage` creation.
///
/// Use this method as a reference, and always call the below logic before any
/// application-specific validation.
#[macro_export]
macro_rules! declare_economic_resource_zome_validation_defaults {
    ( /*$zome_api:ty*/ ) => {
        #[hdk_extern]
        fn validate(validation_data: ValidateData) -> ExternResult<ValidateCallbackResult> {
            let element = validation_data.element;
            let entry = element.into_inner().1;
            let entry = match entry {
                ElementEntry::Present(e) => e,
                _ => return Ok(ValidateCallbackResult::Valid),
            };

            match EntryStorage::try_from(&entry) {
                Ok(resource_storage) => {
                    let record = resource_storage.entry();
                    record.validate()
                        .and_then(|()| { Ok(ValidateCallbackResult::Valid) })
                        .or_else(|e| { Ok(ValidateCallbackResult::Invalid(e)) })
                },
                _ => Ok(ValidateCallbackResult::Valid),
            }
        }
    };
}
