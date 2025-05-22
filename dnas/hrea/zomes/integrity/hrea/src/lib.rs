pub mod rea_economic_resource_to_rea_economic_events;
pub use rea_economic_resource_to_rea_economic_events::*;
pub mod rea_economic_event;
pub use rea_economic_event::*;
pub mod rea_economic_resource;
pub use rea_economic_resource::*;
pub mod rea_intent;
pub use rea_intent::*;
pub mod rea_commitment;
pub use rea_commitment::*;
pub mod rea_recipe_flow;
pub use rea_recipe_flow::*;
pub mod rea_proposal;
pub use rea_proposal::*;
pub mod rea_recipe_exchange;
pub use rea_recipe_exchange::*;
pub mod rea_recipe_process;
pub use rea_recipe_process::*;
pub mod rea_resource_specification;
pub use rea_resource_specification::*;
pub mod rea_unit;
pub use rea_unit::*;
pub mod rea_process;
pub use rea_process::*;
pub mod rea_plan;
pub use rea_plan::*;
pub mod rea_process_specification;
pub use rea_process_specification::*;
pub mod rea_agreement;
pub use rea_agreement::*;
pub mod rea_agent;
pub use rea_agent::*;
use hdi::prelude::*;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[hdk_entry_types]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    ReaAgent(ReaAgent),
    ReaAgreement(ReaAgreement),
    ReaProcessSpecification(ReaProcessSpecification),
    ReaPlan(ReaPlan),
    ReaProcess(ReaProcess),
    ReaUnit(ReaUnit),
    ReaResourceSpecification(ReaResourceSpecification),
    ReaRecipeProcess(ReaRecipeProcess),
    ReaRecipeExchange(ReaRecipeExchange),
    ReaProposal(ReaProposal),
    ReaRecipeFlow(ReaRecipeFlow),
    ReaCommitment(ReaCommitment),
    ReaIntent(ReaIntent),
    ReaEconomicResource(ReaEconomicResource),
    ReaEconomicEvent(ReaEconomicEvent),
}

#[derive(Serialize, Deserialize)]
#[hdk_link_types]
pub enum LinkTypes {
    ReaAgentUpdates,
    AllAgents,
    ReaAgreementUpdates,
    AllAgreements,
    ReaProcessSpecificationUpdates,
    AllProcessSpecifications,
    ReaPlanUpdates,
    AllPlans,
    ReaProcessSpecificationToReaProcesses,
    ReaPlanToReaProcesses,
    ReaProcessUpdates,
    AllProcesses,
    ReaUnitUpdates,
    AllUnits,
    ReaResourceSpecificationUpdates,
    AllResourceSpecifications,
    ReaRecipeProcessUpdates,
    AllRecipeProcesses,
    ReaRecipeExchangeUpdates,
    AllRecipeExchanges,
    ReaProposalUpdates,
    AllProposals,
    ReaRecipeExchangeToReaRecipeFlows,
    ReaRecipeExchangeToReaRecipeFlowsReciprocal,
    ReaRecipeProcessToReaRecipeFlowInputs,
    ReaRecipeProcessToReaRecipeFlowOutputs,
    ReaRecipeFlowUpdates,
    ReaProcessToInputs,
    ReaProcessToOutputs,
    ProviderToReaCommitments,
    ReceiverToReaCommitments,
    ReaAgreementToReaCommitments,
    ReaPlanToReaCommitments,
    ReaPlanToIndependentDemands,
    ReaCommitmentUpdates,
    ReaProcessToReaIntentInputs,
    ReaProcessToReaIntentOutputs,
    ProviderToReaIntents,
    ReceiverToReaIntents,
    ReaIntentUpdates,
    ReaEconomicResourceToReaEconomicResources,
    ReaEconomicResourceUpdates,
    AllEconomicResources,
    ReaProcessToReaEconomicEventInputs,
    ReaProcessToReaEconomicEventOutputs,
    ProviderToReaEconomicEvents,
    ReceiverToReaEconomicEvents,
    ReaAgreementToReaEconomicEvents,
    ReaEconomicEventToReaEconomicEvents,
    ReaEconomicEventUpdates,
    AllEconomicEvents,
    ReaEconomicResourceToReaEconomicEvents,
    CommitmentToFulfillingEconomicEvents,
    IntentToSatisfyingCommitments,
    IntentToSatisfyingEconomicEvents,
}

// Validation you perform during the genesis process. Nobody else on the network performs it, only you.
// There *is no* access to network calls in this callback
#[hdk_extern]
pub fn genesis_self_check(_data: GenesisSelfCheckData) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}

// Validation the network performs when you try to join, you can't perform this validation yourself as you are not a member yet.
// There *is* access to network calls in this function
pub fn validate_agent_joining(
    _agent_pub_key: AgentPubKey,
    _membrane_proof: &Option<MembraneProof>,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}

