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
  resourceQuantity: { numericValue: 1, unit: 'dangling-unit-todo-tidy-up' },
  provider: 'agentid-1-todo',
  receiver: 'agentid-2-todo',
  hasPointInTime: '2019-11-19T04:29:55.056Z',
}

runner.registerScenario('records have stable IDs after update', async (s, t) => {
  const { observation } = await s.players({ observation: config }, true)

  const event = {
    note: 'test event',
    action: 'produce',
    ...testEventProps,
  }

  const createEventResponse = await observation.call('observation', 'economic_event', 'create_event', { event })

  t.ok(createEventResponse.Ok.economicEvent && createEventResponse.Ok.economicEvent.id, 'record created successfully')

  const updateEventResponse = await observation.call('observation', 'economic_event', 'update_event', {
    event: {
      id: createEventResponse.Ok.economicEvent.id,
      note: 'updated event',
    },
  })

  await s.consistency()

  t.equal(createEventResponse.Ok.economicEvent.id, updateEventResponse.Ok.economicEvent.id, 'ID consistent after update')
  t.equal(updateEventResponse.Ok.economicEvent.note, 'updated event', 'field update OK')
})

runner.registerScenario('records can be updated multiple times with same ID', async (s, t) => {
  const { observation } = await s.players({ observation: config }, true)

  const event = {
    note: 'event v1',
    action: 'produce',
    ...testEventProps,
  }

  const createResp = await observation.call('observation', 'economic_event', 'create_event', { event })

  await observation.call('observation', 'economic_event', 'update_event', {
    event: {
      id: createResp.Ok.economicEvent.id,
      note: 'event v2',
    },
  })
  const updateResp2 = await observation.call('observation', 'economic_event', 'update_event', {
    event: {
      id: createResp.Ok.economicEvent.id,
      note: 'event v3',
    },
  })

  await s.consistency()

  t.ok(updateResp2.Ok.economicEvent, 'subsequent update successful')
  t.equal(updateResp2.Ok.economicEvent.note, 'event v3', 'subsequent field update OK')
  t.equal(createResp.Ok.economicEvent.id, updateResp2.Ok.economicEvent.id, 'ID consistency after subsequent update')
})

runner.run()
