use hdk_records::{RecordAPIResult};
pub use hc_zome_rea_economic_event_rpc::*;

/// API interface for the external zome gateway
pub trait API {
    type S: AsRef<str>;

    fn create_economic_event(entry_def_id: Self::S, process_entry_def_id: Self::S,
        event: CreateRequest, new_inventoried_resource: Option<ResourceCreateRequest>
    ) -> RecordAPIResult<ResponseData>;
    fn get_economic_event(entry_def_id: Self::S, address: EconomicEventAddress) -> RecordAPIResult<ResponseData>;
    fn update_economic_event(entry_def_id: Self::S, event: UpdateRequest) -> RecordAPIResult<ResponseData>;
    fn delete_economic_event(revision_id: RevisionHash) -> RecordAPIResult<bool>;
    fn get_all_economic_events(entry_def_id: Self::S) -> RecordAPIResult<EventResponseCollection>;
}

/// Macro to programatically and predictably bind an `API` implementation to a
/// ValueFlows-compatibile module storage API.
///
/// See the associated client-side bindings in `modules/vf-graphql-holochain`
/// as a reference for how this API signature binds to JavaScript application code.
///
#[macro_export]
macro_rules! declare_economic_event_zome_api {
    ( $zome_api:ty ) => {
        #[hdk_extern]
        fn create_economic_event(CreateParams { event, new_inventoried_resource }: CreateParams) -> ExternResult<ResponseData> {
            Ok(<$zome_api>::create_economic_event(
                EVENT_ENTRY_TYPE, PROCESS_ENTRY_TYPE,
                event, new_inventoried_resource,
            )?)
        }

        #[hdk_extern]
        fn get_economic_event(ByAddress { address }: ByAddress<EconomicEventAddress>) -> ExternResult<ResponseData> {
            Ok(<$zome_api>::get_economic_event(EVENT_ENTRY_TYPE, address)?)
        }

        #[hdk_extern]
        fn update_economic_event(UpdateParams { event }: UpdateParams) -> ExternResult<ResponseData> {
            Ok(<$zome_api>::update_economic_event(EVENT_ENTRY_TYPE, event)?)
        }

        #[hdk_extern]
        fn delete_economic_event(ByHeader { address }: ByHeader) -> ExternResult<bool> {
            Ok(<$zome_api>::delete_economic_event(address)?)
        }

        #[hdk_extern]
        fn get_all_economic_events(_: ()) -> ExternResult<EventResponseCollection> {
            Ok(<$zome_api>::get_all_economic_events(EVENT_ENTRY_TYPE)?)
        }
    };
}

/// Macro to generate a default (permissable) validation function for EconomicResource
/// records in the local zome (local field checks only).
/// This is the minimum validation required by any zome, and regardless of other
/// validation rules being implemented it is critical that
/// `record.validate_or_fields()` and `record.validate_action()` be peformed upon
/// `EntryStorage` creation.
///
/// Use this method as a reference, and always call the below logic before any
/// application-specific validation in custom validation rules.
///
#[macro_export]
macro_rules! declare_economic_event_zome_validation_defaults {
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
                Ok(event_storage) => {
                    let record = event_storage.entry();
                    record.validate_or_fields()
                        .and_then(|()| { record.validate_action() })
                        .and_then(|()| { Ok(ValidateCallbackResult::Valid) })
                        .or_else(|e| { Ok(ValidateCallbackResult::Invalid(e)) })
                },
                _ => Ok(ValidateCallbackResult::Valid),
            }
        }
    };
}
