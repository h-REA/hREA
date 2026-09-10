//! Zome-boundary tests for `ReaAgent`.
//!
//! Agent carries the required-string rule on two fields and its own `agentType`
//! vocabulary, which had no test before this file. The two are checked in
//! sequence, so an empty `agentType` and an unknown one must produce different
//! messages; asserting each exactly is what keeps one from standing in for the
//! other.

use holochain::prelude::*;
use hrea_sweettest::{
    classifications, empty_agent, rejection, run, shared_env, ReaAgentUpdateParams,
    UpdateReaAgentInput,
};

#[test]
fn rejects_an_empty_name() {
    run(async {
        let env = shared_env().await;
        let result: Result<Record, _> = env
            .conductor
            .call_fallible(&env.zome(), "create_rea_agent", empty_agent("", "Person"))
            .await;
        let msg = rejection(result.expect_err("an agent with no name must be rejected"));
        assert!(
            msg.contains("Agent name must not be empty"),
            "expected the required-string message for name, got: {msg}"
        );
    })
}

/// The emptiness check on `agentType` runs before the vocabulary check, so an
/// empty type must not be reported as an unknown one.
#[test]
fn rejects_an_empty_agent_type() {
    run(async {
        let env = shared_env().await;
        let result: Result<Record, _> = env
            .conductor
            .call_fallible(&env.zome(), "create_rea_agent", empty_agent("Nameless kind", ""))
            .await;
        let msg = rejection(result.expect_err("an agent with no type must be rejected"));
        assert!(
            msg.contains("Agent agentType must not be empty"),
            "expected the required-string message for agentType, got: {msg}"
        );
    })
}

#[test]
fn rejects_an_agent_type_outside_the_vocabulary() {
    run(async {
        let env = shared_env().await;
        let result: Result<Record, _> = env
            .conductor
            .call_fallible(&env.zome(), "create_rea_agent", empty_agent("HAL", "Robot"))
            .await;
        let msg = rejection(result.expect_err("'Robot' is not a vf:Agent kind here"));
        assert!(
            msg.contains("Agent agentType 'Robot' must be 'Person' or 'Organization'"),
            "expected the agentType vocabulary message, got: {msg}"
        );
    })
}

/// Both members of the vocabulary, because the coordinator branches on the
/// string when it indexes an agent under `all_people` or `all_organizations`:
/// a rule that accepted only one of them would break half of that index.
#[test]
fn accepts_both_members_of_the_vocabulary() {
    run(async {
        let env = shared_env().await;
        let person: Record = env
            .conductor
            .call(&env.zome(), "create_rea_agent", empty_agent("Ada", "Person"))
            .await;
        assert!(person.entry().as_option().is_some(), "the created person carried no entry");

        let org: Record = env
            .conductor
            .call(&env.zome(), "create_rea_agent", empty_agent("Sensorica", "Organization"))
            .await;
        assert!(org.entry().as_option().is_some(), "the created organization carried no entry");
    })
}

/// `validate_update_rea_agent` runs the same field checks as create. This is the
/// only place in the suite that proves an entity-specific vocabulary still binds
/// after creation, which is where a rule wired only into the create callback
/// would go unnoticed.
#[test]
fn rejects_an_update_into_an_unknown_agent_type() {
    run(async {
        let env = shared_env().await;
        let record: Record = env
            .conductor
            .call(&env.zome(), "create_rea_agent", empty_agent("Grace", "Person"))
            .await;

        let result: Result<Record, _> = env
            .conductor
            .call_fallible(
                &env.zome(),
                "update_rea_agent",
                UpdateReaAgentInput {
                    revision_id: record.action_address().clone(),
                    entry: ReaAgentUpdateParams {
                        agent_type: Some("Robot".into()),
                        ..Default::default()
                    },
                },
            )
            .await;
        let msg = rejection(result.expect_err("a person does not become a robot by edit"));
        assert!(
            msg.contains("Agent agentType 'Robot' must be 'Person' or 'Organization'"),
            "expected the agentType vocabulary message, got: {msg}"
        );
    })
}

/// The collection bound applies to `classifiedAs` here, on an entity whose other
/// fields are all scalars. It is the same shared rule as on EconomicEvent, so
/// this case is what shows the rule travels with the field rather than living in
/// one entry type.
#[test]
fn rejects_a_classification_list_over_the_collection_bound() {
    run(async {
        let env = shared_env().await;
        let mut agent = empty_agent("Over-classified co-op", "Organization");
        agent.classified_as = Some(classifications(1025));

        let result: Result<Record, _> = env
            .conductor
            .call_fallible(&env.zome(), "create_rea_agent", agent)
            .await;
        let msg = rejection(result.expect_err("1025 entries is over MAX_COLLECTION_LEN"));
        assert!(
            msg.contains("Agent classifiedAs exceeds the maximum of 1024 entries"),
            "expected the collection bound message, got: {msg}"
        );
    })
}
