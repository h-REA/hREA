const {
  getDNA,
  buildOrchestrator,
} = require('../init')

const runner = buildOrchestrator({
  observation: getDNA('observation'),
})

runner.registerScenario('updates with fields ommitted leave original value intact', async (s, t, { observation }) => {
  const event = {
    note: 'test event',
  }

  const createEventResponse = await observation.call('economic_event', 'create_event', { event })

  t.ok(createEventResponse.Ok.economicEvent && createEventResponse.Ok.economicEvent.id, 'record created successfully')

  const updateEventResponse = await observation.call('economic_event', 'update_event', {
    event: {
      id: createEventResponse.Ok.economicEvent.id,
    },
  })

  await s.consistent()

  const readResponse = await observation.call('economic_event', 'get_event', { address: createEventResponse.Ok.economicEvent.id })

  t.equal(readResponse.Ok.economicEvent.note, 'test event', 'field remains if not provided')
})

runner.registerScenario('updates with fields nulled remove original value', async (s, t, { observation }) => {
  const event = {
    note: 'test event 2',
  }

  const createEventResponse = await observation.call('economic_event', 'create_event', { event })

  t.ok(createEventResponse.Ok.economicEvent && createEventResponse.Ok.economicEvent.id, 'record created successfully')

  const updateEventResponse = await observation.call('economic_event', 'update_event', {
    event: {
      id: createEventResponse.Ok.economicEvent.id,
      note: null,
    },
  })

  await s.consistent()

  const readResponse = await observation.call('economic_event', 'get_event', { address: createEventResponse.Ok.economicEvent.id })

  t.equal(readResponse.Ok.economicEvent.note, undefined, 'field removed if nulled')
})

runner.run()
