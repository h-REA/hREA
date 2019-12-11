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
  console.log('front-response: ',JSON.stringify(resp),'===========================================================================')
  await s.consistency()
  t.deepEqual(resp.data.res, {"resourceSpecification":{"id":"QmUZTB77gxvSuGaWqurHpKrU6oRrw4Hg8AGG1wtAe8Fzhp"}})

  t.ok(resp, 'input resource specification created OK')

})

runner.run()
