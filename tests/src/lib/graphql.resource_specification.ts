import { assert, test } from "vitest";
import { runScenario } from "@holochain/tryorama";
import { setupTest } from "../test_helpers";

test("ResourceSpecification VF 1.0 booleans (substitutable, mediumOfExchange)", async () => {
    await runScenario(async scenario => {
        const server = await setupTest(scenario);

        const create = async (name: string, substitutable: boolean, mediumOfExchange: boolean) =>
            server.executeOperation({
                query: `mutation($rs: ResourceSpecificationCreateParams!) {
                    res: createResourceSpecification(resourceSpecification: $rs) {
                        resourceSpecification { id name substitutable mediumOfExchange }
                    }
                }`,
                variables: { rs: { name, substitutable, mediumOfExchange } },
            });

        // a currency-like spec: substitutable + medium of exchange
        const usd = await create("USD", true, true);
        const usdSpec = usd?.data?.res?.resourceSpecification;
        assert.ok(usdSpec, "USD resource specification created");
        assert.equal(usdSpec.substitutable, true, "substitutable round-trips true");
        assert.equal(usdSpec.mediumOfExchange, true, "mediumOfExchange round-trips true");

        // a non-fungible, non-currency spec
        const widget = await create("Hand-carved widget", false, false);
        const widgetSpec = widget?.data?.res?.resourceSpecification;
        assert.ok(widgetSpec, "widget resource specification created");
        assert.equal(widgetSpec.substitutable, false, "substitutable round-trips false");
        assert.equal(widgetSpec.mediumOfExchange, false, "mediumOfExchange round-trips false");
    });
});
