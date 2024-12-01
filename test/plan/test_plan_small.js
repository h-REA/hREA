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

    // ===CREATE 1 COMMITMENT====
    start = new Date()
    resp = await alice.graphQL(`
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
        plannedWithin: planId,
        note: 'linked commitment 1',
        due: new Date(Date.now() + 86400000),
        ...testCommitmentProps,
      },
    })
    end = new Date()
    console.log('⏱︎  time to create 1 commitment:', (end - start) * 0.001, 'seconds ⏱︎')
    // ===CREATE 1 COMMITMENT ENDS===

    // ===CREATE 4 COMMITMENTS AND 4 PROCESSES===
    // define async function to create a process and commitment
    const createProcessAndCommitment = async (processName, commitmentNote) => {
      const resp = await alice.graphQL(`
        mutation($p: ProcessCreateParams!, $c: CommitmentCreateParams!) {
          process: createProcess(process: $p) {
            process {
              id
            }
          }
          commitment: createCommitment(commitment: $c) {
            commitment {
              id
            }
          }
        }
      `, {
        p: {
          plannedWithin: planId,
          name: processName,
          note: 'linked process note 1',
        },
        c: {
          independentDemandOf: planId,
          plannedWithin: planId,
          note: commitmentNote,
          due: new Date(Date.now() + 86400000),
          ...testCommitmentProps,
        },
      })
      return resp.data.process.process.id
    }
    
    start = new Date()
    let processId1 = await createProcessAndCommitment('linked process name 1', 'linked commitment 1')
    t.ok(processId1, 'process 1 created')
    end = new Date()
    console.log('⏱︎  time to create 1 commitments and 1 processes:', (end - start) * 0.001 , 'seconds ⏱︎')

    // get process
    const start2 = new Date()
    resp = await alice.graphQL(`
      query($id: ID!) {
        process(id: $id) {
          id
        }
      } 
    `, { id: processId1 })

    const end2 = new Date()
    console.log('⏱︎  time to retrieve process 1:', (end2 - start2) * 0.001, 'seconds ⏱︎')
    

    // retrieve plan with processes and commitments
    start = new Date()
    resp = await alice.graphQL(`
      query($id: ID!) {
        plan(id: $id) {
          id
          name
          processes {
            id
            name
            note
            committedInputs {
              id
            }
          }
          nonProcessCommitments {
            id
            note
          }
        }
      }
    `, { id: planId })
    end = new Date()
    console.log('plan with processes and commitments:', resp)
    console.log('⏱︎  time to retrieve plan with processes and commitments:', (end - start) * 0.001, 'seconds ⏱︎')

    // fetch all plans
    start = new Date()
    resp = await alice.graphQL(`
      query {
        plans {
          edges {
            node {
              meta {
                retrievedRevision {
                  time
                }
              }
              id
              revisionId
              name
              note
            }
          }
        }
      }
    `)
    end = new Date()
    console.log('⏱︎  time to retrieve all plans:', (end - start) * 0.001, 'seconds ⏱︎')
  } catch (e) {
    await alice.scenario.cleanUp()
    throw e
  }
  await alice.scenario.cleanUp()
})
