// Some special conveniences exist for link handling when linking between records within the same DNA,
// hence why there are special test cases for this.
const {
  getDNA,
  buildConfig,
  buildRunner,
} = require('../init')

const runner = buildRunner()

const config = buildConfig({
  observation: getDNA('observation'),
  planning: getDNA('planning'),
}, {
  vf_observation: ['planning', 'observation'],
})

runner.registerScenario('updating local link fields syncs fields and associated indexes', async (s, t) => {
  const { alice } = await s.players({ alice: config }, true)

  // SCENARIO: write initial records
  const process = {
    name: 'context process for testing relationships',
  }
  const pResp = await alice.call('observation', 'process', 'create_process', { process })
  t.ok(pResp.Ok.process && pResp.Ok.process.id, 'process created successfully')
  await s.consistency()
  const processId = pResp.Ok.process.id

  const process2 = {
    name: 'second context process for testing relationships',
  }
  const pResp2 = await alice.call('observation', 'process', 'create_process', { process: process2 })
  t.ok(pResp2.Ok.process && pResp2.Ok.process.id, 'process created successfully')
  await s.consistency()
  const differentProcessId = pResp2.Ok.process.id

  const iEvent = {
    note: 'test input event',
    action: 'produce',
    inputOf: processId,
  }
  const ieResp = await alice.call('observation', 'economic_event', 'create_event', { event: iEvent })
  t.ok(ieResp.Ok.economicEvent && ieResp.Ok.economicEvent.id, 'input record created successfully')
  t.equal(ieResp.Ok.economicEvent.inputOf, processId, 'field reference OK in write')
  await s.consistency()
  const iEventId = ieResp.Ok.economicEvent.id

  // ASSERT: test event fields
  let readResponse = await alice.call('observation', 'economic_event', 'get_event', { address: iEventId })
  t.equal(readResponse.Ok.economicEvent && readResponse.Ok.economicEvent.inputOf, processId, 'field reference OK on read')

  // ASSERT: test event input query edge
  readResponse = await alice.call('observation', 'economic_event', 'query_events', { params: { inputOf: processId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 1, 'field query index present')
  t.equal(readResponse.Ok[0] && readResponse.Ok[0].economicEvent && readResponse.Ok[0].economicEvent.id, iEventId, 'query index OK')

  // ASSERT: test process input query edge
  readResponse = await alice.call('observation', 'process', 'query_processes', { params: { inputs: iEventId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 1, 'reciprocal query index present')
  t.equal(readResponse.Ok[0] && readResponse.Ok[0].process && readResponse.Ok[0].process.id, processId, 'reciprocal query index OK')

// :TODO: need to find a new record with a local zome link to test...  maybe EconomicResource.containedIn?
/*
  // SCENARIO: update link field
  const updateEvent = {
    id: iEventId,
    inputOf: differentProcessId,
  }
  const ieResp2 = await alice.call('observation', 'economic_event', 'update_event', { event: updateEvent })
  t.equal(ieResp2.Ok.economicEvent && ieResp2.Ok.economicEvent.inputOf, differentProcessId, 'record link field updated successfully')
  await s.consistency()

  // ASSERT: test event fields
  readResponse = await alice.call('observation', 'economic_event', 'get_event', { address: iEventId })
  t.ok(readResponse.Ok.economicEvent && readResponse.Ok.economicEvent.inputOf, 'field reference OK on read')
  t.equal(readResponse.Ok.economicEvent && readResponse.Ok.economicEvent.inputOf, differentProcessId, 'field updated successfully')

  // ASSERT: test event input query edge
  readResponse = await alice.call('observation', 'economic_event', 'query_events', { params: { inputOf: differentProcessId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 1, 'field query index present')
  t.equal(readResponse.Ok[0] && readResponse.Ok[0].economicEvent && readResponse.Ok[0].economicEvent.id, iEventId, 'field query index updated')

  // ASSERT: test process input query edge
  readResponse = await alice.call('observation', 'process', 'query_processes', { params: { inputs: iEventId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 1, 'process query index present')
  t.equal(readResponse.Ok[0] && readResponse.Ok[0].process && readResponse.Ok[0].process.id, differentProcessId, 'process query index updated')



  // SCENARIO: update link field (no-op)
  const ieResp3 = await alice.call('observation', 'economic_event', 'update_event', { event: updateEvent })
  t.equal(ieResp3.Ok.economicEvent && ieResp3.Ok.economicEvent.inputOf, differentProcessId, 'update with same fields is no-op')
  await s.consistency()

  // ASSERT: test event fields
  readResponse = await alice.call('observation', 'economic_event', 'get_event', { address: iEventId })
  t.equal(readResponse.Ok.economicEvent && readResponse.Ok.economicEvent.inputOf, differentProcessId, 'field update no-op OK')



  // SCENARIO: remove link field
  const wipeEventInput = {
    id: iEventId,
    inputOf: null,
  }
  const ieResp4 = await alice.call('observation', 'economic_event', 'update_event', { event: wipeEventInput })
  t.equal(ieResp4.Ok.economicEvent && ieResp4.Ok.economicEvent.inputOf, undefined, 'update with null value erases field')
  await s.consistency()

  // ASSERT: test event fields
  readResponse = await alice.call('observation', 'economic_event', 'get_event', { address: iEventId })
  t.equal(readResponse.Ok.economicEvent && readResponse.Ok.economicEvent.inputOf, undefined, 'field erased successfully')

  // ASSERT: test event input query edge
  readResponse = await alice.call('observation', 'economic_event', 'query_events', { params: { inputOf: differentProcessId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 0, 'field query index updated')

  // ASSERT: test process input query edge
  readResponse = await alice.call('observation', 'process', 'query_processes', { params: { inputs: iEventId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 0, 'process query index updated')
*/


  // SCENARIO: attempt linking to nonexistent target
  const badEvent = {
    action: 'produce',
    inputOf: "notarealprocess",
  }
  const badResp = await alice.call('observation', 'economic_event', 'create_event', { event: badEvent })
  // :TODO: should result in an error and avoid creating the entry if any invalid fields are provided
  // :TODO: this involves having a deep think about how much transactionality we want to enforce!
  t.equal(badResp.Ok.economicEvent && badResp.Ok.economicEvent.inputOf, undefined, 'create with bad value is ignored')
  await s.consistency()



  // :TODO: updates for fields with other values in the array
})

runner.registerScenario('removing records with linked local indexes clears them in associated records', async (s, t) => {
  const { alice } = await s.players({ alice: config }, true)

  // SCENARIO: write initial records
  const process = {
    name: 'context record for testing relationships',
  }
  const pResp = await alice.call('observation', 'process', 'create_process', { process })
  t.ok(pResp.Ok.process && pResp.Ok.process.id, 'context record created successfully')
  await s.consistency()
  const processId = pResp.Ok.process.id

  const iEvent = {
    note: 'test input event',
    action: 'produce',
    inputOf: processId,
  }
  const ieResp = await alice.call('observation', 'economic_event', 'create_event', { event: iEvent })
  t.ok(ieResp.Ok.economicEvent && ieResp.Ok.economicEvent.id, 'input record created successfully')
  t.equal(ieResp.Ok.economicEvent.inputOf, processId, 'field reference OK in write')
  await s.consistency()
  const iEventId = ieResp.Ok.economicEvent.id

  // ASSERT: test forward link field
  let readResponse = await alice.call('observation', 'economic_event', 'get_event', { address: iEventId })
  t.equal(readResponse.Ok.economicEvent && readResponse.Ok.economicEvent.inputOf, processId, 'field reference OK on read')

  // ASSERT: test reciprocal link field
  readResponse = await alice.call('observation', 'process', 'get_process', { address: processId })
  t.equal(readResponse.Ok.process
    && readResponse.Ok.process.inputs
    && readResponse.Ok.process.inputs[0], iEventId, 'reciprocal field reference OK on read')

  // ASSERT: test commitment input query edge
  readResponse = await alice.call('observation', 'economic_event', 'query_events', { params: { inputOf: processId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 1, 'field query index present')
  t.equal(readResponse.Ok && readResponse.Ok[0] && readResponse.Ok[0].economicEvent && readResponse.Ok[0].economicEvent.id, iEventId, 'query index OK')

  // ASSERT: test process input query edge
  readResponse = await alice.call('observation', 'process', 'query_processes', { params: { inputs: iEventId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 1, 'reciprocal query index present')
  t.equal(readResponse.Ok && readResponse.Ok[0] && readResponse.Ok[0].process && readResponse.Ok[0].process.id, processId, 'reciprocal query index OK')



  // SCENARIO: wipe associated record
  const delResp = await alice.call('observation', 'economic_event', 'delete_event', { address: iEventId })
  t.ok(delResp.Ok, 'input record deleted')
  await s.consistency()

  // ASSERT: test forward link field
  readResponse = await alice.call('observation', 'economic_event', 'get_event', { address: iEventId })
  t.equal(readResponse.Err && readResponse.Err.Internal, 'No entry at this address', 'record deletion OK')

  // ASSERT: test reciprocal link field
  readResponse = await alice.call('observation', 'process', 'get_process', { address: processId })
  t.equal(readResponse.Ok.process
    && readResponse.Ok.process.inputs.length, 0, 'reciprocal field reference removed')

  // ASSERT: test commitment input query edge
  readResponse = await alice.call('observation', 'economic_event', 'query_events', { params: { inputOf: processId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 0, 'field query index removed')

  // ASSERT: test process input query edge
  readResponse = await alice.call('observation', 'process', 'query_processes', { params: { inputs: iEventId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 0, 'reciprocal query index removed')
})

runner.run()
