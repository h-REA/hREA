import { assert, test } from "vitest";
import { runScenario, pause } from "@holochain/tryorama";
import { setupTest, graphQL } from "../../test_helpers";

test("RecipeFlow CRUD API", async () => {
    await runScenario(async scenario => {
        const server = await setupTest(scenario);

        // Create ResourceSpecification
        const resp = await graphQL(
            server, `
                mutation($rs: ResourceSpecificationCreateParams!) {
                    rs: createResourceSpecification(resourceSpecification: $rs) {
                        resourceSpecification {
                            id
                        }
                    }
                }
            `,
            { rs: { name: "test resource spec" } }
        );
        await pause(100);
        const rsId = resp.data.rs.resourceSpecification.id;

        // Create two RecipeProcesses
        const exampleRecipeProcessEntry = {
            name: "lkasdf",
            note: "fffaa"
        };

        const respProcess1 = await graphQL(
            server, `
                mutation($rs: RecipeProcessCreateParams!) {
                    rs: createRecipeProcess(recipeProcess: $rs) {
                        recipeProcess {
                            id
                            revisionId
                            name
                            note
                        }
                    }
                }
            `,
            { rs: exampleRecipeProcessEntry }
        );
        const respProcess1Address = respProcess1.data.rs.recipeProcess.id;
        await pause(100);

        const respProcess2 = await graphQL(
            server, `
                mutation($rs: RecipeProcessCreateParams!) {
                    rs: createRecipeProcess(recipeProcess: $rs) {
                        recipeProcess {
                            id
                            revisionId
                            name
                            note
                        }
                    }
                }
            `,
            { rs: exampleRecipeProcessEntry }
        );
        const respProcess2Address = respProcess2.data.rs.recipeProcess.id;
        await pause(100);

        // Get recipe process 1
        const getRespProcess1 = await graphQL(
            server, `
                query($id: ID!) {
                    res: recipeProcess(id: $id) {
                        id
                        revisionId
                        name
                        note
                    }
                }
            `,
            { id: respProcess1Address }
        );
        assert.ok(getRespProcess1, "get recipe process 1");

        const newUnitRes = await graphQL(
            server, `
                mutation($rs: UnitCreateParams!) {
                    res: createUnit(unit: $rs) {
                        unit {
                            id
                            revisionId
                            label
                            symbol
                            omUnitIdentifier
                            classifiedAs
                        }
                    }
                }
            `,
            {
                rs: {
                    label: "Test Unit",
                    symbol: "TU",
                    omUnitIdentifier: "test_unit_identifier",
                },
            }
        );
        console.log("newUnitRes: ", newUnitRes);
        assert.ok(newUnitRes, "unit created response");
        assert.ok(newUnitRes.data.res, "unit created");
        const newUnitId = newUnitRes.data.res.unit.id;
        console.log("newUnitId: ", newUnitId);

        // Create RecipeFlow
        const exampleEntry = {
            note: "just testing",
            action: "raise",
            resourceConformsTo: rsId,
            stage: respProcess1Address,
            recipeInputOf: respProcess1Address,
            recipeOutputOf: respProcess2Address,
            resourceQuantity: {
                hasUnit: newUnitId,
                hasNumericalValue: 10,
            },
            instructions: "do this",
            providerRole: "provider",
            receiverRole: "receiver",
        };

        const createResp = await graphQL(
            server, `
                mutation($rs: RecipeFlowCreateParams!) {
                    rs: createRecipeFlow(recipeFlow: $rs) {
                        recipeFlow {
                            id
                            revisionId
                            providerRole
                            receiverRole
                            instructions
                            action {
                                id
                            }
                            resourceQuantity {
                                hasNumericalValue
                                hasUnit {
                                    id
                                    label
                                }
                            }
                            recipeInputOf {
                                id
                            }
                            recipeOutputOf {
                                id
                            }
                            note
                        }
                    }
                }
            `,
            { rs: exampleEntry }
        );
        await pause(100);
        console.log("createResp: ", createResp);
        assert.ok(createResp.data.rs.recipeFlow.id, "recipe created");

        const recipeId = createResp.data.rs.recipeFlow.id;
        const recipeRev = createResp.data.rs.recipeFlow.revisionId;

        // Get RecipeFlow
        const getResp = await graphQL(
            server, `
                query($id: ID!) {
                    res: recipeFlow(id: $id) {
                        id
                        revisionId
                        providerRole
                        receiverRole
                        instructions
                        action {
                            id
                        }
                        resourceQuantity {
                            hasNumericalValue
                            hasUnit {
                                id
                                label
                            }
                        }
                        recipeInputOf {
                            id
                        }
                        recipeOutputOf {
                            id
                        }
                        note
                    }
                }
            `,
            { id: recipeId }
        );
        assert.ok(getResp, "recipe read");
        console.log("get recipe resp: ", getResp);
        assert.deepEqual(getResp.data.res, createResp.data.rs.recipeFlow, "record read OK");

        // Query process recipeInputs/Outputs
        const fetchProcess = await graphQL(
            server, `
                query($id: ID!) {
                    res: recipeProcess(id: $id) {
                        id
                        recipeInputs {
                            id
                        }
                        recipeOutputs {
                            id
                        }
                    }
                }
            `,
            { id: respProcess1Address }
        );
        assert.ok(fetchProcess, "recipe process read");
        assert.ok(fetchProcess.data.res, "recipe process read OK");

        // Update RecipeFlow
        const updateResp = await graphQL(
            server, `
                mutation($rs: RecipeFlowUpdateParams!) {
                    res: updateRecipeFlow(recipeFlow: $rs) {
                        recipeFlow {
                            id
                            revisionId
                        }
                    }
                }
            `,
            { rs: { revisionId: recipeRev, ...exampleEntry } }
        );
        await pause(100);
        assert.equal(updateResp.data.res.recipeFlow.id, recipeId, "record ID consistent");
        assert.notEqual(updateResp.data.res.recipeFlow.revisionId, recipeRev, "record updated");

        // Delete RecipeFlow
        const deleteResult = await graphQL(
            server, `
                mutation($revisionId: ID!) {
                    res: deleteRecipeFlow(revisionId: $revisionId)
                }
            `,
            { revisionId: recipeRev }
        );
        await pause(100);
        assert.equal(deleteResult.data.res, true, "delete successful");

        // Clean up
        server.stop();
    });
});