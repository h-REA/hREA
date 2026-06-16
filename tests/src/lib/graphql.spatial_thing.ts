import { assert, test } from "vitest";
import { runScenario } from "@holochain/tryorama";
import { setupTest } from "../test_helpers";

test("SpatialThing VF 1.0 (create, fields, query)", async () => {
    await runScenario(async scenario => {
        const server = await setupTest(scenario);

        const res = await server.executeOperation({
            query: `mutation($rs: SpatialThingCreateParams!) {
                res: createSpatialThing(spatialThing: $rs) {
                    spatialThing { id name mappableAddress lat long alt note }
                }
            }`,
            variables: {
                rs: {
                    name: "Sensorica Lab",
                    mappableAddress: "Montreal, QC",
                    lat: 45.5019,
                    long: -73.5674,
                    alt: 36.0,
                    note: "main hardware lab",
                },
            },
        });
        const st = res?.data?.res?.spatialThing;
        assert.ok(st, "spatial thing created");
        assert.equal(st.name, "Sensorica Lab", "name round-trips");
        assert.equal(st.mappableAddress, "Montreal, QC", "mappableAddress round-trips");
        assert.equal(st.lat, 45.5019, "lat round-trips");
        assert.equal(st.long, -73.5674, "long round-trips");

        const list = await server.executeOperation({
            query: `query { spatialThings { edges { node { id name } } } }`,
        });
        const nodes = (list?.data?.spatialThings?.edges || []).map((e: any) => e.node);
        assert.ok(nodes.find((n: any) => n.id === st.id), "spatialThings query contains the new location");
    });
});
