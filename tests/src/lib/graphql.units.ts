import { assert, test } from "vitest";
import { runScenario } from "@holochain/tryorama";
import { setupTest, graphQL } from "../test_helpers";

test("Unit fetches", async () => {
    await runScenario(async scenario => {
        const server = await setupTest(scenario);

        // create a unit
        console.log("========================Create Unit===========================");
        const unit = await graphQL(
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
                    classifiedAs: ["classification1", "classification2"],
                },
            }
        );

        console.log("createUnit: ", unit?.data?.res?.unit);
        assert.ok(unit?.data?.res?.unit);

        // update the unit
        console.log("========================Update Unit===========================");
        const updatedUnit = await graphQL(
            server, `
                mutation($rs: UnitUpdateParams!) {
                    res: updateUnit(unit: $rs) {
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
                    revisionId: unit?.data?.res?.unit.revisionId,
                    label: "Updated Test Unit",
                    symbol: "UTU",
                    omUnitIdentifier: "updated_test_unit_identifier",
                    classifiedAs: ["updated_classification1", "updated_classification2"],
                },
            }
        );
        console.log("updateUnit: ", updatedUnit);
        // console.log("updateUnit: ", updatedUnit?.data?.res?.unit);
        assert.ok(updatedUnit?.data?.res?.unit);
        assert.equal(
            updatedUnit?.data?.res?.unit.label,
            "Updated Test Unit",
            "Unit label should be updated"
        );

        // get all units
        console.log("========================Get Units===========================");
        const getUnits = await graphQL(
            server, `
                query {
                    units {
                        edges {
                            node {
                                id
                                revisionId
                                label
                                symbol
                                omUnitIdentifier
                                classifiedAs
                            }
                        }
                    }
                }
            `
        );
        console.log("getUnits: ", getUnits?.data?.units?.edges);
        assert.ok(getUnits?.data?.units);

        // get unit by id
        console.log("========================Get Unit by ID===========================");
        console.log("getting unit by id", unit?.data?.res?.unit.id);
        const getUnit = await graphQL(
            server, `
                query($id: ID!) {
                    unit(id: $id) {
                        id
                        revisionId
                        label
                        symbol
                        omUnitIdentifier
                        classifiedAs
                    }
                }
            `,
            {
                id: unit?.data?.res?.unit.id,
            }
        );
        console.log("getUnit: ", getUnit?.data?.unit?.id);
        assert.ok(getUnit?.data?.unit?.id);

        // delete the unit
        console.log("========================Delete Unit===========================");
        const deleteUnit = await graphQL(
            server, `
                mutation($revisionId: ID!) {
                    res: deleteUnit(revisionId: $revisionId)
                }
            `,
            {
                revisionId: unit?.data?.res?.unit.revisionId,
            }
        );
        console.log("deleteUnit: ", deleteUnit?.data?.res);
        assert.ok(deleteUnit?.data?.res);

        // get all units again and make sure the unit is deleted
        console.log("========================Get Units Again===========================");
        const getUnitsAgain = await graphQL(
            server, `
                query {
                    units {
                        edges {
                            node {
                                id
                                label
                                symbol
                                omUnitIdentifier
                                classifiedAs
                            }
                        }
                    }
                }
            `
        );
        console.log("getUnitsAgain: ", getUnitsAgain?.data?.units?.edges);
        assert.ok(getUnitsAgain?.data?.units);
        assert.equal(
            getUnitsAgain?.data?.units?.edges.length,
            0,
            "Unit should be deleted"
        );

        // close the server
        server.stop();
    });
});