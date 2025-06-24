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
                        meta {
                            retrievedRevision {
                                id
                                time
                            }
                        }
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

        const metaTime1 = createAgent?.data?.res?.agent.meta?.retrievedRevision?.time;
        console.log("metaTime1", metaTime1);

        console.log("========================Update Alice===========================");
        const updateAgent = await graphQL(`
            mutation($rs: OrganizationUpdateParams!) {
                res: updateOrganization(organization: $rs) {
                    agent {
                        id
                        revisionId
                        name
                        meta {
                            retrievedRevision {
                                id
                                time
                            }
                        }
                    }
                }
            }
        `,
            {
                rs: {
                    revisionId: createAgent?.data?.res?.agent.revisionId,
                    name: "Alice Updated",
                },
            },
        )

        const metaTime2 = updateAgent?.data?.res?.agent.meta?.retrievedRevision?.time;

        console.log("metaTime2", metaTime2);
        assert.ok(metaTime2 > metaTime1, "Meta time should be updated on agent update");

        console.log("========================Get All Agents===========================");
        const getAllAgents = await graphQL(`
            query {
                agents {
                    edges {
                        node {
                            id
                            revisionId
                            name
                            meta {
                                retrievedRevision {
                                    id
                                    time
                                }
                            }
                        }
                    }
                }
            }
        `);
        console.log("getAllAgents: ", getAllAgents?.data?.agents);
        const firstAgent = getAllAgents?.data?.agents?.edges[0]?.node;
        console.log("First Agent: ", firstAgent);
        const metaTime3 = firstAgent?.meta?.retrievedRevision?.time;
        console.log("metaTime3", metaTime3);

        assert.ok(metaTime3 == metaTime2, "Meta time should be the same as the last update time");
    })
})