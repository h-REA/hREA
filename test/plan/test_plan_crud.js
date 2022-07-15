import test from 'tape'
import { pause } from '@holochain/tryorama'
import {
  buildPlayer,
} from '../init.js'

const exampleEntry = {
  name: 'test plan',
  created: new Date(),
  due: new Date(),
  note: 'just testing, nothing was rly planned',
}
const example2Entry = {
  name: 'test plan 2',
  created: new Date(),
  due: new Date(),
  note: 'just testing 2, nothing was rly planned',
}
const updatedExampleEntry = {
  name: 'updated plan',
  created: new Date(Date.now() + 3600000),
  due: new Date(Date.now() + 3600000),
  note: 'updated the plan to something else',
}

test('Plan record API', async (t) => {
  const alice = await buildPlayer(['plan'])
  try {
    let createResp = await alice.graphQL(`
      mutation($rs: PlanCreateParams!, $rs2: PlanCreateParams!) {
        res: createPlan(plan: $rs) {
          plan {
            id
            revisionId
          }
        }
        res2: createPlan(plan: $rs2) {
          plan {
            id
            revisionId
          }
        }
      }
    `, {
      rs: exampleEntry,
      rs2: example2Entry,
    })
    await pause(100)
    t.ok(createResp.data.res.plan.id, 'record created')
    const pId = createResp.data.res.plan.id
    const p2Id = createResp.data.res2.plan.id
    const r1Id = createResp.data.res.plan.revisionId

    let getResp = await alice.graphQL(`
      query($id: ID!) {
        res: plan(id: $id) {
          id
          revisionId
          name
          created
          due
          note
        }
      }
    `, {
      id: pId,
    })
    t.deepLooseEqual(getResp.data.res, { 'id': pId, revisionId: r1Id, ...exampleEntry }, 'record read OK')

    const queryAllPlans = await alice.graphQL(`
      query {
        res: agreements {
          edges {
            node {
              id
            }
          }
        }
      }
    `,
    )
    t.equal(queryAllPlans.data.res.edges.length, 2, 'query for all plans OK')
    t.deepEqual(queryAllPlans.data.res.edges[1].node.id, pId, 'query for all plans, first plan in order OK')
    t.deepEqual(queryAllPlans.data.res.edges[0].node.id, p2Id, 'query for all plans, second plan in order OK')

    const updateResp = await alice.graphQL(`
      mutation($rs: PlanUpdateParams!) {
        res: updatePlan(plan: $rs) {
          plan {
            id
            revisionId
          }
        }
      }
    `, {
      rs: { revisionId: r1Id, ...updatedExampleEntry },
    })
    await pause(100)
    t.equal(updateResp.data.res.plan.id, pId, 'record updated')
    const r2Id = updateResp.data.res.plan.revisionId

    // now we fetch the Entry again to check that the update was successful
    const updatedGetResp = await alice.graphQL(`
      query($id: ID!) {
        res: plan(id: $id) {
          id
          revisionId
          created
          due
          name
          note
        }
      }
    `, {
      id: pId,
    })
    t.deepLooseEqual(updatedGetResp.data.res, { id: pId, revisionId: r2Id, created: exampleEntry.created, ...updatedExampleEntry }, 'record updated OK')

    const deleteResult = await alice.graphQL(`
      mutation($id: ID!) {
        res: deletePlan(revisionId: $id)
      }
    `, {
      id: r2Id,
    })
    await pause(100)
    t.equal(deleteResult.data.res, true)

    const queryForDeleted = await alice.graphQL(`
      query($id: ID!) {
        res: plan(id: $id) {
          id
        }
      }
    `, {
      id: pId,
    })
    t.equal(queryForDeleted.errors.length, 1, 'querying deleted record is an error')
    t.notEqual(-1, queryForDeleted.errors[0].message.indexOf('No entry at this address'), 'correct error reported')
  } catch (e) {
    await alice.scenario.cleanUp()
    throw e
  }
  await alice.scenario.cleanUp()
})
