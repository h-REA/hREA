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
  const alice = await buildPlayer(['observation', 'planning', 'plan'])
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
    await pause(100)
    t.ok(resp.data.res.plan.id, 'plan created')
    const planId = resp.data.res.plan.id

    resp = await alice.graphQL(`
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
        name: 'linked process name 1',
        note: 'linked process note 1',
      },
      c: {
        independentDemandOf: planId,
        plannedWithin: planId,
        note: 'linked commitment 1',
        due: new Date(Date.now() + 86400000),
        ...testCommitmentProps,
      },
    })
    await pause(100)
    t.ok(resp.data.process.process.id, 'process created')
    t.ok(resp.data.commitment.commitment.id, 'commitment created')
    const pId = resp.data.process.process.id
    const cId = resp.data.commitment.commitment.id

    resp = await alice.graphQL(`
      query {
        process(id: "${pId}") {
          plannedWithin {
            id
          }
        }
        commitment(id: "${cId}") {
          independentDemandOf {
            id
          }
          plannedWithin {
            id
          }
        }
        plan(id: "${planId}") {
          independentDemands {
            id
          }
          processes {
            id
          }
        }
      }
    `,
    )
    t.equal(resp.data.process.plannedWithin.id, planId, 'process -> plan ref OK')
    t.equal(resp.data.commitment.independentDemandOf.id, planId, 'commitment -> plan ref OK')
    t.equal(resp.data.commitment.plannedWithin.id, planId, 'commitment -> plan ref OK')
    t.equal(resp.data.plan.independentDemands.length, 1, 'commitment ref added')
    t.equal(resp.data.plan.independentDemands[0].id, cId, 'commitment ref OK')
    t.equal(resp.data.plan.processes.length, 1, 'process ref added')
    t.equal(resp.data.plan.processes[0].id, pId, 'process ref OK')

    resp = await alice.graphQL(`
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
        name: 'linked process name 2',
        note: 'linked process note 2',
      },
      c: {
        independentDemandOf: planId,
        // plannedWithin: planId, // not able to add this yet as this field is not currently defined in the CreateCommitmentParams object
        note: 'linked commitment 2',
        due: new Date(Date.now() + 86400000),
        ...testCommitmentProps,
      },
    })
    await pause(100)
    t.ok(resp.data.process.process.id, 'event 2 created')
    t.ok(resp.data.commitment.commitment.id, 'commitment 2 created')
    const p2Id = resp.data.process.process.id
    const c2Id = resp.data.commitment.commitment.id

    resp = await alice.graphQL(`
      query {
        plan(id: "${planId}") {
          independentDemands {
            id
          }
          processes {
            id
          }
        }
      }
    `)

    const sortedCIds = [{ id: c2Id }, { id: cId }]
    const sortedPIds = [{ id: p2Id }, { id: pId }]

    t.equal(resp.data.plan.independentDemands.length, 2, '2nd commitment ref added')
    t.equal(resp.data.plan.independentDemands[0].id, sortedCIds[0].id, 'commitment ref 1 OK')
    t.equal(resp.data.plan.independentDemands[1].id, sortedCIds[1].id, 'commitment ref 2 OK')
    t.equal(resp.data.plan.processes.length, 2, '2nd event ref added')
    t.equal(resp.data.plan.processes[0].id, sortedPIds[0].id, 'process ref 1 OK')
    t.equal(resp.data.plan.processes[1].id, sortedPIds[1].id, 'process ref 2 OK')
  } catch (e) {
    await alice.scenario.cleanUp()
    throw e
  }
  await alice.scenario.cleanUp()
})
