import { assert, test } from "vitest";
import { runScenario } from "@holochain/tryorama";
import { setupTest, graphQL } from "../test_helpers";

test("Proposal crud", async () => {
    await runScenario(async scenario => {
        const server = await setupTest(scenario);
        // create an intent
        console.log("========================Create Intent===========================");
        const intent = await server.executeOperation({
            query: `
                mutation($rs: IntentCreateParams!) {
                    res: createIntent(intent: $rs) {
                        intent {
                            id
                        }
                    }
                }
            `,
            variables: {
                rs: {
                    action: 'raise',    
                    name: "Test Intent",
                },
            },
        });
        console.log("createIntent: ", intent?.data?.res?.intent);
        assert.ok(intent?.data?.res?.intent);
        const intentId = intent?.data?.res?.intent.id;

        // create a proposal
        console.log("========================Create Proposal===========================");
        const proposal = await server.executeOperation({
            query: `
                mutation($rs: ProposalCreateParams!) {
                res: createProposal(proposal: $rs) {
                    proposal {
                        id
                        revisionId
                        name
                        note
                        publishes {
                            id
                            action {
                                id
                            }
                        }
                    }
                }
                }
            `,
            variables: {
                rs: {
                    name: "Test Proposal",
                    publishes: [intent?.data?.res?.intent.id],
                },
            },
        });
        console.log("createProposal: ", proposal?.data?.res?.proposal);
        assert.ok(proposal?.data?.res?.proposal);
        assert.equal(proposal?.data?.res?.proposal.publishes.length, 1);
        assert.equal(proposal?.data?.res?.proposal.publishes[0].id, intentId);
    });
});