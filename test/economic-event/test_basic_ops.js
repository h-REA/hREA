import test from 'tape'
import { pause } from '@holochain/tryorama'
import {
  buildPlayer,
  mockAddress,
} from '../init.js'

test('create simplest event', async (t) => {
  // display the filename for context in the terminal and use .warn
  // to override the tap testing log filters
  console.warn(`\n\n${import.meta.url}`)
  const alice = await buildPlayer(['observation'])
  try {
    const { cells: [observation] } = alice

    const event = {
      note: 'test event',
      action: 'raise',
      provider: mockAddress(false),
      receiver: mockAddress(false),
      hasPointInTime: '2019-11-19T12:12:42.739+01:00',
      resourceClassifiedAs: ['some-resource-type'],
      resourceQuantity: { hasNumericalValue: 1 },
      inScopeOf: ['some-accounting-scope'],
    }

    const createEventResponse = await observation.call('economic_event', 'create_economic_event', { event })
    await pause(100)

    t.ok(createEventResponse.economicEvent, 'event created')
    t.deepLooseEqual(createEventResponse.economicEvent.inScopeOf, ['some-accounting-scope'], 'event inScopeOf saved')
  } catch (e) {
    await alice.scenario.cleanUp()
    throw e
  }
  await alice.scenario.cleanUp()
})
