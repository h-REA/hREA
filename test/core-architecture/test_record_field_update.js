import test from 'tape'
import { pause } from '@holochain/tryorama'
import {
  buildPlayer,
  mockAddress,
  mockIdentifier,
} from '../init.js'

const testEventProps = {
  resourceClassifiedAs: ['some-resource-type'],
  resourceQuantity: { hasNumericalValue: 1, hasUnit: mockIdentifier(false) },
  provider: mockAddress(false),
  receiver: mockAddress(false),
  hasPointInTime: '2019-11-19T04:29:55.056Z',
}

test('updates with fields ommitted leave original value intact', async (t) => {
  const alice = await buildPlayer(['observation'])
  const { cells: [observation] } = alice

  const event = {
    note: 'test event',
    action: 'raise',
    ...testEventProps,
  }

  const createEventResponse = await observation.call('economic_event', 'create_economic_event', { event })
  t.ok(createEventResponse.economicEvent && createEventResponse.economicEvent.id, 'record created successfully')
  await pause(100)

  await observation.call('economic_event', 'update_economic_event', {
    event: {
      revisionId: createEventResponse.economicEvent.revisionId,
    },
  })
  await pause(100)

  const readResponse = await observation.call('economic_event', 'get_economic_event', { address: createEventResponse.economicEvent.id })
  t.equal(readResponse.economicEvent.note, 'test event', 'field remains if not provided')

  await alice.scenario.cleanUp()
})

test('updates with fields nulled remove original value', async (t) => {
  const alice = await buildPlayer(['observation'])
  const { cells: [observation] } = alice

  const event = {
    note: 'test event 2',
    action: 'raise',
    ...testEventProps,
  }

  const createEventResponse = await observation.call('economic_event', 'create_economic_event', { event })
  t.ok(createEventResponse.economicEvent && createEventResponse.economicEvent.id, 'record created successfully')
  await pause(100)

  await observation.call('economic_event', 'update_economic_event', {
    event: {
      revisionId: createEventResponse.economicEvent.revisionId,
      action: 'raise',
      note: null,
    },
  })
  await pause(100)

  const readResponse = await observation.call('economic_event', 'get_economic_event', { address: createEventResponse.economicEvent.id })
  t.equal(readResponse.economicEvent.note, undefined, 'field removed if nulled')

  await alice.scenario.cleanUp()
})
