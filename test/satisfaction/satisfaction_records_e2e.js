import test from "tape"
import { pause } from "@holochain/tryorama"
import {
  buildPlayer,
  mockIdentifier,
  mockAgentId,
  sortByIdBuffer,
} from '../init.js'

const testEventProps = {
  action: 'raise',
  resourceClassifiedAs: ['some-resource-type'],
  resourceQuantity: { hasNumericalValue: 1, hasUnit: mockIdentifier(false) },
  provider: mockAgentId(false),
  receiver: mockAgentId(false),
  hasPointInTime: '2019-11-19T04:29:55.056Z',
}

test('satisfactions can be written and read between DNAs by all parties requiring access', async (t) => {
  const alice = await buildPlayer(['planning', 'observation'])
  const { cells: [planning, observation] } = alice

  // SCENARIO: write records
  const intent = {
    note: 'an intent to provide something',
    ...testEventProps,
  }
  const intentResponse = await planning.call('intent', 'create_intent', { intent })
  t.ok(intentResponse.intent && intentResponse.intent.id, 'intent created successfully')
  await pause(100)
  const intentId = intentResponse.intent.id

  const event = {
    note: 'test event which is satisfying an intent',
    action: 'produce',
    ...testEventProps,
  }
  const eventResp = await observation.call('economic_event', 'create_economic_event', { event })
  t.ok(eventResp.economicEvent && eventResp.economicEvent.id, 'event created successfully')
  await pause(100)
  const eventId = eventResp.economicEvent.id

  const satisfaction = {
    satisfies: intentId,
    satisfiedBy: eventId,
    note: 'satisfied by an event',
  }
  const satisfactionResp = await planning.call('satisfaction', 'create_satisfaction', { satisfaction })
  t.ok(satisfactionResp.satisfaction && satisfactionResp.satisfaction.id, 'satisfaction by event created successfully')
  await pause(100)
  const satisfactionId = satisfactionResp.satisfaction.id
  const satisfactionIdObs = [eventId[0], satisfactionId[1]]  // :NOTE: ID in dest network will be same EntryHash, different DnaHash

  // ASSERT: check satisfaction in originating network
  let readResponse = await planning.call('satisfaction', 'get_satisfaction', { address: satisfactionId })
  t.deepEqual(readResponse.satisfaction.satisfiedBy, eventId, 'Satisfaction.satisfiedBy reference saved in planning DNA')
  t.deepEqual(readResponse.satisfaction.satisfies, intentId, 'Satisfaction.satisfies reference saved in planning DNA')

  // ASSERT: check satisfaction in target network
  readResponse = await observation.call('satisfaction', 'get_satisfaction', { address: satisfactionIdObs })
  t.deepEqual(readResponse.satisfaction.satisfiedBy, eventId, 'Satisfaction.satisfiedBy reference saved in observation DNA')
  t.deepEqual(readResponse.satisfaction.satisfies, intentId, 'Satisfaction.satisfies reference saved in observation DNA')

  // ASSERT: check event field refs
  readResponse = await observation.call('economic_event', 'get_economic_event', { address: eventId })
  t.ok(readResponse.economicEvent.satisfies, 'EconomicEvent.satisfies value present')
  t.equal(readResponse.economicEvent.satisfies.length, 1, 'EconomicEvent.satisfies reference saved in observation DNA')
  t.deepEqual(readResponse.economicEvent.satisfies[0], satisfactionIdObs, 'EconomicEvent.satisfies reference OK in observation DNA')

  // ASSERT: check intent field refs
  readResponse = await planning.call('intent', 'get_intent', { address: intentId })
  t.ok(readResponse.intent.satisfiedBy, 'intent.satisfiedBy reciprocal value present')
  t.equal(readResponse.intent.satisfiedBy.length, 1, 'Intent.satisfiedBy reciprocal reference saved')
  t.deepEqual(readResponse.intent.satisfiedBy[0], satisfactionId, 'Intent.satisfiedBy reciprocal satisfaction reference OK in planning DNA')

  // ASSERT: check intent query indexes
  readResponse = await planning.call('satisfaction_index', 'query_satisfactions', { params: { satisfies: intentId } })
  t.equal(readResponse.edges.length, 1, 'read satisfactions by intent OK')
  t.deepEqual(readResponse.edges && readResponse.edges[0] && readResponse.edges[0].node && readResponse.edges[0].node.id, satisfactionId, 'Satisfaction.satisfies indexed correctly in planning DNA')

  // ASSERT: check event query indexes
  readResponse = await observation.call('satisfaction_index', 'query_satisfactions', { params: { satisfiedBy: eventId } })
  t.equal(readResponse.edges.length, 1, 'read satisfactions by event OK')
  t.deepEqual(readResponse.edges && readResponse.edges[0] && readResponse.edges[0].node && readResponse.edges[0].node.id, satisfactionIdObs, 'Satisfaction.satisfiedBy indexed correctly in observation DNA')

  // ASSERT: check intent satisfaction query indexes
  readResponse = await planning.call('intent_index', 'query_intents', { params: { satisfiedBy: satisfactionId } })
  t.equal(readResponse.edges.length, 1, 'indexing satisfactions for intent query OK')
  t.deepEqual(readResponse.edges && readResponse.edges[0] && readResponse.edges[0].node && readResponse.edges[0].node.id, intentId, 'intent query 1 indexed correctly in planning DNA')

  // ASSERT: check event satisfaction query indexes
  readResponse = await observation.call('economic_event_index', 'query_economic_events', { params: { satisfies: satisfactionIdObs } })
  t.equal(readResponse.edges.length, 1, 'indexing satisfactions for event query OK')
  t.deepEqual(readResponse.edges && readResponse.edges[0] && readResponse.edges[0].node && readResponse.edges[0].node.id, eventId, 'event query 1 indexed correctly in observation DNA')



  // SCENARIO: add a commitment-based satisfaction
  const commitment = {
    note: 'test commitment which is satisfying an intent',
    ...testEventProps,
  }
  const commitmentResp = await planning.call('commitment', 'create_commitment', { commitment })
  t.ok(commitmentResp.commitment && commitmentResp.commitment.id, 'commitment created successfully')
  await pause(100)
  const commitmentId = commitmentResp.commitment.id

  const satisfaction2 = {
    satisfies: intentId,
    satisfiedBy: commitmentId,
    note: 'satisfied by a commitment',
  }
  const satisfactionResp2 = await planning.call('satisfaction', 'create_satisfaction', { satisfaction: satisfaction2 })
  t.ok(satisfactionResp2.satisfaction && satisfactionResp2.satisfaction.id, 'satisfaction by commitment created successfully')
  await pause(100)
  const satisfactionId2 = satisfactionResp2.satisfaction.id

  // ASSERT: check commitment field refs
  readResponse = await planning.call('commitment', 'get_commitment', { address: commitmentId })
  t.ok(readResponse.commitment.satisfies, 'Commitment.satisfies value present')
  t.equal(readResponse.commitment.satisfies.length, 1, 'Commitment.satisfies reference saved')
  t.deepEqual(readResponse.commitment.satisfies[0], satisfactionId2, 'Commitment.satisfies reference OK')

  // ASSERT: check intent query indices
  readResponse = await planning.call('satisfaction_index', 'query_satisfactions', { params: { satisfies: intentId } })
  t.equal(readResponse.edges.length, 2, 'appending satisfactions for read OK')

  // :TODO: remove client-side sorting when deterministic time-ordered indexing is implemented
  const sortedSIds = [{ id: satisfactionId }, { id: satisfactionId2 }].sort(sortByIdBuffer)
  readResponse.edges.sort(({ node }, { node: node2 }) => sortByIdBuffer(node, node2))

  t.deepEqual(readResponse.edges && readResponse.edges[0] && readResponse.edges[0].node && readResponse.edges[0].node.id, sortedSIds[0].id, 'satisfaction 1 indexed correctly')
  t.deepEqual(readResponse.edges && readResponse.edges[1] && readResponse.edges[1].node && readResponse.edges[1].node.id, sortedSIds[1].id, 'satisfaction 2 indexed correctly')

  // ASSERT: check intent field refs
  readResponse = await planning.call('intent', 'get_intent', { address: intentId })
  t.equal(readResponse.intent.satisfiedBy.length, 2, 'Intent.satisfiedBy appending OK')

  // :TODO: remove client-side sorting when deterministic time-ordered indexing is implemented
  readResponse.intent.satisfiedBy.sort(sortByIdBuffer)

  t.deepEqual(readResponse.intent.satisfiedBy[0], sortedSIds[0].id, 'Intent.satisfiedBy reference 1 OK')
  t.deepEqual(readResponse.intent.satisfiedBy[1], sortedSIds[1].id, 'Intent.satisfiedBy reference 2 OK')

  // ASSERT: check commitment query indexes
  readResponse = await planning.call('satisfaction_index', 'query_satisfactions', { params: { satisfiedBy: commitmentId } })
  t.equal(readResponse.edges.length, 1, 'read satisfactions by commitment OK')
  t.deepEqual(readResponse.edges && readResponse.edges[0] && readResponse.edges[0].node && readResponse.edges[0].node.id, satisfactionId2, 'Satisfaction.satisfiedBy indexed correctly')

  // ASSERT: check intent satisfaction query indexes
  readResponse = await planning.call('intent_index', 'query_intents', { params: { satisfiedBy: satisfactionId2 } })
  t.equal(readResponse.edges.length, 1, 'appending satisfactions for intent query OK')
  t.deepEqual(readResponse.edges && readResponse.edges[0] && readResponse.edges[0].node && readResponse.edges[0].node.id, intentId, 'intent query 2 indexed correctly')

  await alice.scenario.cleanUp()
})


