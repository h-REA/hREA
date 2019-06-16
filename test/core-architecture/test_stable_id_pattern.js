const {
  getDNA,
  buildOrchestrator,
} = require('../init')

const runner = buildOrchestrator({
  observation: getDNA('observation'),
})

runner.registerScenario('records have stable IDs after update', async (s, t, { observation }) => {
  const event = {
    note: 'test event',
  }

  const createEventResponse = await observation.call('main', 'create_event', { event })

  t.ok(createEventResponse.economicEvent && createEventResponse.economicEvent.id, 'Creating record failed')

  const updateEventResponse = await observation.call('main', 'update_event', {
    event: {
      id: createEventResponse.economicEvent.id,
      note: 'updated event',
    },
  })

  await s.consistent()

  t.equal(createEventResponse.economicEvent.id, updateEventResponse.economicEvent.id, 'ID changed after update')
  t.equal(updateEventResponse.economicEvent.note, 'updated event', 'update had no effect')
})

runner.run()
