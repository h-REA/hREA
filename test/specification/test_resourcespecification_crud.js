const {
  getDNA,
  buildConfig,
  buildRunner,
  buildGraphQL,
} = require('../init')

const runner = buildRunner()

const config = buildConfig({
  specification: getDNA('specification'),
}, {})

const exampleEntry = {
  name: 'TRE',
  image: 'https://holochain.org/something',
  note: 'test resource specification',
}
const updatedExampleEntry = {
  name: 'QUA',
  image: 'https://holochain.org/something-else',
  note: 'test resource specification updated',
}

runner.registerScenario('ResourceSpecification record API', async (s, t) => {
  const { alice } = await s.players({ alice: config }, true)
  alice.graphQL = buildGraphQL(alice)

  let createResp = await alice.graphQL(`
    mutation($rs: ResourceSpecificationCreateParams!) {
      res: createResourceSpecification(resourceSpecification: $rs) {
        resourceSpecification {
          id
        }
      }
    }
    `, {
    rs: exampleEntry,
  })
  await s.consistency()

  t.ok(createResp.data.res.resourceSpecification.id, 'record created')
  const rsId = createResp.data.res.resourceSpecification.id

  let getResp = await alice.graphQL(`
    query($id: ID!) {
      res: resourceSpecification(id: $id) {
        id,
        name,
        image,
        note
      }
    }
    `, {
    id: rsId,
  })

  t.deepEqual(getResp.data.res, { id: rsId, ...exampleEntry }, 'record read OK')

  await alice.graphQL(`
    mutation($rs: ResourceSpecificationUpdateParams!) {
      res: updateResourceSpecification(resourceSpecification: $rs) {
        resourceSpecification {
          id
        }
      }
    }
    `, {
    rs: updatedExampleEntry,
  })
  await s.consistency()

  t.equal(createResp.data.res.resourceSpecification.id, rsId, 'record update OK')

  // now we fetch the Entry again to check that the update was successful
  let updatedGetResp = await alice.graphQL(`
    query($id: ID!) {
      res: resourceSpecification(id: $id) {
        id,
        name,
        image,
        note
      }
    }
  `, {
    id: rsId,
  })

  t.deepEqual(updatedGetResp.data.res, { id: rsId, ...updatedExampleEntry }, 'record properties updated')

  const deleteResult = await alice.graphQL(`
    mutation($id: String!) {
      res: deleteResourceSpecification(id: $id)
    }
  `, {
    id: rsId,
  })
  await s.consistency()

  t.equal(deleteResult.data.res, true)

  let queryForDeleted = await alice.graphQL(`
    query($id: ID!) {
      res: resourceSpecification(id: $id) {
        id,
        name,
        image,
        note
      }
    }
  `, {
    id: rsId,
  })

  t.equal(queryForDeleted.errors.length, 1, 'querying deleted record is an error')
  t.notEqual(-1, queryForDeleted.errors[0].message.indexOf('No entry at this address'), 'correct error reported')
})

runner.run()
