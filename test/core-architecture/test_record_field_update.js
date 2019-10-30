const {
  getDNA,
  buildConfig,
  buildRunner,
} = require('../init')

const runner = buildRunner()

const config = buildConfig({
  observation: getDNA('observation'),
})

runner.registerScenario('updates with fields ommitted leave original value intact', async (s, t) => {
  const { observation } = await s.players({ observation: config }, true)

  const event = {
    note: 'test event',
    action: 'produce',
  }

  const createEventResponse = await observation.call('observation', 'economic_event', 'create_event', { event })
  t.ok(createEventResponse.Ok.economicEvent && createEventResponse.Ok.economicEvent.id, 'record created successfully')
  await s.consistency()

  const updateEventResponse = await observation.call('observation', 'economic_event', 'update_event', {
    event: {
      id: createEventResponse.Ok.economicEvent.id,
    },
  })
  await s.consistency()

  const readResponse = await observation.call('observation', 'economic_event', 'get_event', { address: createEventResponse.Ok.economicEvent.id })
  t.equal(readResponse.Ok.economicEvent.note, 'test event', 'field remains if not provided')
})

runner.registerScenario('updates with fields nulled remove original value', async (s, t) => {
  const { observation } = await s.players({ observation: config }, true)

  const event = {
    note: 'test event 2',
    action: 'produce',
  }

  const createEventResponse = await observation.call('observation', 'economic_event', 'create_event', { event })
  t.ok(createEventResponse.Ok.economicEvent && createEventResponse.Ok.economicEvent.id, 'record created successfully')
  await s.consistency()

  const updateEventResponse = await observation.call('observation', 'economic_event', 'update_event', {
    event: {
      id: createEventResponse.Ok.economicEvent.id,
      action: 'produce',
      note: null,
    },
  })
  await s.consistency()

  const readResponse = await observation.call('observation', 'economic_event', 'get_event', { address: createEventResponse.Ok.economicEvent.id })
  t.equal(readResponse.Ok.economicEvent.note, undefined, 'field removed if nulled')
})

runner.run()
