import { assert, test } from "vitest";
import { runScenario } from "@holochain/tryorama";
import { setupTest, graphQL } from "../test_helpers";

test("Agreement fetches", async () => {
    await runScenario(async scenario => {
        const server = await setupTest(scenario);

        // create an agreement
        console.log("========================Create Agreement===========================");
        const agreement = await graphQL(
            server, `
                mutation($rs: AgreementCreateParams!) {
                    res: createAgreement(agreement: $rs) {
                        agreement {
                            id
                            revisionId
                            name
                            note
                        }
                    }
                }
            `,
            {
                rs: {
                    name: "Test Agreement",
                    note: "This is a test agreement",
                },
            }
        );

        console.log("createAgreement: ", agreement?.data?.res?.agreement);
        assert.ok(agreement?.data?.res?.agreement);

        // update the agreement
        console.log("========================Update Agreement===========================");
        const updatedAgreement = await graphQL(
            server, `
                mutation($rs: AgreementUpdateParams!) {
                    res: updateAgreement(agreement: $rs) {
                        agreement {
                            id
                            revisionId
                            name
                            note
                        }
                    }
                }
            `,
            {
                rs: {
                    revisionId: agreement?.data?.res?.agreement.revisionId,
                    name: "Updated Test Agreement",
                },
            }
        );
        console.log("updateAgreement: ", updatedAgreement?.data?.res?.agreement);
        assert.ok(updatedAgreement?.data?.res?.agreement);
        assert.equal(
            updatedAgreement?.data?.res?.agreement.name,
            "Updated Test Agreement",
            "Agreement name should be updated"
        );

        // update agreement again
        console.log("========================Update Agreement Again===========================");
        const updatedAgreementAgain = await graphQL(
            server, `
                mutation($rs: AgreementUpdateParams!) {
                    res: updateAgreement(agreement: $rs) {
                        agreement {
                            id
                            revisionId
                            name
                            note
                        }
                    }
                }
            `,
            {
                rs: {
                    revisionId: updatedAgreement?.data?.res?.agreement.revisionId,
                    name: "Updated Test Agreement Again",
                    note: "Updated the note now",
                },
            }
        );
        console.log("updateAgreementAgain: ", updatedAgreementAgain?.data?.res?.agreement);
        assert.ok(updatedAgreementAgain?.data?.res?.agreement);
        assert.equal(
            updatedAgreementAgain?.data?.res?.agreement.name,
            "Updated Test Agreement Again",
            "Agreement name should be updated again"
        );

        // get all agreements
        console.log("========================Get Agreements===========================");
        const getAgreements = await graphQL(
            server, `
                query {
                    agreements {
                        edges {
                            node {
                                id
                                revisionId
                                name
                                note
                            }
                        }
                    }
                }
            `
        );
        console.log("getAgreements: ", getAgreements?.data?.agreements?.edges);
        assert.ok(getAgreements?.data?.agreements);

        // get all agreements again
        console.log("========================Get Agreements Again===========================");
        const getAgreementsSecond = await graphQL(
            server, `
                query {
                    agreements {
                        edges {
                            node {
                                id
                                revisionId
                                name
                                note
                            }
                        }
                    }
                }
            `
        );
        console.log("getAgreementsSecond: ", getAgreementsSecond?.data?.agreements?.edges);

        // get agreement by id
        console.log("========================Get Agreement by ID===========================");
        console.log("getting agreement by id", agreement?.data?.res?.agreement.id);
        const getAgreement = await graphQL(
            server, `
                query($id: ID!) {
                    agreement(id: $id) {
                        id
                        revisionId
                    }
                }
            `,
            {
                id: agreement?.data?.res?.agreement.id,
            }
        );
        console.log("getAgreement: ", getAgreement?.data?.agreement?.id);
        assert.ok(getAgreement?.data?.agreement?.id);

        // delete the agreement
        console.log("========================Delete Agreement===========================");
        const deleteAgreement = await graphQL(
            server, `
                mutation($revisionId: ID!) {
                    res: deleteAgreement(revisionId: $revisionId)
                }
            `,
            {
                revisionId: agreement?.data?.res?.agreement.id,
            }
        );
        console.log("deleteAgreement: ", deleteAgreement?.data?.res);
        assert.ok(deleteAgreement?.data?.res);

        // get all agreements again and make sure the agreement is deleted
        console.log("========================Get Agreements Again===========================");
        const getAgreementsAgain = await graphQL(
            server, `
                query {
                    agreements {
                        edges {
                            node {
                                id
                                revisionId
                                name
                                note
                            }
                        }
                    }
                }
            `
        );
        console.log("getAgreementsAgain: ", getAgreementsAgain?.data?.agreements?.edges);
        assert.ok(getAgreementsAgain?.data?.agreements);
        assert.equal(
            getAgreementsAgain?.data?.agreements?.edges.length,
            0,
            "Agreement should be deleted"
        );

        // close the server
        server.stop();
    });
});