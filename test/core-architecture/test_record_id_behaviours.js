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

test('records have stable IDs after update', async (t) => {
  // display the filename for context in the terminal and use .warn
  // to override the tap testing log filters
  console.warn(`\n\n${import.meta.url}`)
  const alice = await buildPlayer(['observation'])
  try {
    const { cells: [observation] } = alice

    const event = {
      note: 'test event',
      action: 'raise',
      ...testEventProps,
    }

    const createEventResponse = await observation.call('economic_event', 'create_economic_event', { event })
    await pause(100)

    t.ok(createEventResponse.economicEvent && createEventResponse.economicEvent.id, 'record created successfully')

    const updateEventResponse = await observation.call('economic_event', 'update_economic_event', {
      event: {
        revisionId: createEventResponse.economicEvent.revisionId,
        note: 'updated event',
      },
    })
    await pause(100)

    t.deepLooseEqual(createEventResponse.economicEvent.id, updateEventResponse.economicEvent.id, 'ID consistent after update')
    t.notDeepEqual(createEventResponse.economicEvent.revisionId, updateEventResponse.economicEvent.revisionId, 'revision ID changed after update')
    t.equal(updateEventResponse.economicEvent.note, 'updated event', 'field update OK')
  } catch (e) {
    await alice.scenario.cleanUp()
    throw e
  }
  await alice.scenario.cleanUp()
})

test('records can be updated multiple times with appropriate revisionID', async (t) => {
  // display the filename for context in the terminal and use .warn
  // to override the tap testing log filters
  console.warn(`\n\n${import.meta.url}`)
  const alice = await buildPlayer(['observation'])
  try {
    const { cells: [observation] } = alice

    const event = {
      note: 'event v1',
      action: 'raise',
      ...testEventProps,
    }

    const createResp = await observation.call('economic_event', 'create_economic_event', { event })
    await pause(100)

    const updateResp1 = await observation.call('economic_event', 'update_economic_event', {
      event: {
        revisionId: createResp.economicEvent.revisionId,
        note: 'event v2',
      },
    })
    await pause(100)

    const updateResp2 = await observation.call('economic_event', 'update_economic_event', {
      event: {
        revisionId: updateResp1.economicEvent.revisionId,
        note: 'event v3',
      },
    })
    await pause(100)

    t.ok(updateResp2.economicEvent, 'subsequent update successful')
    t.equal(updateResp2.economicEvent.note, 'event v3', 'subsequent field update OK')
    t.deepLooseEqual(createResp.economicEvent.id, updateResp2.economicEvent.id, 'ID consistency after subsequent update')
  } catch (e) {
    await alice.scenario.cleanUp()
    throw e
  }
  await alice.scenario.cleanUp()
})
