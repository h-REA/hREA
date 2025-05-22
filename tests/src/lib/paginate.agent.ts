import { assert, test } from "vitest";
import { runScenario } from "@holochain/tryorama";
import { setupTest } from '../test_helpers';

test("Paginate agents test", async () => {
    await runScenario(async scenario => {
        const server = await setupTest(scenario);

        // SIMPLIFY CALL
        async function graphQL(gql: string, variables: any = {}) {
            console.log("executing operation with gql", gql, variables);
        const result = await server.executeOperation({
            query: gql,
            variables,
        });
        return result;
        }

        console.log("========================Create Alice===========================");
        const createQuery = `
            mutation($rs: OrganizationCreateParams!) {
                res: createOrganization(organization: $rs) {
                    agent {
                        id
                    }
                }
            }
            `

        async function createAgent(name: string) {
            return await graphQL(createQuery,
                {
                    rs: {
                        name: name,
                    },
                })
        }

        const createAgent1 = await createAgent("Alice");
        const createAgent2 = await createAgent("Bob");
        const createAgent3 = await createAgent("Charlie");
        const createAgent4 = await createAgent("Dave");
        const createAgent5 = await createAgent("Eve");
        const createAgent6 = await createAgent("Frank");
        const createAgent7 = await createAgent("Grace");
        const createAgent8 = await createAgent("Heidi");

        console.log("=========================Get Charlie, Dave, and Eve===========================");
        const getAgents = await graphQL(`
            query {
                agents(first: 3, last: 5) {
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

        console.log("getAgents", getAgents?.data?.agents?.edges?.map((edge: any) => edge?.node?.name));

        assert.equal(getAgents?.data?.agents?.edges?.length, 3);
        assert.equal(getAgents?.data?.agents?.edges[0]?.node?.name, "Charlie");
        assert.equal(getAgents?.data?.agents?.edges[1]?.node?.name, "Dave");
        assert.equal(getAgents?.data?.agents?.edges[2]?.node?.name, "Eve");

        console.log("=========================Get Alice, Bob, and Charlie===========================");
        const getAgents2 = await graphQL(`
            query {
                agents(last: 3) {
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
        console.log("getAgents2", getAgents2?.data?.agents?.edges?.map((edge: any) => edge?.node?.name));
        assert.equal(getAgents2?.data?.agents?.edges?.length, 3);
        assert.equal(getAgents2?.data?.agents?.edges[0]?.node?.name, "Alice");
        assert.equal(getAgents2?.data?.agents?.edges[1]?.node?.name, "Bob");
        assert.equal(getAgents2?.data?.agents?.edges[2]?.node?.name, "Charlie");

        console.log("=========================Get Alice, Bob, and Charlie by ids===========================");
        const getAgents3 = await graphQL(`
            query($daveId: String!) {
                agents(before: $daveId) {
                    edges {
                        node {
                            id
                            revisionId
                            name
                        }
                    }
                }
            }
        `, {
            daveId: createAgent4?.data?.res?.agent?.id,
        });    

        console.log("getAgents3", getAgents3);
        console.log("getAgents3", getAgents3?.data?.agents?.edges?.map((edge: any) => edge?.node?.name));
        assert.equal(getAgents3?.data?.agents?.edges?.length, 3);
        assert.equal(getAgents3?.data?.agents?.edges[0]?.node?.name, "Alice");
        assert.equal(getAgents3?.data?.agents?.edges[1]?.node?.name, "Bob");
        assert.equal(getAgents3?.data?.agents?.edges[2]?.node?.name, "Charlie");

        console.log("=========================Get Grace and Heidi by ids===========================");
        const getAgents4 = await graphQL(`
            query($frankId: String!) {
                agents(after: $frankId) {
                    edges {
                        node {
                            id
                            revisionId
                            name
                        }
                    }
                }
            }
        `, {
            frankId: createAgent6?.data?.res?.agent?.id,
        });
        console.log("getAgents4", getAgents4);
        console.log("getAgents4", getAgents4?.data?.agents?.edges?.map((edge: any) => edge?.node?.name));
        assert.equal(getAgents4?.data?.agents?.edges?.length, 2);
        assert.equal(getAgents4?.data?.agents?.edges[0]?.node?.name, "Grace");
        assert.equal(getAgents4?.data?.agents?.edges[1]?.node?.name, "Heidi");
        
        console.log("=========================Get Charlie, Dave, and Eve by ids===========================");
        const getAgents5 = await graphQL(`
            query($bobId: String!, $frankId: String!) {
                agents(after: $bobId, before: $frankId) {
                    edges {
                        node {
                            id
                            revisionId
                            name
                        }
                    }
                }
            }
        `,{
            bobId: createAgent2?.data?.res?.agent?.id,
            frankId: createAgent6?.data?.res?.agent?.id,
        });
        console.log("getAgents5", getAgents5);
        console.log("getAgents5", getAgents5?.data?.agents?.edges?.map((edge: any) => edge?.node?.name));
        assert.equal(getAgents5?.data?.agents?.edges?.length, 3);
        assert.equal(getAgents5?.data?.agents?.edges[0]?.node?.name, "Charlie");
        assert.equal(getAgents5?.data?.agents?.edges[1]?.node?.name, "Dave");
        assert.equal(getAgents5?.data?.agents?.edges[2]?.node?.name, "Eve");
        
    })

})