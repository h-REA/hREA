const {
  getDNA,
  buildConfig,
  buildRunner,
  buildGraphQL,
} = require('../init')

const runner = buildRunner()

const config = buildConfig({
  observation: getDNA('observation'),
}, {
})

// required attributes, not involved with test logic
const testEventProps = {
  provider: 'agentid-1-todo',
  receiver: 'agentid-2-todo',
  hasPointInTime: '2019-11-19T04:29:55.056Z',
}

runner.registerScenario('EconomicResource classification fields validation', async (s, t) => {
  const { alice } = await s.players({ alice: config }, true)
  alice.graphQL = buildGraphQL(alice)

  let resp = await alice.graphQL(`
    mutation($e: EconomicEventCreateParams!, $r: EconomicResourceCreateParams) {
      createEconomicEvent(event: $e, newInventoriedResource: $r) {
        economicEvent {
          id
        }
        economicResource {
          id
        }
      }
    }
  `, {
    e: {
      action: 'raise',
      resourceQuantity: { hasNumericalValue: 1, hasUnit: 'kg' },
      ...testEventProps,
    },
    r: {
      name: 'bad resource without ontological distinction capability',
    },
  })
  await s.consistency()

  t.equal(resp.errors.length, 1, 'creating resource without ontological bindings is an error')
  t.notEqual(-1, resp.errors[0].message.indexOf('EconomicResource must have either a specification or classification'), 'correct error reported')

  resp = await alice.graphQL(`
    mutation($e: EconomicEventCreateParams!, $r: EconomicResourceCreateParams) {
      createEconomicEvent(event: $e, newInventoriedResource: $r) {
        economicEvent {
          id
        }
        economicResource {
          id
        }
      }
    }
  `, {
    e: {
      action: 'raise',
      resourceQuantity: { hasNumericalValue: 1, hasUnit: 'kg' },
      resourceClassifiedAs: ['some-classification-url'],
      ...testEventProps,
    },
    r: {
      name: 'good resource with ontological distinction capability',
    },
  })
  await s.consistency()

  t.ok(resp.data.createEconomicEvent.economicEvent.id, 'creating resource with classification is OK')

  resp = await alice.graphQL(`
    mutation($e: EconomicEventCreateParams!, $r: EconomicResourceCreateParams) {
      createEconomicEvent(event: $e, newInventoriedResource: $r) {
        economicEvent {
          id
        }
        economicResource {
          id
        }
      }
    }
  `, {
    e: {
      action: 'raise',
      resourceQuantity: { hasNumericalValue: 1, hasUnit: 'kg' },
      resourceConformsTo: 'some-specification-id',
      ...testEventProps,
    },
    r: {
      name: 'good resource with ontological distinction capability',
    },
  })
  await s.consistency()

  t.ok(resp.data.createEconomicEvent.economicEvent.id, 'creating resource with event specification is OK')

  resp = await alice.graphQL(`
    mutation($e: EconomicEventCreateParams!, $r: EconomicResourceCreateParams) {
      createEconomicEvent(event: $e, newInventoriedResource: $r) {
        economicEvent {
          id
        }
        economicResource {
          id
        }
      }
    }
  `, {
    e: {
      action: 'raise',
      resourceQuantity: { hasNumericalValue: 1, hasUnit: 'kg' },
      ...testEventProps,
    },
    r: {
      name: 'good resource with ontological distinction capability',
      conformsTo: 'some-specification-id',
    },
  })
  await s.consistency()

  t.ok(resp.data.createEconomicEvent.economicEvent.id, 'creating resource with resource specification is OK')
})

runner.run()
