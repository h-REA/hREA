use hdk_records::RecordAPIResult;
pub use hc_zome_rea_economic_event_rpc::*;

/// API interface for the external zome gateway
pub trait API {
    type S: AsRef<str>;

    fn create_economic_event(entry_def_id: Self::S,
        event: CreateRequest, new_inventoried_resource: Option<ResourceCreateRequest>
    ) -> RecordAPIResult<ResponseData>;
    fn get_economic_event(address: EconomicEventAddress) -> RecordAPIResult<ResponseData>;
    fn get_revision(revision_id: ActionHash) -> RecordAPIResult<ResponseData>;
    fn update_economic_event(event: UpdateRequest) -> RecordAPIResult<ResponseData>;
    fn delete_economic_event(revision_id: ActionHash) -> RecordAPIResult<bool>;
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
                EVENT_ENTRY_TYPE,
                event, new_inventoried_resource,
            )?)
        }

        #[hdk_extern]
        fn get_economic_event(ByAddress { address }: ByAddress<EconomicEventAddress>) -> ExternResult<ResponseData> {
            Ok(<$zome_api>::get_economic_event(address)?)
        }

        #[hdk_extern]
        fn get_revision(ByRevision { revision_id }: ByRevision) -> ExternResult<ResponseData> {
            Ok(<$zome_api>::get_revision(revision_id)?)
        }

        #[hdk_extern]
        fn update_economic_event(UpdateParams { event }: UpdateParams) -> ExternResult<ResponseData> {
            Ok(<$zome_api>::update_economic_event(event)?)
        }

        #[hdk_extern]
        fn delete_economic_event(ByRevision { revision_id }: ByRevision) -> ExternResult<bool> {
            Ok(<$zome_api>::delete_economic_event(revision_id)?)
        }
    };
}
