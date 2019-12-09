const {
  getDNA,
  buildConfig,
  buildRunner,
} = require('../init')

const runner = buildRunner()

const config = buildConfig({
  observation: getDNA('observation'),
})

runner.registerScenario('create simplest event', async (s, t) => {
  const { alice } = await s.players({ alice: config }, true)

  const event = {
    note: 'test event',
    action: 'raise',
    provider: 'todo-some-agent-id',
    receiver: 'todo-some-agent-id',
    hasPointInTime: '2019-11-19T12:12:42.739Z',
    resourceClassifiedAs: ['some-resource-type'],
    resourceQuantity: { hasNumericalValue: 1 },
  }

  const createEventResponse = await alice.call('observation', 'economic_event', 'create_event', { event })
  await s.consistency()

  t.ok(createEventResponse.Ok.economicEvent)
})

runner.run()
