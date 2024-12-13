/**
 * Intent query indexes for planning DNA
 *
 * @package hREA
 * @since   2021-08-29
 */

// ACTION

use hdk_semantic_indexes_zome_derive::index_zome;

use hdk::prelude::*;

use vf_attributes_hdk::{
    ActionId,
};
use vf_actions::{
    Action,
    get_builtin_action,
    get_all_builtin_actions,
};

#[derive(Debug, Serialize, Deserialize)]
struct ById {
    id: ActionId,
}

#[hdk_extern]
fn get_action(ById { id }: ById) -> ExternResult<Action> {
    match get_builtin_action(id.as_ref()) {
        Some(action) => Ok(action),
        None => Err(wasm_error!(WasmErrorInner::Guest(format!("No action with ID '{}' available", id.as_ref())))),
    }
}

#[hdk_extern]
fn get_all_actions(_: ()) -> ExternResult<Vec<Action>> {
    Ok(get_all_builtin_actions())
}

// AGENT

mod agent {
    use super::*;
    use hc_zome_rea_agent_rpc::*;

    #[index_zome]
    pub struct Agent {
        commitments_as_provider: Local<commitment, provider>,
        commitments_as_receiver: Local<commitment, receiver>,
        intents_as_provider: Local<intent, provider>,
        intents_as_receiver: Local<intent, receiver>,
        economic_events_as_provider: Local<economic_event, provider>,
        economic_events_as_receiver: Local<economic_event, receiver>,
        inventoried_economic_resources: Local<economic_resource, primary_accountable>,

        // query agents by type
        agent_type: Local<agent, agent_type_internal>::String,
        // :SHONK: redundant loopback index, required for internals of bidirectional index link management.
        // Aside from better support for such edge-cases, the other benefit to obviating this workaround is DHT bloat.
        agent_type_internal: Local<agent, agent_type>,
    }
}


// AGREEMENT


mod agreement {
    use super::*;
    use hc_zome_rea_agreement_rpc::*;
    #[index_zome]
    pub struct Agreement {
        economic_events: Local<economic_event, realization_of>,
        commitments: Local<commitment, clause_of>,
    }
}

// COMMITMENT


mod commitment {
    use super::*;
    use hc_zome_rea_commitment_rpc::*;
    #[index_zome]
    pub struct Commitment {
        fulfilled_by: Local<fulfillment, fulfills>,
        satisfies: Local<satisfaction, satisfied_by>,
        input_of: Local<process, committed_inputs>,
        output_of: Local<process, committed_outputs>,
        clause_of: Local<agreement, commitments>,

        // internal indexes (not part of VF spec)
        provider: Local<agent, commitments_as_provider>,
        receiver: Local<agent, commitments_as_receiver>,
        independent_demand_of: Local<plan, independent_demands>,
        planned_within: Local<plan, non_process_commitments>,
        // in_scope_of: Local<agent, commitments>,
    }
}

// ECONOMIC EVENT


mod economic_event {
    use super::*;
    use hc_zome_rea_economic_event_rpc::*;
    #[index_zome]
    pub struct EconomicEvent {
        input_of: Local<process, observed_inputs>,
        output_of: Local<process, observed_outputs>,
        realization_of: Local<agreement, economic_events>,
        satisfies: Local<satisfaction, satisfied_by>,
        fulfills: Local<fulfillment, fulfilled_by>,

        // internal indexes (not part of REA spec)
        affects: Local<economic_resource, affected_by>,
        provider: Local<agent, economic_events_as_provider>,
        receiver: Local<agent, economic_events_as_receiver>,
    }
}

// ECONOMIC RESOURCE


mod economic_resource {
    use super::*;
    use hc_zome_rea_economic_resource_rpc::*;
    use hc_zome_rea_economic_event_rpc::{
        ResourceResponse as Response,
        ResourceResponseData as ResponseData,
    };
    #[index_zome]
    pub struct EconomicResource {
        contains: Local<economic_resource, contained_in>,
        contained_in: Local<economic_resource, contains>,
        conforms_to: Local<resource_specification, conforming_resources>,

        // internal indexes (not part of REA spec)
        affected_by: Local<economic_event, affects>,
        primary_accountable: Local<agent, inventoried_economic_events>,
    }
}

// FULFILLMENT

use hdk_semantic_indexes_zome_lib::ByAddress; // disambiguate from RPC query struct

mod fulfillment {
    use super::*;
    use hc_zome_rea_fulfillment_rpc::*;
    #[index_zome]
    pub struct Fulfillment {
        fulfilled_by: Local<economic_event, fulfills>,
        fulfills: Local<commitment, fulfilled_by>,
    }
}

// INTENT


mod intent {
    use super::*;
    use hc_zome_rea_intent_rpc::*;
    #[index_zome]
    pub struct Intent {
        satisfied_by: Local<satisfaction, satisfies>,
        input_of: Local<process, intended_inputs>,
        output_of: Local<process, intended_outputs>,
        proposed_in: Local<proposed_intent, publishes>,

