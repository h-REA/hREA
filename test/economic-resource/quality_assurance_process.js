/**
 * Details a QA process where the EconomicResources incoming to a process
 * are split into approved and rejected lots.
 *
 * @package: HoloREA
 * @since:   2020-01-12
 */

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

const fillerUnitId = 'unit-todo-tidy-up'
const fillerEventProps = {
  provider: 'agentid-1-todo',
  receiver: 'agentid-2-todo',
  hasPointInTime: '2019-11-19T04:29:55.056Z',
}

runner.registerScenario('EconomicResource QA processes', async (s, t) => {
  const { alice } = await s.players({ alice: config }, true)
  alice.graphQL = buildGraphQL(alice)

  // primary process driving the real-life QA work

  let resp = await alice.graphQL(`
    mutation($process: ProcessCreateParams!) {
      createProcess(process: $process) {
        process {
          id
        }
      }
    }
  `, {
    process: {
      name: 'quality assurance check of a resource',
    },
  })
  await s.consistency()

  t.ok(resp.data.createProcess.process.id, 'process created OK')
  const processId = resp.data.createProcess.process.id

  // initialise resource inventories

  resp = await alice.graphQL(`
    mutation(
      $e1: EconomicEventCreateParams!,
      $r1: EconomicResourceCreateParams!,
      $e2: EconomicEventCreateParams!,
      $r2: EconomicResourceCreateParams!,
      $e3: EconomicEventCreateParams!,
      $r3: EconomicResourceCreateParams!,
    ) {
      unassessed: createEconomicEvent(event: $e1, newInventoriedResource: $r1) {
        economicResource {
          id
        }
      }
      failing: createEconomicEvent(event: $e2, newInventoriedResource: $r2) {
        economicResource {
          id
        }
      }
      passing: createEconomicEvent(event: $e3, newInventoriedResource: $r3) {
        economicResource {
          id
        }
      }
    }
  `, {
    e1: {
      action: 'raise',
      resourceClassifiedAs: ['unassessed-resource-type'],
      resourceQuantity: { hasNumericalValue: 10, hasUnit: fillerUnitId },
      ...fillerEventProps,
    },
    r1: { note: 'resources inbound to the QA process' },
    e2: {
      action: 'raise',
      resourceClassifiedAs: ['failing-resource-type'],
      resourceQuantity: { hasNumericalValue: 0, hasUnit: fillerUnitId },
      ...fillerEventProps,
    },
    r2: { note: 'resources that have broken, spoiled etc' },
    e3: {
      action: 'raise',
      resourceClassifiedAs: ['passing-resource-type'],
      resourceQuantity: { hasNumericalValue: 0, hasUnit: fillerUnitId },
      ...fillerEventProps,
    },
    r3: { note: 'resources that are acceptable to downstream producers / consumers' },
  })
  await s.consistency()

  t.ok(resp.data.unassessed.economicResource.id, 'input resource created')
  t.ok(resp.data.failing.economicResource.id, 'failing resource created')
  t.ok(resp.data.passing.economicResource.id, 'passing resource created')
  const unassessedResourceId = resp.data.unassessed.economicResource.id
  const failingResourceId = resp.data.failing.economicResource.id
  const passingResourceId = resp.data.passing.economicResource.id

  // step 1: receive resources for QA

  resp = await alice.graphQL(`
    mutation(
      $event: EconomicEventCreateParams!,
    ) {
      createEconomicEvent(event: $event) {
        economicEvent {
          id
        }
      }
    }
  `, {
    event: {
      action: 'accept',
      resourceInventoriedAs: unassessedResourceId,
      resourceQuantity: { hasNumericalValue: 10, hasUnit: fillerUnitId },
      inputOf: processId,
      ...fillerEventProps,
    },
  })
  await s.consistency()

  t.ok(resp.data.createEconomicEvent.economicEvent.id, 'accept event created')

  // step 2: log passes & failures

  resp = await alice.graphQL(`
    mutation(
      $passing: EconomicEventCreateParams!,
      $failing: EconomicEventCreateParams!,
    ) {
      passingEvent: createEconomicEvent(event: $passing) {
        economicEvent {
          id
        }
      }
      failingEvent: createEconomicEvent(event: $failing) {
        economicEvent {
          id
        }
      }
    }
  `, {
    passing: {
      action: 'pass',
      resourceInventoriedAs: unassessedResourceId,
      toResourceInventoriedAs: passingResourceId,
      resourceQuantity: { hasNumericalValue: 6, hasUnit: fillerUnitId },
      outputOf: processId,
      ...fillerEventProps,
    },
    failing: {
      action: 'fail',
      resourceInventoriedAs: unassessedResourceId,
      toResourceInventoriedAs: failingResourceId,
      resourceQuantity: { hasNumericalValue: 3, hasUnit: fillerUnitId },
      outputOf: processId,
      ...fillerEventProps,
    },
  })
  await s.consistency()

  t.ok(resp.data.passingEvent.economicEvent.id, 'passing event created')
  t.ok(resp.data.failingEvent.economicEvent.id, 'failing event created')

  // check final quantities

  resp = await alice.graphQL(`{
    unassessed: economicResource(id: "${unassessedResourceId}") {
      accountingQuantity {
        hasNumericalValue
      }
      onhandQuantity {
        hasNumericalValue
      }
    }
    passing: economicResource(id: "${passingResourceId}") {
      accountingQuantity {
        hasNumericalValue
      }
      onhandQuantity {
        hasNumericalValue
      }
    }
    failing: economicResource(id: "${failingResourceId}") {
      accountingQuantity {
        hasNumericalValue
      }
      onhandQuantity {
        hasNumericalValue
      }
    }
  }`)

  t.equal(resp.data.unassessed.accountingQuantity.hasNumericalValue, 1, 'unassessed quantity checks out')
  t.equal(resp.data.passing.accountingQuantity.hasNumericalValue, 6, 'passing quantity checks out')
  t.equal(resp.data.failing.accountingQuantity.hasNumericalValue, 3, 'failing quantity checks out')
})

runner.run()
