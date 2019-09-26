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
    action: 'consume',
    inputOf: processId,
  }
  const ieResp = await observation.call('economic_event', 'create_event', { event: iEvent })
  t.ok(ieResp.Ok.economicEvent && ieResp.Ok.economicEvent.id, 'input event created successfully')
  t.equal(ieResp.Ok.economicEvent.inputOf, processId, 'event.inputOf reference OK in write')
  await s.consistent()
  const iEventId = ieResp.Ok.economicEvent.id

  const oEvent = {
    note: 'test output event',
    action: 'produce',
    outputOf: processId,
  }
  const oeResp = await observation.call('economic_event', 'create_event', { event: oEvent })
  t.ok(oeResp.Ok.economicEvent && oeResp.Ok.economicEvent.id, 'output event created successfully')
  t.equal(oeResp.Ok.economicEvent.outputOf, processId, 'event.outputOf reference OK in write')
  await s.consistent()
  const oEventId = oeResp.Ok.economicEvent.id

  const iCommitment = {
    note: 'test input commitment',
    inputOf: processId,
  }
  const icResp = await planning.call('commitment', 'create_commitment', { commitment: iCommitment })
  t.ok(icResp.Ok.commitment && icResp.Ok.commitment.id, 'input commitment created successfully')
  t.equal(icResp.Ok.commitment.inputOf, processId, 'commitment.inputOf reference OK in write')
  await s.consistent()
  const iCommitmentId = icResp.Ok.commitment.id

  const oCommitment = {
    note: 'test output commitment',
    outputOf: processId,
  }
  const ocResp = await planning.call('commitment', 'create_commitment', { commitment: oCommitment })
  t.ok(ocResp.Ok.commitment && ocResp.Ok.commitment.id, 'output commitment created successfully')
  t.equal(ocResp.Ok.commitment.outputOf, processId, 'commitment.outputOf reference OK in write')
  await s.consistent()
  const oCommitmentId = ocResp.Ok.commitment.id

  const iIntent = {
    note: 'test input intent',
    inputOf: processId,
  }
  const iiResp = await planning.call('intent', 'create_intent', { intent: iIntent })
  t.ok(iiResp.Ok.intent && iiResp.Ok.intent.id, 'input intent created successfully')
  t.equal(iiResp.Ok.intent.inputOf, processId, 'intent.inputOf reference OK in write')
  await s.consistent()
  const iIntentId = iiResp.Ok.intent.id

  const oIntent = {
    note: 'test output intent',
    outputOf: processId,
  }
  const oiResp = await planning.call('intent', 'create_intent', { intent: oIntent })
  t.ok(oiResp.Ok.intent && oiResp.Ok.intent.id, 'output intent created successfully')
  t.equal(oiResp.Ok.intent.outputOf, processId, 'intent.outputOf reference OK in write')
  await s.consistent()
  const oIntentId = oiResp.Ok.intent.id



  // ASSERT: check input event index links
  readResponse = await observation.call('economic_event', 'get_event', { address: iEventId })
  t.ok(readResponse.Ok.economicEvent && readResponse.Ok.economicEvent.inputOf, 'EconomicEvent.inputOf index saved')
  t.equal(readResponse.Ok.economicEvent && readResponse.Ok.economicEvent.inputOf, processId, 'EconomicEvent.inputOf reference OK in read')

  // ASSERT: check output event index links
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



  // ASSERT: check input commitment index links
  readResponse = await observation.call('commitment', 'get_commitment', { address: iCommitmentId })
  t.ok(readResponse.Ok.commitment && readResponse.Ok.commitment.inputOf, 'commitment.inputOf index saved')
  t.equal(readResponse.Ok.commitment && readResponse.Ok.commitment.inputOf, processId, 'commitment.inputOf reference OK in read')

  // ASSERT: check output commitment index links
  readResponse = await observation.call('commitment', 'get_commitment', { address: oCommitmentId })
  t.ok(readResponse.Ok.commitment && readResponse.Ok.commitment.outputOf, 'commitment.outputOf index saved')
  t.equal(readResponse.Ok.commitment && readResponse.Ok.commitment.outputOf, processId, 'commitment.outputOf reference OK in read')

  // ASSERT: test commitment input query edge
  readResponse = await observation.call('commitment', 'query_commitments', { params: { inputOf: processId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 1, 'commitment input query index present')
  t.equal(readResponse.Ok[0] && readResponse.Ok[0].commitment && readResponse.Ok[0].commitment.id, iCommitmentId, 'commitment input query index created')

  // ASSERT: test commitment output query edge
  readResponse = await observation.call('commitment', 'query_commitments', { params: { outputOf: processId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 1, 'commitment output query index present')
  t.equal(readResponse.Ok[0] && readResponse.Ok[0].commitment && readResponse.Ok[0].commitment.id, oCommitmentId, 'commitment output query index created')

  // ASSERT: check process commitment input query edge
  readResponse = await observation.call('process', 'query_processes', { params: { committedInputs: iCommitmentId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 1, 'process.inputs query succeeded')
  t.equal(readResponse.Ok[0] && readResponse.Ok[0].process && readResponse.Ok[0].process.id, processId, 'process.committedInputs query index created')

  // ASSERT: check process commitment output query edge
  readResponse = await observation.call('process', 'query_processes', { params: { committedOutputs: oCommitmentId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 1, 'process.outputs query succeeded')
  t.equal(readResponse.Ok[0] && readResponse.Ok[0].process && readResponse.Ok[0].process.id, processId, 'process.committedOutputs query index created')



  // ASSERT: check input intent index links
  readResponse = await observation.call('intent', 'get_intent', { address: iIntentId })
  t.ok(readResponse.Ok.intent && readResponse.Ok.intent.inputOf, 'intent.inputOf index saved')
  t.equal(readResponse.Ok.intent && readResponse.Ok.intent.inputOf, processId, 'intent.inputOf reference OK in read')

  // ASSERT: check output intent index links
  readResponse = await observation.call('intent', 'get_intent', { address: oIntentId })
  t.ok(readResponse.Ok.intent && readResponse.Ok.intent.outputOf, 'intent.outputOf index saved')
  t.equal(readResponse.Ok.intent && readResponse.Ok.intent.outputOf, processId, 'intent.outputOf reference OK in read')

  // ASSERT: test intent input query edge
  readResponse = await observation.call('intent', 'query_intents', { params: { inputOf: processId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 1, 'intent input query index present')
  t.equal(readResponse.Ok[0] && readResponse.Ok[0].intent && readResponse.Ok[0].intent.id, iIntentId, 'intent input query index created')

  // ASSERT: test intent output query edge
  readResponse = await observation.call('intent', 'query_intents', { params: { outputOf: processId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 1, 'intent output query index present')
  t.equal(readResponse.Ok[0] && readResponse.Ok[0].intent && readResponse.Ok[0].intent.id, oIntentId, 'intent output query index created')

  // ASSERT: check process intent input query edge
  readResponse = await observation.call('process', 'query_processes', { params: { committedInputs: iIntentId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 1, 'process.inputs query succeeded')
  t.equal(readResponse.Ok[0] && readResponse.Ok[0].process && readResponse.Ok[0].process.id, processId, 'process.committedInputs query index created')

  // ASSERT: check process intent output query edge
  readResponse = await observation.call('process', 'query_processes', { params: { committedOutputs: oIntentId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 1, 'process.outputs query succeeded')
  t.equal(readResponse.Ok[0] && readResponse.Ok[0].process && readResponse.Ok[0].process.id, processId, 'process.committedOutputs query index created')


  // TODO: modify
})

runner.run()
