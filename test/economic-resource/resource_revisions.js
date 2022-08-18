import test from 'tape'
import { pause } from '@holochain/tryorama'
import {
  buildPlayer,
  serializeId, // :NOTE: needed due to mixing of direct API and GraphQL in same test
  mockAddress,
  mockIdentifier,
} from '../init.js'

const testEventProps = {
  action: 'raise',
  provider: mockAddress(),
  receiver: mockAddress(),
  hasPointInTime: '2019-11-19T04:29:55.056Z',
  resourceClassifiedAs: ['test-classification'],
  resourceQuantity: { hasNumericalValue: 1, hasUnit: mockIdentifier() },
}

test('EconomicResource historical data', async (t) => {
  // display the filename for context in the terminal and use .warn
  // to override the tap testing log filters
  console.warn(`\n\n${import.meta.url}`)

  const alice = await buildPlayer(['observation'])
  try {
    const { graphQL, cells: [observation] } = alice

    let resp = await graphQL(`
      mutation($e: EconomicEventCreateParams!, $r: EconomicResourceCreateParams) {
        createEconomicEvent(event: $e, newInventoriedResource: $r) {
          economicEvent {
            id
          }
          economicResource {
            id
            revisionId
          }
        }
      }
    `, {
      e: testEventProps,
      r: {
        name: 'test resource',
      },
    })
    await pause(100)
    t.ok(resp.data.createEconomicEvent.economicResource.id, 'resource instantiated OK')
    const resourceId = resp.data.createEconomicEvent.economicResource.id
    const revisionId1 = resp.data.createEconomicEvent.economicResource.revisionId

    resp = await graphQL(`
      mutation($e: EconomicEventCreateParams!) {
        createEconomicEvent(event: $e) {
          economicEvent {
            id
          }
        }
      }
    `, {
      e: {
        resourceInventoriedAs: resourceId,
        ...testEventProps
      },
      r: {
        name: 'test resource',
      },
    })
    await pause(100)
    t.ok(resp.data.createEconomicEvent.economicEvent.id, 'second event OK')

    resp = await graphQL(`
      mutation($e: EconomicEventCreateParams!) {
        createEconomicEvent(event: $e) {
          economicEvent {
            id
          }
        }
      }
    `, {
      e: {
        resourceInventoriedAs: resourceId,
        ...testEventProps
      },
      r: {
        name: 'test resource',
      },
    })
    await pause(100)
    t.ok(resp.data.createEconomicEvent.economicEvent.id, 'third event OK')

    resp = await graphQL(`
      query {
        economicResource(id: "${resourceId}") {
          id
          revisionId
          meta {
            previousRevision {
              id
            }
          }
        }
      }
    `)
    t.ok(resp.data.economicResource.id, 'resource queried successfully')
    const revisionId3 = resp.data.economicResource.revisionId

    resp = await graphQL(`
      query {
        economicResource(id: "${resourceId}") {
          revision(revisionId: "${revisionId3}") {
            id
            revisionId
            meta {
              previousRevision {
                id
              }
            }
          }
        }
      }
    `)
    t.equal(resp.data.economicResource.revision.id, resourceId, 'revisions have same ID as master record')
    t.equal(resp.data.economicResource.revision.revisionId, revisionId3, 'latest revision matches that queried')
    t.notEqual(resp.data.economicResource.revision.meta.previousRevision.id, revisionId3, 'previous to latest revision differs from latest')
    const revisionId2 = resp.data.economicResource.revision.meta.previousRevision.id

    resp = await graphQL(`
      query {
        economicResource(id: "${resourceId}") {
          revision(revisionId: "${revisionId2}") {
            id
            revisionId
            meta {
              previousRevision {
                id
              }
            }
          }
        }
      }
    `)
    t.equal(resp.data.economicResource.revision.revisionId, revisionId2, 'previous to latest revision matches that queried')
    t.notEqual(resp.data.economicResource.revision.meta.previousRevision.id, revisionId2, 'previous to previous to latest revision differs from previous to latest')
    t.equal(resp.data.economicResource.revision.meta.previousRevision.id, revisionId1, 'iterating backward through revisions leads to first revision')

    resp = await graphQL(`
      query {
        economicResource(id: "${resourceId}") {
          revision(revisionId: "${revisionId1}") {
            id
            revisionId
            meta {
              previousRevision {
                id
              }
            }
          }
        }
      }
    `)
    t.equal(resp.data.economicResource.revision.meta.previousRevision, null, 'there is no previous revision for first revision')
  } catch (e) {
    await alice.scenario.cleanUp()
    throw e
  }
  await alice.scenario.cleanUp()
})
