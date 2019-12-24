const {
  getDNA,
  buildConfig,
  buildRunner,
  buildGraphQL,
} = require('../init')

const runner = buildRunner()

const config = buildConfig({
  observation: getDNA('observation'),
  planning: getDNA('planning'),
  specification: getDNA('specification'),
}, {
  vf_observation: ['planning', 'observation'],
})

runner.registerScenario('inbound Specification link references', async (s, t) => {
  const { alice } = await s.players({ alice: config }, true)
  alice.graphQL = buildGraphQL(alice)

  // setup some records for linking to

  const tempProviderAgentId = 'some-agent-provider'
  const tempReceiverAgentId = 'some-agent-receiver'

  let resp = await alice.graphQL(`
    mutation(
      $rs: ResourceSpecificationCreateParams!,
      $ps: ProcessSpecificationCreateParams!,
      $u: UnitCreateParams!,
    ) {
      res: createResourceSpecification(resourceSpecification: $rs) {
        resourceSpecification {
          id
        }
      }
      pro: createProcessSpecification(processSpecification: $ps) {
        processSpecification {
          id
        }
      }
      uni: createUnit(unit: $u) {
        unit {
          id
        }
      }
    }
  `, {
    rs: {
      name: 'test_resourceSpec',
      note: 'Resource specification to test references with',
    },
    ps: {
      name: 'test_processSpec',
      note: 'Process specification to test references with',
    },
    u: {
      label: 'metres',
      symbol: 'm',
    },
  })
  await s.consistency()

  t.ok(resp.data.res.resourceSpecification.id, 'resource specification created')
  t.ok(resp.data.pro.processSpecification.id, 'process specification created')
  t.ok(resp.data.uni.unit.id, 'unit created')
  const rsId = resp.data.res.resourceSpecification.id
  const psId = resp.data.pro.processSpecification.id
  const uId = resp.data.uni.unit.id

  // test EconomicEvent & Unit refs

  resp = await alice.graphQL(`
    mutation(
      $event: EconomicEventCreateParams!
    ) {
      e: createEconomicEvent(event: $event) {
        economicEvent {
          id
        }
      }
    }
  `, {
    event: {
      action: 'raise',
      provider: tempProviderAgentId,
      receiver: tempReceiverAgentId,
      hasPointInTime: '2019-11-19T04:27:55.056Z',
      resourceConformsTo: rsId,
      resourceQuantity: { hasNumericalValue: 1, hasUnit: uId },
    },
  })
  await s.consistency()

  t.ok(resp.data.e.economicEvent.id, 'referencing event created')
  const eventId = resp.data.e.economicEvent.id

  resp = await alice.graphQL(`{
    economicEvent(id: "${eventId}") {
      resourceConformsTo {
        id
        name
      }
      resourceQuantity {
        hasUnit {
          label
          symbol
        }
      }
    }
  }`)

  t.equal(resp.data.economicEvent.resourceConformsTo.id, rsId, 'EconomicEvent.resourceConformsTo reference OK')
  t.equal(resp.data.economicEvent.resourceQuantity.hasUnit.label, 'metres', 'Measure.hasUnit reference OK')
})

runner.run()
