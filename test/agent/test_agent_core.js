const {
  getDNA,
  buildConfig,
  buildRunner,
  buildPlayer,
} = require('../init')

const runner = buildRunner()

const config = buildConfig({
  agent: getDNA('agent'),
}, {
})

runner.registerScenario('REA economic agent functionality', async (s, t) => {
  const alice = await buildPlayer(s, 'alice', config)

  const agentData = await alice.graphQL(`{
    myAgent {
      id
    }
  }`)

  t.ok(agentData.data.myAgent.id, 'can retrieve own agent ID')
})

runner.run()