// This is the unified validation callback for all entries and link types in this integrity zome
// Below is a match template for all of the variants of `DHT Ops` and entry and link types
// Holochain has already performed the following validation for you:
// - The action signature matches on the hash of its content and is signed by its author
// - The previous action exists, has a lower timestamp than the new action, and incremented sequence number
// - The previous action author is the same as the new action author
// - The timestamp of each action is after the DNA's origin time
// - AgentActivity authorities check that the agent hasn't forked their chain
// - The entry hash in the action matches the entry content
// - The entry type in the action matches the entry content
// - The entry size doesn't exceed the maximum entry size (currently 4MB)
// - Private entry types are not included in the Op content, and public entry types are
// - If the `Op` is an update or a delete, the original action exists and is a `Create` or `Update` action
// - If the `Op` is an update, the original entry exists and is of the same type as the new one
// - If the `Op` is a delete link, the original action exists and is a `CreateLink` action
// - Link tags don't exceed the maximum tag size (currently 1KB)
// - Countersigned entries include an action from each required signer
// You can read more about validation here: https://docs.rs/hdi/latest/hdi/index.html#data-validation
#[hdk_extern]
pub fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
    match op.flattened::<EntryTypes, LinkTypes>()? {
        FlatOp::StoreEntry(store_entry) => match store_entry {
            OpEntry::CreateEntry { app_entry, action } => match app_entry {
                EntryTypes::ReaAgent(rea_agent) => {
                    validate_create_rea_agent(EntryCreationAction::Create(action), rea_agent)
                }
                EntryTypes::ReaAgreement(rea_agreement) => validate_create_rea_agreement(
                    EntryCreationAction::Create(action),
                    rea_agreement,
                ),
                EntryTypes::ReaProcessSpecification(rea_process_specification) => {
                    validate_create_rea_process_specification(
                        EntryCreationAction::Create(action),
                        rea_process_specification,
                    )
                }
                EntryTypes::ReaPlan(rea_plan) => {
                    validate_create_rea_plan(EntryCreationAction::Create(action), rea_plan)
                }
                EntryTypes::ReaProcess(rea_process) => {
                    validate_create_rea_process(EntryCreationAction::Create(action), rea_process)
                }
                EntryTypes::ReaUnit(rea_unit) => {
                    validate_create_rea_unit(EntryCreationAction::Create(action), rea_unit)
                }
                EntryTypes::ReaResourceSpecification(rea_resource_specification) => {
                    validate_create_rea_resource_specification(
                        EntryCreationAction::Create(action),
                        rea_resource_specification,
                    )
                }
                EntryTypes::ReaRecipeProcess(rea_recipe_process) => {
                    validate_create_rea_recipe_process(
                        EntryCreationAction::Create(action),
                        rea_recipe_process,
                    )
                }
                EntryTypes::ReaRecipeExchange(rea_recipe_exchange) => {
                    validate_create_rea_recipe_exchange(
                        EntryCreationAction::Create(action),
                        rea_recipe_exchange,
                    )
                }
                EntryTypes::ReaProposal(rea_proposal) => {
                    validate_create_rea_proposal(EntryCreationAction::Create(action), rea_proposal)
                }
                EntryTypes::ReaRecipeFlow(rea_recipe_flow) => validate_create_rea_recipe_flow(
                    EntryCreationAction::Create(action),
                    rea_recipe_flow,
                ),
                EntryTypes::ReaCommitment(rea_commitment) => validate_create_rea_commitment(
                    EntryCreationAction::Create(action),
                    rea_commitment,
                ),
                EntryTypes::ReaIntent(rea_intent) => {
                    validate_create_rea_intent(EntryCreationAction::Create(action), rea_intent)
                }
                EntryTypes::ReaEconomicResource(rea_economic_resource) => {
                    validate_create_rea_economic_resource(
                        EntryCreationAction::Create(action),
                        rea_economic_resource,
                    )
                }
                EntryTypes::ReaEconomicEvent(rea_economic_event) => {
                    validate_create_rea_economic_event(
                        EntryCreationAction::Create(action),
                        rea_economic_event,
                    )
                }
            },
            OpEntry::UpdateEntry {
                app_entry, action, ..
            } => match app_entry {
                EntryTypes::ReaAgent(rea_agent) => {
                    validate_create_rea_agent(EntryCreationAction::Update(action), rea_agent)
                }
                EntryTypes::ReaAgreement(rea_agreement) => validate_create_rea_agreement(
                    EntryCreationAction::Update(action),
                    rea_agreement,
                ),
                EntryTypes::ReaProcessSpecification(rea_process_specification) => {
                    validate_create_rea_process_specification(
                        EntryCreationAction::Update(action),
                        rea_process_specification,
                    )
                }
                EntryTypes::ReaPlan(rea_plan) => {
                    validate_create_rea_plan(EntryCreationAction::Update(action), rea_plan)
                }
                EntryTypes::ReaProcess(rea_process) => {
                    validate_create_rea_process(EntryCreationAction::Update(action), rea_process)
                }
                EntryTypes::ReaUnit(rea_unit) => {
                    validate_create_rea_unit(EntryCreationAction::Update(action), rea_unit)
                }
                EntryTypes::ReaResourceSpecification(rea_resource_specification) => {
                    validate_create_rea_resource_specification(
                        EntryCreationAction::Update(action),
                        rea_resource_specification,
                    )
                }
                EntryTypes::ReaRecipeProcess(rea_recipe_process) => {
                    validate_create_rea_recipe_process(
                        EntryCreationAction::Update(action),
                        rea_recipe_process,
                    )
                }
                EntryTypes::ReaRecipeExchange(rea_recipe_exchange) => {
                    validate_create_rea_recipe_exchange(
                        EntryCreationAction::Update(action),
                        rea_recipe_exchange,
                    )
                }
                EntryTypes::ReaProposal(rea_proposal) => {
                    validate_create_rea_proposal(EntryCreationAction::Update(action), rea_proposal)
                }
                EntryTypes::ReaRecipeFlow(rea_recipe_flow) => validate_create_rea_recipe_flow(
                    EntryCreationAction::Update(action),
                    rea_recipe_flow,
                ),
                EntryTypes::ReaCommitment(rea_commitment) => validate_create_rea_commitment(
                    EntryCreationAction::Update(action),
                    rea_commitment,
                ),
                EntryTypes::ReaIntent(rea_intent) => {
                    validate_create_rea_intent(EntryCreationAction::Update(action), rea_intent)
                }
                EntryTypes::ReaEconomicResource(rea_economic_resource) => {
                    validate_create_rea_economic_resource(
                        EntryCreationAction::Update(action),
                        rea_economic_resource,
                    )
                }
                EntryTypes::ReaEconomicEvent(rea_economic_event) => {
                    validate_create_rea_economic_event(
                        EntryCreationAction::Update(action),
                        rea_economic_event,
                    )
                }
            },
            _ => Ok(ValidateCallbackResult::Valid),
        },
        FlatOp::RegisterUpdate(update_entry) => match update_entry {
            OpUpdate::Entry { app_entry, action } => {
                let original_action = must_get_action(action.clone().original_action_address)?
                    .action()
                    .to_owned();
                let original_create_action = match EntryCreationAction::try_from(original_action) {
                    Ok(action) => action,
                    Err(e) => {
                        return Ok(ValidateCallbackResult::Invalid(format!(
                            "Expected to get EntryCreationAction from Action: {e:?}"
                        )));
                    }
                };
                match app_entry {
                    EntryTypes::ReaEconomicEvent(rea_economic_event) => {
                        let original_app_entry =
                            must_get_valid_record(action.clone().original_action_address)?;
                        let original_rea_economic_event =
                            match ReaEconomicEvent::try_from(original_app_entry) {
                                Ok(entry) => entry,
                                Err(e) => {
                                    return Ok(ValidateCallbackResult::Invalid(format!(
                                        "Expected to get ReaEconomicEvent from Record: {e:?}"
                                    )));
                                }
                            };
                        validate_update_rea_economic_event(
                            action,
                            rea_economic_event,
                            original_create_action,
                            original_rea_economic_event,
                        )
                    }
                    EntryTypes::ReaEconomicResource(rea_economic_resource) => {
                        let original_app_entry =
                            must_get_valid_record(action.clone().original_action_address)?;
                        let original_rea_economic_resource =
                            match ReaEconomicResource::try_from(original_app_entry) {
                                Ok(entry) => entry,
                                Err(e) => {
                                    return Ok(ValidateCallbackResult::Invalid(format!(
                                        "Expected to get ReaEconomicResource from Record: {e:?}"
                                    )));
                                }
                            };
                        validate_update_rea_economic_resource(
                            action,
                            rea_economic_resource,
                            original_create_action,
                            original_rea_economic_resource,
                        )
                    }
                    EntryTypes::ReaIntent(rea_intent) => {
                        let original_app_entry =
                            must_get_valid_record(action.clone().original_action_address)?;
                        let original_rea_intent = match ReaIntent::try_from(original_app_entry) {
                            Ok(entry) => entry,
                            Err(e) => {
                                return Ok(ValidateCallbackResult::Invalid(format!(
                                    "Expected to get ReaIntent from Record: {e:?}"
                                )));
                            }
                        };
                        validate_update_rea_intent(
                            action,
                            rea_intent,
                            original_create_action,
                            original_rea_intent,
                        )
                    }
                    EntryTypes::ReaCommitment(rea_commitment) => {
                        let original_app_entry =
                            must_get_valid_record(action.clone().original_action_address)?;
                        let original_rea_commitment =
                            match ReaCommitment::try_from(original_app_entry) {
                                Ok(entry) => entry,
                                Err(e) => {
                                    return Ok(ValidateCallbackResult::Invalid(format!(
                                        "Expected to get ReaCommitment from Record: {e:?}"
                                    )));
                                }
                            };
                        validate_update_rea_commitment(
                            action,
                            rea_commitment,
                            original_create_action,
                            original_rea_commitment,
                        )
                    }
                    EntryTypes::ReaRecipeFlow(rea_recipe_flow) => {
                        let original_app_entry =
                            must_get_valid_record(action.clone().original_action_address)?;
                        let original_rea_recipe_flow =
                            match ReaRecipeFlow::try_from(original_app_entry) {
                                Ok(entry) => entry,
                                Err(e) => {
                                    return Ok(ValidateCallbackResult::Invalid(format!(
                                        "Expected to get ReaRecipeFlow from Record: {e:?}"
                                    )));
                                }
                            };
                        validate_update_rea_recipe_flow(
                            action,
                            rea_recipe_flow,
                            original_create_action,
                            original_rea_recipe_flow,
                        )
                    }
                    EntryTypes::ReaProposal(rea_proposal) => {
                        let original_app_entry =
                            must_get_valid_record(action.clone().original_action_address)?;
                        let original_rea_proposal = match ReaProposal::try_from(original_app_entry)
                        {
                            Ok(entry) => entry,
                            Err(e) => {
                                return Ok(ValidateCallbackResult::Invalid(format!(
                                    "Expected to get ReaProposal from Record: {e:?}"
                                )));
                            }
                        };
                        validate_update_rea_proposal(
                            action,
                            rea_proposal,
                            original_create_action,
                            original_rea_proposal,
                        )
                    }
                    EntryTypes::ReaRecipeExchange(rea_recipe_exchange) => {
                        let original_app_entry =
                            must_get_valid_record(action.clone().original_action_address)?;
                        let original_rea_recipe_exchange =
                            match ReaRecipeExchange::try_from(original_app_entry) {
                                Ok(entry) => entry,
                                Err(e) => {
                                    return Ok(ValidateCallbackResult::Invalid(format!(
                                        "Expected to get ReaRecipeExchange from Record: {e:?}"
                                    )));
                                }
                            };
                        validate_update_rea_recipe_exchange(
                            action,
                            rea_recipe_exchange,
                            original_create_action,
                            original_rea_recipe_exchange,
                        )
                    }
                    EntryTypes::ReaRecipeProcess(rea_recipe_process) => {
                        let original_app_entry =
                            must_get_valid_record(action.clone().original_action_address)?;
                        let original_rea_recipe_process =
                            match ReaRecipeProcess::try_from(original_app_entry) {
                                Ok(entry) => entry,
                                Err(e) => {
                                    return Ok(ValidateCallbackResult::Invalid(format!(
                                        "Expected to get ReaRecipeProcess from Record: {e:?}"
                                    )));
                                }
                            };
                        validate_update_rea_recipe_process(
                            action,
                            rea_recipe_process,
                            original_create_action,
                            original_rea_recipe_process,
                        )
                    }
                    EntryTypes::ReaResourceSpecification(rea_resource_specification) => {
                        let original_app_entry =
                            must_get_valid_record(action.clone().original_action_address)?;
                        let original_rea_resource_specification =
                            match ReaResourceSpecification::try_from(original_app_entry) {
                                Ok(entry) => entry,
                                Err(e) => {
                                    return Ok(
                                        ValidateCallbackResult::Invalid(
                                            format!(
                                                "Expected to get ReaResourceSpecification from Record: {e:?}"
                                            ),
                                        ),
                                    );
                                }
                            };
                        validate_update_rea_resource_specification(
                            action,
                            rea_resource_specification,
                            original_create_action,
                            original_rea_resource_specification,
                        )
                    }
                    EntryTypes::ReaUnit(rea_unit) => {
                        let original_app_entry =
                            must_get_valid_record(action.clone().original_action_address)?;
                        let original_rea_unit = match ReaUnit::try_from(original_app_entry) {
                            Ok(entry) => entry,
                            Err(e) => {
                                return Ok(ValidateCallbackResult::Invalid(format!(
                                    "Expected to get ReaUnit from Record: {e:?}"
                                )));
                            }
                        };
                        validate_update_rea_unit(
                            action,
                            rea_unit,
                            original_create_action,
                            original_rea_unit,
                        )
                    }
                    EntryTypes::ReaProcess(rea_process) => {
                        let original_app_entry =
                            must_get_valid_record(action.clone().original_action_address)?;
                        let original_rea_process = match ReaProcess::try_from(original_app_entry) {
                            Ok(entry) => entry,
                            Err(e) => {
                                return Ok(ValidateCallbackResult::Invalid(format!(
                                    "Expected to get ReaProcess from Record: {e:?}"
                                )));
                            }
                        };
                        validate_update_rea_process(
                            action,
                            rea_process,
                            original_create_action,
                            original_rea_process,
                        )
                    }
                    EntryTypes::ReaPlan(rea_plan) => {
                        let original_app_entry =
                            must_get_valid_record(action.clone().original_action_address)?;
                        let original_rea_plan = match ReaPlan::try_from(original_app_entry) {
                            Ok(entry) => entry,
                            Err(e) => {
                                return Ok(ValidateCallbackResult::Invalid(format!(
                                    "Expected to get ReaPlan from Record: {e:?}"
                                )));
                            }
                        };
                        validate_update_rea_plan(
                            action,
                            rea_plan,
                            original_create_action,
                            original_rea_plan,
                        )
                    }
                    EntryTypes::ReaProcessSpecification(rea_process_specification) => {
                        let original_app_entry =
                            must_get_valid_record(action.clone().original_action_address)?;
                        let original_rea_process_specification =
                            match ReaProcessSpecification::try_from(original_app_entry) {
                                Ok(entry) => entry,
                                Err(e) => {
                                    return Ok(
                                        ValidateCallbackResult::Invalid(
                                            format!(
                                                "Expected to get ReaProcessSpecification from Record: {e:?}"
                                            ),
                                        ),
                                    );
                                }
                            };
                        validate_update_rea_process_specification(
                            action,
                            rea_process_specification,
                            original_create_action,
                            original_rea_process_specification,
                        )
                    }
                    EntryTypes::ReaAgreement(rea_agreement) => {
                        let original_app_entry =
                            must_get_valid_record(action.clone().original_action_address)?;
                        let original_rea_agreement =
                            match ReaAgreement::try_from(original_app_entry) {
                                Ok(entry) => entry,
                                Err(e) => {
                                    return Ok(ValidateCallbackResult::Invalid(format!(
                                        "Expected to get ReaAgreement from Record: {e:?}"
                                    )));
                                }
                            };
                        validate_update_rea_agreement(
                            action,
                            rea_agreement,
                            original_create_action,
                            original_rea_agreement,
                        )
                    }
                    EntryTypes::ReaAgent(rea_agent) => {
                        let original_app_entry =
                            must_get_valid_record(action.clone().original_action_address)?;
                        let original_rea_agent = match ReaAgent::try_from(original_app_entry) {
                            Ok(entry) => entry,
                            Err(e) => {
                                return Ok(ValidateCallbackResult::Invalid(format!(
                                    "Expected to get ReaAgent from Record: {e:?}"
                                )));
                            }
                        };
                        validate_update_rea_agent(
                            action,
                            rea_agent,
                            original_create_action,
                            original_rea_agent,
                        )
                    }
                }
            }
            _ => Ok(ValidateCallbackResult::Valid),
        },
        FlatOp::RegisterDelete(delete_entry) => {
            let original_action_hash = delete_entry.clone().action.deletes_address;
            let original_record = must_get_valid_record(original_action_hash)?;
            let original_record_action = original_record.action().clone();
            let original_action = match EntryCreationAction::try_from(original_record_action) {
                Ok(action) => action,
                Err(e) => {
                    return Ok(ValidateCallbackResult::Invalid(format!(
                        "Expected to get EntryCreationAction from Action: {e:?}"
                    )));
                }
            };
            let app_entry_type = match original_action.entry_type() {
                EntryType::App(app_entry_type) => app_entry_type,
                _ => {
                    return Ok(ValidateCallbackResult::Valid);
                }
            };
            let entry = match original_record.entry().as_option() {
                Some(entry) => entry,
                None => {
                    return Ok(ValidateCallbackResult::Invalid(
                        "Original record for a delete must contain an entry".to_string(),
                    ));
                }
            };
            let original_app_entry = match EntryTypes::deserialize_from_type(
                app_entry_type.zome_index,
                app_entry_type.entry_index,
                entry,
            )? {
                Some(app_entry) => app_entry,
                None => {
                    return Ok(ValidateCallbackResult::Invalid(
                        "Original app entry must be one of the defined entry types for this zome"
                            .to_string(),
                    ));
                }
            };
            match original_app_entry {
                EntryTypes::ReaEconomicEvent(original_rea_economic_event) => {
                    validate_delete_rea_economic_event(
                        delete_entry.clone().action,
                        original_action,
                        original_rea_economic_event,
                    )
                }
                EntryTypes::ReaEconomicResource(original_rea_economic_resource) => {
                    validate_delete_rea_economic_resource(
                        delete_entry.clone().action,
                        original_action,
                        original_rea_economic_resource,
                    )
                }
                EntryTypes::ReaIntent(original_rea_intent) => validate_delete_rea_intent(
                    delete_entry.clone().action,
                    original_action,
                    original_rea_intent,
                ),
                EntryTypes::ReaCommitment(original_rea_commitment) => {
                    validate_delete_rea_commitment(
                        delete_entry.clone().action,
                        original_action,
                        original_rea_commitment,
                    )
                }
                EntryTypes::ReaRecipeFlow(original_rea_recipe_flow) => {
                    validate_delete_rea_recipe_flow(
                        delete_entry.clone().action,
                        original_action,
                        original_rea_recipe_flow,
                    )
                }
                EntryTypes::ReaProposal(original_rea_proposal) => validate_delete_rea_proposal(
                    delete_entry.clone().action,
                    original_action,
                    original_rea_proposal,
                ),
                EntryTypes::ReaRecipeExchange(original_rea_recipe_exchange) => {
                    validate_delete_rea_recipe_exchange(
                        delete_entry.clone().action,
                        original_action,
                        original_rea_recipe_exchange,
                    )
                }
                EntryTypes::ReaRecipeProcess(original_rea_recipe_process) => {
                    validate_delete_rea_recipe_process(
                        delete_entry.clone().action,
                        original_action,
                        original_rea_recipe_process,
                    )
                }
                EntryTypes::ReaResourceSpecification(original_rea_resource_specification) => {
                    validate_delete_rea_resource_specification(
                        delete_entry.clone().action,
                        original_action,
                        original_rea_resource_specification,
                    )
                }
                EntryTypes::ReaUnit(original_rea_unit) => validate_delete_rea_unit(
                    delete_entry.clone().action,
                    original_action,
                    original_rea_unit,
                ),
                EntryTypes::ReaProcess(original_rea_process) => validate_delete_rea_process(
                    delete_entry.clone().action,
                    original_action,
                    original_rea_process,
                ),
                EntryTypes::ReaPlan(original_rea_plan) => validate_delete_rea_plan(
                    delete_entry.clone().action,
                    original_action,
                    original_rea_plan,
                ),
                EntryTypes::ReaProcessSpecification(original_rea_process_specification) => {
                    validate_delete_rea_process_specification(
                        delete_entry.clone().action,
                        original_action,
                        original_rea_process_specification,
                    )
                }
                EntryTypes::ReaAgreement(original_rea_agreement) => validate_delete_rea_agreement(
                    delete_entry.clone().action,
                    original_action,
                    original_rea_agreement,
                ),
                EntryTypes::ReaAgent(original_rea_agent) => validate_delete_rea_agent(
                    delete_entry.clone().action,
                    original_action,
                    original_rea_agent,
                ),
            }
        }
        FlatOp::RegisterCreateLink {
            link_type,
            base_address,
            target_address,
            tag,
            action,
        } => match link_type {
            LinkTypes::CommitmentToFulfillingEconomicEvents => {
                validate_create_link_commitment_to_fulfilling_economic_events(
                    action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::IntentToSatisfyingCommitments => {
                validate_create_link_intent_to_satisfying_commitments(
                    action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::IntentToSatisfyingEconomicEvents => {
                validate_create_link_intent_to_satisfying_economic_events(
                    action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::ReaAgentUpdates => {
                validate_create_link_rea_agent_updates(action, base_address, target_address, tag)
            }
            LinkTypes::AllAgents => {
                validate_create_link_all_agents(action, base_address, target_address, tag)
            }
            LinkTypes::ReaAgreementUpdates => validate_create_link_rea_agreement_updates(
                action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::AllAgreements => {
                validate_create_link_all_agreements(action, base_address, target_address, tag)
            }
            LinkTypes::ReaProcessSpecificationUpdates => {
                validate_create_link_rea_process_specification_updates(
                    action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::AllProcessSpecifications => validate_create_link_all_process_specifications(
                action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::ReaPlanUpdates => {
                validate_create_link_rea_plan_updates(action, base_address, target_address, tag)
            }
            LinkTypes::AllPlans => {
                validate_create_link_all_plans(action, base_address, target_address, tag)
            }
            LinkTypes::ReaProcessSpecificationToReaProcesses => {
                validate_create_link_rea_process_specification_to_rea_processes(
                    action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::ReaPlanToReaProcesses => validate_create_link_rea_plan_to_rea_processes(
                action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::ReaProcessUpdates => {
                validate_create_link_rea_process_updates(action, base_address, target_address, tag)
            }
            LinkTypes::AllProcesses => {
                validate_create_link_all_processes(action, base_address, target_address, tag)
            }
            LinkTypes::ReaUnitUpdates => {
                validate_create_link_rea_unit_updates(action, base_address, target_address, tag)
            }
            LinkTypes::AllUnits => {
                validate_create_link_all_units(action, base_address, target_address, tag)
            }
            LinkTypes::ReaResourceSpecificationUpdates => {
                validate_create_link_rea_resource_specification_updates(
                    action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::AllResourceSpecifications => {
                validate_create_link_all_resource_specifications(
                    action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::ReaRecipeProcessUpdates => validate_create_link_rea_recipe_process_updates(
                action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::AllRecipeProcesses => {
                validate_create_link_all_recipe_processes(action, base_address, target_address, tag)
            }
            LinkTypes::ReaRecipeExchangeUpdates => {
                validate_create_link_rea_recipe_exchange_updates(
                    action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::AllRecipeExchanges => {
                validate_create_link_all_recipe_exchanges(action, base_address, target_address, tag)
            }
            LinkTypes::ReaProposalUpdates => {
                validate_create_link_rea_proposal_updates(action, base_address, target_address, tag)
            }
            LinkTypes::AllProposals => {
                validate_create_link_all_proposals(action, base_address, target_address, tag)
            }
            LinkTypes::ReaRecipeExchangeToReaRecipeFlows => {
                validate_create_link_rea_recipe_exchange_to_rea_recipe_flows(
                    action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::ReaRecipeExchangeToReaRecipeFlowsReciprocal => {
                validate_create_link_rea_recipe_exchange_to_rea_recipe_flows(
                    action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::ReaRecipeProcessToReaRecipeFlowInputs => {
                validate_create_link_rea_recipe_process_to_rea_recipe_flows(
                    action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::ReaRecipeProcessToReaRecipeFlowOutputs => {
                validate_create_link_rea_recipe_process_to_rea_recipe_flows(
                    action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::ReaRecipeFlowUpdates => validate_create_link_rea_recipe_flow_updates(
                action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::ReaProcessToInputs => validate_create_link_rea_process_to_rea_commitments(
                action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::ReaProcessToOutputs => validate_create_link_rea_process_to_rea_commitments(
                action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::ProviderToReaCommitments => {
                validate_create_link_rea_agent_to_rea_commitments(
                    action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::ReceiverToReaCommitments => {
                validate_create_link_rea_agent_to_rea_commitments(
                    action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::ReaAgreementToReaCommitments => {
                validate_create_link_rea_agreement_to_rea_commitments(
                    action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::ReaPlanToReaCommitments => validate_create_link_rea_plan_to_rea_commitments(
                action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::ReaPlanToIndependentDemands => {
                validate_create_link_rea_plan_to_rea_commitments(
                    action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::ReaCommitmentUpdates => validate_create_link_rea_commitment_updates(
                action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::ReaProcessToReaIntentInputs => {
                validate_create_link_rea_process_to_rea_intents(
                    action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::ReaProcessToReaIntentOutputs => {
                validate_create_link_rea_process_to_rea_intents(
                    action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::ProviderToReaIntents => validate_create_link_rea_agent_to_rea_intents(
                action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::ReceiverToReaIntents => validate_create_link_rea_agent_to_rea_intents(
                action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::ReaIntentUpdates => {
                validate_create_link_rea_intent_updates(action, base_address, target_address, tag)
            }
            LinkTypes::ReaEconomicResourceToReaEconomicResources => {
                validate_create_link_rea_economic_resource_to_rea_economic_resources(
                    action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::ReaEconomicResourceUpdates => {
                validate_create_link_rea_economic_resource_updates(
                    action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::AllEconomicResources => validate_create_link_all_economic_resources(
                action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::ReaProcessToReaEconomicEventInputs => {
                validate_create_link_rea_process_to_rea_economic_events(
                    action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::ReaProcessToReaEconomicEventOutputs => {
                validate_create_link_rea_process_to_rea_economic_events(
                    action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::ProviderToReaEconomicEvents => {
                validate_create_link_rea_agent_to_rea_economic_events(
                    action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::ReceiverToReaEconomicEvents => {
                validate_create_link_rea_agent_to_rea_economic_events(
                    action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::ReaAgreementToReaEconomicEvents => {
                validate_create_link_rea_agreement_to_rea_economic_events(
                    action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::ReaEconomicEventToReaEconomicEvents => {
                validate_create_link_rea_economic_event_to_rea_economic_events(
                    action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::ReaEconomicEventUpdates => validate_create_link_rea_economic_event_updates(
                action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::AllEconomicEvents => {
                validate_create_link_all_economic_events(action, base_address, target_address, tag)
            }
            LinkTypes::ReaEconomicResourceToReaEconomicEvents => {
                validate_create_link_rea_economic_resource_to_rea_economic_events(
                    action,
                    base_address,
                    target_address,
                    tag,
                )
            }
        },
        FlatOp::RegisterDeleteLink {
            link_type,
            base_address,
            target_address,
            tag,
            original_action,
            action,
        } => match link_type {
            LinkTypes::CommitmentToFulfillingEconomicEvents => {
                validate_delete_link_commitment_to_fulfilling_economic_events(
                    action,
                    original_action,
                    base_address,
                    target_address,
                    tag,
                )
            },
            LinkTypes::IntentToSatisfyingCommitments => {
                validate_delete_link_intent_to_satisfying_commitments(
                    action,
                    original_action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::IntentToSatisfyingEconomicEvents => {
                validate_delete_link_intent_to_satisfying_economic_events(
                    action,
                    original_action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::ReaAgentUpdates => validate_delete_link_rea_agent_updates(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::AllAgents => validate_delete_link_all_agents(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::ReaAgreementUpdates => validate_delete_link_rea_agreement_updates(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::AllAgreements => validate_delete_link_all_agreements(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::ReaProcessSpecificationUpdates => {
                validate_delete_link_rea_process_specification_updates(
                    action,
                    original_action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::AllProcessSpecifications => validate_delete_link_all_process_specifications(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::ReaPlanUpdates => validate_delete_link_rea_plan_updates(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::AllPlans => validate_delete_link_all_plans(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::ReaProcessSpecificationToReaProcesses => {
                validate_delete_link_rea_process_specification_to_rea_processes(
                    action,
                    original_action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::ReaPlanToReaProcesses => validate_delete_link_rea_plan_to_rea_processes(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::ReaProcessUpdates => validate_delete_link_rea_process_updates(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::AllProcesses => validate_delete_link_all_processes(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::ReaUnitUpdates => validate_delete_link_rea_unit_updates(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::AllUnits => validate_delete_link_all_units(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::ReaResourceSpecificationUpdates => {
                validate_delete_link_rea_resource_specification_updates(
                    action,
                    original_action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::AllResourceSpecifications => {
                validate_delete_link_all_resource_specifications(
                    action,
                    original_action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::ReaRecipeProcessUpdates => validate_delete_link_rea_recipe_process_updates(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::AllRecipeProcesses => validate_delete_link_all_recipe_processes(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::ReaRecipeExchangeUpdates => {
                validate_delete_link_rea_recipe_exchange_updates(
                    action,
                    original_action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::AllRecipeExchanges => validate_delete_link_all_recipe_exchanges(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::ReaProposalUpdates => validate_delete_link_rea_proposal_updates(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::AllProposals => validate_delete_link_all_proposals(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::ReaRecipeExchangeToReaRecipeFlows => {
                validate_delete_link_rea_recipe_exchange_to_rea_recipe_flows(
                    action,
                    original_action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::ReaRecipeExchangeToReaRecipeFlowsReciprocal => {
                validate_delete_link_rea_recipe_exchange_to_rea_recipe_flows(
                    action,
                    original_action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::ReaRecipeProcessToReaRecipeFlowInputs => {
                validate_delete_link_rea_recipe_process_to_rea_recipe_flows(
                    action,
                    original_action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::ReaRecipeProcessToReaRecipeFlowOutputs => {
                validate_delete_link_rea_recipe_process_to_rea_recipe_flows(
                    action,
                    original_action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::ReaRecipeFlowUpdates => validate_delete_link_rea_recipe_flow_updates(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::ReaProcessToInputs => validate_delete_link_rea_process_to_rea_commitments(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::ReaProcessToOutputs => validate_delete_link_rea_process_to_rea_commitments(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::ProviderToReaCommitments => {
                validate_delete_link_rea_agent_to_rea_commitments(
                    action,
                    original_action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::ReceiverToReaCommitments => {
                validate_delete_link_rea_agent_to_rea_commitments(
                    action,
                    original_action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::ReaAgreementToReaCommitments => {
                validate_delete_link_rea_agreement_to_rea_commitments(
                    action,
                    original_action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::ReaPlanToReaCommitments => validate_delete_link_rea_plan_to_rea_commitments(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::ReaPlanToIndependentDemands => {
                validate_delete_link_rea_plan_to_rea_commitments(
                    action,
                    original_action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::ReaCommitmentUpdates => validate_delete_link_rea_commitment_updates(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::ReaProcessToReaIntentInputs => {
                validate_delete_link_rea_process_to_rea_intents(
                    action,
                    original_action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::ReaProcessToReaIntentOutputs => {
                validate_delete_link_rea_process_to_rea_intents(
                    action,
                    original_action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::ProviderToReaIntents => validate_delete_link_rea_agent_to_rea_intents(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::ReceiverToReaIntents => validate_delete_link_rea_agent_to_rea_intents(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::ReaIntentUpdates => validate_delete_link_rea_intent_updates(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::ReaEconomicResourceToReaEconomicResources => {
                validate_delete_link_rea_economic_resource_to_rea_economic_resources(
                    action,
                    original_action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::ReaEconomicResourceUpdates => {
                validate_delete_link_rea_economic_resource_updates(
                    action,
                    original_action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::AllEconomicResources => validate_delete_link_all_economic_resources(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::ReaProcessToReaEconomicEventInputs => {
                validate_delete_link_rea_process_to_rea_economic_events(
                    action,
                    original_action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::ReaProcessToReaEconomicEventOutputs => {
                validate_delete_link_rea_process_to_rea_economic_events(
                    action,
                    original_action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::ProviderToReaEconomicEvents => {
                validate_delete_link_rea_agent_to_rea_economic_events(
                    action,
                    original_action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::ReceiverToReaEconomicEvents => {
                validate_delete_link_rea_agent_to_rea_economic_events(
                    action,
                    original_action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::ReaAgreementToReaEconomicEvents => {
                validate_delete_link_rea_agreement_to_rea_economic_events(
                    action,
                    original_action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::ReaEconomicEventToReaEconomicEvents => {
                validate_delete_link_rea_economic_event_to_rea_economic_events(
                    action,
                    original_action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::ReaEconomicEventUpdates => validate_delete_link_rea_economic_event_updates(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::AllEconomicEvents => validate_delete_link_all_economic_events(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::ReaEconomicResourceToReaEconomicEvents => {
                validate_delete_link_rea_economic_resource_to_rea_economic_events(
                    action,
                    original_action,
                    base_address,
                    target_address,
                    tag,
                )
            }
        },
        FlatOp::StoreRecord(store_record) => {
            match store_record {
                // Complementary validation to the `StoreEntry` Op, in which the record itself is validated
                // If you want to optimize performance, you can remove the validation for an entry type here and keep it in `StoreEntry`
                // Notice that doing so will cause `must_get_valid_record` for this record to return a valid record even if the `StoreEntry` validation failed
                OpRecord::CreateEntry { app_entry, action } => match app_entry {
                    EntryTypes::ReaAgent(rea_agent) => {
                        validate_create_rea_agent(EntryCreationAction::Create(action), rea_agent)
                    }
                    EntryTypes::ReaAgreement(rea_agreement) => validate_create_rea_agreement(
                        EntryCreationAction::Create(action),
                        rea_agreement,
                    ),
                    EntryTypes::ReaProcessSpecification(rea_process_specification) => {
                        validate_create_rea_process_specification(
                            EntryCreationAction::Create(action),
                            rea_process_specification,
                        )
                    }
                    EntryTypes::ReaPlan(rea_plan) => {
                        validate_create_rea_plan(EntryCreationAction::Create(action), rea_plan)
                    }
                    EntryTypes::ReaProcess(rea_process) => validate_create_rea_process(
                        EntryCreationAction::Create(action),
                        rea_process,
                    ),
                    EntryTypes::ReaUnit(rea_unit) => {
                        validate_create_rea_unit(EntryCreationAction::Create(action), rea_unit)
                    }
                    EntryTypes::ReaResourceSpecification(rea_resource_specification) => {
                        validate_create_rea_resource_specification(
                            EntryCreationAction::Create(action),
                            rea_resource_specification,
                        )
                    }
                    EntryTypes::ReaRecipeProcess(rea_recipe_process) => {
                        validate_create_rea_recipe_process(
                            EntryCreationAction::Create(action),
                            rea_recipe_process,
                        )
                    }
                    EntryTypes::ReaRecipeExchange(rea_recipe_exchange) => {
                        validate_create_rea_recipe_exchange(
                            EntryCreationAction::Create(action),
                            rea_recipe_exchange,
                        )
                    }
                    EntryTypes::ReaProposal(rea_proposal) => validate_create_rea_proposal(
                        EntryCreationAction::Create(action),
                        rea_proposal,
                    ),
                    EntryTypes::ReaRecipeFlow(rea_recipe_flow) => validate_create_rea_recipe_flow(
                        EntryCreationAction::Create(action),
                        rea_recipe_flow,
                    ),
                    EntryTypes::ReaCommitment(rea_commitment) => validate_create_rea_commitment(
                        EntryCreationAction::Create(action),
                        rea_commitment,
                    ),
                    EntryTypes::ReaIntent(rea_intent) => {
                        validate_create_rea_intent(EntryCreationAction::Create(action), rea_intent)
                    }
                    EntryTypes::ReaEconomicResource(rea_economic_resource) => {
                        validate_create_rea_economic_resource(
                            EntryCreationAction::Create(action),
                            rea_economic_resource,
                        )
                    }
                    EntryTypes::ReaEconomicEvent(rea_economic_event) => {
                        validate_create_rea_economic_event(
                            EntryCreationAction::Create(action),
                            rea_economic_event,
                        )
                    }
                },
                // Complementary validation to the `RegisterUpdate` Op, in which the record itself is validated
                // If you want to optimize performance, you can remove the validation for an entry type here and keep it in `StoreEntry` and in `RegisterUpdate`
                // Notice that doing so will cause `must_get_valid_record` for this record to return a valid record even if the other validations failed
                OpRecord::UpdateEntry {
                    original_action_hash,
                    app_entry,
                    action,
                    ..
                } => {
                    let original_record = must_get_valid_record(original_action_hash)?;
                    let original_action = original_record.action().clone();
                    let original_action = match original_action {
                        Action::Create(create) => EntryCreationAction::Create(create),
                        Action::Update(update) => EntryCreationAction::Update(update),
                        _ => {
                            return Ok(ValidateCallbackResult::Invalid(
                                "Original action for an update must be a Create or Update action"
                                    .to_string(),
                            ));
                        }
                    };
                    match app_entry {
                        EntryTypes::ReaAgent(rea_agent) => {
                            let result = validate_create_rea_agent(
                                EntryCreationAction::Update(action.clone()),
                                rea_agent.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_rea_agent: Option<ReaAgent> = original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                                let original_rea_agent = match original_rea_agent {
                                    Some(rea_agent) => rea_agent,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_rea_agent(
                                    action,
                                    rea_agent,
                                    original_action,
                                    original_rea_agent,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                        EntryTypes::ReaAgreement(rea_agreement) => {
                            let result = validate_create_rea_agreement(
                                EntryCreationAction::Update(action.clone()),
                                rea_agreement.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_rea_agreement: Option<ReaAgreement> = original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                                let original_rea_agreement = match original_rea_agreement {
                                    Some(rea_agreement) => rea_agreement,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_rea_agreement(
                                    action,
                                    rea_agreement,
                                    original_action,
                                    original_rea_agreement,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                        EntryTypes::ReaProcessSpecification(rea_process_specification) => {
                            let result = validate_create_rea_process_specification(
                                EntryCreationAction::Update(action.clone()),
                                rea_process_specification.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_rea_process_specification: Option<
                                    ReaProcessSpecification,
                                > = original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                                let original_rea_process_specification =
                                    match original_rea_process_specification {
                                        Some(rea_process_specification) => {
                                            rea_process_specification
                                        }
                                        None => {
                                            return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                        }
                                    };
                                validate_update_rea_process_specification(
                                    action,
                                    rea_process_specification,
                                    original_action,
                                    original_rea_process_specification,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                        EntryTypes::ReaPlan(rea_plan) => {
                            let result = validate_create_rea_plan(
                                EntryCreationAction::Update(action.clone()),
                                rea_plan.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_rea_plan: Option<ReaPlan> = original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                                let original_rea_plan = match original_rea_plan {
                                    Some(rea_plan) => rea_plan,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_rea_plan(
                                    action,
                                    rea_plan,
                                    original_action,
                                    original_rea_plan,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                        EntryTypes::ReaProcess(rea_process) => {
                            let result = validate_create_rea_process(
                                EntryCreationAction::Update(action.clone()),
                                rea_process.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_rea_process: Option<ReaProcess> = original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                                let original_rea_process = match original_rea_process {
                                    Some(rea_process) => rea_process,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_rea_process(
                                    action,
                                    rea_process,
                                    original_action,
                                    original_rea_process,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                        EntryTypes::ReaUnit(rea_unit) => {
                            let result = validate_create_rea_unit(
                                EntryCreationAction::Update(action.clone()),
                                rea_unit.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_rea_unit: Option<ReaUnit> = original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                                let original_rea_unit = match original_rea_unit {
                                    Some(rea_unit) => rea_unit,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_rea_unit(
                                    action,
                                    rea_unit,
                                    original_action,
                                    original_rea_unit,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                        EntryTypes::ReaResourceSpecification(rea_resource_specification) => {
                            let result = validate_create_rea_resource_specification(
                                EntryCreationAction::Update(action.clone()),
                                rea_resource_specification.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_rea_resource_specification: Option<
                                    ReaResourceSpecification,
                                > = original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                                let original_rea_resource_specification =
                                    match original_rea_resource_specification {
                                        Some(rea_resource_specification) => {
                                            rea_resource_specification
                                        }
                                        None => {
                                            return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                        }
                                    };
                                validate_update_rea_resource_specification(
                                    action,
                                    rea_resource_specification,
                                    original_action,
                                    original_rea_resource_specification,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                        EntryTypes::ReaRecipeProcess(rea_recipe_process) => {
                            let result = validate_create_rea_recipe_process(
                                EntryCreationAction::Update(action.clone()),
                                rea_recipe_process.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_rea_recipe_process: Option<ReaRecipeProcess> =
                                    original_record
                                        .entry()
                                        .to_app_option()
                                        .map_err(|e| wasm_error!(e))?;
                                let original_rea_recipe_process = match original_rea_recipe_process
                                {
                                    Some(rea_recipe_process) => rea_recipe_process,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_rea_recipe_process(
                                    action,
                                    rea_recipe_process,
                                    original_action,
                                    original_rea_recipe_process,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                        EntryTypes::ReaRecipeExchange(rea_recipe_exchange) => {
                            let result = validate_create_rea_recipe_exchange(
                                EntryCreationAction::Update(action.clone()),
                                rea_recipe_exchange.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_rea_recipe_exchange: Option<ReaRecipeExchange> =
                                    original_record
                                        .entry()
                                        .to_app_option()
                                        .map_err(|e| wasm_error!(e))?;
                                let original_rea_recipe_exchange =
                                    match original_rea_recipe_exchange {
                                        Some(rea_recipe_exchange) => rea_recipe_exchange,
                                        None => {
                                            return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                        }
                                    };
                                validate_update_rea_recipe_exchange(
                                    action,
                                    rea_recipe_exchange,
                                    original_action,
                                    original_rea_recipe_exchange,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                        EntryTypes::ReaProposal(rea_proposal) => {
                            let result = validate_create_rea_proposal(
                                EntryCreationAction::Update(action.clone()),
                                rea_proposal.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_rea_proposal: Option<ReaProposal> = original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                                let original_rea_proposal = match original_rea_proposal {
                                    Some(rea_proposal) => rea_proposal,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_rea_proposal(
                                    action,
                                    rea_proposal,
                                    original_action,
                                    original_rea_proposal,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                        EntryTypes::ReaRecipeFlow(rea_recipe_flow) => {
                            let result = validate_create_rea_recipe_flow(
                                EntryCreationAction::Update(action.clone()),
                                rea_recipe_flow.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_rea_recipe_flow: Option<ReaRecipeFlow> =
                                    original_record
                                        .entry()
                                        .to_app_option()
                                        .map_err(|e| wasm_error!(e))?;
                                let original_rea_recipe_flow = match original_rea_recipe_flow {
                                    Some(rea_recipe_flow) => rea_recipe_flow,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_rea_recipe_flow(
                                    action,
                                    rea_recipe_flow,
                                    original_action,
                                    original_rea_recipe_flow,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                        EntryTypes::ReaCommitment(rea_commitment) => {
                            let result = validate_create_rea_commitment(
                                EntryCreationAction::Update(action.clone()),
                                rea_commitment.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_rea_commitment: Option<ReaCommitment> =
                                    original_record
                                        .entry()
                                        .to_app_option()
                                        .map_err(|e| wasm_error!(e))?;
                                let original_rea_commitment = match original_rea_commitment {
                                    Some(rea_commitment) => rea_commitment,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_rea_commitment(
                                    action,
                                    rea_commitment,
                                    original_action,
                                    original_rea_commitment,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                        EntryTypes::ReaIntent(rea_intent) => {
                            let result = validate_create_rea_intent(
                                EntryCreationAction::Update(action.clone()),
                                rea_intent.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_rea_intent: Option<ReaIntent> = original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                                let original_rea_intent = match original_rea_intent {
                                    Some(rea_intent) => rea_intent,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_rea_intent(
                                    action,
                                    rea_intent,
                                    original_action,
                                    original_rea_intent,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                        EntryTypes::ReaEconomicResource(rea_economic_resource) => {
                            let result = validate_create_rea_economic_resource(
                                EntryCreationAction::Update(action.clone()),
                                rea_economic_resource.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_rea_economic_resource: Option<ReaEconomicResource> =
                                    original_record
                                        .entry()
                                        .to_app_option()
                                        .map_err(|e| wasm_error!(e))?;
                                let original_rea_economic_resource =
                                    match original_rea_economic_resource {
                                        Some(rea_economic_resource) => rea_economic_resource,
                                        None => {
                                            return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                        }
                                    };
                                validate_update_rea_economic_resource(
                                    action,
                                    rea_economic_resource,
                                    original_action,
                                    original_rea_economic_resource,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                        EntryTypes::ReaEconomicEvent(rea_economic_event) => {
                            let result = validate_create_rea_economic_event(
                                EntryCreationAction::Update(action.clone()),
                                rea_economic_event.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_rea_economic_event: Option<ReaEconomicEvent> =
                                    original_record
                                        .entry()
                                        .to_app_option()
                                        .map_err(|e| wasm_error!(e))?;
                                let original_rea_economic_event = match original_rea_economic_event
                                {
                                    Some(rea_economic_event) => rea_economic_event,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_rea_economic_event(
                                    action,
                                    rea_economic_event,
                                    original_action,
                                    original_rea_economic_event,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                    }
                }
                // Complementary validation to the `RegisterDelete` Op, in which the record itself is validated
                // If you want to optimize performance, you can remove the validation for an entry type here and keep it in `RegisterDelete`
                // Notice that doing so will cause `must_get_valid_record` for this record to return a valid record even if the `RegisterDelete` validation failed
                OpRecord::DeleteEntry {
                    original_action_hash,
                    action,
                    ..
                } => {
                    let original_record = must_get_valid_record(original_action_hash)?;
                    let original_action = original_record.action().clone();
                    let original_action = match original_action {
                        Action::Create(create) => EntryCreationAction::Create(create),
                        Action::Update(update) => EntryCreationAction::Update(update),
                        _ => {
                            return Ok(ValidateCallbackResult::Invalid(
                                "Original action for a delete must be a Create or Update action"
                                    .to_string(),
                            ));
                        }
                    };
                    let app_entry_type = match original_action.entry_type() {
                        EntryType::App(app_entry_type) => app_entry_type,
                        _ => {
                            return Ok(ValidateCallbackResult::Valid);
                        }
                    };
                    let entry = match original_record.entry().as_option() {
                        Some(entry) => entry,
                        None => {
                            return Ok(ValidateCallbackResult::Invalid(
                                "Original record for a delete must contain an entry".to_string(),
                            ));
                        }
                    };
                    let original_app_entry = match EntryTypes::deserialize_from_type(
                        app_entry_type.zome_index,
                        app_entry_type.entry_index,
                        entry,
                    )? {
                        Some(app_entry) => app_entry,
                        None => {
                            return Ok(
                                ValidateCallbackResult::Invalid(
                                    "Original app entry must be one of the defined entry types for this zome"
                                        .to_string(),
                                ),
                            );
                        }
                    };
                    match original_app_entry {
                        EntryTypes::ReaAgent(original_rea_agent) => {
                            validate_delete_rea_agent(action, original_action, original_rea_agent)
                        }
                        EntryTypes::ReaAgreement(original_rea_agreement) => {
                            validate_delete_rea_agreement(
                                action,
                                original_action,
                                original_rea_agreement,
                            )
                        }
                        EntryTypes::ReaProcessSpecification(original_rea_process_specification) => {
                            validate_delete_rea_process_specification(
                                action,
                                original_action,
                                original_rea_process_specification,
                            )
                        }
                        EntryTypes::ReaPlan(original_rea_plan) => {
                            validate_delete_rea_plan(action, original_action, original_rea_plan)
                        }
                        EntryTypes::ReaProcess(original_rea_process) => {
                            validate_delete_rea_process(
                                action,
                                original_action,
                                original_rea_process,
                            )
                        }
                        EntryTypes::ReaUnit(original_rea_unit) => {
                            validate_delete_rea_unit(action, original_action, original_rea_unit)
                        }
                        EntryTypes::ReaResourceSpecification(
                            original_rea_resource_specification,
                        ) => validate_delete_rea_resource_specification(
                            action,
                            original_action,
                            original_rea_resource_specification,
                        ),
                        EntryTypes::ReaRecipeProcess(original_rea_recipe_process) => {
                            validate_delete_rea_recipe_process(
                                action,
                                original_action,
                                original_rea_recipe_process,
                            )
                        }
                        EntryTypes::ReaRecipeExchange(original_rea_recipe_exchange) => {
                            validate_delete_rea_recipe_exchange(
                                action,
                                original_action,
                                original_rea_recipe_exchange,
                            )
                        }
                        EntryTypes::ReaProposal(original_rea_proposal) => {
                            validate_delete_rea_proposal(
                                action,
                                original_action,
                                original_rea_proposal,
                            )
                        }
                        EntryTypes::ReaRecipeFlow(original_rea_recipe_flow) => {
                            validate_delete_rea_recipe_flow(
                                action,
                                original_action,
                                original_rea_recipe_flow,
                            )
                        }
                        EntryTypes::ReaCommitment(original_rea_commitment) => {
                            validate_delete_rea_commitment(
                                action,
                                original_action,
                                original_rea_commitment,
                            )
                        }
                        EntryTypes::ReaIntent(original_rea_intent) => {
                            validate_delete_rea_intent(action, original_action, original_rea_intent)
                        }
                        EntryTypes::ReaEconomicResource(original_rea_economic_resource) => {
                            validate_delete_rea_economic_resource(
                                action,
                                original_action,
                                original_rea_economic_resource,
                            )
                        }
                        EntryTypes::ReaEconomicEvent(original_rea_economic_event) => {
                            validate_delete_rea_economic_event(
                                action,
                                original_action,
                                original_rea_economic_event,
                            )
                        }
                    }
                }
                // Complementary validation to the `RegisterCreateLink` Op, in which the record itself is validated
                // If you want to optimize performance, you can remove the validation for an entry type here and keep it in `RegisterCreateLink`
                // Notice that doing so will cause `must_get_valid_record` for this record to return a valid record even if the `RegisterCreateLink` validation failed
                OpRecord::CreateLink {
                    base_address,
                    target_address,
                    tag,
                    link_type,
                    action,
                } => match link_type {
                    LinkTypes::CommitmentToFulfillingEconomicEvents => {
                        validate_create_link_commitment_to_fulfilling_economic_events(
                            action,
                            base_address,
                            target_address,
                            tag,
                        )
                    },
                    LinkTypes::IntentToSatisfyingCommitments => {
                        validate_create_link_intent_to_satisfying_commitments(
                            action,
                            base_address,
                            target_address,
                            tag,
                        )
                    },
                    LinkTypes::IntentToSatisfyingEconomicEvents => {
                        validate_create_link_intent_to_satisfying_economic_events(
                            action,
                            base_address,
                            target_address,
                            tag,
                        )
                    },
                    LinkTypes::ReaAgentUpdates => validate_create_link_rea_agent_updates(
                        action,
                        base_address,
                        target_address,
                        tag,
                    ),
                    LinkTypes::AllAgents => validate_create_link_all_agents(
                        action,
                        base_address,
                        target_address,
                        tag,
                    ),
                    LinkTypes::ReaAgreementUpdates => validate_create_link_rea_agreement_updates(
                        action,
                        base_address,
                        target_address,
                        tag,
                    ),
                    LinkTypes::AllAgreements => validate_create_link_all_agreements(
                        action,
                        base_address,
                        target_address,
                        tag,
                    ),
                    LinkTypes::ReaProcessSpecificationUpdates => {
                        validate_create_link_rea_process_specification_updates(
                            action,
                            base_address,
                            target_address,
                            tag,
                        )
                    }
                    LinkTypes::AllProcessSpecifications => {
                        validate_create_link_all_process_specifications(
                            action,
                            base_address,
                            target_address,
                            tag,
                        )
                    }
                    LinkTypes::ReaPlanUpdates => validate_create_link_rea_plan_updates(
                        action,
                        base_address,
                        target_address,
                        tag,
                    ),
                    LinkTypes::AllPlans => {
                        validate_create_link_all_plans(action, base_address, target_address, tag)
                    }
                    LinkTypes::ReaProcessSpecificationToReaProcesses => {
                        validate_create_link_rea_process_specification_to_rea_processes(
                            action,
                            base_address,
                            target_address,
                            tag,
                        )
                    }
                    LinkTypes::ReaPlanToReaProcesses => {
                        validate_create_link_rea_plan_to_rea_processes(
                            action,
                            base_address,
                            target_address,
                            tag,
                        )
                    }
                    LinkTypes::ReaProcessUpdates => validate_create_link_rea_process_updates(
                        action,
                        base_address,
                        target_address,
                        tag,
                    ),
                    LinkTypes::AllProcesses => validate_create_link_all_processes(
                        action,
                        base_address,
                        target_address,
                        tag,
                    ),
                    LinkTypes::ReaUnitUpdates => validate_create_link_rea_unit_updates(
                        action,
                        base_address,
                        target_address,
                        tag,
                    ),
                    LinkTypes::AllUnits => {
                        validate_create_link_all_units(action, base_address, target_address, tag)
                    }
                    LinkTypes::ReaResourceSpecificationUpdates => {
                        validate_create_link_rea_resource_specification_updates(
                            action,
                            base_address,
                            target_address,
                            tag,
                        )
                    }
                    LinkTypes::AllResourceSpecifications => {
                        validate_create_link_all_resource_specifications(
                            action,
                            base_address,
                            target_address,
                            tag,
                        )
                    }
                    LinkTypes::ReaRecipeProcessUpdates => {
                        validate_create_link_rea_recipe_process_updates(
                            action,
                            base_address,
                            target_address,
                            tag,
                        )
                    }
                    LinkTypes::AllRecipeProcesses => validate_create_link_all_recipe_processes(
                        action,
                        base_address,
                        target_address,
                        tag,
                    ),
                    LinkTypes::ReaRecipeExchangeUpdates => {
                        validate_create_link_rea_recipe_exchange_updates(
                            action,
                            base_address,
                            target_address,
                            tag,
                        )
                    }
                    LinkTypes::AllRecipeExchanges => validate_create_link_all_recipe_exchanges(
                        action,
                        base_address,
                        target_address,
                        tag,
                    ),
                    LinkTypes::ReaProposalUpdates => validate_create_link_rea_proposal_updates(
                        action,
                        base_address,
                        target_address,
                        tag,
                    ),
                    LinkTypes::AllProposals => validate_create_link_all_proposals(
                        action,
                        base_address,
                        target_address,
                        tag,
                    ),
                    LinkTypes::ReaRecipeExchangeToReaRecipeFlows => {
                        validate_create_link_rea_recipe_exchange_to_rea_recipe_flows(
                            action,
                            base_address,
                            target_address,
                            tag,
                        )
                    }
                    LinkTypes::ReaRecipeExchangeToReaRecipeFlowsReciprocal => {
                        validate_create_link_rea_recipe_exchange_to_rea_recipe_flows(
                            action,
                            base_address,
                            target_address,
                            tag,
                        )
                    }
                    LinkTypes::ReaRecipeProcessToReaRecipeFlowInputs => {
                        validate_create_link_rea_recipe_process_to_rea_recipe_flows(
                            action,
                            base_address,
                            target_address,
                            tag,
                        )
                    }
                    LinkTypes::ReaRecipeProcessToReaRecipeFlowOutputs => {
                        validate_create_link_rea_recipe_process_to_rea_recipe_flows(
                            action,
                            base_address,
                            target_address,
                            tag,
                        )
                    }
                    LinkTypes::ReaRecipeFlowUpdates => {
                        validate_create_link_rea_recipe_flow_updates(
                            action,
                            base_address,
                            target_address,
                            tag,
                        )
                    }
                    LinkTypes::ReaProcessToInputs => {
                        validate_create_link_rea_process_to_rea_commitments(
                            action,
                            base_address,
                            target_address,
                            tag,
                        )
                    }
                    LinkTypes::ReaProcessToOutputs => {
                        validate_create_link_rea_process_to_rea_commitments(
                            action,
                            base_address,
                            target_address,
                            tag,
                        )
                    }
                    LinkTypes::ProviderToReaCommitments => {
                        validate_create_link_rea_agent_to_rea_commitments(
                            action,
                            base_address,
                            target_address,
                            tag,
                        )
                    }
                    LinkTypes::ReceiverToReaCommitments => {
                        validate_create_link_rea_agent_to_rea_commitments(
                            action,
                            base_address,
                            target_address,
                            tag,
                        )
                    }
                    LinkTypes::ReaAgreementToReaCommitments => {
                        validate_create_link_rea_agreement_to_rea_commitments(
                            action,
                            base_address,
                            target_address,
                            tag,
                        )
                    }
                    LinkTypes::ReaPlanToReaCommitments => {
                        validate_create_link_rea_plan_to_rea_commitments(
                            action,
                            base_address,
                            target_address,
                            tag,
                        )
                    }
                    LinkTypes::ReaPlanToIndependentDemands => {
                        validate_create_link_rea_plan_to_rea_commitments(
                            action,
                            base_address,
                            target_address,
                            tag,
                        )
                    }
                    LinkTypes::ReaCommitmentUpdates => validate_create_link_rea_commitment_updates(
                        action,
                        base_address,
                        target_address,
                        tag,
                    ),
                    LinkTypes::ReaProcessToReaIntentOutputs => {
                        validate_create_link_rea_process_to_rea_intents(
                            action,
                            base_address,
                            target_address,
                            tag,
                        )
                    }
                    LinkTypes::ReaProcessToReaIntentInputs => {
                        validate_create_link_rea_process_to_rea_intents(
                            action,
                            base_address,
                            target_address,
                            tag,
                        )
                    }
                    LinkTypes::ProviderToReaIntents => {
                        validate_create_link_rea_agent_to_rea_intents(
                            action,
                            base_address,
                            target_address,
                            tag,
                        )
                    }
                    LinkTypes::ReceiverToReaIntents => {
                        validate_create_link_rea_agent_to_rea_intents(
                            action,
                            base_address,
                            target_address,
                            tag,
                        )
                    }
                    LinkTypes::ReaIntentUpdates => validate_create_link_rea_intent_updates(
                        action,
                        base_address,
                        target_address,
                        tag,
                    ),
                    LinkTypes::ReaEconomicResourceToReaEconomicResources => {
                        validate_create_link_rea_economic_resource_to_rea_economic_resources(
                            action,
                            base_address,
                            target_address,
                            tag,
                        )
                    }
                    LinkTypes::ReaEconomicResourceUpdates => {
                        validate_create_link_rea_economic_resource_updates(
                            action,
                            base_address,
                            target_address,
                            tag,
                        )
                    }
                    LinkTypes::AllEconomicResources => validate_create_link_all_economic_resources(
                        action,
                        base_address,
                        target_address,
                        tag,
                    ),
                    LinkTypes::ReaProcessToReaEconomicEventInputs => {
                        validate_create_link_rea_process_to_rea_economic_events(
                            action,
                            base_address,
                            target_address,
                            tag,
                        )
                    }
                    LinkTypes::ReaProcessToReaEconomicEventOutputs => {
                        validate_create_link_rea_process_to_rea_economic_events(
                            action,
                            base_address,
                            target_address,
                            tag,
                        )
                    }
                    LinkTypes::ProviderToReaEconomicEvents => {
                        validate_create_link_rea_agent_to_rea_economic_events(
                            action,
                            base_address,
                            target_address,
                            tag,
                        )
                    }
                    LinkTypes::ReceiverToReaEconomicEvents => {
                        validate_create_link_rea_agent_to_rea_economic_events(
                            action,
                            base_address,
                            target_address,
                            tag,
                        )
                    }
                    LinkTypes::ReaAgreementToReaEconomicEvents => {
                        validate_create_link_rea_agreement_to_rea_economic_events(
                            action,
                            base_address,
                            target_address,
                            tag,
                        )
                    }
                    LinkTypes::ReaEconomicEventToReaEconomicEvents => {
                        validate_create_link_rea_economic_event_to_rea_economic_events(
                            action,
                            base_address,
                            target_address,
                            tag,
                        )
                    }
                    LinkTypes::ReaEconomicEventUpdates => {
                        validate_create_link_rea_economic_event_updates(
                            action,
                            base_address,
                            target_address,
                            tag,
                        )
                    }
                    LinkTypes::AllEconomicEvents => validate_create_link_all_economic_events(
                        action,
                        base_address,
                        target_address,
                        tag,
                    ),
                    LinkTypes::ReaEconomicResourceToReaEconomicEvents => {
                        validate_create_link_rea_economic_resource_to_rea_economic_events(
                            action,
                            base_address,
                            target_address,
                            tag,
                        )
                    }
                },
                // Complementary validation to the `RegisterDeleteLink` Op, in which the record itself is validated
                // If you want to optimize performance, you can remove the validation for an entry type here and keep it in `RegisterDeleteLink`
                // Notice that doing so will cause `must_get_valid_record` for this record to return a valid record even if the `RegisterDeleteLink` validation failed
                OpRecord::DeleteLink {
                    original_action_hash,
                    base_address,
                    action,
                } => {
                    let record = must_get_valid_record(original_action_hash)?;
                    let create_link = match record.action() {
                        Action::CreateLink(create_link) => create_link.clone(),
                        _ => {
                            return Ok(ValidateCallbackResult::Invalid(
                                "The action that a DeleteLink deletes must be a CreateLink"
                                    .to_string(),
                            ));
                        }
                    };
                    let link_type = match LinkTypes::from_type(
                        create_link.zome_index,
                        create_link.link_type,
                    )? {
                        Some(lt) => lt,
                        None => {
                            return Ok(ValidateCallbackResult::Valid);
                        }
                    };
                    match link_type {
                        LinkTypes::CommitmentToFulfillingEconomicEvents => {
                            validate_delete_link_commitment_to_fulfilling_economic_events(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::IntentToSatisfyingCommitments => {
                            validate_delete_link_intent_to_satisfying_commitments(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::IntentToSatisfyingEconomicEvents => {
                            validate_delete_link_intent_to_satisfying_economic_events(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::ReaAgentUpdates => validate_delete_link_rea_agent_updates(
                            action,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        ),
                        LinkTypes::AllAgents => validate_delete_link_all_agents(
                            action,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        ),
                        LinkTypes::ReaAgreementUpdates => {
                            validate_delete_link_rea_agreement_updates(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::AllAgreements => validate_delete_link_all_agreements(
                            action,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        ),
                        LinkTypes::ReaProcessSpecificationUpdates => {
                            validate_delete_link_rea_process_specification_updates(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::AllProcessSpecifications => {
                            validate_delete_link_all_process_specifications(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::ReaPlanUpdates => validate_delete_link_rea_plan_updates(
                            action,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        ),
                        LinkTypes::AllPlans => validate_delete_link_all_plans(
                            action,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        ),
                        LinkTypes::ReaProcessSpecificationToReaProcesses => {
                            validate_delete_link_rea_process_specification_to_rea_processes(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::ReaPlanToReaProcesses => {
                            validate_delete_link_rea_plan_to_rea_processes(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::ReaProcessUpdates => validate_delete_link_rea_process_updates(
                            action,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        ),
                        LinkTypes::AllProcesses => validate_delete_link_all_processes(
                            action,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        ),
                        LinkTypes::ReaUnitUpdates => validate_delete_link_rea_unit_updates(
                            action,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        ),
                        LinkTypes::AllUnits => validate_delete_link_all_units(
                            action,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        ),
                        LinkTypes::ReaResourceSpecificationUpdates => {
                            validate_delete_link_rea_resource_specification_updates(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::AllResourceSpecifications => {
                            validate_delete_link_all_resource_specifications(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::ReaRecipeProcessUpdates => {
                            validate_delete_link_rea_recipe_process_updates(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::AllRecipeProcesses => validate_delete_link_all_recipe_processes(
                            action,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        ),
                        LinkTypes::ReaRecipeExchangeUpdates => {
                            validate_delete_link_rea_recipe_exchange_updates(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::AllRecipeExchanges => validate_delete_link_all_recipe_exchanges(
                            action,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        ),
                        LinkTypes::ReaProposalUpdates => validate_delete_link_rea_proposal_updates(
                            action,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        ),
                        LinkTypes::AllProposals => validate_delete_link_all_proposals(
                            action,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        ),
                        LinkTypes::ReaRecipeExchangeToReaRecipeFlows => {
                            validate_delete_link_rea_recipe_exchange_to_rea_recipe_flows(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::ReaRecipeExchangeToReaRecipeFlowsReciprocal => {
                            validate_delete_link_rea_recipe_exchange_to_rea_recipe_flows(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::ReaRecipeProcessToReaRecipeFlowInputs => {
                            validate_delete_link_rea_recipe_process_to_rea_recipe_flows(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::ReaRecipeProcessToReaRecipeFlowOutputs => {
                            validate_delete_link_rea_recipe_process_to_rea_recipe_flows(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::ReaRecipeFlowUpdates => {
                            validate_delete_link_rea_recipe_flow_updates(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::ReaProcessToInputs => {
                            validate_delete_link_rea_process_to_rea_commitments(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::ReaProcessToOutputs => {
                            validate_delete_link_rea_process_to_rea_commitments(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::ProviderToReaCommitments => {
                            validate_delete_link_rea_agent_to_rea_commitments(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::ReceiverToReaCommitments => {
                            validate_delete_link_rea_agent_to_rea_commitments(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::ReaAgreementToReaCommitments => {
                            validate_delete_link_rea_agreement_to_rea_commitments(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::ReaPlanToReaCommitments => {
                            validate_delete_link_rea_plan_to_rea_commitments(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::ReaPlanToIndependentDemands => {
                            validate_delete_link_rea_plan_to_rea_commitments(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::ReaCommitmentUpdates => {
                            validate_delete_link_rea_commitment_updates(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::ReaProcessToReaIntentInputs => {
                            validate_delete_link_rea_process_to_rea_intents(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::ReaProcessToReaIntentOutputs => {
                            validate_delete_link_rea_process_to_rea_intents(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::ProviderToReaIntents => {
                            validate_delete_link_rea_agent_to_rea_intents(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::ReceiverToReaIntents => {
                            validate_delete_link_rea_agent_to_rea_intents(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::ReaIntentUpdates => validate_delete_link_rea_intent_updates(
                            action,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        ),
                        LinkTypes::ReaEconomicResourceToReaEconomicResources => {
                            validate_delete_link_rea_economic_resource_to_rea_economic_resources(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::ReaEconomicResourceUpdates => {
                            validate_delete_link_rea_economic_resource_updates(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::AllEconomicResources => {
                            validate_delete_link_all_economic_resources(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::ReaProcessToReaEconomicEventInputs => {
                            validate_delete_link_rea_process_to_rea_economic_events(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::ReaProcessToReaEconomicEventOutputs => {
                            validate_delete_link_rea_process_to_rea_economic_events(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::ProviderToReaEconomicEvents => {
                            validate_delete_link_rea_agent_to_rea_economic_events(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::ReceiverToReaEconomicEvents => {
                            validate_delete_link_rea_agent_to_rea_economic_events(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::ReaAgreementToReaEconomicEvents => {
                            validate_delete_link_rea_agreement_to_rea_economic_events(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::ReaEconomicEventToReaEconomicEvents => {
                            validate_delete_link_rea_economic_event_to_rea_economic_events(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::ReaEconomicEventUpdates => {
                            validate_delete_link_rea_economic_event_updates(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::AllEconomicEvents => validate_delete_link_all_economic_events(
                            action,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        ),
                        LinkTypes::ReaEconomicResourceToReaEconomicEvents => {
                            validate_delete_link_rea_economic_resource_to_rea_economic_events(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                    }
                }
                OpRecord::CreatePrivateEntry { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::UpdatePrivateEntry { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::CreateCapClaim { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::CreateCapGrant { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::UpdateCapClaim { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::UpdateCapGrant { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::Dna { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::OpenChain { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::CloseChain { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::InitZomesComplete { .. } => Ok(ValidateCallbackResult::Valid),
                _ => Ok(ValidateCallbackResult::Valid),
            }
        }
        FlatOp::RegisterAgentActivity(agent_activity) => match agent_activity {
            OpActivity::CreateAgent { agent, action } => {
                let previous_action = must_get_action(action.prev_action)?;
                match previous_action.action() {
                        Action::AgentValidationPkg(
                            AgentValidationPkg { membrane_proof, .. },
                        ) => validate_agent_joining(agent, membrane_proof),
                        _ => {
                            Ok(
                                ValidateCallbackResult::Invalid(
                                    "The previous action for a `CreateAgent` action must be an `AgentValidationPkg`"
                                        .to_string(),
                                ),
                            )
                        }
                    }
            }
            _ => Ok(ValidateCallbackResult::Valid),
        },
    }
}
