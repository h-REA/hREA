const {
  getDNA,
  buildConfig,
  buildRunner,
  buildPlayer,
  mockAgentId,
} = require('../init')

const runner = buildRunner()

const config = buildConfig()

const fillerProps = {
  provider: mockAgentId(),
  receiver: mockAgentId(),
  hasPointInTime: '2019-11-19T04:27:55.056Z',
}

runner.registerScenario('inbound Specification link references', async (s, t) => {
  const alice = await buildPlayer(s, config, ['observation', 'planning', 'specification'])

  // setup some records for linking to
  let resp = await alice.graphQL(`
    mutation(
      $ps: ProcessSpecificationCreateParams!,
      $u: UnitCreateParams!,
    ) {
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

  t.ok(resp.data.pro.processSpecification.id, 'process specification created')
  t.ok(resp.data.uni.unit.id, 'unit created')
  const psId = resp.data.pro.processSpecification.id
  const uId = resp.data.uni.unit.id

  resp = await alice.graphQL(`
    mutation(
      $rs: ResourceSpecificationCreateParams!,
    ) {
      res: createResourceSpecification(resourceSpecification: $rs) {
        resourceSpecification {
          id
          defaultUnitOfEffort {
            id
          }
        }
      }
    }
  `, {
    rs: {
      name: 'test_resourceSpec',
      note: 'Resource specification to test references with',
      defaultUnitOfEffort: uId,
    },
  })
  await s.consistency()

  t.ok(resp.data.res.resourceSpecification.id, 'resource specification created')
  t.ok(resp.data.res.resourceSpecification.defaultUnitOfEffort.id, 'resource specification default unit ok')
  const rsId = resp.data.res.resourceSpecification.id

  // test simple links

  resp = await alice.graphQL(`
    mutation(
      $process: ProcessCreateParams!,
      $event: EconomicEventCreateParams!,
      $resource: EconomicResourceCreateParams!,
      $commitment: CommitmentCreateParams!,
      $intent: IntentCreateParams!,
    ) {
      p: createProcess(process: $process) {
        process {
          id
        }
      }
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
    process: {
      name: 'manufacture a resource',
      basedOn: psId,
    },
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

  t.ok(resp.data.p.process.id, 'referencing process created')
  t.ok(resp.data.e.economicEvent.id, 'referencing event created')
  t.ok(resp.data.e.economicResource.id, 'referencing resource created')
  t.ok(resp.data.c.commitment.id, 'referencing commitment created')
  t.ok(resp.data.i.intent.id, 'referencing intent created')
  const processId = resp.data.p.process.id
  const eventId = resp.data.e.economicEvent.id
  const resourceId = resp.data.e.economicResource.id
  const commitmentId = resp.data.c.commitment.id
  const intentId = resp.data.i.intent.id

  resp = await alice.graphQL(`{
    process(id: "${processId}") {
      basedOn {
        id
      }
    }
    economicEvent(id: "${eventId}") {
      action {
        id
      }
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
      unitOfEffort {
        id
      }
    }
    commitment(id: "${commitmentId}") {
      action {
        id
      }
      resourceConformsTo {
        id
      }
    }
    intent(id: "${intentId}") {
      action {
        id
      }
      resourceConformsTo {
        id
      }
    }
  }`)

  t.equal(resp.data.process.basedOn.id, psId, 'Process.basedOn reference OK')
  t.equal(resp.data.economicEvent.resourceConformsTo.id, rsId, 'EconomicEvent.resourceConformsTo reference OK')
  t.equal(resp.data.economicEvent.resourceQuantity.hasUnit.label, 'metres', 'Measure.hasUnit reference OK')
  t.equal(resp.data.economicResource.conformsTo.id, rsId, 'EconomicResource.conformsTo reference OK')
  t.equal(resp.data.economicResource.unitOfEffort.id, uId, 'EconomicResource.unitOfEffort reference OK')
  t.equal(resp.data.commitment.resourceConformsTo.id, rsId, 'Commitment.reesourceConformsTo reference OK')
  t.equal(resp.data.intent.resourceConformsTo.id, rsId, 'Intent.reesourceConformsTo reference OK')
  t.equal(resp.data.economicEvent.action.id, 'raise', 'EconomicEvent.action reference OK')
  t.equal(resp.data.commitment.action.id, 'raise', 'Commitment.action reference OK')
  t.equal(resp.data.intent.action.id, 'raise', 'Intent.action reference OK')

  // test EconomicResource stage

  resp = await alice.graphQL(`
    mutation(
      $event: EconomicEventCreateParams!,
    ) {
      e: createEconomicEvent(event: $event) {
        economicEvent {
          id
        }
      }
    }
  `, {
    event: {
      action: 'produce',
      outputOf: processId,
      resourceInventoriedAs: resourceId,
      resourceQuantity: { hasNumericalValue: 1, hasUnit: uId },
      ...fillerProps,
    },
  })

  t.ok(resp.data.e.economicEvent.id, 'resource output event created')

  resp = await alice.graphQL(`{
    economicResource(id: "${resourceId}") {
      stage {
        id
      }
    }
  }`)

  t.equal(resp.data.economicResource.stage.id, psId, 'EconomicResource.stage updates in response to process outputs')
})

runner.run()
