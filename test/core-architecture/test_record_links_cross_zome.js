import test from 'tape'
import { pause } from '@holochain/tryorama'
// Some special conveniences exist for link handling when linking between records within the same DNA,
// hence why there are special test cases for this.
import {
  buildPlayer,
  mockAgentId,
  mockIdentifier,
} from '../init.js'

const testEventProps = {
  resourceClassifiedAs: ['some-resource-type'],
  resourceQuantity: { hasNumericalValue: 1, hasUnit: mockIdentifier(false) },
  provider: mockAgentId(false),
  receiver: mockAgentId(false),
  hasPointInTime: '2019-11-19T04:29:55.056Z',
}

test('updating local link fields syncs fields and associated indexes', async (t) => {
  const alice = await buildPlayer(['observation'])
  const { cells: [observation] } = alice

  // SCENARIO: write initial records
  const process = {
    name: 'context process for testing relationships',
  }
  const pResp = await observation.call('process', 'create_process', { process })
  t.ok(pResp.process && pResp.process.id, 'process created successfully')
  await pause(100)
  const processId = pResp.process.id

  const process2 = {
    name: 'second context process for testing relationships',
  }
  const pResp2 = await observation.call('process', 'create_process', { process: process2 })
  t.ok(pResp2.process && pResp2.process.id, 'process created successfully')
  await pause(100)
  const differentProcessId = pResp2.process.id

  const iEvent = {
    note: 'test input event',
    action: 'consume',
    inputOf: processId,
    ...testEventProps,
  }
  const ieResp = await observation.call('economic_event', 'create_economic_event', { event: iEvent })
  t.ok(ieResp.economicEvent && ieResp.economicEvent.id, 'input record created successfully')
  t.deepEqual(ieResp.economicEvent.inputOf, processId, 'field reference OK in write')
  await pause(100)
  const iEventId = ieResp.economicEvent.id

  // ASSERT: test event fields
  let readResponse = await observation.call('economic_event', 'get_economic_event', { address: iEventId })
  t.deepEqual(readResponse.economicEvent && readResponse.economicEvent.inputOf, processId, 'field reference OK on read')

  // ASSERT: test event input query edge
  readResponse = await observation.call('economic_event_index', 'query_economic_events', { params: { inputOf: processId } })
  t.equal(readResponse.edges && readResponse.edges.length, 1, 'field query index present')
  t.deepEqual(readResponse.edges[0] && readResponse.edges[0].node && readResponse.edges[0].node.id, iEventId, 'query index OK')

  // ASSERT: test process input query edge
  readResponse = await observation.call('process_index', 'query_processes', { params: { inputs: iEventId } })
  t.equal(readResponse.edges && readResponse.edges.length, 1, 'reciprocal query index present')
  t.deepEqual(readResponse.edges[0] && readResponse.edges[0].node && readResponse.edges[0].node.id, processId, 'reciprocal query index OK')

  // :TODO: need to find a new record with a local zome link to test, since EconomicEvent is not updateable
  /*
  // SCENARIO: update link field
  const updateEvent = {
    id: iEventId,
    inputOf: differentProcessId,
  }
  const ieResp2 = await observation.call('economic_event', 'update_economic_event', { event: updateEvent })
  t.equal(ieResp2.economicEvent && ieResp2.economicEvent.inputOf, differentProcessId, 'record link field updated successfully')
  await pause(100)

  // ASSERT: test event fields
  readResponse = await observation.call('economic_event', 'get_economic_event', { address: iEventId })
  t.ok(readResponse.economicEvent && readResponse.economicEvent.inputOf, 'field reference OK on read')
  t.equal(readResponse.economicEvent && readResponse.economicEvent.inputOf, differentProcessId, 'field updated successfully')

  // ASSERT: test event input query edge
  readResponse = await observation.call('economic_event_index', 'query_economic_events', { params: { inputOf: differentProcessId } })
  t.equal(readResponse.Ok && readResponse.edges.length, 1, 'field query index present')
  t.equal(readResponse.edges[0] && readResponse.edges[0].economicEvent && readResponse.edges[0].economicEvent.id, iEventId, 'field query index updated')

  // ASSERT: test process input query edge
  readResponse = await observation.call('process_index', 'query_processes', { params: { inputs: iEventId } })
  t.equal(readResponse.Ok && readResponse.edges.length, 1, 'process query index present')
  t.equal(readResponse.edges[0] && readResponse.edges[0].process && readResponse.edges[0].process.id, differentProcessId, 'process query index updated')

  // SCENARIO: update link field (no-op)
  const ieResp3 = await observation.call('economic_event', 'update_economic_event', { event: updateEvent })
  t.equal(ieResp3.economicEvent && ieResp3.economicEvent.inputOf, differentProcessId, 'update with same fields is no-op')
  await pause(100)

  // ASSERT: test event fields
  readResponse = await observation.call('economic_event', 'get_economic_event', { address: iEventId })
  t.equal(readResponse.economicEvent && readResponse.economicEvent.inputOf, differentProcessId, 'field update no-op OK')

  // SCENARIO: remove link field
  const wipeEventInput = {
    id: iEventId,
    inputOf: null,
  }
  const ieResp4 = await observation.call('economic_event', 'update_economic_event', { event: wipeEventInput })
  t.equal(ieResp4.economicEvent && ieResp4.economicEvent.inputOf, undefined, 'update with null value erases field')
  await pause(100)

  // ASSERT: test event fields
  readResponse = await observation.call('economic_event', 'get_economic_event', { address: iEventId })
  t.equal(readResponse.economicEvent && readResponse.economicEvent.inputOf, undefined, 'field erased successfully')

  // ASSERT: test event input query edge
  readResponse = await observation.call('economic_event_index', 'query_economic_events', { params: { inputOf: differentProcessId } })
  t.equal(readResponse.Ok && readResponse.edges.length, 0, 'field query index updated')

  // ASSERT: test process input query edge
  readResponse = await observation.call('process_index', 'query_processes', { params: { inputs: iEventId } })
  t.equal(readResponse.Ok && readResponse.edges.length, 0, 'process query index updated')
*/

  // SCENARIO: attempt linking to nonexistent target
  // :TODO: need to re-test this, the below fails for unrelated validation reasons
  // const badEvent = {
  //   action: 'consume',
  //   inputOf: "notarealprocess",
  //   ...testEventProps,
  // }
  // const badResp = await observation.call('economic_event', 'create_economic_event', { event: badEvent })
  // :TODO: should result in an error and avoid creating the entry if any invalid fields are provided
  // :TODO: this involves having a deep think about how much transactionality we want to enforce!

  // :TODO: updates for fields with other values in the array
  await alice.scenario.cleanUp()
})

