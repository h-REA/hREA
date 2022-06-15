import test from 'tape'
import { pause } from '@connoropolous/tryorama'
import {
  buildPlayer,
  mockAgentId,
} from '../init.js'

test('create simplest event', async (t) => {
  const alice = await buildPlayer(['observation'])
  const { cells: [observation] } = alice

  const event = {
    note: 'test event',
    action: 'raise',
    provider: mockAgentId(false),
    receiver: mockAgentId(false),
    hasPointInTime: '2019-11-19T12:12:42.739+01:00',
    resourceClassifiedAs: ['some-resource-type'],
    resourceQuantity: { hasNumericalValue: 1 },
    inScopeOf: ['some-accounting-scope'],
  }

  const createEventResponse = await observation.call('economic_event', 'create_economic_event', { event })
  await pause(100)

  t.ok(createEventResponse.economicEvent, 'event created')
  t.deepLooseEqual(createEventResponse.economicEvent.inScopeOf, ['some-accounting-scope'], 'event inScopeOf saved')

  await alice.scenario.cleanUp()
})
