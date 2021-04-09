const {
  buildConfig,
  buildRunner,
  buildPlayer,
  mockAgentId,
} = require('../init')

const runner = buildRunner()

const config = buildConfig()

runner.registerScenario('create simplest event', async (s, t) => {
  const { cells: [alice] } = await buildPlayer(s, config, ['observation'])

  const event = {
    note: 'test event',
    action: 'raise',
    provider: mockAgentId(false),
    receiver: mockAgentId(false),
    hasPointInTime: '2019-11-19T12:12:42.739+01:00',
    resourceClassifiedAs: ['some-resource-type'],
    resourceQuantity: { hasNumericalValue: 1 },
    inScopeOf: ['some-accounting-scope'],
  }

  const createEventResponse = await alice.call('economic_event', 'create_event', { event })
  await s.consistency()

  t.ok(createEventResponse.economicEvent, 'event created')
  t.deepEqual(createEventResponse.economicEvent.inScopeOf, ['some-accounting-scope'], 'event inScopeOf saved')
})

runner.run()
