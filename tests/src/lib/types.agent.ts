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

    // fetch organizations
    const orgs = await graphQL(`
      query {
        organizations {
          edges {
            node {
              id
              revisionId
              name
            }
          }
        }
      }
    `)
    console.log("Orgs: ", orgs);
    assert.equal(orgs.data.organizations.edges.length, 1);
    assert.equal(orgs.data.organizations.edges[0].node.name, "Alice");

    // fetch people
    const people = await graphQL(`
      query {
        people {
          edges {
            node {
              id
              revisionId
              name
            }
          }
        }
      }
    `)
    console.log("People: ", people);
    assert.equal(people.data.people.edges.length, 0);
    assert.equal(people.data.people.edges[0], undefined);

    console.log("========================Create Bob===========================");
    const createAgent2 = await graphQL(`
      mutation($rs: AgentCreateParams!) {
        res: createPerson(person: $rs) {
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
    console.log("Create Bob: ", createAgent2);
    // fetch organizations
    const orgs2 = await graphQL(`
      query {
        people {
          edges {
            node {
              id
              revisionId
              name
            }
          }
        }
      }
    `)
    console.log("People: ", orgs2);
    assert.equal(orgs2.data.people.edges.length, 1);
    assert.equal(orgs2.data.people.edges[0].node.name, "Bob");
  })
})