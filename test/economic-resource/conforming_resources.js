const {
  getDNA,
  buildConfig,
  buildRunner,
  buildGraphQL,
} = require('../init')

const runner = buildRunner()

const config = buildConfig({
  observation: getDNA('observation'),
  specification: getDNA('specification'),
}, {
  vf_specification: ['observation', 'specification'],
})

const testEventProps = {
  action: 'raise',
  provider: 'agentid-1-todo',
  receiver: 'agentid-2-todo',
  hasPointInTime: '2019-11-19T04:29:55.056Z',
  resourceQuantity: { hasNumericalValue: 1, hasUnit: 'unit-todo-tidy-up' },
}

runner.registerScenario('can locate EconomicResources conforming to a ResourceSpecification', async (s, t) => {
  const { alice } = await s.players({ alice: config }, true)
  alice.graphQL = buildGraphQL(alice)

  let resp = await alice.graphQL(`
    mutation(
      $rs: ResourceSpecificationCreateParams!,
    ) {
      rs: createResourceSpecification(resourceSpecification: $rs) {
        resourceSpecification {
          id
        }
      }
    }
  `, {
    rs: {
      name: 'test resource spec',
    },
  })
  await s.consistency()

  t.ok(resp.data.rs.resourceSpecification.id, 'ResourceSpecification created')
  const rsId = resp.data.rs.resourceSpecification.id

  resp = await alice.graphQL(`
    mutation(
      $e1: EconomicEventCreateParams!,
      $r1: EconomicResourceCreateParams!,
      $e2: EconomicEventCreateParams!,
      $r2: EconomicResourceCreateParams!,
    ) {
      r1: createEconomicEvent(event: $e1, newInventoriedResource: $r1) {
        economicResource {
          id
        }
      }
      r2: createEconomicEvent(event: $e2, newInventoriedResource: $r2) {
        economicResource {
          id
        }
      }
    }
  `, {
    e1: {
      resourceConformsTo: rsId,
      ...testEventProps,
    },
    r1: {},
    e2: {
      resourceConformsTo: rsId,
      ...testEventProps,
    },
    r2: {},
  })
  await s.consistency()

  t.ok(resp.data.r1.economicResource.id, 'first resource created')
  t.ok(resp.data.r2.economicResource.id, 'second resource created')
  const resource1Id = resp.data.r1.economicResource.id
  const resource2Id = resp.data.r2.economicResource.id

  resp = await alice.graphQL(`{
    rs: resourceSpecification(id: "${rsId}") {
      conformingResources {
        id
      }
    }
  }`)
console.error(resp)
  t.equal(resp.data.rs.conformingResources.length, 2, 'all resources indexed via ResourceSpecification link')
  t.equal(resp.data.rs.conformingResources[0].id, resource1Id, 'resource 1 ref OK')
  t.equal(resp.data.rs.conformingResources[1].id, resource2Id, 'resource 2 ref OK')
})

runner.run()
