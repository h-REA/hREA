import { assert, test } from "vitest";
import { runScenario } from "@holochain/tryorama";
import { setupTest } from "../test_helpers";

test("AgreementBundle VF 1.0 (create, agreements grouping, query)", async () => {
    await runScenario(async scenario => {
        const server = await setupTest(scenario);

        // create an agreement to group
        const agRes = await server.executeOperation({
            query: `mutation($rs: AgreementCreateParams!) {
                res: createAgreement(agreement: $rs) { agreement { id } }
            }`,
            variables: { rs: { name: "Co-op charter", note: "founding agreement" } },
        });
        const agreementId = agRes?.data?.res?.agreement?.id;
        assert.ok(agreementId, "agreement created");

        // bundle it
        const res = await server.executeOperation({
            query: `mutation($rs: AgreementBundleCreateParams!) {
                res: createAgreementBundle(agreementBundle: $rs) {
                    agreementBundle { id name agreements { id } }
                }
            }`,
            variables: { rs: { name: "2026 agreements", agreements: [agreementId] } },
        });
        const bundle = res?.data?.res?.agreementBundle;
        assert.ok(bundle, "agreement bundle created");
        assert.equal(bundle.name, "2026 agreements", "name round-trips");
        assert.ok(bundle.agreements?.find((a: any) => a.id === agreementId), "bundle resolves its grouped agreement");

        const list = await server.executeOperation({
            query: `query { agreementBundles { edges { node { id name } } } }`,
        });
        const nodes = (list?.data?.agreementBundles?.edges || []).map((e: any) => e.node);
        assert.ok(nodes.find((n: any) => n.id === bundle.id), "agreementBundles query contains the new bundle");
    });
});
