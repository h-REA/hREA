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

const testEventProps = {
  provider: 'agentid-1-todo',
  receiver: 'agentid-2-todo',
  hasPointInTime: '2019-11-19T04:29:55.056Z',
  resourceClassifiedAs: ['todo-this-shouldnt-be-needed'],
  resourceQuantity: { hasNumericalValue: 1, hasUnit: 'dangling-unit-todo-tidy-up' },
}

runner.registerScenario('EconomicResource composition / containment functionality', async (s, t) => {
  const { alice } = await s.players({ alice: config }, true)
  const graphQL = buildGraphQL(alice)

  // SCENARIO: write initial records
  const resourceSpecificationId = 'dangling-resource-specification-todo-tidy-up'
  const inputEvent = {
    note: 'container resource instantiation event',
    action: 'raise',
    ...testEventProps,
  }
  const inputResource = {
    note: 'container resource',
    conformsTo: resourceSpecificationId,
  }
  const cResp1 = await alice.call('observation', 'economic_event', 'create_event', { event: inputEvent, new_inventoried_resource: inputResource })
  await s.consistency()
  const event1 = cResp1.Ok.economicEvent;
  const resource1 = cResp1.Ok.economicResource;
  t.ok(event1.id, 'event created successfully')
  t.ok(resource1.id, 'resource created successfully')
  const eventId1 = event1.id
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
  const cResp2 = await alice.call('observation', 'economic_event', 'create_event', { event: inputEvent2, new_inventoried_resource: inputResource2 })
  await s.consistency()
  t.ok(cResp2.Ok, 'internal resource created successfully')
  const resource2 = cResp2.Ok.economicResource;
  const resourceId2 = resource2.id

  let readResp = await alice.call('observation', 'economic_resource', 'get_resource', { address: resourceId1 })
  let readResource = readResp.Ok.economicResource
  t.ok(readResource.id, 'container resource retrieval OK')
  t.equal(readResource.contains && readResource.contains.length, 1, 'container resource reference inserted')
  t.equal(readResource.contains && readResource.contains[0], resourceId2, 'container resource reference OK')

  readResp = await alice.call('observation', 'economic_resource', 'get_resource', { address: resourceId2 })
  readResource = readResp.Ok.economicResource
  t.ok(readResource.id, 'contained resource retrieval OK')
  t.equal(readResource.containedIn, resourceId1, 'contained resource reference OK')


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
  const cResp3 = await alice.call('observation', 'economic_event', 'create_event', { event: inputEvent3, new_inventoried_resource: inputResource3 })
  await s.consistency()
  t.ok(cResp3.Ok, 'additional internal resource created successfully')
  const resource3 = cResp3.Ok.economicResource;
  const resourceId3 = resource3.id

  readResp = await alice.call('observation', 'economic_resource', 'get_resource', { address: resourceId1 })
  readResource = readResp.Ok.economicResource
  t.ok(readResource.id, 'container resource re-retrieval OK')
  t.equal(readResource.contains && readResource.contains.length, 2, 'container resource reference appended')
  t.equal(readResource.contains && readResource.contains[0], resourceId2, 'container resource reference A OK')
  t.equal(readResource.contains && readResource.contains[1], resourceId3, 'container resource reference B OK')


  // SCENARIO: update to remove links
  const updateResource3 = {
    id: resourceId3,
    containedIn: null,
    note: 'standalone resource',
  }
  const uResp3 = await alice.call('observation', 'economic_resource', 'update_resource', { resource: updateResource3 })
  await s.consistency()
  t.ok(uResp3.Ok, 'internal resource updated successfully')

  readResp = await alice.call('observation', 'economic_resource', 'get_resource', { address: resourceId1 })
  readResource = readResp.Ok.economicResource
  t.ok(readResource.id, 'container resource re-retrieval OK')
  t.equal(readResource.contains && readResource.contains.length, 1, 'container resource reference removed after update')
  t.equal(readResource.contains && readResource.contains[0], resourceId2, 'container resource remaining reference OK')



  // ASSERT: load records via GraphQL layer to test query endpoints

  readResp = await graphQL(`
  {
    container: economicResource(id: "${resourceId1}") {
      contains {
        id
      }
    }
    contained: economicResource(id: "${resourceId2}") {
      containedIn {
        id
      }
    }
  }`)

  t.equal(readResp.data.container.contains.length, 1, 'contains ref present in GraphQL API')
  t.equal(readResp.data.container.contains[0].id, resourceId2, 'contains ref OK in GraphQL API')
  t.equal(readResp.data.contained.containedIn.id, resourceId1, 'containedIn ref OK in GraphQL API')


  // SCENARIO: delete resource, check links are removed
  // :TODO: needs some thought
  // const dResp = await alice.call('observation', 'economic_resource', 'delete_resource', { address: resourceId3 })
  // await s.consistency()
  // t.ok(dResp.Ok, 'resource deleted successfully')

  // readResp = await alice.call('observation', 'economic_resource', 'get_resource', { address: resourceId1 })
  // readResource = readResp.Ok.economicResource
  // t.ok(readResource.id, 'container resource re-retrieval OK')
  // t.equal(readResource.contains && readResource.contains.length, 0, 'container resource reference removed after deletion')
})

runner.run()
