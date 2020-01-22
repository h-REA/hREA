use hdk::{
    error::{ ZomeApiResult, /* ZomeApiError */ },
};

use hdk_graph_helpers::{
    records::{
        create_record,
        read_record_entry,
        update_record,
        delete_record,
    },
};

use vf_core::type_aliases::{
    ProposalAddress,
};
use vf_proposal::identifiers::{
    PROPOSAL_ENTRY_TYPE,
    PROPOSAL_BASE_ENTRY_TYPE,
    PROPOSAL_INITIAL_ENTRY_LINK_TYPE,
};
use vf_proposal::proposal::{
    Entry,
    UpdateRequest,
    CreateRequest,
    ResponseData as Response,
    construct_response,
    get_link_fields,
};

pub fn receive_create_proposal(prop_to: CreateRequest) -> ZomeApiResult<Response> {
    let (base_address, entry_resp): (ProposalAddress, Entry) = create_record(
        PROPOSAL_BASE_ENTRY_TYPE,
        PROPOSAL_ENTRY_TYPE,
        PROPOSAL_INITIAL_ENTRY_LINK_TYPE,
        prop_to.to_owned(),
    )?;
    Ok(construct_response(&base_address, &entry_resp, get_link_fields(&base_address) ))
}
pub fn receive_get_proposal(address: ProposalAddress) -> ZomeApiResult<Response> {
    Ok(construct_response(&address, &read_record_entry(&address)?, get_link_fields(&address) ))
}
pub fn receive_update_proposal(prop_to: UpdateRequest) -> ZomeApiResult<Response> {
    handle_update_proposal(&prop_to)
}
pub fn receive_delete_proposal(address: ProposalAddress) -> ZomeApiResult<bool> {
    delete_record::<Entry>(&address)
}

fn handle_update_proposal(prop_to: &UpdateRequest) -> ZomeApiResult<Response> {
    let address = prop_to.get_id();
    let new_entry = update_record(PROPOSAL_ENTRY_TYPE, &address, prop_to)?;
    Ok(construct_response(address, &new_entry, get_link_fields(&address) ))
}
