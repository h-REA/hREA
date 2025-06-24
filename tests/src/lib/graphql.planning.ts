import { assert, test } from "vitest";
import { runScenario } from "@holochain/tryorama";
import { setupTest, graphQL } from "../test_helpers";

export const PLAN_RETURN_FIELDS = `query($id: ID!) {
    plan(id: $id) {
        id
        revisionId
        name
        note
        meta {
            retrievedRevision {
                id
                time
            }
        }
        independentDemands {
            id
            revisionId
            action {
                id
                label
            }
            receiver {
                id
                name
            }
            resourceQuantity {
                hasNumericalValue
                hasUnit {
                    id
                    label
                }
            }
            resourceConformsTo {
                id
                name
                defaultUnitOfResource {
                    id
                    label
                }
            }
        }
        nonProcessCommitments {
            id
            revisionId
            stage {
                id
            }
            action {
                id
                label
            }
            finished
            fulfilledBy {
                id
            }
            receiver {
                id
                name
            }
            provider {
                id
                name
            }
            resourceQuantity {
                hasNumericalValue
                hasUnit {
                    id
                }
            }
            resourceConformsTo {
                id
                name
                defaultUnitOfResource {
                    id
                }
            }
            clauseOf {
                id
                name
                note
                revisionId
                commitments {
                    finished
                    fulfilledBy {
                        id
                    }
                    id
                    revisionId
                    provider {
                        id
                        name
                    }
                    receiver {
                        id
                        name
                    }
                    hasBeginning
                    action {
                        id
                        label
                    }
                    resourceConformsTo {
                        id
                        name
                        defaultUnitOfResource {
                            id
                        }
                        resourceClassifiedAs
                    }
                    resourceQuantity {
                        hasNumericalValue
                        hasUnit {
                            id
                        }
                    }
                }
            }
        }
        processes {
            id
            revisionId
            name
            meta {
                retrievedRevision {
                    id
                    time
                }
            }
            basedOn {
                id
                name
            }
            committedInputs {
                id
                revisionId
                hasBeginning
                action {
                    id
                    label
                }
                meta {
                    retrievedRevision {
                        id
                        time
                    }
                }
                provider {
                    id
                    name
                }
                receiver {
                    id
                    name
                }
                resourceQuantity {
                    hasNumericalValue
                    hasUnit {
                        id
                    }
                }
                resourceConformsTo {
                    id
                    name
                    defaultUnitOfResource {
                        id
                    }
                    resourceClassifiedAs
                }
                finished
                fulfilledBy {
                    id
                }
                clauseOf {
                    id
                    name
                    note
                    revisionId
                    commitments {
                        finished
                        fulfilledBy {
                            id
                        }
                        id
                        revisionId
                        provider {
                            id
                            name
                        }
                        receiver {
                            id
                            name
                        }
                        hasBeginning
                        action {
                            id
                            label
                        }
                        resourceConformsTo {
                            id
                            name
                            defaultUnitOfResource {
                                id
                            }
                            resourceClassifiedAs
                        }
                        resourceQuantity {
                            hasNumericalValue
                            hasUnit {
                                id
                                label
                            }
                        }
                    }
                }
            }
            committedOutputs {
                id
                revisionId
                hasBeginning
                action {
                    id
                    label
                }
                meta {
                    retrievedRevision {
                        id
                        time
                    }
                }
                provider {
                    id
                    name
                }
                receiver {
                    id
                    name
                }
                resourceQuantity {
                    hasNumericalValue
                    hasUnit {
                        id
                    }
                }
                resourceConformsTo {
                    id
                    name
                    resourceClassifiedAs
                }
                finished
                fulfilledBy {
                    id
                }
                clauseOf {
                    id
                    name
                    note
                    revisionId
                    commitments {
                        id
                        revisionId
                        provider {
                            id
                            name
                        }
                        receiver {
                            id
                            name
                        }
                        hasBeginning
                        finished
                        fulfilledBy {
                            id
                        }
                        action {
                            id
                            label
                        }
                        resourceConformsTo {
                            id
                            name
                            defaultUnitOfResource {
                                id
                            }
                        }
                        resourceQuantity {
                            hasNumericalValue
                            hasUnit {
                                id
                            }
                        }
                    }
                }
            }
        }
    }
}
`;

