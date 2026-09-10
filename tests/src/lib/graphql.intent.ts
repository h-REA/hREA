import { assert, test } from "vitest";
import { runScenario } from "@holochain/tryorama";
import { setupTest, graphQL } from "../test_helpers";

// Covers the two link-backed fields whose zome functions were missing after the
// Holochain 0.6 migration: Intent.satisfiedBy and Process.intendedOutputs.
test("Intent links: satisfiedBy and process intendedOutputs", async () => {
  await runScenario(async scenario => {
    const server = await setupTest(scenario);

    const alice = await graphQL(
      server,
      `mutation($rs: OrganizationCreateParams!) {
        res: createOrganization(organization: $rs) {
          agent {
            id
          }
        }
      }`,
      { rs: { name: "Alice" } },
    );
    const aliceId = alice?.data?.res?.agent?.id;
    assert(aliceId, "Agent ID should be defined");

    const processRes = await graphQL(
      server,
      `mutation($rs: ProcessCreateParams!) {
        res: createProcess(process: $rs) {
          process {
            id
          }
        }
      }`,
      { rs: { name: "Test Process" } },
    );
    const processId = processRes?.data?.res?.process?.id;
    assert(processId, "Process ID should be defined");

    const intentRes = await graphQL(
      server,
      `mutation($rs: IntentCreateParams!) {
        res: createIntent(intent: $rs) {
          intent {
            id
            outputOf {
              id
            }
          }
        }
      }`,
      {
        rs: {
          action: "produce",
          provider: aliceId,
          receiver: aliceId,
          outputOf: processId,
          note: "Intended output",
        },
      },
    );
    assert(!intentRes?.errors, `createIntent errors: ${JSON.stringify(intentRes?.errors)}`);
    const intentId = intentRes?.data?.res?.intent?.id;
    assert(intentId, "Intent ID should be defined");
    assert(intentRes?.data?.res?.intent?.outputOf?.id === processId, "Intent outputOf should be the process");

    const commitmentRes = await graphQL(
      server,
      `mutation($rs: CommitmentCreateParams!) {
        res: createCommitment(commitment: $rs) {
          commitment {
            id
            satisfies {
              id
            }
          }
        }
      }`,
      {
        rs: {
          action: "produce",
          provider: aliceId,
          receiver: aliceId,
          satisfies: intentId,
          note: "Satisfying commitment",
        },
      },
    );
    assert(!commitmentRes?.errors, `createCommitment errors: ${JSON.stringify(commitmentRes?.errors)}`);
    const commitmentId = commitmentRes?.data?.res?.commitment?.id;
    assert(commitmentId, "Commitment ID should be defined");
    assert(commitmentRes?.data?.res?.commitment?.satisfies?.id === intentId, "Commitment satisfies should be the intent");

    const intentQuery = await graphQL(
      server,
      `query($id: ID!) {
        intent(id: $id) {
          id
          satisfiedBy {
            id
          }
        }
      }`,
      { id: intentId },
    );
    assert(!intentQuery?.errors, `intent.satisfiedBy errors: ${JSON.stringify(intentQuery?.errors)}`);
    const satisfiedBy = intentQuery?.data?.intent?.satisfiedBy;
    assert(Array.isArray(satisfiedBy) && satisfiedBy.length === 1, "Intent should be satisfied by exactly one commitment");
    assert(satisfiedBy[0].id === commitmentId, "Intent.satisfiedBy should return the commitment");

    const processQuery = await graphQL(
      server,
      `query($id: ID!) {
        process(id: $id) {
          id
          intendedOutputs {
            id
          }
        }
      }`,
      { id: processId },
    );
    assert(!processQuery?.errors, `process.intendedOutputs errors: ${JSON.stringify(processQuery?.errors)}`);
    const intendedOutputs = processQuery?.data?.process?.intendedOutputs;
    assert(Array.isArray(intendedOutputs) && intendedOutputs.length === 1, "Process should have exactly one intended output");
    assert(intendedOutputs[0].id === intentId, "Process.intendedOutputs should return the intent");

    // An economic event that satisfies the intent must show up under Intent.observedBy,
    // and must not leak into satisfiedBy (distinct link types).
    const eventRes = await graphQL(
      server,
      `mutation($rs: EconomicEventCreateParams!) {
        res: createEconomicEvent(event: $rs) {
          economicEvent {
            id
          }
        }
      }`,
      {
        rs: {
          action: "produce",
          provider: aliceId,
          receiver: aliceId,
          hasBeginning: new Date(Date.now()).toDateString(),
          satisfies: [intentId],
          note: "Satisfying event",
        },
      },
    );
    assert(!eventRes?.errors, `createEconomicEvent errors: ${JSON.stringify(eventRes?.errors)}`);
    const eventId = eventRes?.data?.res?.economicEvent?.id;
    assert(eventId, "Event ID should be defined");

    const intentAfterEvent = await graphQL(
      server,
      `query($id: ID!) {
        intent(id: $id) {
          id
          satisfiedBy {
            id
          }
          observedBy {
            id
          }
        }
      }`,
      { id: intentId },
    );
    assert(!intentAfterEvent?.errors, `intent.observedBy errors: ${JSON.stringify(intentAfterEvent?.errors)}`);
    const observedBy = intentAfterEvent?.data?.intent?.observedBy;
    assert(Array.isArray(observedBy) && observedBy.length === 1, "Intent should be observed by exactly one economic event");
    assert(observedBy[0].id === eventId, "Intent.observedBy should return the economic event");
    assert(intentAfterEvent?.data?.intent?.satisfiedBy?.length === 1, "Intent.satisfiedBy should still hold exactly the commitment");
  });
});
