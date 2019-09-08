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

runner.registerScenario('process query indexes and relationships', async (s, t, { planning, observation }) => {
  // SCENARIO: write records
  const process = {
    name: 'test process for linking logic',
  }
  const pResp = await observation.call('process', 'create_process', { process })
  t.ok(pResp.Ok.process && pResp.Ok.process.id, 'process created successfully')
  await s.consistent()
  const processId = pResp.Ok.process.id

  const iEvent = {
    note: 'test input event',
    inputOf: processId,
  }
  const ieResp = await observation.call('economic_event', 'create_event', { event: iEvent })
  t.ok(ieResp.Ok.economicEvent && ieResp.Ok.economicEvent.id, 'input event created successfully')
  t.equal(ieResp.Ok.economicEvent.inputOf, processId, 'event.inputOf reference OK in write')
  await s.consistent()
  const iEventId = ieResp.Ok.economicEvent.id

  const oEvent = {
    note: 'test output event',
    outputOf: processId,
  }
  const oeResp = await observation.call('economic_event', 'create_event', { event: oEvent })
  t.ok(oeResp.Ok.economicEvent && oeResp.Ok.economicEvent.id, 'output event created successfully')
  await s.consistent()
  const oEventId = oeResp.Ok.economicEvent.id

  // :TODO: commitment, intent



  // ASSERT: check input event index links
  readResponse = await observation.call('economic_event', 'get_event', { address: iEventId })
  t.ok(readResponse.Ok.economicEvent && readResponse.Ok.economicEvent.inputOf, 'EconomicEvent.inputOf index saved')
  t.equal(readResponse.Ok.economicEvent && readResponse.Ok.economicEvent.inputOf, processId, 'EconomicEvent.inputOf reference OK in read')

  readResponse = await observation.call('economic_event', 'get_event', { address: oEventId })
  t.ok(readResponse.Ok.economicEvent && readResponse.Ok.economicEvent.outputOf, 'EconomicEvent.outputOf index saved')
  t.equal(readResponse.Ok.economicEvent && readResponse.Ok.economicEvent.outputOf, processId, 'EconomicEvent.outputOf reference OK in read')

  // ASSERT: test event input query edge
  readResponse = await observation.call('economic_event', 'query_events', { params: { inputOf: processId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 1, 'event input query index present')
  t.equal(readResponse.Ok[0] && readResponse.Ok[0].economicEvent && readResponse.Ok[0].economicEvent.id, iEventId, 'event input query index created')

  // ASSERT: test event output query edge
  readResponse = await observation.call('economic_event', 'query_events', { params: { outputOf: processId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 1, 'event output query index present')
  t.equal(readResponse.Ok[0] && readResponse.Ok[0].economicEvent && readResponse.Ok[0].economicEvent.id, oEventId, 'event output query index created')

  // ASSERT: check process event input query edge
  readResponse = await observation.call('process', 'query_processes', { params: { inputs: iEventId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 1, 'process.inputs query succeeded')
  t.equal(readResponse.Ok[0] && readResponse.Ok[0].process && readResponse.Ok[0].process.id, processId, 'process.inputs query index created')

  // ASSERT: check process event output query edge
  readResponse = await observation.call('process', 'query_processes', { params: { outputs: oEventId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 1, 'process.outputs query succeeded')
  t.equal(readResponse.Ok[0] && readResponse.Ok[0].process && readResponse.Ok[0].process.id, processId, 'process.outputs query index created')


  // SCENARIO: modify events
})

runner.run()
