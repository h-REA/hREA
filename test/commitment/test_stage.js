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

    // ===CREATE 1 COMMITMENTS AND 1 PROCESSES===
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
    end = new Date()

    console.log('⏱︎  time to create 1 commitments and 1 processes:', (end - start) * 0.001 , 'seconds ⏱︎')

    // ===ADD ORGANIZATION===
    const exampleOrganization = {
      name: 'test organization',
      image: 'https://org.png',
      classifiedAs: ['org'],
      note: 'test organization note',
    }
    start = new Date()
    let createOrgResp = await alice.graphQL(
      `
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
        rs: exampleOrganization,
      },
    )
    end = new Date()
    console.log('⏱︎  time to create organization:', (end - start) * 0.001, 'seconds ⏱︎')
    let organizationId = createOrgResp.data.res.agent.id

    // ===ADD UNIT DOLLAR===
    start = new Date()
    resp = await alice.graphQL(`
      mutation($u: UnitCreateParams!) {
        res: createUnit(unit: $u) {
          unit {
            id
          }
        }
      }
    `, {
      u: {
        label: 'US Dollar',
        symbol: 'USD',
      },
    })
    end = new Date()
    console.log('⏱︎  time to create unit:', (end - start) * 0.001, 'seconds ⏱︎')
    t.ok(resp.data.res.unit.id, 'unit created')
    const unitId = resp.data.res.unit.id

    // ===ADD RESOURCE SPECIFICATION===
    start = new Date()
    resp = await alice.graphQL(`
      mutation($rs: ResourceSpecificationCreateParams!) {
        res: createResourceSpecification(resourceSpecification: $rs) {
          resourceSpecification {
            id
          }
        }
      }
    `, {
      rs: {
        name: 'test resource specification',
        defaultUnitOfResource: unitId,
        note: 'test resource specification note',
      },
    })
    end = new Date()
    console.log("RS ID", resp.data.res.resourceSpecification.id)
    console.log('⏱︎  time to create resource specification:', (end - start) * 0.001, 'seconds ⏱︎')
    t.ok(resp.data.res.resourceSpecification.id, 'resource specification created')
    const resourceSpecificationId = resp.data.res.resourceSpecification.id

    // ===ADD INPUT AND OUTPUT COMMITMENTS TO PROCESS===
    const addInputAndOutputCommitments = async (processId) => {
      let com3 = {
        'action': 'pickup',
        'inputOf': processId,
        'stage': processId,
        'provider': organizationId,
        'receiver': organizationId,
        'due': '2019-11-19T04:29:55.056Z',
        'resourceQuantity': { hasNumericalValue: 1, hasUnit: unitId },
        'resourceClassifiedAs': ['some-resource-type'],
        'note': 'some input will be provided',
      }

      resp = await alice.graphQL(`
        mutation($c1: CommitmentCreateParams!) {
          commitment1: createCommitment(commitment: $c1) {
            commitment {
              id
            }
          }
        }
      `, {
        c1: com3,
      })
    }

    
    start = new Date()
    console.log("creating commitments", start)
    await addInputAndOutputCommitments(processId1)
    end = new Date()
    console.log("commitments created", end)
    console.log('⏱︎  time to add 4 input and 4 output commitments to 4 process:', (end - start) * 0.001, 'seconds ⏱︎')

    // ===RETRIEVE FULL PLAN===
    start = new Date()
    resp = await alice.graphQL(`
      query {
        plan(id: "${planId}") {
          independentDemands {
            id
          }
          processes {
            id
            revisionId
            name
            meta {
              retrievedRevision {
                id
                time
              }
            }
            committedInputs {
              id
              revisionId
              hasBeginning
              action {
                id
                label
              }
              meta {
                retrievedRevision {
                  id
                  time
                }
              }
              providerId
              provider {
                id
                name
              }
            }
            committedOutputs {
              id
              revisionId
              hasBeginning
              action {
                id
                label
              }
              meta {
                retrievedRevision {
                  id
                  time
                }
              }
              providerId
              provider {
                id
                name
              }
              receiverId
              resourceQuantity {
                hasNumericalValue
                hasUnitId
              }
            }
          }
        }
      }
    `,
    )
    console.log("==RESP==", JSON.stringify(resp))
    end = new Date()
  } catch (e) {
    await alice.scenario.cleanUp()
    throw e
  }
  await alice.scenario.cleanUp()
})
