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
  
    // Create second event
    console.log("========================Create Second Economic Event===========================");
    const secondEventRes = await graphQL(
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
          action: "consume",
          provider: alice?.data?.res?.agent.id,
          receiver: alice?.data?.res?.agent.id,
          hasBeginning: date,
          note: "Second test event",
        }
      }
    );
    console.log("createSecondEconomicEvent: ", secondEventRes, secondEventRes?.data?.res);
    assert(secondEventRes?.data?.res?.economicEvent.id, "Second Event ID should be defined");
    assert(secondEventRes?.data?.res?.economicEvent.revisionId, "Second Event revision ID should be defined");
    assert(secondEventRes?.data?.res?.economicEvent.note === "Second test event", "Second Event note should be defined");

    // Fetch all economic events
    console.log("========================Fetch All Economic Events===========================");
    const allEventsRes = await graphQL(
      server,
      `query {
        economicEvents {
          edges {
            node {
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
        }
      }`,
    );
    console.log("fetchAllEconomicEvents: ", allEventsRes, allEventsRes?.data?.economicEvents?.edges);
    assert(allEventsRes?.data?.economicEvents.edges.length == 2, "There should be two economic events");
    assert(allEventsRes?.data?.economicEvents.edges[0].node.id, "First Event ID should be defined");
    assert(allEventsRes?.data?.economicEvents.edges[0].node.revisionId, "First Event revision ID should be defined");
    assert(allEventsRes?.data?.economicEvents.edges[0].node.note, "First Event note should be defined");

    // Create resource specification
    console.log("========================Create Resource Specification===========================");
    const resourceSpecRes = await graphQL(
      server,
      `mutation($rs: ResourceSpecificationCreateParams!) {
        res: createResourceSpecification(resourceSpecification: $rs) {
          resourceSpecification {
            id
            revisionId
            name
          }
        }
      }`,
      {
        rs: {
          name: "Test Resource Specification",
        }
      }
    );
    console.log("createResourceSpecification: ", resourceSpecRes, resourceSpecRes?.data?.res);
    assert(resourceSpecRes?.data?.res?.resourceSpecification.id, "Resource Specification ID should be defined");
    assert(resourceSpecRes?.data?.res?.resourceSpecification.revisionId, "Resource Specification revision ID should be defined");
    assert(resourceSpecRes?.data?.res?.resourceSpecification.name === "Test Resource Specification", "Resource Specification name should be defined");

    // create economic event with resource
    console.log("========================Create Economic Event with Resource===========================");
    const resourceEventRes = await graphQL(
      server,
      `mutation($rs: EconomicEventCreateParams!, $re: EconomicResourceCreateParams!) {
        res: createEconomicEvent(event: $rs, newInventoriedResource: $re) {
          economicEvent {
            id
            revisionId
            note
          }
        }
      }`,
      {
        rs: {
          action: "raise",
          provider: alice?.data?.res?.agent.id,
          receiver: alice?.data?.res?.agent.id,
          hasBeginning: date,
          note: "Resource event",
          resourceQuantity: {
            hasNumericalValue: 100,
          }
        },
        re: {
          name: "Test Resource",
          conformsTo: resourceSpecRes?.data?.res?.resourceSpecification.id,
        }
      }
    );
    console.log("createEconomicEventWithResource: ", resourceEventRes, resourceEventRes?.data?.res);
    assert(resourceEventRes?.data?.res?.economicEvent.id, "Resource Event ID should be defined");
    assert(resourceEventRes?.data?.res?.economicEvent.revisionId, "Resource Event revision ID should be defined");
    assert(resourceEventRes?.data?.res?.economicEvent.note === "Resource event", "Resource Event note should be defined");

    // Fetch all economic resources
    console.log("========================Fetch All Economic Resources===========================");
    const allResourcesRes = await graphQL(
      server,
      `query {
        economicResources {
          edges {
            node {
              id
              revisionId
              name
              conformsTo {
                id
                revisionId
                name
              }
              onhandQuantity {
                hasNumericalValue
                hasUnit {
                  id
                }
              }
              accountingQuantity {
                hasNumericalValue
                hasUnit {
                  id
                }
              }
            }
          }
        }
      }`,
    );
    console.log("fetchAllEconomicResources: ", allResourcesRes, allResourcesRes?.data?.economicResources?.edges);
    assert(allResourcesRes?.data?.economicResources.edges.length > 0, "There should be at least one economic resource");
    assert(allResourcesRes?.data?.economicResources.edges[0].node.id, "Resource ID should be defined");
    assert(allResourcesRes?.data?.economicResources.edges[0].node.revisionId, "Resource revision ID should be defined");
    assert(allResourcesRes?.data?.economicResources.edges[0].node.name, "Resource name should be defined");
    assert(allResourcesRes?.data?.economicResources.edges[0].node.conformsTo.id, "Resource specification ID should be defined");
    assert(allResourcesRes?.data?.economicResources.edges[0].node.conformsTo.revisionId, "Resource specification revision ID should be defined");
    assert(allResourcesRes?.data?.economicResources.edges[0].node.conformsTo.name, "Resource specification name should be defined");

    // Add event that adds to the resource
    console.log("========================Add Event to Resource===========================");
    const addToResourceEventRes = await graphQL(
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
            resourceInventoriedAs {
              id
              revisionId
              name
            }
          }
        }
      }`,
      {
        rs: {
          action: "raise",
          provider: alice?.data?.res?.agent.id,
          receiver: alice?.data?.res?.agent.id,
          hasBeginning: date,
          note: "Add to resource event",
          resourceInventoriedAs: allResourcesRes?.data?.economicResources.edges[0].node.id,
          resourceQuantity: {
            hasNumericalValue: 1,
          }
        }
      }
    );
    console.log("addToResourceEvent: ", addToResourceEventRes, addToResourceEventRes?.data?.res);
    assert(addToResourceEventRes?.data?.res?.economicEvent.id, "Add to Resource Event ID should be defined");
    assert(addToResourceEventRes?.data?.res?.economicEvent.revisionId, "Add to Resource Event revision ID should be defined");
    assert(addToResourceEventRes?.data?.res?.economicEvent.note === "Add to resource event", "Add to Resource Event note should be defined");
    assert(addToResourceEventRes?.data?.res?.economicEvent.resourceInventoriedAs.id, "Resource Inventoried As ID should be defined");
    assert(addToResourceEventRes?.data?.res?.economicEvent.resourceInventoriedAs.revisionId, "Resource Inventoried As revision ID should be defined");
    assert(addToResourceEventRes?.data?.res?.economicEvent.resourceInventoriedAs.name === "Test Resource", "Resource Inventoried As name should be defined");
    
    // Get the updated resource
    console.log("========================Fetch Updated Resource===========================");
    const updatedResourceRes = await graphQL(
      server,
      `query($id: ID!) {
        res: economicResource(id: $id) {
          id
          revisionId
          name
          conformsTo {
            id
            revisionId
            name
          }
          onhandQuantity {
            hasNumericalValue
            hasUnit {
              id
            }
          }
          accountingQuantity {
            hasNumericalValue
            hasUnit {
              id
            }
          }
        }
      }`,
      {
        id: addToResourceEventRes?.data?.res?.economicEvent.resourceInventoriedAs.id,
      }
    );
    console.log("fetchUpdatedResource: ", updatedResourceRes, updatedResourceRes?.data?.res);
    assert(updatedResourceRes?.data?.res?.id, "Updated Resource ID should be defined");
    assert(updatedResourceRes?.data?.res?.revisionId, "Updated Resource revision ID should be defined");
    assert(updatedResourceRes?.data?.res?.name === "Test Resource", "Updated Resource name should be defined");
    assert(updatedResourceRes?.data?.res?.conformsTo.id, "Updated Resource specification ID should be defined");
    assert(updatedResourceRes?.data?.res?.conformsTo.revisionId, "Updated Resource specification revision ID should be defined");
    assert(updatedResourceRes?.data?.res?.conformsTo.name === "Test Resource Specification", "Updated Resource specification name should be defined");
    // resource should equal 101
    assert(updatedResourceRes?.data?.res?.onhandQuantity.hasNumericalValue === 101, "Updated Resource onhand quantity should be 101");
    assert(updatedResourceRes?.data?.res?.accountingQuantity.hasNumericalValue === 101, "Updated Resource accounting quantity should be 101");
  });
});