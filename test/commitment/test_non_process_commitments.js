import test from 'tape'
import { pause } from '@holochain/tryorama'
import {
  buildPlayer,
  mockIdentifier,
  mockAddress,
} from '../init.js'

const testCommitmentProps = {
  action: 'raise',
  resourceClassifiedAs: ['some-resource-type'],
  resourceQuantity: { hasNumericalValue: 1, hasUnit: mockIdentifier() },
  provider: mockAddress(),
  receiver: mockAddress(),
}

test('Plan links & queries', async (t) => {
  // display the filename for context in the terminal and use .warn
  // to override the tap testing log filters
  console.warn(`\n\n${import.meta.url}`)
  const alice = await buildPlayer(['combined'])

  // ===CREATE PLAN===
  let start = new Date()
  try {
    let resp = await alice.graphQL(`
      mutation($rs: PlanCreateParams!) {
        res: createPlan(plan: $rs) {
          plan {
            id
          }
        }
      }
    `, {
      rs: {
        name: 'test plan',
        created: new Date(),
        due: new Date(),
        note: 'just testing, nothing was rly planned',
      },
    })
    let end = new Date()
    console.log('⏱︎  time to create plan:', (end - start) * 0.001, 'seconds ⏱︎')
    t.ok(resp.data.res.plan.id, 'plan created')
    const planId = resp.data.res.plan.id


    // ===CREATE AGREEMENT FOR COMMITMENT===
    start = new Date()
    let createResp = await alice.graphQL(
        `
        mutation($rs: AgreementCreateParams!) {
          res: createAgreement(agreement: $rs) {
            agreement {
              id
              revisionId
            }
          }
        }
      `,
        {
          rs: {
            name: 'test agreement',
            created: new Date(),
            note: 'just testing, nothing was rly agreed',
          },
        },
      )

    // ===CREATE 1 COMMITMENT===
    // define async function to create a process and commitment
    const createNonProcessCommitment = async (commitmentNote) => {
      const resp = await alice.graphQL(`
        mutation($c: CommitmentCreateParams!) {
          commitment: createCommitment(commitment: $c) {
            commitment {
              id
              revisionId
            }
          }
        }
      `, {
        c: {
          independentDemandOf: planId,
          plannedWithin: planId,
          note: commitmentNote,
          due: new Date(Date.now() + 86400000),
          clauseOf: createResp.data.res.agreement.id,
          ...testCommitmentProps,
        },
      })
      return resp.data
    }
    
    start = new Date()
    let res1 = await createNonProcessCommitment('linked commitment 1')
    end = new Date()
    console.log('⏱︎  time to create 1 commitments: ', (end - start) * 0.001 , 'seconds ⏱︎')

    // ===CREATE PAYMENT COMMITMENT===
    start = new Date()
    let res2 = await alice.graphQL(`
      mutation($c: CommitmentCreateParams!) {
        commitment: createCommitment(commitment: $c) {
          commitment {
            id
          }
        }
      }
    `, {
      c: {
        independentDemandOf: planId,
        // plannedWithin: planId,
        note: 'linked commitment 2',
        due: new Date(),
        clauseOf: createResp.data.res.agreement.id,
        ...testCommitmentProps,
      },
    })
    end = new Date()
    console.log('⏱︎  time to create 1 commitments: ', (end - start) * 0.001 , 'seconds ⏱︎')
    console.log('commitment: ', res2.data.commitment)

    // ===QUERY COMMITMENTS===
    start = new Date()
    async function fetchFullPlan() {
      return await alice.graphQL(`
        query($id: ID!) {
          plan(id: $id) {
            nonProcessCommitments {
              id
              note
              fulfilledBy {
                fulfilledBy {
                  id
                }
              }
              clauseOf {
                  id
                  name
                  note
                  revisionId
                  commitments {
                      finished
                      fulfilledBy {
                          id
                      }
                      id
                      revisionId
                      providerId
                      receiverId
                      due
                  }
              }
            }
          }
        }
      `, {
        id: planId,
      })
    }
    let planResp = await fetchFullPlan()
    end = new Date()
    console.log('⏱︎  time to query commitments:', (end - start) * 0.001, 'seconds ⏱︎')
    console.log('plan: ', JSON.stringify(planResp.data.plan))

    // ===EDIT COMMITMENT TO HAVE NO CLAUSE OF===
    console.log(JSON.stringify(res1))

    async function deleteAndCreate() {

      // delete agreement
      await alice.graphQL(`
        mutation {
          deleteAgreement(revisionId: "${createResp.data.res.agreement.revisionId}")
        }
      `)

      // set commitment to have no clauseOf
      let res3 = await alice.graphQL(`
        mutation($c: CommitmentUpdateParams!) {
          commitment: updateCommitment(commitment: $c) {
            commitment {
              id
              clauseOf {
                id
              }
            }
          }
        }
      `, {
        c: {
          revisionId: res1.commitment.commitment.revisionId,
          clauseOf: null,
        },
      })
      console.log('updated commitment: ', res3.data.commitment)


      // create new agreement
      createResp = await alice.graphQL(
        `
        mutation($rs: AgreementCreateParams!) {
          res: createAgreement(agreement: $rs) {
            agreement {
              id
              revisionId
            }
          }
        }
      `,
        {
          rs: {
            name: 'test agreement',
            created: new Date(),
            note: 'just testing, nothing was rly agreed',
          },
        },
      )

      // add new agreement to commitment
      await alice.graphQL(`
        mutation($c: CommitmentUpdateParams!) {
          commitment: updateCommitment(commitment: $c) {
            commitment {
              id
            }
          }
        }
      `, {
        c: {
          revisionId: res1.commitment.commitment.revisionId,
          clauseOf: createResp.data.res.agreement.id,
        },
      })

      // create new payment
      await alice.graphQL(`
        mutation($c: CommitmentCreateParams!) {
          commitment: createCommitment(commitment: $c) {
            commitment {
              id
            }
          }
        }
      `, {
        c: {
          independentDemandOf: planId,
          // plannedWithin: planId,
          note: 'linked commitment 3',
          due: new Date(),
          clauseOf: createResp.data.res.agreement.id,
          ...testCommitmentProps,
        },
      })
    }

    await deleteAndCreate()
    await deleteAndCreate()
    await deleteAndCreate()
    
    // set commitment to have no clauseOf
    let res3 = await alice.graphQL(`
      mutation($c: CommitmentUpdateParams!) {
        commitment: updateCommitment(commitment: $c) {
          commitment {
            id
            clauseOf {
              id
            }
          }
        }
      }
    `, {
      c: {
        revisionId: res1.commitment.commitment.revisionId,
        clauseOf: null,
      },
    })

    let planResp2 = await fetchFullPlan()
    console.log('plan: ', JSON.stringify(planResp2.data.plan))

} catch (e) {
    await alice.scenario.cleanUp()
    throw e
  }
  await alice.scenario.cleanUp()
})