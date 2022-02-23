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

#[hdk_extern]
fn entry_defs(_: ()) -> ExternResult<EntryDefsCallbackResult> {
    Ok(EntryDefsCallbackResult::from(vec![
        temp_path::path::Path::entry_def(),
        EntryDef {
            id: CAP_STORAGE_ENTRY_DEF_ID.into(),
            visibility: EntryVisibility::Private,
            crdt_type: CrdtType,
            required_validations: 1.into(),
            required_validation_type: RequiredValidationType::default(),
        },
        EntryDef {
            id: COMMITMENT_ENTRY_TYPE.into(),
            visibility: EntryVisibility::Public,
            crdt_type: CrdtType,
            required_validations: 2.into(),
            required_validation_type: RequiredValidationType::default(),
        }
    ]))
}

#[derive(Debug, Serialize, Deserialize)]
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
    Ok(handle_get_commitment(COMMITMENT_ENTRY_TYPE, address)?)
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateParams {
    pub commitment: UpdateRequest,
}

#[hdk_extern]
fn update_commitment(UpdateParams { commitment }: UpdateParams) -> ExternResult<ResponseData> {
    Ok(handle_update_commitment(COMMITMENT_ENTRY_TYPE, commitment)?)
}

#[derive(Debug, Serialize, Deserialize)]
struct ByHeader {
    pub address: RevisionHash,
}

#[hdk_extern]
fn delete_commitment(ByHeader { address }: ByHeader) -> ExternResult<bool> {
    Ok(handle_delete_commitment(address)?)
}
