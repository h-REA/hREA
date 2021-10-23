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

runner.registerScenario('updates with fields ommitted leave original value intact', async (s, t) => {
  const { cells: [observation] } = await buildPlayer(s, config, ['observation'])

  const event = {
    note: 'test event',
    action: 'raise',
    ...testEventProps,
  }

  const createEventResponse = await observation.call('economic_event', 'create_economic_event', { event })
  t.ok(createEventResponse.economicEvent && createEventResponse.economicEvent.id, 'record created successfully')
  await s.consistency()

  await observation.call('economic_event', 'update_economic_event', {
    event: {
      revisionId: createEventResponse.economicEvent.revisionId,
    },
  })
  await s.consistency()

  const readResponse = await observation.call('economic_event', 'get_economic_event', { address: createEventResponse.economicEvent.id })
  t.equal(readResponse.economicEvent.note, 'test event', 'field remains if not provided')
})

const runner2 = buildRunner()

runner2.registerScenario('updates with fields nulled remove original value', async (s, t) => {
  const { cells: [observation] } = await buildPlayer(s, config, ['observation'])

  const event = {
    note: 'test event 2',
    action: 'raise',
    ...testEventProps,
  }

  const createEventResponse = await observation.call('economic_event', 'create_economic_event', { event })
  t.ok(createEventResponse.economicEvent && createEventResponse.economicEvent.id, 'record created successfully')
  await s.consistency()

  await observation.call('economic_event', 'update_economic_event', {
    event: {
      revisionId: createEventResponse.economicEvent.revisionId,
      action: 'raise',
      note: null,
    },
  })
  await s.consistency()

  const readResponse = await observation.call('economic_event', 'get_economic_event', { address: createEventResponse.economicEvent.id })
  t.equal(readResponse.economicEvent.note, undefined, 'field removed if nulled')
})

runner.run()
runner2.run()