test("Full plan", async () => {
    await runScenario(async scenario => {
        const server = await setupTest(scenario);
        const createAgentQuery = `
        mutation($rs: OrganizationCreateParams!) {
            res: createOrganization(organization: $rs) {
                agent {
                    id
                }
            }
        }
        `
        async function createEntry(fields: Object, query: string) {
            return await graphQL(server, query,
                {
                    rs: {
                        ...fields
                    },
                })
        }

        // create 2 agents
        const createAgent1 = await createEntry({ name: "Alice" }, createAgentQuery);
        const createAgent2 = await createEntry({ name: "Bob" }, createAgentQuery);

        // create a plan
        const createPlanQuery = `
        mutation($rs: PlanCreateParams!) {
            res: createPlan(plan: $rs) {
                plan {
                    id
                    revisionId
                }
            }
        }
        `
        const createPlan = await createEntry({ name: "Test Plan", note: "This is a test plan" }, createPlanQuery);
        console.log("createPlan: ", createPlan?.data?.res?.plan);

        // create 5 processes
        const createProcessQuery = `
        mutation($rs: ProcessCreateParams!) {
            res: createProcess(process: $rs) {
                process {
                    id
                    revisionId
                }
            }
        }
        `

        const createCommitmentQuery = `
        mutation($rs: CommitmentCreateParams!) {
            res: createCommitment(commitment: $rs) {
                commitment {
                    id
                    revisionId
                }
            }
        }
        `
        
        for (let i = 0; i < 5; i++) {
            const createProcess = await createEntry({
                name: `Test Process ${i + 1}`,
                plannedWithin: createPlan?.data?.res?.plan.id,
                finished: false,
            }, createProcessQuery);
            console.log("createProcess: ", createProcess?.errors);
            console.log("createProcess: ", createProcess?.data?.res?.process);
            
            // create 5 input commitments per process
            for (let j = 0; j < 10; j++) {
                const createCommitment = await createEntry({
                    action: 'raise',
                    note: `Test Commitment ${j + 1}`,
                    inputOf: createProcess?.data?.res?.process.id,
                    provider: createAgent1?.data?.res?.agent.id,
                    receiver: createAgent2?.data?.res?.agent.id,
                    hasBeginning: new Date(),
                }, createCommitmentQuery);
                console.log("createCommitment: ", createCommitment?.errors);
                console.log("createCommitment: ", createCommitment?.data?.res?.commitment);
            }

            // create 5 output commitments per process
            for (let j = 0; j < 10; j++) {
                const createCommitment = await createEntry({
                    action: 'raise',
                    note: `Test Commitment ${j + 1}`,
                    outputOf: createProcess?.data?.res?.process.id,
                    provider: createAgent1?.data?.res?.agent.id,
                    receiver: createAgent2?.data?.res?.agent.id,
                }, createCommitmentQuery);
                console.log("createCommitment: ", createCommitment?.errors);
                console.log("createCommitment: ", createCommitment?.data?.res?.commitment);
            }
        }

        // create 5 independent demands (commitments) that reference plan
        for (let i = 0; i < 5; i++) {
            const createCommitment = await createEntry({
                action: 'raise',
                note: `Test Independent Demand ${i + 1}`,
                independentDemandOf: createPlan?.data?.res?.plan.id,
                provider: createAgent1?.data?.res?.agent.id,
                receiver: createAgent2?.data?.res?.agent.id,
            }, createCommitmentQuery);
            console.log("createCommitment: ", createCommitment?.errors);
            console.log("createCommitment: ", createCommitment?.data?.res?.commitment);
        }

        // create 10 non-process commitments (commitments) that reference plan
        for (let i = 0; i < 10; i++) {
            const createCommitment = await createEntry({
                action: 'raise',
                note: `Test Non-Process Commitment ${i + 1}`,
                plannedWithin: createPlan?.data?.res?.plan.id,
                provider: createAgent1?.data?.res?.agent.id,
                receiver: createAgent2?.data?.res?.agent.id,
            }, createCommitmentQuery);
            console.log("createCommitment: ", createCommitment?.errors);
            console.log("createCommitment: ", createCommitment?.data?.res?.commitment);
        }

        // fetch everything
        const startTime = new Date();
        const res = await graphQL(server, PLAN_RETURN_FIELDS,
            {
                id: createPlan?.data?.res?.plan.id,
            })
        const endTime = new Date();
        console.log("res: ", res?.errors);
        console.log("Plan: ", JSON.stringify(res?.data?.plan, null, 2));
        console.log("Time to retrieve plan: ", endTime.getTime() - startTime.getTime());
        assert.ok(res?.data?.plan);
    });
});
