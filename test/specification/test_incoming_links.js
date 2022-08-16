import test from 'tape'
import { pause } from '@holochain/tryorama'
import {
  buildPlayer,
  mockAddress,
} from '../init.js'

const fillerProps = {
  provider: mockAddress(),
  receiver: mockAddress(),
  hasPointInTime: '2019-11-19T04:27:55.056Z',
}

test('inbound Specification link references', async (t) => {
  // display the filename for context in the terminal and use .warn
  // to override the tap testing log filters
  console.warn(`\n\n${import.meta.url}`)
  const alice = await buildPlayer(['observation', 'planning', 'specification'])
  try {
    // setup some records for linking to
    let resp = await alice.graphQL(`
      mutation(
        $ps: ProcessSpecificationCreateParams!,
        $u: UnitCreateParams!,
        $u2: UnitCreateParams!,
      ) {
        pro: createProcessSpecification(processSpecification: $ps) {
          processSpecification {
            id
          }
        }
        eUnit: createUnit(unit: $u) {
          unit {
            id
          }
        }
        rUnit: createUnit(unit: $u2) {
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
        label: 'hours',
        symbol: 'h',
      },
      u2: {
        label: 'kilos',
        symbol: 'kg',
      },
    })
    await pause(100)

    t.ok(resp.data.pro.processSpecification.id, 'process specification created')
    t.ok(resp.data.eUnit.unit.id, 'effort unit created')
    t.ok(resp.data.rUnit.unit.id, 'resource unit created')
    const psId = resp.data.pro.processSpecification.id
    const uId = resp.data.eUnit.unit.id
    const u2Id = resp.data.rUnit.unit.id

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
            defaultUnitOfResource {
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
        defaultUnitOfResource: u2Id,
      },
    })
    await pause(100)

    t.ok(resp.data.res.resourceSpecification.id, 'resource specification created')
    t.ok(resp.data.res.resourceSpecification.defaultUnitOfEffort.id, 'resource specification default unit of effort ok')
    t.ok(resp.data.res.resourceSpecification.defaultUnitOfResource.id, 'resource specification default unit of resource ok')
    const rsId = resp.data.res.resourceSpecification.id

    // test simple links

    resp = await alice.graphQL(`
      mutation(
        $process: ProcessCreateParams!,
        $event: EconomicEventCreateParams!,
        $event2: EconomicEventCreateParams!,
        $resource: EconomicResourceCreateParams!,
        $resource2: EconomicResourceCreateParams!,
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
        e2: createEconomicEvent(event: $event2, newInventoriedResource: $resource2) {
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
      // with unit id specified
      event: {
        action: 'raise',
        resourceConformsTo: rsId,
        resourceQuantity: { hasNumericalValue: 1, hasUnit: u2Id },
        ...fillerProps,
      },
      resource: {
        name: 'langths of string',
      },
      // without unit id specified
      // should resort to default
      event2: {
        action: 'raise',
        resourceConformsTo: rsId,
        resourceQuantity: { hasNumericalValue: 1, hasUnit: undefined },
        ...fillerProps,
      },
      resource2: {
        name: 'resource that should use default unit',
      },
      commitment: {
        action: 'raise',
        resourceConformsTo: rsId,
        resourceQuantity: { hasNumericalValue: 1, hasUnit: u2Id },
        ...fillerProps,
      },
      intent: {
        action: 'raise',
        resourceConformsTo: rsId,
        resourceQuantity: { hasNumericalValue: 3, hasUnit: u2Id },
        ...fillerProps,
      },
    })
    await pause(100)

    t.ok(resp.data.p.process.id, 'referencing process created')
    t.ok(resp.data.e.economicEvent.id, 'referencing event created')
    t.ok(resp.data.e.economicResource.id, 'referencing resource created')
    t.ok(resp.data.e2.economicEvent.id, 'referencing second event created')
    t.ok(resp.data.e2.economicResource.id, 'referencing second resource created')
    t.ok(resp.data.c.commitment.id, 'referencing commitment created')
    t.ok(resp.data.i.intent.id, 'referencing intent created')
    const processId = resp.data.p.process.id
    const eventId = resp.data.e.economicEvent.id
    const resourceId = resp.data.e.economicResource.id
    const resource2Id = resp.data.e2.economicResource.id
    const commitmentId = resp.data.c.commitment.id
    const intentId = resp.data.i.intent.id

    resp = await alice.graphQL(`
    query {
      process: process(id: "${processId}") {
        basedOn {
          id
        }
      }
      economicEvent: economicEvent(id: "${eventId}") {
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
      economicResource: economicResource(id: "${resourceId}") {
        accountingQuantity {
          hasNumericalValue
          hasUnit {
            id
          }
        }
        onhandQuantity {
          hasNumericalValue
          hasUnit {
            id
          }
        }
        conformsTo {
          id
        }
        unitOfEffort {
          id
        }
      }
      resource2: economicResource(id: "${resource2Id}") {
        accountingQuantity {
          hasNumericalValue
          hasUnit {
            id
          }
        }
        onhandQuantity {
          hasNumericalValue
          hasUnit {
            id
          }
        }
        conformsTo {
          id
        }
        unitOfEffort {
          id
        }
      }
      commitment: commitment(id: "${commitmentId}") {
        action {
          id
        }
        resourceConformsTo {
          id
        }
      }
      intent: intent(id: "${intentId}") {
        action {
          id
        }
        resourceConformsTo {
          id
        }
      }
    }`)

    t.equal(resp.data.process.basedOn.id, psId, 'Process.basedOn reference OK')
    t.equal(resp.data.commitment.resourceConformsTo.id, rsId, 'Commitment.reesourceConformsTo reference OK')
    t.equal(resp.data.intent.resourceConformsTo.id, rsId, 'Intent.reesourceConformsTo reference OK')
    t.equal(resp.data.economicEvent.action.id, 'raise', 'EconomicEvent.action reference OK')
    t.equal(resp.data.commitment.action.id, 'raise', 'Commitment.action reference OK')
    t.equal(resp.data.intent.action.id, 'raise', 'Intent.action reference OK')
    t.equal(resp.data.economicEvent.resourceConformsTo.id, rsId, 'EconomicEvent.resourceConformsTo reference OK')
    t.equal(resp.data.economicEvent.resourceQuantity.hasUnit.label, 'kilos', 'Measure.hasUnit reference OK')
    t.equal(resp.data.economicResource.conformsTo.id, rsId, 'EconomicResource.conformsTo reference OK')
    t.equal(resp.data.economicResource.unitOfEffort.id, uId, 'EconomicResource.unitOfEffort reference OK')
    t.equal(resp.data.economicResource.accountingQuantity.hasUnit.id, u2Id, 'EconomicResource.accountingQuantity.hasUnit.id reference OK when set manually')
    t.equal(resp.data.economicResource.onhandQuantity.hasUnit.id, u2Id, 'EconomicResource.onhandQuantity.hasUnit.id reference OK when set manually')
    // the ones that should have been set from the defaultUnitOfResource
    t.equal(resp.data.resource2.accountingQuantity.hasUnit.id, u2Id, 'EconomicResource.accountingQuantity.hasUnit.id reference OK when inferred from spec')
    t.equal(resp.data.resource2.onhandQuantity.hasUnit.id, u2Id, 'EconomicResource.onhandQuantity.hasUnit.id reference OK when inferred from spec')

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
        resourceQuantity: { hasNumericalValue: 1, hasUnit: u2Id },
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
  } catch (e) {
    await alice.scenario.cleanUp()
    throw e
  }
  await alice.scenario.cleanUp()
})
