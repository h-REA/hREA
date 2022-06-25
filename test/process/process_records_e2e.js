import test from 'tape'
import { pause } from '@connoropolous/tryorama'
import {
  buildPlayer,
  mockAddress,
  mockIdentifier,
} from '../init.js'

const testEventProps = {
  provider: mockAddress(false),
  receiver: mockAddress(false),
  hasPointInTime: '2019-11-19T12:12:42.739+01:00',
  resourceClassifiedAs: ['resource-type-uri'],
  resourceQuantity: { hasNumericalValue: 1, hasUnit: mockIdentifier(false) },
}

test('process local query indexes and relationships', async (t) => {
  const alice = await buildPlayer(['observation'])
  const { cells: [observation] } = alice

  // SCENARIO: write records
  const process = {
    name: 'test process for linking logic',
  }
  const pResp = await observation.call('process', 'create_process', { process })
  console.log(pResp)
  t.ok(pResp.process && pResp.process.id, 'process created successfully')
  await pause(100)
  const processId = pResp.process.id

  const iEvent = {
    note: 'test input event',
    action: 'consume',
    inputOf: processId,
    ...testEventProps,
  }
  const ieResp = await observation.call('economic_event', 'create_economic_event', { event: iEvent })
  console.log('event respose: ', ieResp)
  t.ok(ieResp.economicEvent && ieResp.economicEvent.id, 'input event created successfully')
  t.deepLooseEqual(ieResp.economicEvent.inputOf, processId, 'event.inputOf reference OK in write')
  await pause(100)
  const iEventId = ieResp.economicEvent.id

  const oEvent = {
    note: 'test output event',
    action: 'produce',
    outputOf: processId,
    ...testEventProps,
  }
  const oeResp = await observation.call('economic_event', 'create_economic_event', { event: oEvent })
  t.ok(oeResp.economicEvent && oeResp.economicEvent.id, 'output event created successfully')
  t.deepLooseEqual(oeResp.economicEvent.outputOf, processId, 'event.outputOf reference OK in write')
  await pause(100)
  const oEventId = oeResp.economicEvent.id

  // ASSERT: check input event index links
  let readResponse = await observation.call('economic_event', 'get_economic_event', { address: iEventId })
  t.ok(readResponse.economicEvent && readResponse.economicEvent.inputOf, 'EconomicEvent.inputOf index saved')
  t.deepLooseEqual(readResponse.economicEvent && readResponse.economicEvent.inputOf, processId, 'EconomicEvent.inputOf reference OK in read')

  // ASSERT: check output event index links
  readResponse = await observation.call('economic_event', 'get_economic_event', { address: oEventId })
  t.ok(readResponse.economicEvent && readResponse.economicEvent.outputOf, 'EconomicEvent.outputOf index saved')
  t.deepLooseEqual(readResponse.economicEvent && readResponse.economicEvent.outputOf, processId, 'EconomicEvent.outputOf reference OK in read')

  // ASSERT: test event input query edge
  readResponse = await observation.call('economic_event_index', 'query_economic_events', { params: { inputOf: processId } })
  t.deepLooseEqual(readResponse && readResponse.edges && readResponse.edges.length, 1, 'event input query index present')
  t.deepLooseEqual(readResponse.edges && readResponse.edges[0] && readResponse.edges[0].node && readResponse.edges[0].node.id, iEventId, 'event input query index created')

  // ASSERT: test event output query edge
  readResponse = await observation.call('economic_event_index', 'query_economic_events', { params: { outputOf: processId } })
  t.deepLooseEqual(readResponse && readResponse.edges && readResponse.edges.length, 1, 'event output query index present')
  t.deepLooseEqual(readResponse.edges && readResponse.edges[0] && readResponse.edges[0].node && readResponse.edges[0].node.id, oEventId, 'event output query index created')

  // ASSERT: check process event input query edge
  readResponse = await observation.call('process_index', 'query_processes', { params: { observedInputs: iEventId } })
  t.deepLooseEqual(readResponse && readResponse.edges && readResponse.edges.length, 1, 'process.observedInputs query succeeded')
  t.deepLooseEqual(readResponse.edges && readResponse.edges[0] && readResponse.edges[0].node.id, processId, 'process.observedInputs query index created')

  // ASSERT: check process event output query edge
  readResponse = await observation.call('process_index', 'query_processes', { params: { observedOutputs: oEventId } })
  t.deepLooseEqual(readResponse && readResponse.edges && readResponse.edges.length, 1, 'process.observedOutputs query succeeded')
  t.deepLooseEqual(readResponse.edges && readResponse.edges[0] && readResponse.edges[0].node && readResponse.edges[0].node.id, processId, 'process.observedOutputs query index created')

  await alice.scenario.cleanUp()
})

