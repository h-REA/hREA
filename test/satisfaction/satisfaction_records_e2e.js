const {
  getDNA,
  buildOrchestrator,
} = require('../init')

const runner = buildOrchestrator({
  observation: getDNA('observation'),
  planning: getDNA('planning'),
}, {
  vf_observation: ['planning', 'observation'],
})

runner.registerScenario('satisfactions can be written and read between DNAs by all parties requiring access', async (s, t, { planning, observation }) => {
  // SCENARIO: write records
  const intent = {
    note: 'an intent to provide something',
  }
  const intentResponse = await planning.call('intent', 'create_intent', { intent })
  t.ok(intentResponse.Ok.intent && intentResponse.Ok.intent.id, 'intent created successfully')
  await s.consistent()
  const intentId = intentResponse.Ok.intent.id

  const event = {
    note: 'test event which is satisfying an intent',
  }
  const eventResp = await observation.call('economic_event', 'create_event', { event })
  t.ok(eventResp.Ok.economicEvent && eventResp.Ok.economicEvent.id, 'event created successfully')
  await s.consistent()
  const eventId = eventResp.Ok.economicEvent.id

  const satisfaction = {
    satisfies: intentId,
    satisfiedBy: eventId,
    note: 'satisfied by an event',
  }
  const satisfactionResp = await planning.call('satisfaction', 'create_satisfaction', { satisfaction })
  t.ok(satisfactionResp.Ok.satisfaction && satisfactionResp.Ok.satisfaction.id, 'satisfaction by event created successfully')
  await s.consistent()
  const satisfactionId = satisfactionResp.Ok.satisfaction.id

  // ASSERT: check satisfaction in originating network
  let readResponse = await planning.call('satisfaction', 'get_satisfaction', { address: satisfactionId })
  t.equal(readResponse.Ok.satisfaction.satisfiedBy, eventId, 'Satisfaction.satisfiedBy reference saved')
  t.equal(readResponse.Ok.satisfaction.satisfies, intentId, 'Satisfaction.satisfies reference saved')

  // ASSERT: check satisfaction in target network
  readResponse = await observation.call('satisfaction', 'get_satisfaction', { address: satisfactionId })
  t.equal(readResponse.Ok.satisfaction.satisfiedBy, eventId, 'Satisfaction.satisfiedBy reference saved')
  t.equal(readResponse.Ok.satisfaction.satisfies, intentId, 'Satisfaction.satisfies reference saved')

  // ASSERT: check event field refs
  readResponse = await observation.call('economic_event', 'get_event', { address: eventId })
  t.ok(readResponse.Ok.economicEvent.satisfies, 'EconomicEvent.satisfies value present')
  t.equal(readResponse.Ok.economicEvent.satisfies.length, 1, 'EconomicEvent.satisfies reference saved')
  t.equal(readResponse.Ok.economicEvent.satisfies[0], satisfactionId, 'EconomicEvent.satisfies reference OK')

  // ASSERT: check intent field refs
  readResponse = await planning.call('intent', 'get_intent', { address: intentId })
  t.ok(readResponse.Ok.intent.satisfiedBy, 'intent.satisfiedBy reciprocal value present')
  t.equal(readResponse.Ok.intent.satisfiedBy.length, 1, 'Intent.satisfiedBy reciprocal reference saved')
  t.equal(readResponse.Ok.intent.satisfiedBy[0], satisfactionId, 'Intent.satisfiedBy reciprocal satisfaction reference OK')

  // ASSERT: check intent query indexes
  readResponse = await planning.call('satisfaction', 'query_satisfactions', { params: { satisfies: intentId } })
  t.equal(readResponse.Ok.length, 1, 'read satisfactions by intent OK')
  t.equal(readResponse.Ok[0].satisfaction.id, satisfactionId, 'Satisfaction.satisfies indexed correctly')

  // ASSERT: check event query indexes
  readResponse = await observation.call('satisfaction', 'query_satisfactions', { params: { satisfiedBy: eventId } })
  t.equal(readResponse.Ok.length, 1, 'read satisfactions by event OK')
  t.equal(readResponse.Ok[0].satisfaction.id, satisfactionId, 'Satisfaction.satisfiedBy indexed correctly')

  // ASSERT: check intent satisfaction query indexes
  readResponse = await planning.call('intent', 'query_intents', { params: { satisfiedBy: satisfactionId } })
  t.equal(readResponse.Ok.length, 1, 'indexing satisfactions for intent query OK')
  t.equal(readResponse.Ok[0].intent.id, intentId, 'intent query 1 indexed correctly')

  // ASSERT: check event satisfaction query indexes
  readResponse = await observation.call('event', 'query_events', { params: { satisfies: satisfactionId } })
  t.equal(readResponse.Ok.length, 1, 'indexing satisfactions for event query OK')
  t.equal(readResponse.Ok[0].economicEvent.id, intentId, 'event query 1 indexed correctly')



  // SCENARIO: add a commitment-based satisfaction
  const commitment = {
    note: 'test commitment which is satisfying an intent',
  }
  const commitmentResp = await planning.call('commitment', 'create_commitment', { commitment })
  t.ok(commitmentResp.Ok.commitment && commitmentResp.Ok.commitment.id, 'commitment created successfully')
  await s.consistent()
  const commitmentId = commitmentResp.Ok.commitment.id

  const satisfaction2 = {
    satisfies: intentId,
    satisfiedBy: commitmentId,
    note: 'satisfied by a commitment',
  }
  const satisfactionResp2 = await planning.call('satisfaction', 'create_satisfaction', { satisfaction2 })
  t.ok(satisfactionResp2.Ok.satisfaction && satisfactionResp2.Ok.satisfaction.id, 'satisfaction by commitment created successfully')
  await s.consistent()
  const satisfactionId2 = satisfactionResp2.Ok.satisfaction.id

  // ASSERT: check intent query indices
  readResponse = await planning.call('satisfaction', 'query_satisfactions', { params: { satisfies: intentId } })
  t.equal(readResponse.Ok.length, 2, 'appending satisfactions for read OK')
  t.equal(readResponse.Ok[0].satisfaction.id, satisfactionId, 'satisfaction 1 indexed correctly')
  t.equal(readResponse.Ok[1].satisfaction.id, satisfactionId2, 'satisfaction 2 indexed correctly')

  // ASSERT: check intent field refs
  readResponse = await planning.call('intent', 'get_intent', { address: intentId })
  t.equal(readResponse.Ok.intent.satisfiedBy.length, 2, 'Intent.satisfiedBy appending OK')
  t.equal(readResponse.Ok.intent.satisfiedBy[0], satisfactionId, 'Intent.satisfiedBy reference 1 OK')
  t.equal(readResponse.Ok.intent.satisfiedBy[1], satisfactionId2, 'Intent.satisfiedBy reference 2 OK')

  // ASSERT: check commitment query indexes
  readResponse = await planning.call('satisfaction', 'query_satisfactions', { params: { satisfiedBy: commitmentId } })
  t.equal(readResponse.Ok.length, 1, 'read satisfactions by commitment OK')
  t.equal(readResponse.Ok[0].satisfaction.id, satisfactionId, 'Satisfaction.satisfiedBy indexed correctly')

  // ASSERT: check intent satisfaction query indexes
  readResponse = await planning.call('intent', 'query_intents', { params: { satisfiedBy: satisfactionId2 } })
  t.equal(readResponse.Ok.length, 1, 'appending satisfactions for intent query OK')
  t.equal(readResponse.Ok[0].intent.id, intentId, 'intent query 2 indexed correctly')
})

runner.run()
