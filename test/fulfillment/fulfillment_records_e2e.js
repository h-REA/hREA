const {
  getDNA,
  buildConfig,
  runner,
} = require('../init')

const config = buildConfig({
  observation: getDNA('observation'),
  planning: getDNA('planning'),
}, {
  vf_observation: ['planning', 'observation'],
})

runner.registerScenario('links can be written and read between DNAs', async (s, t) => {
  const { alice } = await s.players({ alice: config }, true)

  // SCENARIO: write records
  const commitment = {
    note: 'a commitment to provide something',
  }
  const commitmentResponse = await alice.call('planning', 'commitment', 'create_commitment', { commitment })
  t.ok(commitmentResponse.Ok.commitment && commitmentResponse.Ok.commitment.id, 'commitment created successfully')
  await s.consistency()
  const commitmentId = commitmentResponse.Ok.commitment.id

  const event = {
    note: 'test event which is fulfilling a commitment',
    action: 'produce',
  }
  const eventResp = await alice.call('observation', 'economic_event', 'create_event', { event })
  t.ok(eventResp.Ok.economicEvent && eventResp.Ok.economicEvent.id, 'event created successfully')
  await s.consistency()
  const eventId = eventResp.Ok.economicEvent.id

  const fulfillment = {
    fulfills: commitmentId,
    fulfilledBy: eventId,
    note: 'fulfillment indicating the relationship',
  }
  const fulfillmentResp = await alice.call('planning', 'fulfillment', 'create_fulfillment', { fulfillment })
  t.ok(fulfillmentResp.Ok.fulfillment && fulfillmentResp.Ok.fulfillment.id, 'fulfillment created successfully')
  await s.consistency()
  const fulfillmentId = fulfillmentResp.Ok.fulfillment.id

  // ASSERT: check fulfillment in originating network
  let readResponse = await alice.call('planning', 'fulfillment', 'get_fulfillment', { address: fulfillmentId })
  t.equal(readResponse.Ok.fulfillment.fulfilledBy, eventId, 'Fulfillment.fulfilledBy reference saved')
  t.equal(readResponse.Ok.fulfillment.fulfills, commitmentId, 'Fulfillment.fulfills reference saved')

  // ASSERT: check event
  readResponse = await alice.call('observation', 'economic_event', 'get_event', { address: eventId })
  t.ok(readResponse.Ok.economicEvent.fulfills, 'EconomicEvent.fulfills value present')
  t.equal(readResponse.Ok.economicEvent.fulfills.length, 1, 'EconomicEvent.fulfills reference saved')
  t.equal(readResponse.Ok.economicEvent.fulfills[0], fulfillmentId, 'EconomicEvent.fulfills reference OK')

  // ASSERT: check commitment
  readResponse = await alice.call('planning', 'commitment', 'get_commitment', { address: commitmentId })
  t.ok(readResponse.Ok.commitment.fulfilledBy, 'Commitment.fulfilledBy reciprocal value present')
  t.equal(readResponse.Ok.commitment.fulfilledBy.length, 1, 'Commitment.fulfilledBy reciprocal reference saved')
  t.equal(readResponse.Ok.commitment.fulfilledBy[0], fulfillmentId, 'Commitment.fulfilledBy reciprocal fulfillment reference OK')

  // ASSERT: check fulfillment in target network
  readResponse = await alice.call('observation', 'fulfillment', 'get_fulfillment', { address: fulfillmentId })
  t.equal(readResponse.Ok.fulfillment.fulfilledBy, eventId, 'Fulfillment.fulfilledBy reference saved')
  t.equal(readResponse.Ok.fulfillment.fulfills, commitmentId, 'Fulfillment.fulfills reference saved')

  // ASSERT: check forward query indexes
  readResponse = await alice.call('planning', 'fulfillment', 'query_fulfillments', { params: { fulfills: commitmentId } })
  t.equal(readResponse.Ok.length, 1, 'read fulfillments by commitment OK')
  t.equal(readResponse.Ok[0].fulfillment.id, fulfillmentId, 'Fulfillment.fulfills indexed correctly')

  // ASSERT: check reverse query indexes
  readResponse = await alice.call('observation', 'fulfillment', 'query_fulfillments', { params: { fulfilledBy: eventId } })
  t.equal(readResponse.Ok.length, 1, 'read fulfillments by event OK')
  t.equal(readResponse.Ok[0].fulfillment.id, fulfillmentId, 'Fulfillment.fulfilledBy indexed correctly')



  // SCENARIO: add another fulfillment
  const fulfillment2 = {
    fulfills: commitmentId,
    fulfilledBy: eventId,
    note: 'fulfillment indicating another relationship',
  }
  const fulfillmentResp2 = await alice.call('planning', 'fulfillment', 'create_fulfillment', { fulfillment: fulfillment2 })
  t.ok(fulfillmentResp2.Ok.fulfillment && fulfillmentResp2.Ok.fulfillment.id, 'additional fulfillment created successfully')
  await s.consistency()
  const fulfillmentId2 = fulfillmentResp2.Ok.fulfillment.id

  // ASSERT: check forward query indices
  readResponse = await alice.call('planning', 'fulfillment', 'query_fulfillments', { params: { fulfills: commitmentId } })
  t.equal(readResponse.Ok.length, 2, 'appending fulfillments for read OK')
  t.equal(readResponse.Ok[0].fulfillment.id, fulfillmentId, 'fulfillment 1 indexed correctly')
  t.equal(readResponse.Ok[1].fulfillment.id, fulfillmentId2, 'fulfillment 2 indexed correctly')

  // ASSERT: ensure append is working on the event read side
  readResponse = await alice.call('observation', 'economic_event', 'get_event', { address: eventId })
  t.equal(readResponse.Ok.economicEvent.fulfills.length, 2, 'EconomicEvent.fulfills appending OK')
  t.equal(readResponse.Ok.economicEvent.fulfills[0], fulfillmentId, 'EconomicEvent.fulfills reference 1 OK')
  t.equal(readResponse.Ok.economicEvent.fulfills[1], fulfillmentId2, 'EconomicEvent.fulfills reference 2 OK')

  // ASSERT: ensure query indices on the event read side
  readResponse = await alice.call('observation', 'economic_event', 'query_events', { params: { fulfills: fulfillmentId } })
  t.equal(readResponse.Ok.length, 1, 'appending fulfillments for event query OK')
  t.equal(readResponse.Ok[0].economicEvent.id, eventId, 'event query indexed correctly')

  // ASSERT: ensure append is working on the commitment read side
  readResponse = await alice.call('planning', 'commitment', 'get_commitment', { address: commitmentId })
  t.equal(readResponse.Ok.commitment.fulfilledBy.length, 2, 'Commitment.fulfilledBy appending OK')
  t.equal(readResponse.Ok.commitment.fulfilledBy[0], fulfillmentId, 'Commitment.fulfilledBy reference 1 OK')
  t.equal(readResponse.Ok.commitment.fulfilledBy[1], fulfillmentId2, 'Commitment.fulfilledBy reference 2 OK')

  // ASSERT: ensure query indices on the commitment read side
  readResponse = await alice.call('planning', 'commitment', 'query_commitments', { params: { fulfilledBy: fulfillmentId } })
  t.equal(readResponse.Ok.length, 1, 'appending fulfillments for commitment query OK')
  t.equal(readResponse.Ok[0].commitment.id, commitmentId, 'commitment query indexed correctly')

  // ASSERT: check reciprocal query indexes
  readResponse = await alice.call('observation', 'fulfillment', 'query_fulfillments', { params: { fulfilledBy: eventId } })
  t.equal(readResponse.Ok.length, 2, 'read fulfillments by event OK')
  t.equal(readResponse.Ok[0].fulfillment.id, fulfillmentId, 'fulfillment 1 indexed correctly')
  t.equal(readResponse.Ok[1].fulfillment.id, fulfillmentId2, 'fulfillment 2 indexed correctly')
})

runner.run()
