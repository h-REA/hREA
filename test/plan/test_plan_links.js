import test from 'tape'
import { pause } from '@connoropolous/tryorama'
import { extractEdges } from '@valueflows/vf-graphql-holochain/build/connection.js'
import {
  buildPlayer,
  mockIdentifier,
  mockAgentId,
  sortById,
} from '../init.js'

const testCommitmentProps = {
  action: 'raise',
  resourceClassifiedAs: ['some-resource-type'],
  resourceQuantity: { hasNumericalValue: 1, hasUnit: mockIdentifier() },
  provider: mockIdentifier(true),
  receiver: mockIdentifier(true),
}

test('Plan links & queries', async (t) => {
  const alice = await buildPlayer(['observation', 'planning', 'plan'])

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

  // resp = await alice.graphQL(`
  //   mutation($p: ProcessCreateParams!) {
  //     process: createProcess(process: $p) {
  //       process {
  //         id
  //       }
  //     }
  //   }
  // `, {
  //   p: {
  //     plannedWithin: planId,
  //     name: 'linked process name 1',
  //     note: 'linked process note 1',
  //   },
  // })
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
      // plannedWithin: planId,
      note: 'linked commitment 1',
      due: new Date(Date.now() + 86400000),
      ...testCommitmentProps,
    },
  })
  await pause(100)
  console.log('response:', resp)
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
      }
      plan(id: "${planId}") {
        independentDemands {
          id
        }
        processes {
          edges {
            node {
              id
            }
          }
        }
      }
    }
  `,
  )
  t.equal(resp.data.process.plannedWithin.id, planId, 'process -> plan ref OK')
  t.equal(resp.data.commitment.independentDemandOf.id, planId, 'commitment -> plan ref OK')
  // t.equal(resp.data.commitment.plannedWithin.id, planId, 'commitment -> plan ref OK')
  t.equal(resp.data.plan.independentDemands.length, 1, 'commitment ref added')
  t.equal(resp.data.plan.independentDemands[0].id, cId, 'commitment ref OK')
  t.equal(resp.data.plan.processes.edges.length, 1, 'process ref added')
  t.equal(resp.data.plan.processes.edges[0].node.id, pId, 'process ref OK')

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
          edges {
            node {
              id
            }
          }
        }
      }
    }
  `)

  // :TODO: remove client-side sorting when deterministic time-ordered indexing is implemented
  const sortedCIds = [{ id: cId }, { id: c2Id }].sort(sortById)
  resp.data.plan.independentDemands.sort(sortById)
  const sortedPIds = [{ id: pId }, { id: p2Id }].sort(sortById)
  let processes = extractEdges(resp.data.plan.processes).sort(sortById)

  t.equal(resp.data.plan.independentDemands.length, 2, '2nd commitment ref added')
  t.equal(resp.data.plan.independentDemands[0].id, sortedCIds[0].id, 'commitment ref 1 OK')
  t.equal(resp.data.plan.independentDemands[1].id, sortedCIds[1].id, 'commitment ref 2 OK')
  t.equal(resp.data.plan.processes.edges.length, 2, '2nd event ref added')
  t.equal(processes.length, 2, '2nd event ref added')
  t.equal(processes[0].id, sortedPIds[0].id, 'process ref 1 OK')
  t.equal(processes[1].id, sortedPIds[1].id, 'process ref 2 OK')

  await alice.scenario.cleanUp()
})
