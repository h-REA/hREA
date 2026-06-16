import { assert, test } from "vitest";
import { runScenario } from "@holochain/tryorama";
import { setupTest } from "../test_helpers";

test("ProductBatch VF 1.0 (create, batchNumber, query)", async () => {
    await runScenario(async scenario => {
        const server = await setupTest(scenario);

        const res = await server.executeOperation({
            query: `mutation($rs: ProductBatchCreateParams!) {
                res: createProductBatch(productBatch: $rs) {
                    productBatch { id batchNumber }
                }
            }`,
            variables: { rs: { batchNumber: "LOT-2026-001" } },
        });
        const batch = res?.data?.res?.productBatch;
        assert.ok(batch, "product batch created");
        assert.equal(batch.batchNumber, "LOT-2026-001", "batchNumber round-trips");

        const list = await server.executeOperation({
            query: `query { productBatches { edges { node { id batchNumber } } } }`,
        });
        const nodes = (list?.data?.productBatches?.edges || []).map((e: any) => e.node);
        assert.ok(nodes.find((n: any) => n.id === batch.id), "productBatches query contains the new batch");
    });
});
