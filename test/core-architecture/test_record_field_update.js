const {
  getDNA,
  buildConfig,
  buildRunner,
} = require('../init')

const runner = buildRunner()

const config = buildConfig({
  observation: getDNA('observation'),
})

const testEventProps = {
  resourceClassifiedAs: ['some-resource-type'],
  resourceQuantity: { hasNumericalValue: 1, hasUnit: 'dangling-unit-todo-tidy-up' },
  provider: 'agentid-1-todo',
  receiver: 'agentid-2-todo',
  hasPointInTime: '2019-11-19T04:29:55.056Z',
}

runner.registerScenario('updates with fields ommitted leave original value intact', async (s, t) => {
  const { observation } = await s.players({ observation: config }, true)

  const event = {
    note: 'test event',
    action: 'raise',
    ...testEventProps,
  }

  const createEventResponse = await observation.call('observation', 'economic_event', 'create_economic_event', { event })
  t.ok(createEventResponse.Ok.economicEvent && createEventResponse.Ok.economicEvent.id, 'record created successfully')
  await s.consistency()

  const updateEventResponse = await observation.call('observation', 'economic_event', 'update_economic_event', {
    event: {
      id: createEventResponse.Ok.economicEvent.id,
    },
  })
  await s.consistency()

  const readResponse = await observation.call('observation', 'economic_event', 'get_economic_event', { address: createEventResponse.Ok.economicEvent.id })
  t.equal(readResponse.Ok.economicEvent.note, 'test event', 'field remains if not provided')
})

runner.registerScenario('updates with fields nulled remove original value', async (s, t) => {
  const { observation } = await s.players({ observation: config }, true)

  const event = {
    note: 'test event 2',
    action: 'raise',
    ...testEventProps,
  }

  const createEventResponse = await observation.call('observation', 'economic_event', 'create_economic_event', { event })
  t.ok(createEventResponse.Ok.economicEvent && createEventResponse.Ok.economicEvent.id, 'record created successfully')
  await s.consistency()

  const updateEventResponse = await observation.call('observation', 'economic_event', 'update_economic_event', {
    event: {
      id: createEventResponse.Ok.economicEvent.id,
      action: 'raise',
      note: null,
    },
  })
  await s.consistency()

  const readResponse = await observation.call('observation', 'economic_event', 'get_economic_event', { address: createEventResponse.Ok.economicEvent.id })
  t.equal(readResponse.Ok.economicEvent.note, undefined, 'field removed if nulled')
})

runner.run()
