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

test("Proposal purpose offer/request (VF 1.0)", async () => {
    await runScenario(async scenario => {
        const server = await setupTest(scenario);

        const mkIntent = async (name: string) => {
            const res = await server.executeOperation({
                query: `mutation($rs: IntentCreateParams!) { res: createIntent(intent: $rs) { intent { id } } }`,
                variables: { rs: { action: 'raise', name } },
            });
            assert.ok(res?.data?.res?.intent, `intent ${name} created`);
            return res.data.res.intent.id;
        };

        const mkProposal = async (name: string, purpose: string, intentId: string) =>
            server.executeOperation({
                query: `mutation($rs: ProposalCreateParams!) { res: createProposal(proposal: $rs) { proposal { id purpose } } }`,
                variables: { rs: { name, purpose, publishes: [intentId] } },
            });

        // create an offer proposal and confirm purpose round-trips
        const offer = await mkProposal("Test Offer", "offer", await mkIntent("Offer intent"));
        assert.ok(offer?.data?.res?.proposal, "offer proposal created");
        assert.equal(offer.data.res.proposal.purpose, "offer", "purpose round-trips as offer");
        const offerId = offer.data.res.proposal.id;

        // create a request proposal
        const request = await mkProposal("Test Request", "request", await mkIntent("Request intent"));
        assert.ok(request?.data?.res?.proposal, "request proposal created");
        assert.equal(request.data.res.proposal.purpose, "request", "purpose round-trips as request");
        const requestId = request.data.res.proposal.id;

        // an invalid purpose must be rejected (ProposalPurpose enum guard)
        const bad = await mkProposal("Bad", "banana", await mkIntent("Bad intent"));
        assert.ok(bad?.errors && bad.errors.length > 0, "invalid purpose rejected");

        // offers query returns the offer, excludes the request
        const offers = await server.executeOperation({
            query: `query { offers { edges { node { id purpose } } } }`,
        });
        const offerNodes = (offers?.data?.offers?.edges || []).map((e: any) => e.node);
        assert.ok(offerNodes.find((n: any) => n.id === offerId), "offers contains the offer");
        assert.ok(!offerNodes.find((n: any) => n.id === requestId), "offers excludes the request");
        offerNodes.forEach((n: any) => assert.equal(n.purpose, "offer", "all offers have purpose offer"));

        // requests query returns the request, excludes the offer
        const requests = await server.executeOperation({
            query: `query { requests { edges { node { id purpose } } } }`,
        });
        const requestNodes = (requests?.data?.requests?.edges || []).map((e: any) => e.node);
        assert.ok(requestNodes.find((n: any) => n.id === requestId), "requests contains the request");
        assert.ok(!requestNodes.find((n: any) => n.id === offerId), "requests excludes the offer");
    });
});