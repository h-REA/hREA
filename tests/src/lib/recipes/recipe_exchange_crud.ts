import { assert, test } from "vitest";
import { runScenario, pause } from "@holochain/tryorama";
import { setupTest, graphQL } from "../../test_helpers";

test("Plan record API", async () => {
    await runScenario(async scenario => {
        const server = await setupTest(scenario);

        const exampleEntry = {
            name: "lkasdf",
            note: "fffaa"
        };

        // Create RecipeExchange
        const createResp = await graphQL(
            server, `
                mutation($rs: RecipeExchangeCreateParams!) {
                    rs: createRecipeExchange(recipeExchange: $rs) {
                        recipeExchange {
                            id
                            revisionId
                            name
                            note
                        }
                    }
                }
            `,
            { rs: exampleEntry }
        );
        await pause(500);
        assert.ok(createResp?.data?.rs?.recipeExchange?.id, "recipe created");
        const recipeId = createResp.data.rs.recipeExchange.id;

        // Read RecipeExchange
        const readResp = await graphQL(
            server, `
                query($id: ID!) {
                    res: recipeExchange(id: $id) {
                        id
                        revisionId
                        name
                        note
                    }
                }
            `,
            { id: recipeId }
        );
        assert.ok(readResp, "recipe read");
        assert.deepEqual(
            readResp.data.res,
            createResp.data.rs.recipeExchange,
            "recipe read matches create"
        );

        // Create RecipeFlow 1
        const flowResp = await graphQL(
            server, `
                mutation($rs: RecipeFlowCreateParams!) {
                    rs: createRecipeFlow(recipeFlow: $rs) {
                        recipeFlow {
                            id
                            revisionId
                            note
                        }
                    }
                }
            `,
            {
                rs: {
                    note: "recipe flow 1",
                    action: "raise",
                    resourceConformsTo: recipeId,
                    stage: recipeId,
                    recipeClauseOf: recipeId,
                },
            }
        );
        await pause(500);

        // Create RecipeFlow 2
        const flowResp2 = await graphQL(
            server, `
                mutation($rs: RecipeFlowCreateParams!) {
                    rs: createRecipeFlow(recipeFlow: $rs) {
                        recipeFlow {
                            id
                            revisionId
                            note
                        }
                    }
                }
            `,
            {
                rs: {
                    note: "recipe flow 1",
                    action: "raise",
                    resourceConformsTo: recipeId,
                    stage: recipeId,
                    recipeReciprocalClauseOf: recipeId,
                },
            }
        );
        await pause(500);

        // Fetch all RecipeExchanges
        const fetchAllResp = await graphQL(
            server, `
                query {
                    res: recipeExchanges {
                        edges {
                            node {
                                id
                                revisionId
                                name
                                note
                                recipeClauses {
                                    id
                                    revisionId
                                    note
                                }
                                recipeReciprocalClauses {
                                    id
                                    revisionId
                                    note
                                }
                            }
                        }
                    }
                }
            `
        );
        assert.ok(fetchAllResp, "recipeExchanges fetched");
        assert.ok(fetchAllResp.data.res.edges.length > 0, "recipeExchanges not empty");
        const firstRecipeExchange = fetchAllResp.data.res.edges[0].node;
        assert.ok(firstRecipeExchange.recipeClauses.length > 0, "recipeClauses not empty");
        assert.ok(firstRecipeExchange.recipeReciprocalClauses.length > 0, "recipeReciprocalClauses not empty");

        // Clean up
        server.stop();
    });
});