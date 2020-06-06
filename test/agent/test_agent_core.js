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
  const aliceAddr = alice.instance('agent').agentAddress

  let res = await alice.graphQL(`{
    myAgent {
      id
      name
    }
  }`)

  // :TODO: register personas & proof sharing with the network

  t.ok(res.data.myAgent.id, 'agent A can retrieve own agent ID')
  t.ok(res.data.myAgent.name, 'agent A can retrieve own agent name')

  res = await alice.graphQL(`{
    agents {
      id
      name
    }
  }`)

  t.equal(res.data.agents.length, 1, 'can load agents list')
  t.equal(res.data.agents[0].id, aliceAddr, 'own agent ID returned in list')

  const bob = await buildPlayer(s, 'bob', config)
  const bobAddr = bob.instance('agent').agentAddress

  res = await bob.graphQL(`{
    myAgent {
      id
      name
    }
  }`)
  t.ok(res.data.myAgent.id, 'agent B can retrieve own agent ID')

  await s.consistency() // wait for Bob's join to propagate to Alice

  res = await alice.graphQL(`{
    agents {
      id
      name
    }
  }`)
  t.equal(res.data.agents.length, 2, 'can load agents list after second agent joins')
  t.equal(res.data.agents[1].id, aliceAddr, 'own agent ID returned in list')
  t.equal(res.data.agents[0].id, bobAddr, 'new agent ID returned in list')

  res = await alice.graphQL(`{
    agent(id: "${bobAddr}") {
      id
      name
    }
  }`)
  t.equal(res.data.agent.id, bobAddr, 'can load other agent details by ID')
})

runner.run()
