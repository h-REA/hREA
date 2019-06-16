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

runner.registerScenario('records can be updated multiple times with same ID', async (s, t, { observation }) => {
  const event = {
    note: 'event v1',
  }

  const createResp = await observation.call('main', 'create_event', { event })

  await observation.call('main', 'update_event', {
    event: {
      id: createResp.economicEvent.id,
      note: 'event v2',
    },
  })
  const updateResp2 = await observation.call('main', 'update_event', {
    event: {
      id: createResp.economicEvent.id,
      note: 'event v3',
    },
  })

  await s.consistent()

  t.ok(updateResp2.economicEvent, 'subsequent update failed')
  t.equal(updateResp2.economicEvent.note, 'event v3', 'subsequent update had no effect')
  t.equal(createResp.economicEvent.id, updateResp2.economicEvent.id, 'ID changed after subsequent update')
})

runner.run()
