import { assert, test } from "vitest";
import { runScenario } from "@holochain/tryorama";
import { setupTest } from "../test_helpers";

test("Claim VF 1.0 (create, action, triggeredBy, claims query)", async () => {
    await runScenario(async scenario => {
        const server = await setupTest(scenario);

        // an agent to act as provider/receiver on the triggering event
        const agentRes = await server.executeOperation({
            query: `mutation($rs: OrganizationCreateParams!) {
                res: createOrganization(organization: $rs) { agent { id } }
            }`,
            variables: { rs: { name: "Settler Co-op" } },
        });
        const agentId = agentRes?.data?.res?.agent?.id;
        assert.ok(agentId, "agent created");

        // the economic event that already occurred, which the claim is made against
        const eventRes = await server.executeOperation({
            query: `mutation($rs: EconomicEventCreateParams!) {
                res: createEconomicEvent(event: $rs) { economicEvent { id } }
            }`,
            variables: { rs: { action: "produce", provider: agentId, receiver: agentId, note: "triggering event" } },
        });
        const eventId = eventRes?.data?.res?.economicEvent?.id;
        assert.ok(eventId, "economic event created");

        // the vf:Claim, in reciprocity for the event
        const claimRes = await server.executeOperation({
            query: `mutation($rs: ClaimCreateParams!) {
                res: createClaim(claim: $rs) {
                    claim { id action { id } triggeredBy { id } note finished }
                }
            }`,
            variables: { rs: { action: "produce", triggeredBy: eventId, note: "reciprocity claim", finished: false } },
        });
        const claim = claimRes?.data?.res?.claim;
        assert.ok(claim, "claim created");
        assert.equal(claim.action.id, "produce", "claim action round-trips");
        assert.equal(claim.triggeredBy.id, eventId, "claim triggeredBy resolves to the triggering event");
        assert.equal(claim.note, "reciprocity claim", "claim note round-trips");

        // claims collection query returns the new claim
        const claimsRes = await server.executeOperation({
            query: `query { claims { edges { node { id action { id } } } } }`,
        });
        const nodes = (claimsRes?.data?.claims?.edges || []).map((e: any) => e.node);
        assert.ok(nodes.find((n: any) => n.id === claim.id), "claims query contains the new claim");
    });
});
