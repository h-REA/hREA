const {
  getDNA,
  buildConfig,
  buildRunner,
  buildPlayer,
} = require('../init')

const runner = buildRunner()

const config = buildConfig()
const config2 = buildConfig()

runner.registerScenario('Agent registration API (happ-agent-registration module)', async (s, t) => {
  const { cells: [alice] } = await buildPlayer(s, config, ['agent'])
  const aliceAddr = alice.instance('agents').agentAddress  // :TODO: update for latest tryorama

  await s.consistency()

  let resp = await alice.call('agents', 'agent_registration', 'get_registered_agents', {})
  t.equal(resp.Ok[0], aliceAddr, 'querying agent is included in registered agent list as they themselves are accessing')
  t.equal(resp.Ok.length, 1, 'only single agent is returned')

  resp = await alice.call('agents', 'agent_registration', 'get_registered_agents', {})
  t.equal(resp.Ok.length, 1, 'agent is only recorded once')

  resp = await alice.call('agents', 'agent_registration', 'is_registered', { pubKey: aliceAddr })
  t.equal(resp.Ok, true, 'can check own registration status')

  resp = await alice.call('agents', 'agent_registration', 'is_registered', { pubKey: 'blablabla' })
  t.equal(resp.Ok, false, 'can check other registration statuses')

  // Load Bob
  const { cells: [bob] } = await buildPlayer(s, config2, ['agent'])
  const bobAddr = bob.instance('agents').agentAddress

  // Bob hits the DNA for the first time
  resp = await bob.call('agents', 'agent_registration', 'get_registered_agents', {})

  await s.consistency()

  resp = await alice.call('agents', 'agent_registration', 'is_registered', { pubKey: bobAddr })
  t.equal(resp.Ok, true, 'other agents detected after they have accessed')

  resp = await alice.call('agents', 'agent_registration', 'get_registered_agents', {})
  t.equal(resp.Ok.length, 2, 'new agents are recorded')
})

runner.run()