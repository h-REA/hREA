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

runner.registerScenario('ResourceSpecification record API', async (s, t) => {
  const { alice } = await s.players({ alice: config }, true)
  alice.graphQL = buildGraphQL(alice, t)

  let resp = await alice.graphQL(`
    mutation($rs: ResourceSpecificationCreateParams!) {
      res: createResourceSpecification(resourceSpecification: $rs) {
        resourceSpecification {
          id
        }
      }
    }
    `, {
      rs: {
      name: 'TRE',
      image: 'https://holochain.org/something',
      note: 'test resource specification',
    },
  })
  console.log(JSON.stringify(resp,null,2))
  console.log('response: ',resp,'===========================================================================')
  await s.consistency()

  t.ok(resp, 'input resource specification created OK')

  // t.equal(resp.data.inputEvent.fulfills.length, 1, 'input event fulfillment ref added')
})

runner.run()
