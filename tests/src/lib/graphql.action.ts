import { assert, test } from "vitest";
import { runScenario } from "@holochain/tryorama";
import { setupTest } from "../test_helpers";

test("Action fetches", async () => {
  await runScenario(async scenario => {
    const server = await setupTest(scenario);
    const getActions = await server.executeOperation({
      query: `
        query {
          actions {
            id
            label
            resourceEffect
            onhandEffect
            inputOutput
            pairsWith
          }
        }
      `,
    });
    assert.ok(getActions?.data?.actions?.length > 0, "No actions found");

    // Check if the first action has an id
    const firstActionId = getActions?.data?.actions[0]?.id;
    assert.ok(firstActionId, "First action does not have an id");

    // Fetch that action by id
    const getActionById = await server.executeOperation({
      query: `
        query($id: ID!) {
          action(id: $id) {
            id
            resourceEffect
          }
        }
      `,
      variables: { id: firstActionId },
    });

    // check that the action has a resourceEffect
    const actionResourceEffect = getActionById?.data?.action?.resourceEffect;
    assert.ok(actionResourceEffect, "Action does not have a resourceEffect");
  })
})