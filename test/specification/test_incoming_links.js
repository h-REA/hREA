const {
  getDNA,
  buildConfig,
  buildRunner,
  buildPlayer,
} = require('../init')

const runner = buildRunner()

const config = buildConfig({
  observation: getDNA('observation'),
  planning: getDNA('planning'),
  specification: getDNA('specification'),
}, {
  vf_observation: ['planning', 'observation'],
})

const tempProviderAgentId = 'some-agent-provider'
const tempReceiverAgentId = 'some-agent-receiver'
const fillerProps = {
  provider: tempProviderAgentId,
  receiver: tempReceiverAgentId,
  hasPointInTime: '2019-11-19T04:27:55.056Z',
}

runner.registerScenario('inbound Specification link references', async (s, t) => {
  const alice = await buildPlayer(s, 'alice', config)

  // setup some records for linking to
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
      $event: EconomicEventCreateParams!,
      $resource: EconomicResourceCreateParams!,
      $commitment: CommitmentCreateParams!,
      $intent: IntentCreateParams!,
    ) {
      e: createEconomicEvent(event: $event, newInventoriedResource: $resource) {
        economicEvent {
          id
        }
        economicResource {
          id
        }
      }
      c: createCommitment(commitment: $commitment) {
        commitment {
          id
        }
      }
      i: createIntent(intent: $intent) {
        intent {
          id
        }
      }
    }
  `, {
    event: {
      action: 'raise',
      resourceConformsTo: rsId,
      resourceQuantity: { hasNumericalValue: 1, hasUnit: uId },
      ...fillerProps,
    },
    resource: {
      name: 'langths of string',
    },
    commitment: {
      action: 'raise',
      resourceConformsTo: rsId,
      resourceQuantity: { hasNumericalValue: 1, hasUnit: uId },
      ...fillerProps,
    },
    intent: {
      action: 'raise',
      resourceConformsTo: rsId,
      resourceQuantity: { hasNumericalValue: 3, hasUnit: uId },
      ...fillerProps,
    },
  })
  await s.consistency()

  t.ok(resp.data.e.economicEvent.id, 'referencing event created')
  t.ok(resp.data.e.economicResource.id, 'referencing resource created')
  t.ok(resp.data.c.commitment.id, 'referencing commitment created')
  t.ok(resp.data.i.intent.id, 'referencing intent created')
  const eventId = resp.data.e.economicEvent.id
  const resourceId = resp.data.e.economicResource.id
  const commitmentId = resp.data.c.commitment.id
  const intentId = resp.data.i.intent.id

  resp = await alice.graphQL(`{
    economicEvent(id: "${eventId}") {
      resourceConformsTo {
        id
      }
      resourceQuantity {
        hasUnit {
          label
          symbol
        }
      }
    }
    economicResource(id: "${resourceId}") {
      conformsTo {
        id
      }
    }
    commitment(id: "${commitmentId}") {
      resourceConformsTo {
        id
      }
    }
    intent(id: "${intentId}") {
      resourceConformsTo {
        id
      }
    }
  }`)

  t.equal(resp.data.economicEvent.resourceConformsTo.id, rsId, 'EconomicEvent.resourceConformsTo reference OK')
  t.equal(resp.data.economicEvent.resourceQuantity.hasUnit.label, 'metres', 'Measure.hasUnit reference OK')
  t.equal(resp.data.economicResource.conformsTo.id, rsId, 'EconomicResource.conformsTo reference OK')
  t.equal(resp.data.commitment.resourceConformsTo.id, rsId, 'Commitment.reesourceConformsTo reference OK')
  t.equal(resp.data.intent.resourceConformsTo.id, rsId, 'Intent.reesourceConformsTo reference OK')
})

runner.run()
