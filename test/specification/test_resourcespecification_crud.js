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
  id: "QmUZTB77gxvSuGaWqurHpKrU6oRrw4Hg8AGG1wtAe8Fzhp",
  name: 'QUA',
  image: 'https://holochain.org/something-else',
  note: 'test resource specification updated',
}

runner.registerScenario('ResourceSpecification record API', async (s, t) => {
  const { alice } = await s.players({ alice: config }, true)
  alice.graphQL = buildGraphQL(alice, t)
  let createResp = await alice.graphQL(`
    mutation($rs: ResourceSpecificationCreateParams!) {
      res: createResourceSpecification(resourceSpecification: $rs) {
        resourceSpecification {
          id
        }
      }
    }
    `, {
      rs:exampleEntry ,
  })
  t.deepEqual(createResp.data.res, {"resourceSpecification":{"id":"QmUZTB77gxvSuGaWqurHpKrU6oRrw4Hg8AGG1wtAe8Fzhp"}})// create test
  await s.consistency()
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
      id: "QmUZTB77gxvSuGaWqurHpKrU6oRrw4Hg8AGG1wtAe8Fzhp"
  })
  t.deepEqual(getResp.data.res, { "id": "QmUZTB77gxvSuGaWqurHpKrU6oRrw4Hg8AGG1wtAe8Fzhp", ...exampleEntry })// read test

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

  t.deepEqual(createResp.data.res, {
    "resourceSpecification": {
      "id": "QmUZTB77gxvSuGaWqurHpKrU6oRrw4Hg8AGG1wtAe8Fzhp"
    }
  })// update test
  await s.consistency()
  //now we fetch the Entry again to check that the update was successful
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
      id: "QmUZTB77gxvSuGaWqurHpKrU6oRrw4Hg8AGG1wtAe8Fzhp"
  })
  t.deepEqual(updatedGetResp.data.res, updatedExampleEntry)//check Entry being updated

  const deleteResult = await alice.graphQL(`
  mutation($id: String!) {
    res: deleteResourceSpecification(id: $id)
  }
  `, {
    id: updatedExampleEntry.id,
  })
  t.equal(deleteResult.data.res,true)

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
      id: "QmUZTB77gxvSuGaWqurHpKrU6oRrw4Hg8AGG1wtAe8Fzhp"
  })
  // t.deepEqual(updatedGetResp.data.res, updatedExampleEntry)
  console.log('ranromStringToGrep:',JSON.stringify(queryForDeleted))
})

runner.run()