test('process remote query indexes and relationships', async (t) => {
  const alice = await buildPlayer(['observation', 'planning'])
  const { cells: [observation, planning] } = alice

  const process = {
    name: 'test process for remote linking logic',
  }
  const pResp = await observation.call('process', 'create_process', { process })
  t.ok(pResp.process && pResp.process.id, 'process created successfully')
  await pause(100)
  const processId = pResp.process.id

  const iCommitment = {
    note: 'test input commitment',
    action: 'consume',
    inputOf: processId,
    ...testEventProps,
  }
  const icResp = await planning.call('commitment', 'create_commitment', { commitment: iCommitment })
  t.ok(icResp.commitment && icResp.commitment.id, 'input commitment created successfully')
  t.deepLooseEqual(icResp.commitment.inputOf, processId, 'commitment.inputOf reference OK in write')
  await pause(100)
  const iCommitmentId = icResp.commitment.id

  const oCommitment = {
    note: 'test output commitment',
    action: 'produce',
    outputOf: processId,
    ...testEventProps,
  }
  const ocResp = await planning.call('commitment', 'create_commitment', { commitment: oCommitment })
  t.ok(ocResp.commitment && ocResp.commitment.id, 'output commitment created successfully')
  t.deepLooseEqual(ocResp.commitment.outputOf, processId, 'commitment.outputOf reference OK in write')
  await pause(100)
  const oCommitmentId = ocResp.commitment.id

  const iIntent = {
    note: 'test input intent',
    action: 'consume',
    inputOf: processId,
    ...testEventProps,
  }
  const iiResp = await planning.call('intent', 'create_intent', { intent: iIntent })
  t.ok(iiResp.intent && iiResp.intent.id, 'input intent created successfully')
  t.deepLooseEqual(iiResp.intent.inputOf, processId, 'intent.inputOf reference OK in write')
  await pause(100)
  const iIntentId = iiResp.intent.id

  const oIntent = {
    note: 'test output intent',
    action: 'produce',
    outputOf: processId,
    ...testEventProps,
  }
  const oiResp = await planning.call('intent', 'create_intent', { intent: oIntent })
  t.ok(oiResp.intent && oiResp.intent.id, 'output intent created successfully')
  t.deepLooseEqual(oiResp.intent.outputOf, processId, 'intent.outputOf reference OK in write')
  await pause(100)
  const oIntentId = oiResp.intent.id

  // ASSERT: check input commitment index links
  let readResponse = await planning.call('commitment', 'get_commitment', { address: iCommitmentId })
  t.ok(readResponse.commitment && readResponse.commitment.inputOf, 'commitment.inputOf index saved')
  t.deepLooseEqual(readResponse.commitment && readResponse.commitment.inputOf, processId, 'commitment.inputOf reference OK in read')

  // ASSERT: check output commitment index links
  readResponse = await planning.call('commitment', 'get_commitment', { address: oCommitmentId })
  t.ok(readResponse.commitment && readResponse.commitment.outputOf, 'commitment.outputOf index saved')
  t.deepLooseEqual(readResponse.commitment && readResponse.commitment.outputOf, processId, 'commitment.outputOf reference OK in read')

  // ASSERT: test commitment input query edge
  readResponse = await planning.call('commitment_index', 'query_commitments', { params: { inputOf: processId } })
  t.deepLooseEqual(readResponse && readResponse.edges && readResponse.edges.length, 1, 'commitment input query index present')
  t.deepLooseEqual(readResponse.edges && readResponse.edges[0] && readResponse.edges[0].node && readResponse.edges[0].node.id, iCommitmentId, 'commitment input query index created')

  // ASSERT: test commitment output query edge
  readResponse = await planning.call('commitment_index', 'query_commitments', { params: { outputOf: processId } })
  t.deepLooseEqual(readResponse && readResponse.edges && readResponse.edges.length, 1, 'commitment output query index present')
  t.deepLooseEqual(readResponse.edges && readResponse.edges[0] && readResponse.edges[0].node && readResponse.edges[0].node.id, oCommitmentId, 'commitment output query index created')

  // ASSERT: check process commitment input query edge
  readResponse = await observation.call('process_index', 'query_processes', { params: { committedInputs: iCommitmentId } })
  t.deepLooseEqual(readResponse && readResponse.edges && readResponse.edges.length, 1, 'process.committedInputs query succeeded')
  t.deepLooseEqual(readResponse.edges && readResponse.edges[0] && readResponse.edges[0].node && readResponse.edges[0].node.id, processId, 'process.committedInputs query index created')

  // ASSERT: check process commitment output query edge
  readResponse = await observation.call('process_index', 'query_processes', { params: { committedOutputs: oCommitmentId } })
  t.deepLooseEqual(readResponse && readResponse.edges && readResponse.edges.length, 1, 'process.committedOutputs query succeeded')
  t.deepLooseEqual(readResponse.edges && readResponse.edges[0] && readResponse.edges[0].node && readResponse.edges[0].node.id, processId, 'process.committedOutputs query index created')

  // ASSERT: check input intent index links
  readResponse = await planning.call('intent', 'get_intent', { address: iIntentId })
  t.ok(readResponse.intent && readResponse.intent.inputOf, 'intent.inputOf index saved')
  t.deepLooseEqual(readResponse.intent && readResponse.intent.inputOf, processId, 'intent.inputOf reference OK in read')

  // ASSERT: check output intent index links
  readResponse = await planning.call('intent', 'get_intent', { address: oIntentId })
  t.ok(readResponse.intent && readResponse.intent.outputOf, 'intent.outputOf index saved')
  t.deepLooseEqual(readResponse.intent && readResponse.intent.outputOf, processId, 'intent.outputOf reference OK in read')

  // ASSERT: test intent input query edge
  readResponse = await planning.call('intent_index', 'query_intents', { params: { inputOf: processId } })
  t.deepLooseEqual(readResponse && readResponse.edges && readResponse.edges.length, 1, 'intent input query index present')
  t.deepLooseEqual(readResponse.edges && readResponse.edges[0] && readResponse.edges[0].node && readResponse.edges[0].node.id, iIntentId, 'intent input query index created')

  // ASSERT: test intent output query edge
  readResponse = await planning.call('intent_index', 'query_intents', { params: { outputOf: processId } })
  t.deepLooseEqual(readResponse && readResponse.edges && readResponse.edges.length, 1, 'intent output query index present')
  t.deepLooseEqual(readResponse.edges && readResponse.edges[0] && readResponse.edges[0].node && readResponse.edges[0].node.id, oIntentId, 'intent output query index created')

  // ASSERT: check process intent input query edge
  readResponse = await observation.call('process_index', 'query_processes', { params: { intendedInputs: iIntentId } })
  t.deepLooseEqual(readResponse && readResponse.edges && readResponse.edges.length, 1, 'process.intendedInputs query succeeded')
  t.deepLooseEqual(readResponse.edges && readResponse.edges[0] && readResponse.edges[0].node && readResponse.edges[0].node.id, processId, 'process.intendedInputs query index created')

  // ASSERT: check process intent output query edge
  readResponse = await observation.call('process_index', 'query_processes', { params: { intendedOutputs: oIntentId } })
  t.deepLooseEqual(readResponse && readResponse.edges && readResponse.edges.length, 1, 'process.intendedOutputs query succeeded')
  t.deepLooseEqual(readResponse.edges && readResponse.edges[0] && readResponse.edges[0].node && readResponse.edges[0].node.id, processId, 'process.intendedOutputs query index created')

  // TODO: modify

  await alice.scenario.cleanUp()
})
