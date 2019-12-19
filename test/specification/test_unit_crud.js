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
  label: 'TRE',
  symbol: 'https://holochain.org/something',
}
const updatedExampleEntry = {
  id: "QmdQHWnViFiLezMCPjs1KGjB9XJ7ijwfvLCpdgMuVKR9pc",
  label: 'QUA',
  symbol: 'https://holochain.org/something-else',
}

runner.registerScenario('Unit record API', async (s, t) => {
  const { alice } = await s.players({ alice: config }, true)
  alice.graphQL = buildGraphQL(alice)
  let createResp = await alice.graphQL(`
    mutation($rs: UnitCreateParams!) {
      res: createUnit(unit: $rs) {
        unit {
          id
        }
      }
    }
    `, {
      rs:exampleEntry ,
  })
  t.deepEqual(createResp.data.res, {"unit":{"id":"QmdQHWnViFiLezMCPjs1KGjB9XJ7ijwfvLCpdgMuVKR9pc"}})// create test
  await s.consistency()
  let getResp = await alice.graphQL(`
    query($id: ID!) {
      res: unit(id: $id) {
        id,
        label,
        symbol
      }
    }
    `, {
      id: "QmdQHWnViFiLezMCPjs1KGjB9XJ7ijwfvLCpdgMuVKR9pc"
  })

  t.deepEqual(getResp.data.res, { "id": "QmdQHWnViFiLezMCPjs1KGjB9XJ7ijwfvLCpdgMuVKR9pc", ...exampleEntry })// read test

  await alice.graphQL(`
    mutation($rs: UnitUpdateParams!) {
      res: updateUnit(unit: $rs) {
        unit {
          id
        }
      }
    }
    `, {
      rs: updatedExampleEntry,
  })
  await s.consistency()
  t.deepEqual(createResp.data.res, {
    "unit": {
      "id": "QmdQHWnViFiLezMCPjs1KGjB9XJ7ijwfvLCpdgMuVKR9pc"
    }
  })// update test
  await s.consistency()
  //now we fetch the Entry again to check that the update was successful
  let updatedGetResp = await alice.graphQL(`
    query($id: ID!) {
      res: unit(id: $id) {
        id,
        label,
        symbol
      }
    }
  `, {
    id: "QmdQHWnViFiLezMCPjs1KGjB9XJ7ijwfvLCpdgMuVKR9pc"
  })
  t.deepEqual(updatedGetResp.data.res, updatedExampleEntry)//check Entry being updated

  const deleteResult = await alice.graphQL(`
    mutation($id: String!) {
      res: deleteUnit(id: $id)
    }
  `, {
    id: "QmdQHWnViFiLezMCPjs1KGjB9XJ7ijwfvLCpdgMuVKR9pc",
  })
  t.equal(deleteResult.data.res,true)
  await s.consistency()

  let queryForDeleted = await alice.graphQL(`
    query($id: ID!) {
      res: unit(id: $id) {
        id,
        label,
        symbol
      }
    }
  `, {
    id: "QmdQHWnViFiLezMCPjs1KGjB9XJ7ijwfvLCpdgMuVKR9pc"
  })
  t.equal(queryForDeleted.data.res,null)
})

runner.run()
