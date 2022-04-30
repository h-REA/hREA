const {
  buildConfig,
  buildRunner,
  buildPlayer,
  mockIdentifier,
  mockAgentId,
} = require('../init')

const runner = buildRunner()

const config = buildConfig()

const testEventProps = {
  action: 'raise',
  resourceClassifiedAs: ['some-resource-type'],
  resourceQuantity: { hasNumericalValue: 1, hasUnit: mockIdentifier(false) },
  provider: mockAgentId(false),
  receiver: mockAgentId(false),
  hasPointInTime: '2019-11-19T04:29:55.056Z',
}

runner.registerScenario('links can be written and read between DNAs', async (s, t) => {
  const { cells: [planning, observation] } = await buildPlayer(s, config, ['planning', 'observation'])

  // SCENARIO: write records
  const commitment = {
    note: 'a commitment to provide something',
    ...testEventProps,
  }
  const commitmentResponse = await planning.call('commitment', 'create_commitment', { commitment })
  t.ok(commitmentResponse.commitment && commitmentResponse.commitment.id, 'commitment created successfully')
  await s.consistency()
  const commitmentId = commitmentResponse.commitment.id

  const event = {
    note: 'test event which is fulfilling a commitment',
    action: 'produce',
    ...testEventProps,
  }
  const eventResp = await observation.call('economic_event', 'create_economic_event', { event })
  t.ok(eventResp.economicEvent && eventResp.economicEvent.id, 'event created successfully')
  await s.consistency()
  const eventId = eventResp.economicEvent.id

  const fulfillment = {
    fulfills: commitmentId,
    fulfilledBy: eventId,
    note: 'fulfillment indicating the relationship',
  }
  const fulfillmentResp = await planning.call('fulfillment', 'create_fulfillment', { fulfillment })
  t.ok(fulfillmentResp.fulfillment && fulfillmentResp.fulfillment.id, 'fulfillment created successfully')
  await s.consistency()
  const fulfillmentId = fulfillmentResp.fulfillment.id

  // ASSERT: check fulfillment in originating network
  let readResponse = await planning.call('fulfillment', 'get_fulfillment', { address: fulfillmentId })
  t.deepEqual(readResponse.fulfillment.fulfilledBy, eventId, 'Fulfillment.fulfilledBy reference saved')
  t.deepEqual(readResponse.fulfillment.fulfills, commitmentId, 'Fulfillment.fulfills reference saved')

  // ASSERT: check event
  readResponse = await observation.call('economic_event', 'get_economic_event', { address: eventId })
  console.log('readResponse', readResponse)
  t.ok(readResponse.economicEvent.fulfills, 'EconomicEvent.fulfills value present')
  t.equal(readResponse.economicEvent.fulfills.length, 1, 'EconomicEvent.fulfills reference saved')
  t.deepEqual(readResponse.economicEvent.fulfills[0][1], fulfillmentId[1], 'EconomicEvent.fulfills reference OK')

  // ASSERT: check commitment
  readResponse = await planning.call('commitment', 'get_commitment', { address: commitmentId })
  t.ok(readResponse.commitment.fulfilledBy, 'Commitment.fulfilledBy reciprocal value present')
  t.equal(readResponse.commitment.fulfilledBy.length, 1, 'Commitment.fulfilledBy reciprocal reference saved')
  t.deepEqual(readResponse.commitment.fulfilledBy[0], fulfillmentId, 'Commitment.fulfilledBy reciprocal fulfillment reference OK')

  // ASSERT: check fulfillment in target network
  readResponse = await observation.call('fulfillment', 'get_fulfillment', { address: fulfillmentId })
  t.deepEqual(readResponse.fulfillment.fulfilledBy, eventId, 'Fulfillment.fulfilledBy reference saved')
  t.deepEqual(readResponse.fulfillment.fulfills, commitmentId, 'Fulfillment.fulfills reference saved')

  // ASSERT: check forward query indexes
  readResponse = await planning.call('fulfillment_index', 'query_fulfillments', { params: { fulfills: commitmentId } })
  t.equal(readResponse.length, 1, 'read fulfillments by commitment OK')
  t.deepEqual(readResponse.Ok[0].fulfillment.id, fulfillmentId, 'Fulfillment.fulfills indexed correctly')

  // ASSERT: check reverse query indexes
  readResponse = await observation.call('fulfillment_index', 'query_fulfillments', { params: { fulfilledBy: eventId } })
  t.equal(readResponse.length, 1, 'read fulfillments by event OK')
  t.deepEqual(readResponse.Ok[0].fulfillment.id, fulfillmentId, 'Fulfillment.fulfilledBy indexed correctly')



  // SCENARIO: add another fulfillment
  const fulfillment2 = {
    fulfills: commitmentId,
    fulfilledBy: eventId,
    note: 'fulfillment indicating another relationship',
  }
  const fulfillmentResp2 = await planning.call('fulfillment', 'create_fulfillment', { fulfillment: fulfillment2 })
  t.ok(fulfillmentResp2.fulfillment && fulfillmentResp2.fulfillment.id, 'additional fulfillment created successfully')
  await s.consistency()
  const fulfillmentId2 = fulfillmentResp2.fulfillment.id

  // ASSERT: check forward query indices
  readResponse = await planning.call('fulfillment_index', 'query_fulfillments', { params: { fulfills: commitmentId } })
  t.equal(readResponse.length, 2, 'appending fulfillments for read OK')
  t.deepEqual(readResponse.Ok[0].fulfillment.id, fulfillmentId, 'fulfillment 1 indexed correctly')
  t.deepEqual(readResponse.Ok[1].fulfillment.id, fulfillmentId2, 'fulfillment 2 indexed correctly')

  // ASSERT: ensure append is working on the event read side
  readResponse = await observation.call('economic_event', 'get_economic_event', { address: eventId })
  t.equal(readResponse.economicEvent.fulfills.length, 2, 'EconomicEvent.fulfills appending OK')
  t.deepEqual(readResponse.economicEvent.fulfills[0], fulfillmentId, 'EconomicEvent.fulfills reference 1 OK')
  t.deepEqual(readResponse.economicEvent.fulfills[1], fulfillmentId2, 'EconomicEvent.fulfills reference 2 OK')

  // ASSERT: ensure query indices on the event read side
  readResponse = await observation.call('economic_event_index', 'query_economic_events', { params: { fulfills: fulfillmentId } })
  t.equal(readResponse.length, 1, 'appending fulfillments for event query OK')
  t.deepEqual(readResponse.Ok[0].economicEvent.id, eventId, 'event query indexed correctly')

  // ASSERT: ensure append is working on the commitment read side
  readResponse = await planning.call('commitment', 'get_commitment', { address: commitmentId })
  t.equal(readResponse.commitment.fulfilledBy.length, 2, 'Commitment.fulfilledBy appending OK')
  t.deepEqual(readResponse.commitment.fulfilledBy[0], fulfillmentId, 'Commitment.fulfilledBy reference 1 OK')
  t.deepEqual(readResponse.commitment.fulfilledBy[1], fulfillmentId2, 'Commitment.fulfilledBy reference 2 OK')

  // ASSERT: ensure query indices on the commitment read side
  readResponse = await planning.call('commitment_index', 'query_commitments', { params: { fulfilledBy: fulfillmentId } })
  t.equal(readResponse.length, 1, 'appending fulfillments for commitment query OK')
  t.deepEqual(readResponse.Ok[0].commitment.id, commitmentId, 'commitment query indexed correctly')

  // ASSERT: check reciprocal query indexes
  readResponse = await observation.call('fulfillment_index', 'query_fulfillments', { params: { fulfilledBy: eventId } })
  t.equal(readResponse.length, 2, 'read fulfillments by event OK')
  t.deepEqual(readResponse.Ok[0].fulfillment.id, fulfillmentId, 'fulfillment 1 indexed correctly')
  t.deepEqual(readResponse.Ok[1].fulfillment.id, fulfillmentId2, 'fulfillment 2 indexed correctly')
})

runner.run()
