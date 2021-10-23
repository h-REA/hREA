const {
  buildConfig,
  buildRunner,
  buildPlayer,
  mockAgentId,
  mockIdentifier,
} = require('../init')

const runner = buildRunner()

const config = buildConfig()

const testEventProps = {
  resourceClassifiedAs: ['some-resource-type'],
  resourceQuantity: { hasNumericalValue: 1, hasUnit: mockIdentifier(false) },
  provider: mockAgentId(false),
  receiver: mockAgentId(false),
  hasPointInTime: '2019-11-19T04:29:55.056Z',
}

runner.registerScenario('records have stable IDs after update', async (s, t) => {
  const { cells: [observation] } = await buildPlayer(s, config, ['observation'])

  const event = {
    note: 'test event',
    action: 'raise',
    ...testEventProps,
  }

  const createEventResponse = await observation.call('economic_event', 'create_economic_event', { event })
  await s.consistency()

  t.ok(createEventResponse.economicEvent && createEventResponse.economicEvent.id, 'record created successfully')

  const updateEventResponse = await observation.call('economic_event', 'update_economic_event', {
    event: {
      revisionId: createEventResponse.economicEvent.revisionId,
      note: 'updated event',
    },
  })
  await s.consistency()

  t.deepEqual(createEventResponse.economicEvent.id, updateEventResponse.economicEvent.id, 'ID consistent after update')
  t.notDeepEqual(createEventResponse.economicEvent.revisionId, updateEventResponse.economicEvent.revisionId, 'revision ID changed after update')
  t.equal(updateEventResponse.economicEvent.note, 'updated event', 'field update OK')
})

const runner2 = buildRunner()

runner2.registerScenario('records can be updated multiple times with appropriate revisionID', async (s, t) => {
  const { cells: [observation] } = await buildPlayer(s, config, ['observation'])

  const event = {
    note: 'event v1',
    action: 'raise',
    ...testEventProps,
  }

  const createResp = await observation.call('economic_event', 'create_economic_event', { event })
  await s.consistency()

  const updateResp1 = await observation.call('economic_event', 'update_economic_event', {
    event: {
      revisionId: createResp.economicEvent.revisionId,
      note: 'event v2',
    },
  })
  await s.consistency()

  const updateResp2 = await observation.call('economic_event', 'update_economic_event', {
    event: {
      revisionId: updateResp1.economicEvent.revisionId,
      note: 'event v3',
    },
  })
  await s.consistency()

  t.ok(updateResp2.economicEvent, 'subsequent update successful')
  t.equal(updateResp2.economicEvent.note, 'event v3', 'subsequent field update OK')
  t.deepEqual(createResp.economicEvent.id, updateResp2.economicEvent.id, 'ID consistency after subsequent update')
})

runner.run()
runner2.run()