test('removing records with linked local indexes clears them in associated records', async (t) => {
  const alice = await buildPlayer(['observation'])
  const { cells: [observation] } = alice

  // SCENARIO: write initial records
  const process = {
    name: 'context record for testing relationships',
  }
  const pResp = await observation.call('process', 'create_process', { process })
  t.ok(pResp.process && pResp.process.id, 'context record created successfully')
  await pause(100)
  const processId = pResp.process.id

  const iEvent = {
    note: 'test input event',
    action: 'consume',
    inputOf: processId,
    ...testEventProps,
  }
  const ieResp = await observation.call('economic_event', 'create_economic_event', { event: iEvent })
  t.ok(ieResp.economicEvent && ieResp.economicEvent.id, 'input record created successfully')
  t.deepEqual(ieResp.economicEvent.inputOf, processId, 'field reference OK in write')
  await pause(100)
  const iEventId = ieResp.economicEvent.id
  const iEventRev = ieResp.economicEvent.revisionId

  // ASSERT: test forward link field
  let readResponse = await observation.call('economic_event', 'get_economic_event', { address: iEventId })
  t.deepEqual(readResponse.economicEvent && readResponse.economicEvent.inputOf, processId, 'field reference OK on read')

  // ASSERT: test reciprocal link field
  readResponse = await observation.call('process', 'get_process', { address: processId })
  t.deepEqual(readResponse.process &&
    readResponse.process.inputs &&
    readResponse.process.inputs[0], iEventId, 'reciprocal field reference OK on read')

  // ASSERT: test commitment input query edge
  readResponse = await observation.call('economic_event_index', 'query_economic_events', { params: { inputOf: processId } })
  t.equal(readResponse && readResponse.edges && readResponse.edges.length, 1, 'field query index present')
  t.deepEqual(readResponse && readResponse.edges && readResponse.edges[0] && readResponse.edges[0].node && readResponse.edges[0].node.id, iEventId, 'query index OK')

  // ASSERT: test process input query edge
  readResponse = await observation.call('process_index', 'query_processes', { params: { inputs: iEventId } })
  t.equal(readResponse && readResponse.edges.length, 1, 'reciprocal query index present')
  t.deepEqual(readResponse && readResponse.edges[0] && readResponse.edges[0].node && readResponse.edges[0].node.id, processId, 'reciprocal query index OK')

  // SCENARIO: wipe associated record
  const delResp = await observation.call('economic_event', 'delete_economic_event', { revisionId: iEventRev })
  t.ok(delResp, 'input record deleted')
  await pause(100)

  // ASSERT: test forward link field
  try {
    readResponse = await observation.call('economic_event', 'get_economic_event', { address: iEventId })
  } catch (err) {
    t.ok(err.data.data.includes('No entry at this address'), 'record deletion OK')
  }

  // ASSERT: test reciprocal link field
  readResponse = await observation.call('process', 'get_process', { address: processId })
  t.notOk(readResponse.process.inputs, 'reciprocal field reference removed')

  // ASSERT: test commitment input query edge
  readResponse = await observation.call('economic_event_index', 'query_economic_events', { params: { inputOf: processId } })
  t.equal(readResponse && readResponse.edges.length, 0, 'field query index removed')

  // ASSERT: test process input query edge
  readResponse = await observation.call('process_index', 'query_processes', { params: { inputs: iEventId } })
  t.equal(readResponse && readResponse.edges.length, 0, 'reciprocal query index removed')

  await alice.scenario.cleanUp()
})
