const {
  buildConfig,
  buildRunner,
  buildPlayer,
  mockIdentifier,
  mockAgentId,
  sortByIdBuffer,
  sortIdBuffers,
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
  const fulfillmentIdObs = [eventId[0], fulfillmentId[1]]  // :NOTE: ID in dest network will be same EntryHash, different DnaHash

  // ASSERT: check fulfillment in originating network
  let readResponse = await planning.call('fulfillment', 'get_fulfillment', { address: fulfillmentId })
  t.deepEqual(readResponse.fulfillment.fulfilledBy, eventId, 'Fulfillment.fulfilledBy reference saved in planning DNA')
  t.deepEqual(readResponse.fulfillment.fulfills, commitmentId, 'Fulfillment.fulfills reference saved in planning DNA')

  // ASSERT: check event
  readResponse = await observation.call('economic_event', 'get_economic_event', { address: eventId })
  console.log('readResponse', readResponse)
  t.ok(readResponse.economicEvent.fulfills, 'EconomicEvent.fulfills value present')
  t.equal(readResponse.economicEvent.fulfills.length, 1, 'EconomicEvent.fulfills reference saved in observation DNA')
  t.deepEqual(readResponse.economicEvent.fulfills[0], fulfillmentIdObs, 'EconomicEvent.fulfills reference OK in observation DNA')

  // ASSERT: check commitment
  readResponse = await planning.call('commitment', 'get_commitment', { address: commitmentId })
  t.ok(readResponse.commitment.fulfilledBy, 'Commitment.fulfilledBy reciprocal value present')
  t.equal(readResponse.commitment.fulfilledBy.length, 1, 'Commitment.fulfilledBy reciprocal reference saved in planning DNA')
  t.deepEqual(readResponse.commitment.fulfilledBy[0], fulfillmentId, 'Commitment.fulfilledBy reciprocal fulfillment reference OK in planning DNA')

  // ASSERT: check fulfillment in destination network
  readResponse = await observation.call('fulfillment', 'get_fulfillment', { address: fulfillmentIdObs })
  t.deepEqual(readResponse.fulfillment.fulfilledBy, eventId, 'Fulfillment.fulfilledBy reference saved in observation DNA')
  t.deepEqual(readResponse.fulfillment.fulfills, commitmentId, 'Fulfillment.fulfills reference saved in observation DNA')

  // ASSERT: check forward query indexes
  readResponse = await planning.call('fulfillment_index', 'query_fulfillments', { params: { fulfills: commitmentId } })
  t.equal(readResponse.edges.length, 1, 'read fulfillments by commitment OK')
  t.deepEqual(readResponse.edges && readResponse.edges[0] && readResponse.edges[0].node && readResponse.edges[0].node.id, fulfillmentId, 'Fulfillment.fulfills indexed correctly')

  // ASSERT: check reverse query indexes
  readResponse = await observation.call('fulfillment_index', 'query_fulfillments', { params: { fulfilledBy: eventId } })
  t.equal(readResponse.edges.length, 1, 'read fulfillments by event OK')
  t.deepEqual(readResponse.edges && readResponse.edges[0] && readResponse.edges[0].node && readResponse.edges[0].node.id[1], fulfillmentId[1], 'Fulfillment.fulfilledBy indexed correctly in observation DNA')



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
  const fulfillmentId2Obs = [eventId[0], fulfillmentId2[1]]

  // ASSERT: check forward query indices
  readResponse = await planning.call('fulfillment_index', 'query_fulfillments', { params: { fulfills: commitmentId } })
  t.equal(readResponse.edges.length, 2, 'appending fulfillments for read OK')
  t.deepEqual(readResponse.edges && readResponse.edges[0] && readResponse.edges[0].node && readResponse.edges[0].node.id, fulfillmentId, 'fulfillment 1 indexed correctly')
  t.deepEqual(readResponse.edges && readResponse.edges[1] && readResponse.edges[1].node && readResponse.edges[1].node.id, fulfillmentId2, 'fulfillment 2 indexed correctly')

  // ASSERT: ensure append is working on the event read side
  readResponse = await observation.call('economic_event', 'get_economic_event', { address: eventId })

  // :TODO: remove client-side sorting when deterministic time-ordered indexing is implemented
  const sortedFIds = [{ id: fulfillmentId }, { id: fulfillmentId2 }].sort(sortByIdBuffer)
  const sortedFIdsObs = [{ id: fulfillmentIdObs }, { id: fulfillmentId2Obs }].sort(sortByIdBuffer)
  readResponse.economicEvent.fulfills.sort(sortIdBuffers)

  t.equal(readResponse.economicEvent.fulfills.length, 2, 'EconomicEvent.fulfills appending OK')
  t.deepEqual(readResponse.economicEvent.fulfills[0], sortedFIdsObs[0].id, 'EconomicEvent.fulfills reference 1 OK in observation DNA')
  t.deepEqual(readResponse.economicEvent.fulfills[1], sortedFIdsObs[1].id, 'EconomicEvent.fulfills reference 2 OK in observation DNA')
  // :TODO: test fulfillment reference in planning DNA

  // ASSERT: ensure query indices on the event read side
  readResponse = await observation.call('economic_event_index', 'query_economic_events', { params: { fulfills: fulfillmentIdObs } })
  t.equal(readResponse.edges.length, 1, 'appending fulfillments for event query OK')
  t.deepEqual(readResponse.edges && readResponse.edges[0] && readResponse.edges[0].node && readResponse.edges[0].node.id, eventId, 'event query indexed correctly')

  // ASSERT: ensure append is working on the commitment read side
  readResponse = await planning.call('commitment', 'get_commitment', { address: commitmentId })

  // :TODO: remove client-side sorting when deterministic time-ordered indexing is implemented
  readResponse.commitment.fulfilledBy.sort(sortIdBuffers)

  t.equal(readResponse.commitment.fulfilledBy.length, 2, 'Commitment.fulfilledBy appending OK')
  t.deepEqual(readResponse.commitment.fulfilledBy[0], sortedFIds[0].id, 'Commitment.fulfilledBy reference 1 OK')
  t.deepEqual(readResponse.commitment.fulfilledBy[1], sortedFIds[1].id, 'Commitment.fulfilledBy reference 2 OK')

  // ASSERT: ensure query indices on the commitment read side
  readResponse = await planning.call('commitment_index', 'query_commitments', { params: { fulfilledBy: fulfillmentId } })
  t.equal(readResponse.edges.length, 1, 'appending fulfillments for commitment query OK')
  t.deepEqual(readResponse.edges && readResponse.edges[0] && readResponse.edges[0].node && readResponse.edges[0].node.id, commitmentId, 'commitment query indexed correctly')

  // ASSERT: check reciprocal query indexes
  readResponse = await observation.call('fulfillment_index', 'query_fulfillments', { params: { fulfilledBy: eventId } })

  // :TODO: remove client-side sorting when deterministic time-ordered indexing is implemented
  readResponse.edges.sort(({ node }, { node: node2 }) => sortByIdBuffer(node, node2))

  t.equal(readResponse.edges.length, 2, 'read fulfillments by event OK')
  t.deepEqual(readResponse.edges && readResponse.edges[0] && readResponse.edges[0].node && readResponse.edges[0].node.id, sortedFIdsObs[0].id, 'fulfillment 1 indexed correctly in observation DNA')
  t.deepEqual(readResponse.edges && readResponse.edges[1] && readResponse.edges[1].node && readResponse.edges[1].node.id, sortedFIdsObs[1].id, 'fulfillment 2 indexed correctly in observation DNA')
})

runner.run()
