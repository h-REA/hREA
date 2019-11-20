const {
  getDNA,
  buildConfig,
  buildRunner,
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
}

runner.registerScenario('EconomicResource & EconomicEvent record interactions', async (s, t) => {
  const { alice } = await s.players({ alice: config }, true)

  // SCENARIO: write initial records
  const process = {
    name: 'test process for linking logic',
  }
  const pResp = await alice.call('observation', 'process', 'create_process', { process })
  t.ok(pResp.Ok.process && pResp.Ok.process.id, 'process created successfully')
  await s.consistency()
  const processId = pResp.Ok.process.id

  const resourceUnitId = 'dangling-unit-todo-tidy-up'
  const resourceSpecificationId = 'dangling-resource-specification-todo-tidy-up'
  const inputEvent = {
    note: 'test resource instantiation event',
    action: 'raise',
    resourceClassifiedAs: ['http://www.productontology.org/doc/Apple.ttl'],
    resourceQuantity: { hasNumericalValue: 8, hasUnit: resourceUnitId },
    ...testEventProps,
  }
  const inputResource = {
    note: 'test resource observed in inventory',
    conformsTo: resourceSpecificationId,
  }
  const cResp1 = await alice.call('observation', 'economic_event', 'create_event', { event: inputEvent, new_inventoried_resource: inputResource })

  await s.consistency()
  const event = cResp1.Ok.economicEvent;
  const resource = cResp1.Ok.economicResource;
  t.ok(event.id, 'event created successfully')
  t.ok(resource.id, 'resource created successfully')
  t.equal(event.resourceInventoriedAs, resource.id, 'resource event link OK')
  t.equal(resource.accountingQuantity.hasNumericalValue, 8, 'resource initial quantity OK')
  const eventId = event.id
  const resourceId = resource.id


  // SCENARIO: resource field initialisation
  // :TODO: 'unit of effort is set from the event ResourceSpecification\'s unit of effort'
  // :TODO: 'unit of effort overrides the ResourceSpecification unit of effort if indicated in the resource'
  t.deepEqual(event.resourceClassifiedAs, resource.classifiedAs, 'classification is set from the linked event\'s resource classifications')
  // :TODO: 'stage should be set to the ProcessSpecification of the output process of the event'
  // :TODO: should only modify actions cause this behaviour?



  // SCENARIO: resource move events
  let newEvent = {
    resourceInventoriedAs: resourceId,
    action: 'move',
    atLocation: 'some-location-id-todo',
    resourceQuantity: { hasNumericalValue: 8, hasUnit: resourceUnitId },
    ...testEventProps,
  }
  let eventResp = await alice.call('observation', 'economic_event', 'create_event', { event: newEvent })
  await s.consistency()
  t.ok(eventResp.Ok, 'appending move event OK')

  let readResp = await alice.call('observation', 'economic_resource', 'get_resource', { address: resourceId })
  let readResource = readResp.Ok.economicResource
  t.equal(readResource.currentLocation, 'some-location-id-todo', 'MOVE events update the resource location if a new location is provided')



  // SCENARIO: resource state
  newEvent = {
    resourceInventoriedAs: resourceId,
    action: 'pass',
    outputOf: processId,
    resourceQuantity: { hasNumericalValue: 8, hasUnit: resourceUnitId },
    ...testEventProps,
  }
  eventResp = await alice.call('observation', 'economic_event', 'create_event', { event: newEvent })
  await s.consistency()
  t.ok(eventResp.Ok, 'appending event OK')
  readResp = await alice.call('observation', 'economic_resource', 'get_resource', { address: resourceId })
  readResource = readResp.Ok.economicResource
  t.ok(readResource.id, 'resource retrieval OK')
  t.equal(readResource.state, 'pass', 'state should be set to initial action if creating event is PASS or FAIL')



  // SCENARIO: resource math basics
  newEvent = {
    resourceInventoriedAs: resourceId,
    action: 'raise',
    resourceQuantity: { hasNumericalValue: 8, hasUnit: resourceUnitId },
    ...testEventProps,
  }
  eventResp = await alice.call('observation', 'economic_event', 'create_event', { event: newEvent })
  await s.consistency()
  t.ok(eventResp.Ok, 'appending event OK')

  readResp = await alice.call('observation', 'economic_resource', 'get_resource', { address: resourceId })
  readResource = readResp.Ok.economicResource
  t.ok(readResource.id, 'resource retrieval OK')
  t.deepEqual(readResource.accountingQuantity, { hasNumericalValue: 16, hasUnit: resourceUnitId }, 'incrementing events increase the accounting quantity of a resource')
  t.deepEqual(readResource.onhandQuantity, { hasNumericalValue: 16, hasUnit: resourceUnitId }, 'incrementing events increase the on-hand quantity of a resource')

  newEvent = {
    resourceInventoriedAs: resourceId,
    action: 'lower',
    resourceQuantity: { hasNumericalValue: 2, hasUnit: resourceUnitId },
    ...testEventProps,
  }
  eventResp = await alice.call('observation', 'economic_event', 'create_event', { event: newEvent })
  await s.consistency()
  t.ok(eventResp.Ok, 'appending event OK')

  readResp = await alice.call('observation', 'economic_resource', 'get_resource', { address: resourceId })
  readResource = readResp.Ok.economicResource
  t.deepEqual(readResource.accountingQuantity, { hasNumericalValue: 14, hasUnit: resourceUnitId }, 'decrementing events decrease the accounting quantity of a resource')
  t.deepEqual(readResource.onhandQuantity, { hasNumericalValue: 14, hasUnit: resourceUnitId }, 'decrementing events decrease the on-hand quantity of a resource')

  newEvent = {
    resourceInventoriedAs: resourceId,
    action: 'transfer-custody',
    resourceQuantity: { hasNumericalValue: 1, hasUnit: resourceUnitId },
    ...testEventProps,
  }
  eventResp = await alice.call('observation', 'economic_event', 'create_event', { event: newEvent })
  await s.consistency()
  t.ok(eventResp.Ok, 'appending event OK')

  readResp = await alice.call('observation', 'economic_resource', 'get_resource', { address: resourceId })
  readResource = readResp.Ok.economicResource
  t.deepEqual(readResource.accountingQuantity, { hasNumericalValue: 14, hasUnit: resourceUnitId }, 'transfer-custody does not update accountingQuantity')
  t.deepEqual(readResource.onhandQuantity, { hasNumericalValue: 13, hasUnit: resourceUnitId }, 'transfer-custody updates onhandQuantity')

  newEvent = {
    resourceInventoriedAs: resourceId,
    action: 'transfer-all-rights',
    resourceQuantity: { hasNumericalValue: 1, hasUnit: resourceUnitId },
    ...testEventProps,
  }
  eventResp = await alice.call('observation', 'economic_event', 'create_event', { event: newEvent })
  await s.consistency()
  t.ok(eventResp.Ok, 'appending event OK')

  readResp = await alice.call('observation', 'economic_resource', 'get_resource', { address: resourceId })
  readResource = readResp.Ok.economicResource
  t.deepEqual(readResource.accountingQuantity, { hasNumericalValue: 13, hasUnit: resourceUnitId }, 'transfer-all-rights updates accountingQuantity')
  t.deepEqual(readResource.onhandQuantity, { hasNumericalValue: 13, hasUnit: resourceUnitId }, 'transfer-all-rights does not update onhandQuantity')



  // SCENARIO: secondary resource for inventory transfer tests
  const inputEvent2 = {
    note: 'event to instantiate receiving resource',
    action: 'raise',
    resourceClassifiedAs: ['http://www.productontology.org/doc/Apple.ttl'],
    resourceQuantity: { hasNumericalValue: 0, hasUnit: resourceUnitId },
    ...testEventProps,
  }
  const inputResource2 = {
    note: 'receiver test resource',
    conformsTo: resourceSpecificationId,
  }
  const cResp2 = await alice.call('observation', 'economic_event', 'create_event', { event: inputEvent2, new_inventoried_resource: inputResource2 })
  await s.consistency()
  const event2 = cResp2.Ok.economicEvent;
  const resource2 = cResp2.Ok.economicResource;
  t.ok(event2.id, '2nd event created successfully')
  t.ok(resource2.id, '2nd resource created successfully')
  const resourceId2 = resource2.id


  // SCENARIO: resource transfer behaviour
  newEvent = {
    resourceInventoriedAs: resourceId,
    toResourceInventoriedAs: resourceId2,
    action: 'transfer-complete',
    resourceQuantity: { hasNumericalValue: 3, hasUnit: resourceUnitId },
    ...testEventProps,
  }
  eventResp = await alice.call('observation', 'economic_event', 'create_event', { event: newEvent })
  await s.consistency()
  t.ok(eventResp.Ok, 'appending event OK')

  readResp = await alice.call('observation', 'economic_resource', 'get_resource', { address: resourceId })
  readResource = readResp.Ok.economicResource
  t.deepEqual(readResource.accountingQuantity, { hasNumericalValue: 10, hasUnit: resourceUnitId }, 'transfer events decrease the accounting quantity of the sending resource')
  t.deepEqual(readResource.onhandQuantity, { hasNumericalValue: 10, hasUnit: resourceUnitId }, 'transfer events decrease the onhand quantity of the sending resource')

  readResp = await alice.call('observation', 'economic_resource', 'get_resource', { address: resourceId2 })
  readResource = readResp.Ok.economicResource
  t.deepEqual(readResource.accountingQuantity, { hasNumericalValue: 3, hasUnit: resourceUnitId }, 'transfer events increase the accounting quantity of the receiving resource')
  t.deepEqual(readResource.onhandQuantity, { hasNumericalValue: 3, hasUnit: resourceUnitId }, 'transfer events increase the onhand quantity of the receiving resource')


  // SCENARIO: field update tests for event bindings
  // :TODO: 'should take on the unit of effort from the most recent event\'s related ResourceSpecification'
  // :TODO: 'should take on the stage of the most recent event\'s related output ProcessSpecification'


  newEvent = {
    resourceInventoriedAs: resourceId,
    action: 'fail',
    outputOf: processId,
    resourceQuantity: { hasNumericalValue: 3, hasUnit: resourceUnitId },
    ...testEventProps,
  }
  eventResp = await alice.call('observation', 'economic_event', 'create_event', { event: newEvent })
  await s.consistency()
  t.ok(eventResp.Ok, 'appending event OK')

  readResp = await alice.call('observation', 'economic_resource', 'get_resource', { address: resourceId })
  readResource = readResp.Ok.economicResource
  t.equal(readResource.state, 'fail', 'should take on the last PASS | FAIL event action as its state')

  newEvent = {
    resourceInventoriedAs: resourceId,
    resourceClassifiedAs: ['http://www.productontology.org/doc/Manure_spreader.ttl'],
    action: 'raise',
    resourceQuantity: { hasNumericalValue: 1, hasUnit: resourceUnitId },
    ...testEventProps,
  }
  eventResp = await alice.call('observation', 'economic_event', 'create_event', { event: newEvent })
  await s.consistency()
  t.ok(eventResp.Ok, 'appending event OK')

  readResp = await alice.call('observation', 'economic_resource', 'get_resource', { address: resourceId })
  readResource = readResp.Ok.economicResource
  t.deepEqual(readResource.classifiedAs,
    ['http://www.productontology.org/doc/Apple.ttl', 'http://www.productontology.org/doc/Manure_spreader.ttl'],
    'creating an associated event with a new ResourceClassification type appends the classification to the resource\'s existing classifications'
  )
})

runner.run()
