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

const testEventProps = {
  action: 'produce',
  provider: 'agentid-1-todo',
  receiver: 'agentid-2-todo',
  hasPointInTime: '2019-11-19T04:29:55.056Z',
  resourceClassifiedAs: ['resource-type-uri'],
  resourceQuantity: { numericValue: 1, unit: 'dangling-unit-todo-tidy-up' },
}

runner.registerScenario('process query indexes and relationships', async (s, t) => {
  const { alice } = await s.players({ alice: config }, true)

  // SCENARIO: write records
  const process = {
    name: 'test process for linking logic',
  }
  const pResp = await alice.call('observation', 'process', 'create_process', { process })
  t.ok(pResp.Ok.process && pResp.Ok.process.id, 'process created successfully')
  await s.consistency()
  const processId = pResp.Ok.process.id

  const iEvent = {
    note: 'test input event',
    action: 'consume',
    inputOf: processId,
    ...testEventProps,
  }
  const ieResp = await alice.call('observation', 'economic_event', 'create_event', { event: iEvent })
  t.ok(ieResp.Ok.economicEvent && ieResp.Ok.economicEvent.id, 'input event created successfully')
  t.equal(ieResp.Ok.economicEvent.inputOf, processId, 'event.inputOf reference OK in write')
  await s.consistency()
  const iEventId = ieResp.Ok.economicEvent.id

  const oEvent = {
    note: 'test output event',
    action: 'produce',
    outputOf: processId,
    ...testEventProps,
  }
  const oeResp = await alice.call('observation', 'economic_event', 'create_event', { event: oEvent })
  t.ok(oeResp.Ok.economicEvent && oeResp.Ok.economicEvent.id, 'output event created successfully')
  t.equal(oeResp.Ok.economicEvent.outputOf, processId, 'event.outputOf reference OK in write')
  await s.consistency()
  const oEventId = oeResp.Ok.economicEvent.id

  const iCommitment = {
    note: 'test input commitment',
    inputOf: processId,
    ...testEventProps,
  }
  const icResp = await alice.call('planning', 'commitment', 'create_commitment', { commitment: iCommitment })
  t.ok(icResp.Ok.commitment && icResp.Ok.commitment.id, 'input commitment created successfully')
  t.equal(icResp.Ok.commitment.inputOf, processId, 'commitment.inputOf reference OK in write')
  await s.consistency()
  const iCommitmentId = icResp.Ok.commitment.id

  const oCommitment = {
    note: 'test output commitment',
    outputOf: processId,
    ...testEventProps,
  }
  const ocResp = await alice.call('planning', 'commitment', 'create_commitment', { commitment: oCommitment })
  t.ok(ocResp.Ok.commitment && ocResp.Ok.commitment.id, 'output commitment created successfully')
  t.equal(ocResp.Ok.commitment.outputOf, processId, 'commitment.outputOf reference OK in write')
  await s.consistency()
  const oCommitmentId = ocResp.Ok.commitment.id

  const iIntent = {
    note: 'test input intent',
    inputOf: processId,
    ...testEventProps,
  }
  const iiResp = await alice.call('planning', 'intent', 'create_intent', { intent: iIntent })
  t.ok(iiResp.Ok.intent && iiResp.Ok.intent.id, 'input intent created successfully')
  t.equal(iiResp.Ok.intent.inputOf, processId, 'intent.inputOf reference OK in write')
  await s.consistency()
  const iIntentId = iiResp.Ok.intent.id

  const oIntent = {
    note: 'test output intent',
    outputOf: processId,
    ...testEventProps,
  }
  const oiResp = await alice.call('planning', 'intent', 'create_intent', { intent: oIntent })
  t.ok(oiResp.Ok.intent && oiResp.Ok.intent.id, 'output intent created successfully')
  t.equal(oiResp.Ok.intent.outputOf, processId, 'intent.outputOf reference OK in write')
  await s.consistency()
  const oIntentId = oiResp.Ok.intent.id



  // ASSERT: check input event index links
  readResponse = await alice.call('observation', 'economic_event', 'get_event', { address: iEventId })
  t.ok(readResponse.Ok.economicEvent && readResponse.Ok.economicEvent.inputOf, 'EconomicEvent.inputOf index saved')
  t.equal(readResponse.Ok.economicEvent && readResponse.Ok.economicEvent.inputOf, processId, 'EconomicEvent.inputOf reference OK in read')

  // ASSERT: check output event index links
  readResponse = await alice.call('observation', 'economic_event', 'get_event', { address: oEventId })
  t.ok(readResponse.Ok.economicEvent && readResponse.Ok.economicEvent.outputOf, 'EconomicEvent.outputOf index saved')
  t.equal(readResponse.Ok.economicEvent && readResponse.Ok.economicEvent.outputOf, processId, 'EconomicEvent.outputOf reference OK in read')

  // ASSERT: test event input query edge
  readResponse = await alice.call('observation', 'economic_event', 'query_events', { params: { inputOf: processId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 1, 'event input query index present')
  t.equal(readResponse.Ok[0] && readResponse.Ok[0].economicEvent && readResponse.Ok[0].economicEvent.id, iEventId, 'event input query index created')

  // ASSERT: test event output query edge
  readResponse = await alice.call('observation', 'economic_event', 'query_events', { params: { outputOf: processId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 1, 'event output query index present')
  t.equal(readResponse.Ok[0] && readResponse.Ok[0].economicEvent && readResponse.Ok[0].economicEvent.id, oEventId, 'event output query index created')

  // ASSERT: check process event input query edge
  readResponse = await alice.call('observation', 'process', 'query_processes', { params: { inputs: iEventId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 1, 'process.inputs query succeeded')
  t.equal(readResponse.Ok[0] && readResponse.Ok[0].process && readResponse.Ok[0].process.id, processId, 'process.inputs query index created')

  // ASSERT: check process event output query edge
  readResponse = await alice.call('observation', 'process', 'query_processes', { params: { outputs: oEventId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 1, 'process.outputs query succeeded')
  t.equal(readResponse.Ok[0] && readResponse.Ok[0].process && readResponse.Ok[0].process.id, processId, 'process.outputs query index created')



  // ASSERT: check input commitment index links
  readResponse = await alice.call('planning', 'commitment', 'get_commitment', { address: iCommitmentId })
  t.ok(readResponse.Ok.commitment && readResponse.Ok.commitment.inputOf, 'commitment.inputOf index saved')
  t.equal(readResponse.Ok.commitment && readResponse.Ok.commitment.inputOf, processId, 'commitment.inputOf reference OK in read')

  // ASSERT: check output commitment index links
  readResponse = await alice.call('planning', 'commitment', 'get_commitment', { address: oCommitmentId })
  t.ok(readResponse.Ok.commitment && readResponse.Ok.commitment.outputOf, 'commitment.outputOf index saved')
  t.equal(readResponse.Ok.commitment && readResponse.Ok.commitment.outputOf, processId, 'commitment.outputOf reference OK in read')

  // ASSERT: test commitment input query edge
  readResponse = await alice.call('planning', 'commitment', 'query_commitments', { params: { inputOf: processId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 1, 'commitment input query index present')
  t.equal(readResponse.Ok[0] && readResponse.Ok[0].commitment && readResponse.Ok[0].commitment.id, iCommitmentId, 'commitment input query index created')

  // ASSERT: test commitment output query edge
  readResponse = await alice.call('planning', 'commitment', 'query_commitments', { params: { outputOf: processId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 1, 'commitment output query index present')
  t.equal(readResponse.Ok[0] && readResponse.Ok[0].commitment && readResponse.Ok[0].commitment.id, oCommitmentId, 'commitment output query index created')

  // ASSERT: check process commitment input query edge
  readResponse = await alice.call('observation', 'process', 'query_processes', { params: { committedInputs: iCommitmentId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 1, 'process.committedInputs query succeeded')
  t.equal(readResponse.Ok[0] && readResponse.Ok[0].process && readResponse.Ok[0].process.id, processId, 'process.committedInputs query index created')

  // ASSERT: check process commitment output query edge
  readResponse = await alice.call('observation', 'process', 'query_processes', { params: { committedOutputs: oCommitmentId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 1, 'process.committedOutputs query succeeded')
  t.equal(readResponse.Ok[0] && readResponse.Ok[0].process && readResponse.Ok[0].process.id, processId, 'process.committedOutputs query index created')



  // ASSERT: check input intent index links
  readResponse = await alice.call('planning', 'intent', 'get_intent', { address: iIntentId })
  t.ok(readResponse.Ok.intent && readResponse.Ok.intent.inputOf, 'intent.inputOf index saved')
  t.equal(readResponse.Ok.intent && readResponse.Ok.intent.inputOf, processId, 'intent.inputOf reference OK in read')

  // ASSERT: check output intent index links
  readResponse = await alice.call('planning', 'intent', 'get_intent', { address: oIntentId })
  t.ok(readResponse.Ok.intent && readResponse.Ok.intent.outputOf, 'intent.outputOf index saved')
  t.equal(readResponse.Ok.intent && readResponse.Ok.intent.outputOf, processId, 'intent.outputOf reference OK in read')

  // ASSERT: test intent input query edge
  readResponse = await alice.call('planning', 'intent', 'query_intents', { params: { inputOf: processId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 1, 'intent input query index present')
  t.equal(readResponse.Ok[0] && readResponse.Ok[0].intent && readResponse.Ok[0].intent.id, iIntentId, 'intent input query index created')

  // ASSERT: test intent output query edge
  readResponse = await alice.call('planning', 'intent', 'query_intents', { params: { outputOf: processId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 1, 'intent output query index present')
  t.equal(readResponse.Ok[0] && readResponse.Ok[0].intent && readResponse.Ok[0].intent.id, oIntentId, 'intent output query index created')

  // ASSERT: check process intent input query edge
  readResponse = await alice.call('observation', 'process', 'query_processes', { params: { intendedInputs: iIntentId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 1, 'process.intendedInputs query succeeded')
  t.equal(readResponse.Ok[0] && readResponse.Ok[0].process && readResponse.Ok[0].process.id, processId, 'process.intendedInputs query index created')

  // ASSERT: check process intent output query edge
  readResponse = await alice.call('observation', 'process', 'query_processes', { params: { intendedOutputs: oIntentId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 1, 'process.intendedOutputs query succeeded')
  t.equal(readResponse.Ok[0] && readResponse.Ok[0].process && readResponse.Ok[0].process.id, processId, 'process.intendedOutputs query index created')


  // TODO: modify
})

runner.run()
