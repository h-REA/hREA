const {
  buildConfig,
  buildRunner,
  buildPlayer,
} = require('../init')

const runner = buildRunner()

const config = buildConfig()

runner.registerScenario('create simplest event', async (s, t) => {
  const { cells: [alice] } = await buildPlayer(s, config, ['observation'])

  const event = {
    note: 'test event',
    action: 'raise',
    provider: 'todo-some-agent-id',
    receiver: 'todo-some-agent-id',
    hasPointInTime: '2019-11-19T12:12:42.739Z',
    resourceClassifiedAs: ['some-resource-type'],
    resourceQuantity: { hasNumericalValue: 1 },
    inScopeOf: ['some-accounting-scope'],
  }

  const createEventResponse = await alice.call('observation', 'economic_event', 'create_event', { event })
  await s.consistency()

  t.ok(createEventResponse.Ok.economicEvent, 'event created')
  t.deepEqual(createEventResponse.Ok.economicEvent.inScopeOf, ['some-accounting-scope'], 'event inScopeOf saved')
})

runner.run()
