import test from 'tape'
import { pause } from '@holochain/tryorama'
import {
  buildPlayer,
  mockAddress,
  mockIdentifier,
  sortById,
} from '../init.js'

const testEventProps = {
  action: 'raise',
  provider: mockAddress(),
  receiver: mockAddress(),
  resourceQuantity: { hasNumericalValue: 1.0, hasUnit: mockIdentifier() },
}

test('Event/Resource list APIs', async (t) => {
  const alice = await buildPlayer(['observation'])
  try {
    let resp = await alice.graphQL(`
      mutation(
        $e1: EconomicEventCreateParams!,
        $r1: EconomicResourceCreateParams!,
        $e2: EconomicEventCreateParams!,
        $r2: EconomicResourceCreateParams!,
      ) {
        r1: createEconomicEvent(event: $e1, newInventoriedResource: $r1) {
          economicEvent {
            id
          }
          economicResource {
            id
          }
        }
        r2: createEconomicEvent(event: $e2, newInventoriedResource: $r2) {
          economicEvent {
            id
          }
          economicResource {
            id
          }
        }
      }
    `, {
      e1: {
        resourceClassifiedAs: ['some-type-of-resource'],
        hasPointInTime: new Date(),
        ...testEventProps,
      },
      r1: { note: 'resource A' },
      e2: {
        resourceClassifiedAs: ['another-type-of-resource'],
        hasPointInTime: new Date(),
        ...testEventProps,
      },
      r2: { note: 'resource B' },
    })
    await pause(100)

    t.ok(resp.data.r1.economicResource.id, 'first resource created')
    t.ok(resp.data.r2.economicResource.id, 'second resource created')
    t.ok(resp.data.r1.economicEvent.id, 'first event created')
    t.ok(resp.data.r2.economicEvent.id, 'second event created')
    const resource1Id = resp.data.r1.economicResource.id
    const resource2Id = resp.data.r2.economicResource.id
    const event1Id = resp.data.r1.economicEvent.id
    const event2Id = resp.data.r2.economicEvent.id

    resp = await alice.graphQL(`mutation(
      $e1: EconomicEventCreateParams!,
      $e2: EconomicEventCreateParams!,
      $e3: EconomicEventCreateParams!,
    ) {
      e1: createEconomicEvent(event: $e1) {
        economicEvent {
          id
        }
      }
      e2: createEconomicEvent(event: $e2) {
        economicEvent {
          id
        }
      }
      e3: createEconomicEvent(event: $e3) {
        economicEvent {
          id
        }
      }
    }`, {
      e1: {
        resourceInventoriedAs: resource1Id,
        hasPointInTime: new Date(Date.now() - 1000),
        ...testEventProps,
      },
      e2: {
        resourceInventoriedAs: resource1Id,
        hasPointInTime: new Date(),
        ...testEventProps,
      },
      e3: {
        resourceInventoriedAs: resource2Id,
        hasPointInTime: new Date(),
        ...testEventProps,
      },
    })
    await pause(100)

    t.ok(resp.data.e1.economicEvent.id, '1st additional event created')
    t.ok(resp.data.e2.economicEvent.id, '2nd additional event created')
    t.ok(resp.data.e3.economicEvent.id, '3rd additional event created')
    const event3Id = resp.data.e1.economicEvent.id
    const event4Id = resp.data.e2.economicEvent.id
    const event5Id = resp.data.e3.economicEvent.id

    resp = await alice.graphQL(`{
      economicEvents(last: 10) {
        pageInfo {
          startCursor
          endCursor
        }
        edges {
          cursor
          node {
            id
          }
        }
      }
      economicResources(last: 10) {
        pageInfo {
          startCursor
          endCursor
        }
        edges {
          cursor
          node {
            id
          }
        }
      }
    }`)

    t.equal(resp.data.economicEvents.edges.length, 5, 'all events correctly retrievable')
    t.deepLooseEqual(
      resp.data.economicEvents.edges.map(e => e.node),
      [{ id: event5Id }, { id: event4Id }, { id: event3Id }, { id: event2Id }, { id: event1Id }],
      'event IDs OK',
    )
    t.deepLooseEqual(
      resp.data.economicEvents.edges.map(e => e.cursor),
      [event5Id, event4Id, event3Id, event2Id, event1Id],
      'event cursors OK',
    )
    t.equal(resp.data.economicEvents.pageInfo.startCursor, event5Id, 'event start offset cursor OK')
    t.equal(resp.data.economicEvents.pageInfo.endCursor, event5Id, 'event end offset cursor OK')
    t.equal(resp.data.economicResources.edges.length, 2, 'all resources correctly retrievable')
    t.deepLooseEqual(
      resp.data.economicResources.edges.map(e => e.node),
      [{ id: resource2Id }, { id: resource1Id }],
      'resource IDs OK',
    )
    t.deepLooseEqual(
      resp.data.economicResources.edges.map(e => e.cursor),
      [resource2Id, resource1Id],
      'resource cursors OK',
    )
    t.equal(resp.data.economicResources.pageInfo.startCursor, resource2Id, 'resource start offset cursor OK')
    t.equal(resp.data.economicResources.pageInfo.endCursor, resource1Id, 'resource end offset cursor OK')
  } catch (e) {
    await alice.scenario.cleanUp()
    throw e
  }
  await alice.scenario.cleanUp()
})
