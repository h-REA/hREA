import { assert, test } from "vitest";
import { runScenario } from "@holochain/tryorama";
import { setupTest } from '../test_helpers';

test("Agent test", async () => {
  await runScenario(async scenario => {
    const server = await setupTest(scenario);

    // SIMPLIFY CALL
    async function graphQL(gql: string, variables: any = {}) {
      const result = await server.executeOperation({
        query: gql,
        variables,
      });
      return result;
    }

    console.log("========================Create Alice===========================");
    const createAgent = await graphQL(`
      mutation($rs: OrganizationCreateParams!) {
        res: createOrganization(organization: $rs) {
          agent {
            id
            revisionId
            name
          }
        }
      }
    `,
      {
        rs: {
          name: "Alice",
        },
      },
    )

    console.log("========================Create Bob===========================");
    const createSecondAgent = await graphQL(`
      mutation($rs: OrganizationCreateParams!) {
        res: createOrganization(organization: $rs) {
          agent {
            id
            revisionId
            name
          }
        }
      }
    `,
      {
        rs: {
          name: "Bob",
        },
      },
    )
    console.log("createAgent: ", createAgent, createSecondAgent);

    console.log("bob res", createAgent?.data?.res?.agent);

    console.log("=========================Get Agents #1===========================");
    const getAgents = await graphQL(`
      query {
        agents {
          edges {
            node {
              id
              revisionId
              name
            }
          }
        }
      }
    `);

    console.log("getAgents: ", getAgents?.data?.agents?.edges);

    assert(getAgents?.data?.agents?.edges.length === 2, "Expected 2 agents to be created");
    
    console.log("=========================Delete Alice===========================");
    // delete agent
    const deleteAgent = await graphQL(`
      mutation($revisionId: ID!) {
        res: deleteOrganization(revisionId: $revisionId)
      }
    `,
      {
        revisionId: createAgent?.data?.res?.agent?.id
      }
    )

    console.log(deleteAgent);

    console.log("=========================Get Agents #2===========================");
    // try fetching again
    const getAgents2 = await graphQL(`
      query {
        agents {
          edges {
            node {
              id
              revisionId
              name
            }
          }
        }
      }
    `);
    console.log("getAgents2: ", getAgents2?.data?.agents?.edges);

    assert(getAgents2?.data?.agents?.edges.length === 1, "Expected 1 agent to be deleted"); 

    console.log("=========================Update Bob #1===========================");
    console.log("bob debug", JSON.stringify(createSecondAgent?.data?.res?.agent));
    console.log({
      name: "Bob Updated",
      revisionId: createSecondAgent?.data?.res?.agent?.id,
    });

    // update agent
    const updateAgent = await graphQL(`
      mutation($rs: OrganizationUpdateParams!) {
        res: updateOrganization(organization: $rs) {
          agent {
            id
            name
          }
        }
      }
    `,
      {
        rs: {
          name: "Bob Updated",
          revisionId: createSecondAgent?.data?.res?.agent?.id,
        },
      }
    )
    console.log("updateAgent: ", updateAgent);
    assert(updateAgent?.data?.res?.agent?.name === "Bob Updated", "Expected agent name to be updated");
  
    console.log("=========================Update Bob #2===========================");
    // update agent again
    const updateAgent2 = await graphQL(`
      mutation($rs: OrganizationUpdateParams!) {
        res: updateOrganization(organization: $rs) {
          agent {
            id
            revisionId
            name
          }
        }
      }
    `,
      {
        rs: {
          name: "Bob Updated Again",
          revisionId: createSecondAgent?.data?.res?.agent?.revisionId,
        },
      }
    )
    console.log("updateAgent2: ", updateAgent2);
    assert(updateAgent2?.data?.res?.agent?.name === "Bob Updated Again", "Expected agent name to be updated again");

    console.log("=========================Get Agents #3===========================");
    // fetch agents and make sure the name is updated
    const getAgents3 = await graphQL(`
      query {
        agents {
          edges {
            node {
              id
              revisionId
              name
            }
          }
        }
      }
    `);
    console.log("getAgents 3: ", getAgents3?.data?.agents?.edges);
    // assert length is 1
    assert(getAgents3?.data?.agents?.edges.length === 1, "Expected 1 agent to be updated");
    assert(getAgents3?.data?.agents?.edges[0]?.node?.name === "Bob Updated Again", "Expected agent name to be updated again");
    // assert(getAgents3?.data?.agents?.edges[0]?.node?.id === createSecondAgent?.data?.res?.agent?.id, "Expected agent id to be the same");
  
    console.log("=========================Delete Bob===========================");
    // delete agent
    const deleteAgent2 = await graphQL(`
      mutation($revisionId: ID!) {
        res: deleteOrganization(revisionId: $revisionId)
      }
    `,
      {
        revisionId: createSecondAgent?.data?.res?.agent?.revisionId
      }
    )
    console.log("deleteAgent2: ", deleteAgent2);
    assert(deleteAgent2?.data?.res === true, "Expected agent to be deleted");
    console.log("=========================Get Agents #4===========================");
    // try fetching again
    const getAgents4 = await graphQL(`
      query {
        agents {
          edges {
            node {
              id
              revisionId
              name
            }
          }
        }
      }
    `);
    console.log("getAgents4: ", getAgents4?.data?.agents?.edges);
    assert(getAgents4?.data?.agents?.edges.length === 0, "Expected 0 agents to be deleted");
  });
});