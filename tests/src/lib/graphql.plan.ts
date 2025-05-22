import { assert, test } from "vitest";
import { runScenario } from "@holochain/tryorama";
import { setupTest, graphQL } from "../test_helpers";

test("Plan crud", async () => {
  await runScenario(async scenario => {
    const server = await setupTest(scenario);
    // create a plan
    console.log("========================Create Plan===========================");
    const plan = await server.executeOperation({
      query: `
        mutation($rs: PlanCreateParams!) {
          res: createPlan(plan: $rs) {
            plan {
              id
              revisionId
              name
              note
            }
          }
        }
      `,
      variables: {
        rs: {
          name: "Test Plan",
          note: "This is a test plan",
        },
      },
    });

    console.log("createPlan: ", plan?.data?.res?.plan);
    assert.ok(plan?.data?.res?.plan);

    // update the plan
    console.log("========================Update Plan===========================");
    const updatedPlan = await server.executeOperation({
      query: `
        mutation($rs: PlanUpdateParams!) {
          res: updatePlan(plan: $rs) {
            plan {
              id
              revisionId
              name
              note
            }
          }
        }
      `,
      variables: {
        rs: {
          revisionId: plan?.data?.res?.plan.revisionId,
          name: "Updated Test Plan",
        },
      },
    });
    console.log("updatePlan: ", updatedPlan?.data?.res?.plan);
    assert.ok(updatedPlan?.data?.res?.plan);
    assert.equal(
      updatedPlan?.data?.res?.plan.name,
      "Updated Test Plan",
      "Plan name should be updated"
    );

    // update plan again
    console.log("========================Update Plan Again===========================");
    const updatedPlanAgain = await server.executeOperation({
      query: `
        mutation($rs: PlanUpdateParams!) {
          res: updatePlan(plan: $rs) {
            plan {
              id
              revisionId
              name
              note
            }
          }
        }
      `,
      variables: {
        rs: {
          revisionId: updatedPlan?.data?.res?.plan.revisionId,
          name: "Updated Test Plan Again",
          note: "Updated the note now",
        },
      },
    });
    console.log("updatePlanAgain: ", updatedPlanAgain?.data?.res?.plan);
    assert.ok(updatedPlanAgain?.data?.res?.plan);
    assert.equal(
      updatedPlanAgain?.data?.res?.plan.name,
      "Updated Test Plan Again",
      "Plan name should be updated again"
    );

    // get all plans
    console.log("========================Get Plans===========================");
    const getPlans = await server.executeOperation({
      query: `
        query {
          plans {
            edges {
              node {
                id
                revisionId
                name
                note
                independentDemands {
                  id
                }
              }
            }
          }
        }
      `,
    });
    console.log("getPlans: ", getPlans?.data?.plans?.edges);
    assert.ok(getPlans?.data?.plans);

    // get plan by id
    console.log("========================Get Plan by ID===========================");
    console.log("getting plan by id", plan?.data?.res?.plan.id);
    const getPlan = await server.executeOperation({
      query: `
        query($id: ID!) {
          plan(id: $id) {
            id
            revisionId
          }
        }
      `,
      variables: {
        id: plan?.data?.res?.plan.id,
      },
    });
    console.log("getPlan: ", getPlan?.data?.plan?.id);
    assert.ok(getPlan?.data?.plan?.id);

    // delete the plan
    console.log("========================Delete Plan===========================");
    
    const deletePlan = await graphQL(  
      server, `
        mutation($revisionId: ID!) {
          res: deletePlan(revisionId: $revisionId)
        }
      `,
      {
        revisionId: plan?.data?.res?.plan.id,
      }
    )
    console.log("deletePlan: ", deletePlan?.data?.res);
    assert.ok(deletePlan?.data?.res);

    // get all plans again and make sure the plan is deleted
    console.log("========================Get Plans Again===========================");
    const getPlansAgain = await server.executeOperation({
      query: `
        query {
          plans {
            edges {
              node {
                id
                revisionId
                name
                note
              }
            }
          }
        }
      `,
    });
    console.log("getPlansAgain: ", getPlansAgain?.data?.plans?.edges);
    assert.ok(getPlansAgain?.data?.plans);
    assert.equal(
      getPlansAgain?.data?.plans?.edges.length,
      0,
      "Plan should be deleted"
    );
    // close the server
    server.stop();
  })
})