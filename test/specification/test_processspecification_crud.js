const {
  buildConfig,
  buildRunner,
  buildPlayer,
} = require('../init')

const runner = buildRunner()

const config = buildConfig()

const exampleEntry = {
  name: 'TPE',
  note: 'test process specification',
}
const updatedExampleEntry = {
  name: 'UPE',
  note: 'updated process specification',
}

runner.registerScenario('ProcessSpecification record API', async (s, t) => {
  const alice = await buildPlayer(s, config, ['specification'])

  let createResp = await alice.graphQL(`
    mutation($rs: ProcessSpecificationCreateParams!) {
      res: createProcessSpecification(processSpecification: $rs) {
        processSpecification {
          id
        }
      }
    }
  `, {
    rs: exampleEntry,
  })
  await s.consistency()

  t.ok(createResp.data.res.processSpecification.id, 'record created')
  const psId = createResp.data.res.processSpecification.id
  const psRev = createResp.data.res.processSpecification.revisionId

  let getResp = await alice.graphQL(`
    query($id: ID!) {
      res: processSpecification(id: $id) {
        id
        name
        note
      }
    }
  `, {
    id: psId,
  })

  t.deepEqual(getResp.data.res, { 'id': psId, ...exampleEntry }, 'record read OK')

  const updateResp = await alice.graphQL(`
    mutation($rs: ProcessSpecificationUpdateParams!) {
      res: updateProcessSpecification(processSpecification: $rs) {
        processSpecification {
          id
        }
      }
    }
  `, {
    rs: { revisionId: psRev, ...updatedExampleEntry },
  })
  await s.consistency()

  t.equal(updateResp.data.res.processSpecification.id, psId, 'record updated')

  // now we fetch the Entry again to check that the update was successful
  const updatedGetResp = await alice.graphQL(`
    query($id: ID!) {
      res: processSpecification(id: $id) {
        id
        name
        note
      }
    }
  `, {
    id: psId,
  })
  t.deepEqual(updatedGetResp.data.res, { id: psId, ...updatedExampleEntry }, 'record updated OK')

  const deleteResult = await alice.graphQL(`
    mutation($id: ID!) {
      res: deleteProcessSpecification(id: $id)
    }
  `, {
    id: psId,
  })
  await s.consistency()

  t.equal(deleteResult.data.res, true)

  const queryForDeleted = await alice.graphQL(`
    query($id: ID!) {
      res: processSpecification(id: $id) {
        id
      }
    }
  `, {
    id: psId,
  })

  t.equal(queryForDeleted.errors.length, 1, 'querying deleted record is an error')
  t.notEqual(-1, queryForDeleted.errors[0].message.indexOf('No entry at this address'), 'correct error reported')
})

runner.run()
