const {
  getDNA,
  buildConfig,
  buildRunner,
  buildPlayer,
} = require('../init')

const runner = buildRunner()

const config = buildConfig({
  observation: getDNA('observation'),
})

const testEventProps = {
  action: 'raise',
  provider: 'agentid-1-todo',
  receiver: 'agentid-2-todo',
  resourceQuantity: { hasNumericalValue: 1, hasUnit: 'unit-todo-tidy-up' },
}

runner.registerScenario('Event/Resource list APIs', async (s, t) => {
  const alice = await buildPlayer(s, 'alice', config)

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
      hasPointInTime: '2019-11-19T04:29:55.000Z',
      ...testEventProps,
    },
    r1: { note: 'resource A' },
    e2: {
      resourceClassifiedAs: ['another-type-of-resource'],
      hasPointInTime: '2019-11-19T04:29:56.000Z',
      ...testEventProps,
    },
    r2: { note: 'resource B' },
  })
  await s.consistency()

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
      hasPointInTime: '2019-11-20T04:29:55.000Z',
      ...testEventProps,
    },
    e2: {
      resourceInventoriedAs: resource1Id,
      hasPointInTime: '2019-11-21T04:29:55.000Z',
      ...testEventProps,
    },
    e3: {
      resourceInventoriedAs: resource2Id,
      hasPointInTime: '2019-11-22T04:29:55.000Z',
      ...testEventProps,
    },
  })
  await s.consistency()

  t.ok(resp.data.e1.economicEvent.id, '1st additional event created')
  t.ok(resp.data.e2.economicEvent.id, '2nd additional event created')
  t.ok(resp.data.e3.economicEvent.id, '3rd additional event created')
  const event3Id = resp.data.e1.economicEvent.id
  const event4Id = resp.data.e2.economicEvent.id
  const event5Id = resp.data.e3.economicEvent.id

  resp = await alice.graphQL(`{
    allEconomicEvents {
      id
    }
    allEconomicResources {
      id
    }
  }`)

  t.equal(resp.data.allEconomicEvents.length, 5, 'all events correctly retrievable')
  t.deepEqual(
    resp.data.allEconomicEvents,
    [{ id: event1Id }, { id: event2Id }, { id: event3Id }, { id: event4Id }, { id: event5Id }].reverse(),
    'event IDs OK'
  )
  t.equal(resp.data.allEconomicResources.length, 2, 'all resources correctly retrievable')
  t.deepEqual(
    resp.data.allEconomicResources,
    [{ id: resource1Id }, { id: resource2Id }].reverse(),
    'resource IDs OK'
  )
})

runner.run()
