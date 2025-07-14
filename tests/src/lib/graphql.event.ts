import { assert, test } from "vitest";
import { runScenario } from "@holochain/tryorama";
import { setupTest, graphQL } from "../test_helpers";

test("Event add, update", async () => {
  console.log("Running Event add, update test");
  await runScenario(async scenario => {
    console.log("Starting scenario");
    const server = await setupTest(scenario);

    console.log("========================Create Alice===========================");
    const alice = await graphQL(
    server,`
      mutation($rs: OrganizationCreateParams!) {
        res: createOrganization(organization: $rs) {
          agent {
            id
            revisionId
          }
        }
      }
    `,
      {
        rs: {
          name: "Alice",
        },
      },
    );
    console.log("createAgent: ", alice?.data?.res.agent.id);
    
    console.log("========================Create Commitment===========================");
    const commitmentRes = await graphQL(
      server,
      `mutation($rs: CommitmentCreateParams!) {
          res: createCommitment(commitment: $rs) {
            commitment {
              id
              revisionId
              inputOf {
                id
              }
              receiver {
                id
              }
              provider {
                id
                revisionId
                name 
              }
            }
          }
      }`,
      {
        rs: {
          action: "produce",
          provider: alice?.data?.res?.agent.id,
          receiver: alice?.data?.res?.agent.id,
        }
      }
    );
      
    console.log("createCommitment: ", commitmentRes, commitmentRes?.data?.res);

    console.log("========================Create Economic Event===========================");
    const date = new Date(Date.now())
    const eventRes = await graphQL(
      server,
      `mutation($rs: EconomicEventCreateParams!) {
          res: createEconomicEvent(event: $rs) {
            economicEvent {
              id
              revisionId
              note
              inputOf {
                id
              }
              outputOf {
                id
              }
              provider {
                id
                revisionId
                name 
              }
              receiver {
                id
                revisionId
                name
              }
            }
          }
      }`,
      {
        rs: {
          action: "produce",
          provider: alice?.data?.res?.agent.id,
          receiver: alice?.data?.res?.agent.id,
          hasBeginning: date,
          // inputOf: commitmentRes?.data?.res?.commitment.id,
          note: "Test event",
        }
      }
    );
    console.log("createEconomicEvent: ", eventRes, eventRes?.data?.res);
    assert(eventRes?.data?.res?.economicEvent.id, "Event ID should be defined");
    assert(eventRes?.data?.res?.economicEvent.revisionId, "Event revision ID should be defined");
    console.log("========================Update Economic Event===========================");
    const updatedEventRes = await graphQL(
      server,
      `mutation($rs: EconomicEventUpdateParams!) {
          res: updateEconomicEvent(event: $rs) {
            economicEvent {
              id
              revisionId
              note
              provider {
                id
                revisionId
                name 
              }
              hasBeginning
            }
          }
      }`,
      {
        rs: {
          revisionId: eventRes?.data?.res?.economicEvent.revisionId,
          note: "Updated test event",
        }
      }
    );
    console.log("updateEconomicEvent: ", updatedEventRes, updatedEventRes?.data?.res);
    assert(updatedEventRes?.data?.res?.economicEvent.id, "Updated Event ID should be defined");
    assert(updatedEventRes?.data?.res?.economicEvent.revisionId, "Updated Event revision ID should be defined");
    assert(updatedEventRes?.data?.res?.economicEvent.note === "Updated test event", "Event note should be updated");
  });
});