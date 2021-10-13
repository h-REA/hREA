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

runner.registerScenario('satisfactions can be written and read between DNAs by all parties requiring access', async (s, t) => {
  const { cells: [planning, observation] } = await buildPlayer(s, config, ['planning', 'observation'])

  // SCENARIO: write records
  const intent = {
    note: 'an intent to provide something',
    ...testEventProps,
  }
  const intentResponse = await planning.call('intent', 'create_intent', { intent })
  t.ok(intentResponse.intent && intentResponse.intent.id, 'intent created successfully')
  await s.consistency()
  const intentId = intentResponse.intent.id

  const event = {
    note: 'test event which is satisfying an intent',
    action: 'produce',
    ...testEventProps,
  }
  const eventResp = await observation.call('economic_event', 'create_event', { event })
  t.ok(eventResp.economicEvent && eventResp.economicEvent.id, 'event created successfully')
  await s.consistency()
  const eventId = eventResp.economicEvent.id

  const satisfaction = {
    satisfies: intentId,
    satisfiedBy: eventId,
    note: 'satisfied by an event',
  }
  const satisfactionResp = await planning.call('satisfaction', 'create_satisfaction', { satisfaction })
  t.ok(satisfactionResp.satisfaction && satisfactionResp.satisfaction.id, 'satisfaction by event created successfully')
  await s.consistency()
  const satisfactionId = satisfactionResp.satisfaction.id

  // ASSERT: check satisfaction in originating network
  let readResponse = await planning.call('satisfaction', 'get_satisfaction', { address: satisfactionId })
  t.deepEqual(readResponse.satisfaction.satisfiedBy, eventId, 'Satisfaction.satisfiedBy reference saved')
  t.deepEqual(readResponse.satisfaction.satisfies, intentId, 'Satisfaction.satisfies reference saved')

  // ASSERT: check satisfaction in target network
  readResponse = await observation.call('satisfaction', 'get_satisfaction', { address: satisfactionId })
  t.deepEqual(readResponse.satisfaction.satisfiedBy, eventId, 'Satisfaction.satisfiedBy reference saved')
  t.deepEqual(readResponse.satisfaction.satisfies, intentId, 'Satisfaction.satisfies reference saved')

  // ASSERT: check event field refs
  readResponse = await observation.call('economic_event', 'get_event', { address: eventId })
  t.ok(readResponse.economicEvent.satisfies, 'EconomicEvent.satisfies value present')
  t.equal(readResponse.economicEvent.satisfies.length, 1, 'EconomicEvent.satisfies reference saved')
  t.deepEqual(readResponse.economicEvent.satisfies[0], satisfactionId, 'EconomicEvent.satisfies reference OK')

  // ASSERT: check intent field refs
  readResponse = await planning.call('intent', 'get_intent', { address: intentId })
  t.ok(readResponse.intent.satisfiedBy, 'intent.satisfiedBy reciprocal value present')
  t.equal(readResponse.intent.satisfiedBy.length, 1, 'Intent.satisfiedBy reciprocal reference saved')
  t.deepEqual(readResponse.intent.satisfiedBy[0], satisfactionId, 'Intent.satisfiedBy reciprocal satisfaction reference OK')

  // ASSERT: check intent query indexes
  readResponse = await planning.call('satisfaction_index', 'query_satisfactions', { params: { satisfies: intentId } })
  t.equal(readResponse.length, 1, 'read satisfactions by intent OK')
  t.deepEqual(readResponse.Ok[0].satisfaction.id, satisfactionId, 'Satisfaction.satisfies indexed correctly')

  // ASSERT: check event query indexes
  readResponse = await observation.call('satisfaction_index', 'query_satisfactions', { params: { satisfiedBy: eventId } })
  t.equal(readResponse.length, 1, 'read satisfactions by event OK')
  t.deepEqual(readResponse.Ok[0].satisfaction.id, satisfactionId, 'Satisfaction.satisfiedBy indexed correctly')

  // ASSERT: check intent satisfaction query indexes
  readResponse = await planning.call('intent_index', 'query_intents', { params: { satisfiedBy: satisfactionId } })
  t.equal(readResponse.length, 1, 'indexing satisfactions for intent query OK')
  t.deepEqual(readResponse.Ok[0].intent.id, intentId, 'intent query 1 indexed correctly')

  // ASSERT: check event satisfaction query indexes
  readResponse = await observation.call('economic_event_index', 'query_economic_events', { params: { satisfies: satisfactionId } })
  t.equal(readResponse.length, 1, 'indexing satisfactions for event query OK')
  t.deepEqual(readResponse.Ok[0].economicEvent.id, eventId, 'event query 1 indexed correctly')



  // SCENARIO: add a commitment-based satisfaction
  const commitment = {
    note: 'test commitment which is satisfying an intent',
    ...testEventProps,
  }
  const commitmentResp = await planning.call('commitment', 'create_commitment', { commitment })
  t.ok(commitmentResp.commitment && commitmentResp.commitment.id, 'commitment created successfully')
  await s.consistency()
  const commitmentId = commitmentResp.commitment.id

  const satisfaction2 = {
    satisfies: intentId,
    satisfiedBy: commitmentId,
    note: 'satisfied by a commitment',
  }
  const satisfactionResp2 = await planning.call('satisfaction', 'create_satisfaction', { satisfaction: satisfaction2 })
  t.ok(satisfactionResp2.satisfaction && satisfactionResp2.satisfaction.id, 'satisfaction by commitment created successfully')
  await s.consistency()
  const satisfactionId2 = satisfactionResp2.satisfaction.id

  // ASSERT: check commitment field refs
  readResponse = await planning.call('commitment', 'get_commitment', { address: commitmentId })
  t.ok(readResponse.commitment.satisfies, 'Commitment.satisfies value present')
  t.equal(readResponse.commitment.satisfies.length, 1, 'Commitment.satisfies reference saved')
  t.deepEqual(readResponse.commitment.satisfies[0], satisfactionId2, 'Commitment.satisfies reference OK')

  // ASSERT: check intent query indices
  readResponse = await planning.call('satisfaction_index', 'query_satisfactions', { params: { satisfies: intentId } })
  t.equal(readResponse.length, 2, 'appending satisfactions for read OK')
  t.deepEqual(readResponse.Ok[0].satisfaction.id, satisfactionId2, 'satisfaction 2 indexed correctly')
  t.deepEqual(readResponse.Ok[1].satisfaction.id, satisfactionId, 'satisfaction 1 indexed correctly')

  // ASSERT: check intent field refs
  readResponse = await planning.call('intent', 'get_intent', { address: intentId })
  t.equal(readResponse.intent.satisfiedBy.length, 2, 'Intent.satisfiedBy appending OK')
  t.deepEqual(readResponse.intent.satisfiedBy[0], satisfactionId2, 'Intent.satisfiedBy reference 2 OK')
  t.deepEqual(readResponse.intent.satisfiedBy[1], satisfactionId, 'Intent.satisfiedBy reference 1 OK')

  // ASSERT: check commitment query indexes
  readResponse = await planning.call('satisfaction_index', 'query_satisfactions', { params: { satisfiedBy: commitmentId } })
  t.equal(readResponse.length, 1, 'read satisfactions by commitment OK')
  t.deepEqual(readResponse.Ok[0].satisfaction.id, satisfactionId2, 'Satisfaction.satisfiedBy indexed correctly')

  // ASSERT: check intent satisfaction query indexes
  readResponse = await planning.call('intent_index', 'query_intents', { params: { satisfiedBy: satisfactionId2 } })
  t.equal(readResponse.length, 1, 'appending satisfactions for intent query OK')
  t.deepEqual(readResponse.Ok[0].intent.id, intentId, 'intent query 2 indexed correctly')
})

runner.run()
