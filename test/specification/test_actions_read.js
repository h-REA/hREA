// :NOTE: this is a minimal test- actual actions are tested in Rust unit tests

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

runner.registerScenario('Built-in action API', async (s, t) => {
  const { alice } = await s.players({ alice: config }, true)
  alice.graphQL = buildGraphQL(alice)

  const queryAllResp = await alice.graphQL(`
    {
      allActions {
        id
      }
    }
  `, {})

  t.equal(queryAllResp.data.allActions.length, 18, 'all action builtins present')

  const getResp = await alice.graphQL(`
    query($id: ID!) {
      action(id: $id) {
        id
        label
        resourceEffect
        inputOutput
        pairsWith
      }
    }
  `, {
    id: 'raise',
  })

  t.deepEqual(getResp.data.action, {
    id: 'raise',
    label: 'raise',
    resourceEffect: 'increment',
    inputOutput: 'notApplicable',
    pairsWith: 'notApplicable',
  }, 'record read OK')
})

runner.run()
