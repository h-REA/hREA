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
  name: 'TPE',
  note: 'test process specification',
}
const updatedExampleEntry = {
  id: 'Qmeow6ozaLXV5nsuqAjQDjcQrugbJTdFb5Lv7R6iBUwQ8p',
  name: 'UPE',
  note: 'updated process specification',
}

runner.registerScenario('ProcessSpecification record API', async (s, t) => {
  const { alice } = await s.players({ alice: config }, true)
  alice.graphQL = buildGraphQL(alice)
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
  t.deepEqual(createResp.data.res, {'processSpecification': {'id': 'Qmeow6ozaLXV5nsuqAjQDjcQrugbJTdFb5Lv7R6iBUwQ8p'}})// create test
  await s.consistency()
  let getResp = await alice.graphQL(`
    query($id: ID!) {
      res: processSpecification(id: $id) {
        id,
        name,
        note
      }
    }
  `, {
    id: 'Qmeow6ozaLXV5nsuqAjQDjcQrugbJTdFb5Lv7R6iBUwQ8p',
  })
  t.deepEqual(getResp.data.res, { 'id': 'Qmeow6ozaLXV5nsuqAjQDjcQrugbJTdFb5Lv7R6iBUwQ8p', ...exampleEntry })// read test

  await alice.graphQL(`
    mutation($rs: ProcessSpecificationUpdateParams!) {
      res: updateProcessSpecification(processSpecification: $rs) {
        processSpecification {
          id
        }
      }
    }
  `, {
    rs: updatedExampleEntry,
  })
  await s.consistency()

  t.deepEqual(createResp.data.res, {
    'processSpecification': {
      'id': 'Qmeow6ozaLXV5nsuqAjQDjcQrugbJTdFb5Lv7R6iBUwQ8p',
    },
  })// update test
  await s.consistency()
  // now we fetch the Entry again to check that the update was successful
  let updatedGetResp = await alice.graphQL(`
    query($id: ID!) {
      res: processSpecification(id: $id) {
        id,
        name,
        note
      }
    }
  `, {
    id: 'Qmeow6ozaLXV5nsuqAjQDjcQrugbJTdFb5Lv7R6iBUwQ8p',
  })
  t.deepEqual(updatedGetResp.data.res, updatedExampleEntry)// check Entry being updated

  const deleteResult = await alice.graphQL(`
    mutation($id: String!) {
      res: deleteProcessSpecification(id: $id)
    }
  `, {
    id: 'Qmeow6ozaLXV5nsuqAjQDjcQrugbJTdFb5Lv7R6iBUwQ8p',
  })
  t.equal(deleteResult.data.res, true)
  await s.consistency()
  let queryForDeleted = await alice.graphQL(`
    query($id: ID!) {
      res: processSpecification(id: $id) {
        id,
        name,
        note
      }
    }
  `, {
    id: 'Qmeow6ozaLXV5nsuqAjQDjcQrugbJTdFb5Lv7R6iBUwQ8p',
  })
  t.equal(queryForDeleted.data.res, null)
})

runner.run()
