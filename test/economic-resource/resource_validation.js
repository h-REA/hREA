import test from 'tape'
import { pause } from '@holochain/tryorama'
import {
  buildPlayer,
  mockAddress,
  mockIdentifier,
} from '../init.js'

// required attributes, not involved with test logic
const testEventProps = {
  provider: mockAddress(),
  receiver: mockAddress(),
  hasPointInTime: '2019-11-19T04:29:55.056Z',
}
const kilograms = mockIdentifier()
const exampleEntry = {
  name: 'TRE',
  image: 'https://holochain.org/something',
  note: 'test resource specification',
}

test('EconomicResource classification fields validation', async (t) => {
  // display the filename for context in the terminal and use .warn
  // to override the tap testing log filters
  console.warn(`\n\n${import.meta.url}`)
  const alice = await buildPlayer(['observation', 'specification'])
  try {
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
        resourceQuantity: { hasNumericalValue: 1, hasUnit: kilograms },
        ...testEventProps,
      },
      r: {
        name: 'bad resource without ontological distinction capability',
      },
    })
    await pause(100)
    console.log('resp', JSON.stringify(resp))
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
        resourceQuantity: { hasNumericalValue: 1, hasUnit: kilograms },
        resourceClassifiedAs: ['some-classification-url'],
        ...testEventProps,
      },
      r: {
        name: 'good resource with ontological distinction capability',
      },
    })
    await pause(100)

    t.ok(resp.data.createEconomicEvent.economicEvent.id, 'creating resource with classification is OK')

    const createResp = await alice.graphQL(`
      mutation($rs: ResourceSpecificationCreateParams!) {
        res: createResourceSpecification(resourceSpecification: $rs) {
          resourceSpecification {
            id
            revisionId
          }
        }
      }
      `, {
      rs: exampleEntry,
    })
    await pause(100)

    const rsId = createResp.data.res.resourceSpecification.id

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
        resourceQuantity: { hasNumericalValue: 1, hasUnit: kilograms },
        resourceConformsTo: rsId,
        ...testEventProps,
      },
      r: {
        name: 'good resource with ontological distinction capability',
      },
    })
    await pause(100)

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
        resourceQuantity: { hasNumericalValue: 1, hasUnit: kilograms },
        ...testEventProps,
      },
      r: {
        name: 'good resource with ontological distinction capability',
        conformsTo: rsId,
      },
    })
    await pause(100)

    t.ok(resp.data.createEconomicEvent.economicEvent.id, 'creating resource with resource specification is OK')
  } catch (e) {
    await alice.scenario.cleanUp()
    throw e
  }
  await alice.scenario.cleanUp()
})