        // internal indexes (not part of VF spec)
        provider: Local<agent, intents_as_provider>,
        receiver: Local<agent, intents_as_receiver>,
    }
}

// PLAN

mod plan {
    use super::*;
    use hc_zome_rea_plan_rpc::*;
    #[index_zome]
    pub struct Plan {
        processes: Local<process, planned_within>,
        non_process_commitments: Local<commitment, planned_within>,
        independent_demands: Local<commitment, independent_demand_of>,
    }
}

// PROCESS

mod process {
    use super::*;
    use hc_zome_rea_process_rpc::*;
    #[index_zome(query_fn_name="query_processes",read_all_fn_name="read_all_processes")]
    pub struct Process {
        observed_inputs: Local<economic_event, input_of>,
        observed_outputs: Local<economic_event, output_of>,
        committed_inputs: Local<commitment, input_of>,
        committed_outputs: Local<commitment, output_of>,
        intended_inputs: Local<intent, input_of>,
        intended_outputs: Local<intent, output_of>,
        planned_within: Local<plan, processes>,
    }
}

// PROCESS SPECIFICATION

mod process_specification {
    use super::*;
    use hc_zome_rea_process_specification_rpc::*;
    #[index_zome]
    pub struct ProcessSpecification {
        // :NOTE: blank means only the `read_all_` and `register_new_` APIs will be generated
    }
}

// PROPOSAL

mod proposal {
    use super::*;
    use hc_zome_rea_proposal_rpc::*;
    #[index_zome]
    pub struct Proposal {
        publishes: Local<proposed_intent, published_in>,
        published_to: Local<proposed_to, proposed>,
    }
}

// PROPOSED INTENT

mod proposed_intent {
    use super::*;
    use hc_zome_rea_proposed_intent_rpc::*;
    use hdk_semantic_indexes_zome_lib::ByAddress; // disambiguate from RPC query struct

    #[index_zome]
    pub struct ProposedIntent {
        published_in: Local<proposal, publishes>,
        publishes: Local<intent, proposed_in>,
    }

}

// PROPOSED TO

mod proposed_to {
    use super::*;
    use hc_zome_rea_proposed_to_rpc::*;
    use hdk_semantic_indexes_zome_lib::ByAddress; // disambiguate from RPC query struct
    
    #[index_zome]
    pub struct ProposedTo {
        proposed: Local<proposal, published_to>,
        // :TODO: figure out best approach for managing agent identifiers. Should there be a wrapper record to make treating them as records easier?
        // proposed_to: Local<agent, proposals>, // :TODO: finalise reciprocal query edge name
    }
    
}

// RESOURCE SPECIFICATION

mod resource_specification {
    use super::*;
    use hc_zome_rea_resource_specification_rpc::*;
    use hdk_semantic_indexes_zome_lib::ByAddress; // disambiguate from record query struct
    
    #[index_zome]
    pub struct ResourceSpecification {
        conforming_resources: Local<economic_resource, conforms_to>,
    }
    
}

// SATISFACTION

// mod satisfaction_for_observation {
//     use super::*;
//     use hc_zome_rea_satisfaction_rpc::*;
//     use hdk_semantic_indexes_zome_lib::ByAddress; // disambiguate from RPC query struct
    
//     #[index_zome]
//     struct Satisfaction {
//         // :NOTE: this gets updated by shadowed local record storage zome, not the remote one in Planning DNA
//         satisfied_by: Local<economic_event, satisfies>,
//     }    
// }

mod satisfaction_for_planning {
    use super::*;
    use hc_zome_rea_satisfaction_rpc::*;
    use hdk_semantic_indexes_zome_lib::ByAddress; // disambiguate from RPC query struct

    #[index_zome]
    pub struct Satisfaction {
        satisfies: Local<intent, satisfied_by>,
        satisfied_by: Local<commitment, satisfies>,
    }

}

// UNIT

mod unit {
    use hc_zome_rea_unit_rpc::*;
    use super::*;
    #[index_zome(record_read_fn_name="__internal_get_unit_by_hash")]
    pub struct Unit {
        // :NOTE: blank means only the `read_all_` and `register_new_` APIs will be generated
    }
}


// RECIPE FLOW

mod recipe_flow {
    use super::*;
    use hc_zome_rea_recipe_flow_rpc::*;
    #[index_zome(query_fn_name="query_recipe_flows", read_all_fn_name="read_all_recipe_flows")]
    pub struct RecipeFlow {
        recipe_input_of: Local<recipe_process, recipe_inputs>,
        recipe_output_of: Local<recipe_process, recipe_outputs>,
    }
}

// RECIPE PROCESS

mod recipe_process {
    use super::*;
    use hc_zome_rea_recipe_process_rpc::*;
    #[index_zome(query_fn_name="query_recipe_processes", read_all_fn_name="read_all_recipe_processes")]
    pub struct RecipeProcess {
        recipe_inputs: Local<recipe_flow, recipe_input_of>,
        recipe_outputs: Local<recipe_flow, recipe_output_of>,
    }
}