const {
  buildConfig,
  buildRunner,
  buildPlayer,
} = require('../init')

const runner = buildRunner()

const config = buildConfig()

const exampleEntry = {
  name: 'test plan',
  created: new Date(),
  due: new Date(),
  note: 'just testing, nothing was rly planned',
}
const updatedExampleEntry = {
  name: 'updated plan',
  created: new Date(Date.now() + 3600000),
  due: new Date(Date.now() + 3600000),
  note: 'updated the plan to something else',
}

runner.registerScenario('Plan record API', async (s, t) => {
  const alice = await buildPlayer(s, config, ['plan'])

  let createResp = await alice.graphQL(`
    mutation($rs: PlanCreateParams!) {
      res: createPlan(plan: $rs) {
        plan {
          id
          revisionId
        }
      }
    }
  `, {
    rs: exampleEntry,
  })
  await s.consistency()
  console.log(createResp)
  t.ok(createResp.data.res.plan.id, 'record created')
  const aId = createResp.data.res.plan.id
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
    id: aId,
  })
  console.log('get response:', getResp)
  t.deepEqual(getResp.data.res, { 'id': aId, revisionId: r1Id, ...exampleEntry }, 'record read OK')

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
  await s.consistency()
  t.equal(updateResp.data.res.plan.id, aId, 'record updated')
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
    id: aId,
  })
  t.deepEqual(updatedGetResp.data.res, { id: aId, revisionId: r2Id, created: exampleEntry.created, ...updatedExampleEntry }, 'record updated OK')

  const deleteResult = await alice.graphQL(`
    mutation($id: ID!) {
      res: deletePlan(revisionId: $id)
    }
  `, {
    id: r2Id,
  })
  await s.consistency()
  console.log('delete id:', r2Id)
  t.equal(deleteResult.data.res, true)

  const queryForDeleted = await alice.graphQL(`
    query($id: ID!) {
      res: plan(id: $id) {
        id
      }
    }
  `, {
    id: aId,
  })
  t.equal(queryForDeleted.errors.length, 1, 'querying deleted record is an error')
  t.notEqual(-1, queryForDeleted.errors[0].message.indexOf('No entry at this address'), 'correct error reported')
})

runner.run()
