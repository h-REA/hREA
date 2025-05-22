import { assert, test } from "vitest";
import { runScenario } from "@holochain/tryorama";
import { setupTest, graphQL } from "../test_helpers";

test("Plan crud", async () => {
  await runScenario(async scenario => {
    const server = await setupTest(scenario);

    console.log("========================Create Alice===========================");
    const alice = await graphQL(
    server,`
      mutation($rs: OrganizationCreateParams!) {
        res: createOrganization(organization: $rs) {
          agent {
            id
            revisionId
          }
        }
      }
    `,
      {
        rs: {
          name: "Alice",
        },
      },
    );
    console.log("createAgent: ", alice?.data?.res.agent.id);
    
    console.log("========================Create Commitment===========================");
    const commitmentRes = await graphQL(
      server,
      `mutation($rs: CommitmentCreateParams!) {
          res: createCommitment(commitment: $rs) {
            commitment {
              id
              revisionId
              inputOf {
                id
              }
              receiver {
                id
              }
              provider {
                id
                revisionId
                name 
              }
            }
          }
      }`,
      {
        rs: {
          action: "produce",
          provider: alice?.data?.res?.agent.id,
          receiver: alice?.data?.res?.agent.id,
        }
      }
    );
      
    console.log("createCommitment: ", commitmentRes, commitmentRes?.data?.res);

    // create agreement
    const agreement = await graphQL(
      server,`
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

    // create a commitment for the agreement
    const commitment = await graphQL(
      server,`
        mutation($rs: CommitmentCreateParams!) {
          res: createCommitment(commitment: $rs) {
            commitment {
              id
              revisionId
              clauseOf {
                id
                revisionId
                name
                note
              }
            }
          }
        }
      `,
      {
        rs: {
          action: "produce",
          provider: alice?.data?.res?.agent.id,
          receiver: alice?.data?.res?.agent.id,
          clauseOf: agreement?.data?.res?.agreement.id,
        }
      }
    );

    console.log("createCommitment: ", commitment?.data?.res?.commitment);

    // retrieve the agreement with the commitment
    const getAgreement = await graphQL(
      server,`
        query($id: ID!) {
          agreement(id: $id) {
            id
            revisionId
            name
            note
            commitments {
              id
              revisionId
              inputOf {
                id
              }
              receiver {
                id
              }
              provider {
                id
                revisionId
                name 
              }
            }
          }
        }
      `,
      {
        id: agreement?.data?.res?.agreement.id,
      }
    );

    console.log("getAgreement: ", getAgreement?.data?.agreement);

    assert.ok(getAgreement?.data?.agreement);
    assert.equal(getAgreement?.data?.agreement.id, agreement?.data?.res?.agreement.id);
    assert.equal(getAgreement?.data?.agreement.name, agreement?.data?.res?.agreement.name);
  });
});