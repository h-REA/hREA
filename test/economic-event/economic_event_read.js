import test from 'tape'
import { pause } from '@holochain/tryorama'
import {
  buildPlayer,
  mockAddress,
  mockIdentifier,
} from '../init.js'

const testEventProps = {
  action: 'raise',
  provider: mockAddress(),
  receiver: mockAddress(),
  resourceQuantity: { hasNumericalValue: 1.0, hasUnit: mockIdentifier() },
}

test('Event/Resource list APIs', async (t) => {
  // display the filename for context in the terminal and use .warn
  // to override the tap testing log filters
  console.warn(`\n\n${import.meta.url}`)
  const alice = await buildPlayer(['combined'])
  try {
    const exampleEntry = {
      name: 'TRE',
      image: 'https://holochain.org/something',
      note: 'test resource specification',
    }

    let createResp = await alice.graphQL(`
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

    t.ok(createResp.data.res.resourceSpecification.id, 'record created')
    const rsId = createResp.data.res.resourceSpecification.id

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
        resourceConformsTo: rsId,
        ...testEventProps,
      },
      r1: { note: 'resource A' },
      e2: {
        resourceClassifiedAs: ['another-type-of-resource'],
        hasPointInTime: new Date(),
        resourceConformsTo: rsId,
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

    // read all resources
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
              note
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
    
    // resp = await alice.graphQL(`{
    //     economicEvents(last: 10) {
    //       pageInfo {
    //         startCursor
    //         endCursor
    //       }
    //       edges {
    //         cursor
    //         node {
    //           id
    //           resourceClassifiedAs
    //           hasPointInTime
    //           note
    //         }
    //       }
    //     }
    //   }`)

    console.log('events:', JSON.stringify(resp.data))
    t.equal(resp.data.economicEvents.edges.length, 2, 'two events found')

    // test inputOf process
    const process = {
      name: 'test process',
    }
    const pResp = await alice.graphQL(`
      mutation($p: ProcessCreateParams!) {
        res: createProcess(process: $p) {
          process {
            id
          }
        }
      }
      `, {
      p: process,
    })
    t.ok(pResp.data.res.process.id, 'process created')
    const processId = pResp.data.res.process.id

    const economicEventWithProcess = {
      resourceClassifiedAs: ['some-type-of-resource'],
      hasPointInTime: new Date(),
      resourceConformsTo: rsId,
      ...testEventProps,
      action: 'pickup',
      inputOf: processId,
    }

    const eventResp = await alice.graphQL(`
      mutation($e: EconomicEventCreateParams!) {
        res: createEconomicEvent(event: $e) {
          economicEvent {
            id
            inputOf {
              id
              name
            }
          }
        }
      }
      `, {
      e: economicEventWithProcess,
    })

    console.log('event with process:', JSON.stringify(eventResp.data))

    t.ok(eventResp.data.res.economicEvent.id, 'event with process created')

    // create fulfillment
    const commitment = {
      action: 'raise',
      due: new Date(Date.now() + 86400000),
      resourceClassifiedAs: ['some-resource-type'],
      resourceQuantity: { hasNumericalValue: 1, hasUnit: mockIdentifier() },
      provider: mockAddress(),
      receiver: mockAddress(),
    }
    const cResp = await alice.graphQL(`
      mutation($c: CommitmentCreateParams!) {
        res: createCommitment(commitment: $c) {
          commitment {
            id
          }
        }
      }
      `, {
      c: commitment,
    })

    const fulfillment = {
      note: 'test fulfillment',
      fulfilledBy: eventResp.data.res.economicEvent.id,
      fulfills: cResp.data.res.commitment.id,
    }
    const fResp = await alice.graphQL(`
      mutation($f: FulfillmentCreateParams!) {
        res: createFulfillment(fulfillment: $f) {
          fulfillment {
            id
            fulfilledBy {
              id
              note
            }
            fulfills {
              id
              note
            }
          }
        }
      }
      `, {
      f: fulfillment,
    })

    console.log('fulfillment:', JSON.stringify(fResp.data))
    t.ok(fResp.data.res.fulfillment.id, 'fulfillment created')

    // return economic event with fulfillment
    const eventQueryResp = await alice.graphQL(`
      query {
        economicEvents(last: 10) {
          edges {
            node {
              id
              fulfills {
                id
                note
              }
            }
          }
        }
      }
    `)

    console.log('event with fulfillment:', JSON.stringify(eventQueryResp.data))
    t.ok(eventQueryResp.data.economicEvents.edges[0].node.fulfilledBy, 'event with fulfillment returned')

} catch (e) {
    await alice.scenario.cleanUp()
    throw e
  }
  await alice.scenario.cleanUp()
})
