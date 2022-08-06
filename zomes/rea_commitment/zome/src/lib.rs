/**
 * Commitment zome API definition
 *
 * # Remarks
 *
 * Defines the top-level zome configuration needed by Holochain's build system
 * to bundle the app. This basically involves wiring up the `entry!` type macros
 * and `define_zome!` definition to the standard Rust code in the rest of this
 * module.
 *
 * @package: HoloREA
 * @since:   2019-02-06
 */
use hdk::prelude::*;

use hc_zome_rea_commitment_rpc::*;
use hc_zome_rea_commitment_lib::*;
use hc_zome_rea_commitment_storage::*;
use hc_zome_rea_commitment_storage_consts::*;

#[hdk_extern]
fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
    match op {
        Op::StoreRecord { .. } => Ok(ValidateCallbackResult::Valid),
        Op::StoreEntry { entry, .. } => validate_entry(entry),
        Op::RegisterCreateLink { .. } => Ok(ValidateCallbackResult::Valid),
        Op::RegisterDeleteLink { .. } => Ok(ValidateCallbackResult::Valid),
        Op::RegisterUpdate { .. } => Ok(ValidateCallbackResult::Valid),
        Op::RegisterDelete { .. } => Ok(ValidateCallbackResult::Valid),
        Op::RegisterAgentActivity { .. } => Ok(ValidateCallbackResult::Valid),
    }
}

fn validate_entry(entry: Entry) -> ExternResult<ValidateCallbackResult> {
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

#[hdk_extern]
fn entry_defs(_: ()) -> ExternResult<EntryDefsCallbackResult> {
    Ok(EntryDefsCallbackResult::from(vec![
        PathEntry::entry_def(),
        CommitmentAddress::entry_def(),
        EntryDef {
            id: CAP_STORAGE_ENTRY_DEF_ID.into(),
            visibility: EntryVisibility::Private,
            required_validations: 1.into(),
        },
        EntryDef {
            id: COMMITMENT_ENTRY_TYPE.into(),
            visibility: EntryVisibility::Public,
            required_validations: 2.into(),
        }
    ]))
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateParams {
    pub commitment: CreateRequest,
}

#[hdk_extern]
fn create_commitment(CreateParams { commitment }: CreateParams) -> ExternResult<ResponseData> {
    Ok(handle_create_commitment(COMMITMENT_ENTRY_TYPE, commitment)?)
}

#[derive(Debug, Serialize, Deserialize)]
struct ByAddress {
    pub address: CommitmentAddress,
}

#[hdk_extern]
fn get_commitment(ByAddress { address }: ByAddress) -> ExternResult<ResponseData> {
    Ok(handle_get_commitment(address)?)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateParams {
    pub commitment: UpdateRequest,
}

#[hdk_extern]
fn update_commitment(UpdateParams { commitment }: UpdateParams) -> ExternResult<ResponseData> {
    Ok(handle_update_commitment(commitment)?)
}

#[hdk_extern]
fn delete_commitment(ByRevision { revision_id }: ByRevision) -> ExternResult<bool> {
    Ok(handle_delete_commitment(revision_id)?)
}
