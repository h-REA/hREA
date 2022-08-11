import test from 'tape'
import { pause } from '@holochain/tryorama'
import {
  buildPlayer,
  serializeId, // :NOTE: needed due to mixing of direct API and GraphQL in same test
  mockAddress,
  mockIdentifier,
} from '../init.js'
import { deserializeId } from '@valueflows/vf-graphql-holochain/build/connection.js'

const testEventProps = {
  provider: mockAddress(false),
  receiver: mockAddress(false),
  hasPointInTime: '2019-11-19T04:29:55.056Z',
  resourceClassifiedAs: ['todo-this-shouldnt-be-needed'],
  resourceQuantity: { hasNumericalValue: 1, hasUnit: mockIdentifier(false) },
}

const exampleEntry = {
  name: 'TRE',
  image: 'https://holochain.org/something',
  note: 'test resource specification',
}

test('EconomicResource composition / containment functionality', async (t) => {
  // display the filename for context in the terminal and use .warn
  // to override the tap testing log filters
  console.warn(`\n\n${import.meta.url}`)
  const alice = await buildPlayer(['observation', 'specification'])
  try {
    const { graphQL, cells: [observation] } = alice

    // SCENARIO: write initial records
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

    const resourceSpecificationId = deserializeId(createResp.data.res.resourceSpecification.id)
    const inputEvent = {
      note: 'container resource instantiation event',
      action: 'raise',
      ...testEventProps,
    }
    const inputResource = {
      note: 'container resource',
      conformsTo: resourceSpecificationId,
    }
    const cResp1 = await observation.call('economic_event', 'create_economic_event', { event: inputEvent, newInventoriedResource: inputResource })
    await pause(100)
    const event1 = cResp1.economicEvent
    const resource1 = cResp1.economicResource
    t.ok(event1.id, 'event created successfully')
    t.ok(resource1.id, 'resource created successfully')
    // const eventId1 = event1.id
    const resourceId1 = resource1.id

    const inputEvent2 = {
      note: 'contained resource instantiation event',
      action: 'raise',
      ...testEventProps,
    }
    const inputResource2 = {
      containedIn: resourceId1,
      conformsTo: resourceSpecificationId,
      note: 'internal resource',
    }
    const cResp2 = await observation.call('economic_event', 'create_economic_event', { event: inputEvent2, newInventoriedResource: inputResource2 })
    await pause(100)
    t.ok(cResp2.economicResource, 'internal resource created successfully')
    const resource2 = cResp2.economicResource
    const resourceId2 = resource2.id

    let readResp = await observation.call('economic_resource', 'get_economic_resource', { address: resourceId1 })
    let readResource = readResp.economicResource
    t.ok(readResource.id, 'container resource retrieval OK')
    t.equal(readResource.contains && readResource.contains.length, 1, 'container resource reference inserted')
    t.deepLooseEqual(readResource.contains && readResource.contains[0], resourceId2, 'container resource reference OK')

    readResp = await observation.call('economic_resource', 'get_economic_resource', { address: resourceId2 })
    readResource = readResp.economicResource
    t.ok(readResource.id, 'contained resource retrieval OK')
    t.deepLooseEqual(readResource.containedIn, resourceId1, 'contained resource reference OK')

    // SCENARIO: add more internal resources
    const inputEvent3 = {
      note: 'contained resource instantiation event 2',
      action: 'raise',
      ...testEventProps,
    }
    const inputResource3 = {
      containedIn: resourceId1,
      conformsTo: resourceSpecificationId,
      note: 'another internal resource',
    }
    const cResp3 = await observation.call('economic_event', 'create_economic_event', { event: inputEvent3, newInventoriedResource: inputResource3 })
    await pause(100)
    t.ok(cResp3.economicResource, 'additional internal resource created successfully')
    const resource3 = cResp3.economicResource
    const resourceId3 = resource3.id

    readResp = await observation.call('economic_resource', 'get_economic_resource', { address: resourceId1 })

    readResource = readResp.economicResource
    t.deepLooseEqual(readResource.id, resourceId1, 'container resource re-retrieval OK')
    t.equal(readResource.contains && readResource.contains.length, 2, 'container resource reference appended')
    t.deepLooseEqual(readResource.contains && readResource.contains[0], resourceId2, 'container resource reference B OK')
    t.deepLooseEqual(readResource.contains && readResource.contains[1], resourceId3, 'container resource reference A OK')

    // SCENARIO: update to remove links
    const updateResource3 = {
      revisionId: resource3.revisionId,
      containedIn: null,
      note: 'standalone resource',
    }
    const uResp3 = await observation.call('economic_resource', 'update_economic_resource', { resource: updateResource3 })
    await pause(100)
    t.ok(uResp3.economicResource, 'internal resource updated successfully')

    readResp = await observation.call('economic_resource', 'get_economic_resource', { address: resourceId1 })
    readResource = readResp.economicResource
    t.ok(readResource.id, 'container resource re-retrieval OK')
    t.equal(readResource.contains && readResource.contains.length, 1, 'container resource reference removed after update')
    t.deepLooseEqual(readResource.contains && readResource.contains[0], resourceId2, 'container resource remaining reference OK')

    // ASSERT: load records via GraphQL layer to test query endpoints

    await pause(100)
    readResp = await graphQL(`
    {
      container: economicResource(id: "${serializeId(resourceId1)}") {
        contains {
          id
        }
      }
      contained: economicResource(id: "${serializeId(resourceId2)}") {
        containedIn {
          id
        }
      }
    }`)

    t.equal(readResp.data.container.contains.length, 1, 'contains ref present in GraphQL API')
    t.equal(readResp.data.container.contains[0].id, serializeId(resourceId2), 'contains ref OK in GraphQL API')
    t.equal(readResp.data.contained.containedIn.id, serializeId(resourceId1), 'containedIn ref OK in GraphQL API')
  } catch (e) {
    await alice.scenario.cleanUp()
    throw e
  }
  await alice.scenario.cleanUp()
})
