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
  resourceClassifiedAs: ['some-resource-type'],
  resourceQuantity: { hasNumericalValue: 1, hasUnit: 'dangling-unit-todo-tidy-up' },
  provider: 'agentid-1-todo',
  receiver: 'agentid-2-todo',
  due: '2019-11-19T04:29:55.056Z',
}

runner.registerScenario('updating remote link fields syncs fields and associated indexes', async (s, t) => {
  const { alice } = await s.players({ alice: config }, true)

  // SCENARIO: write initial records
  const process = {
    name: 'context record for testing relationships',
  }
  const pResp = await alice.call('observation', 'process', 'create_process', { process })
  t.ok(pResp.Ok.process && pResp.Ok.process.id, 'target record created successfully')
  await s.consistency()
  const processId = pResp.Ok.process.id

  const process2 = {
    name: 'second context record for testing relationships',
  }
  const pResp2 = await alice.call('observation', 'process', 'create_process', { process: process2 })
  t.ok(pResp2.Ok.process && pResp2.Ok.process.id, 'secondary record created successfully')
  await s.consistency()
  const differentProcessId = pResp2.Ok.process.id

  const iCommitment = {
    note: 'test input commitment',
    inputOf: processId,
    ...testEventProps,
  }
  const icResp = await alice.call('planning', 'commitment', 'create_commitment', { commitment: iCommitment })
  t.ok(icResp.Ok.commitment && icResp.Ok.commitment.id, 'input record created successfully')
  t.equal(icResp.Ok.commitment.inputOf, processId, 'field reference OK in write')
  await s.consistency()
  const iCommitmentId = icResp.Ok.commitment.id

  // ASSERT: test forward link field
  let readResponse = await alice.call('planning', 'commitment', 'get_commitment', { address: iCommitmentId })
  t.equal(readResponse.Ok.commitment && readResponse.Ok.commitment.inputOf, processId, 'field reference OK on read')

  // ASSERT: test reciprocal link field
  readResponse = await alice.call('observation', 'process', 'get_process', { address: processId })
  t.equal(readResponse.Ok.process
    && readResponse.Ok.process.committedInputs
    && readResponse.Ok.process.committedInputs[0], iCommitmentId, 'reciprocal field reference OK on read')

  // ASSERT: test commitment input query edge
  readResponse = await alice.call('planning', 'commitment', 'query_commitments', { params: { inputOf: processId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 1, 'field query index present')
  t.equal(readResponse.Ok && readResponse.Ok[0] && readResponse.Ok[0].commitment && readResponse.Ok[0].commitment.id, iCommitmentId, 'query index OK')

  // ASSERT: test process input query edge
  readResponse = await alice.call('observation', 'process', 'query_processes', { params: { committedInputs: iCommitmentId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 1, 'reciprocal query index present')
  t.equal(readResponse.Ok && readResponse.Ok[0] && readResponse.Ok[0].process && readResponse.Ok[0].process.id, processId, 'reciprocal query index OK')



  // SCENARIO: update link field
  const updateCommitment = {
    id: iCommitmentId,
    inputOf: differentProcessId,
  }
  const ieResp2 = await alice.call('planning', 'commitment', 'update_commitment', { commitment: updateCommitment })
  t.equal(ieResp2.Ok.commitment && ieResp2.Ok.commitment.inputOf, differentProcessId, 'record link field updated successfully')
  await s.consistency()

  // ASSERT: test commitment fields
  readResponse = await alice.call('planning', 'commitment', 'get_commitment', { address: iCommitmentId })
  t.ok(readResponse.Ok.commitment && readResponse.Ok.commitment.inputOf, 'field reference OK on read')
  t.equal(readResponse.Ok.commitment && readResponse.Ok.commitment.inputOf, differentProcessId, 'field updated successfully')

  // ASSERT: test new commitment input query edge
  readResponse = await alice.call('planning', 'commitment', 'query_commitments', { params: { inputOf: differentProcessId } })
  t.equal(readResponse.Ok && readResponse.Ok[0]
    && readResponse.Ok[0].commitment
    && readResponse.Ok[0].commitment.id, iCommitmentId, 'new field query index applied')

  // ASSERT: test stale commitment input query edge
  readResponse = await alice.call('planning', 'commitment', 'query_commitments', { params: { inputOf: processId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 0, 'stale field query index removed')

  // ASSERT: test process input query edge
  readResponse = await alice.call('observation', 'process', 'query_processes', { params: { committedInputs: iCommitmentId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 1, 'reciprocal query index count ok')
  t.equal(readResponse.Ok && readResponse.Ok[0]
    && readResponse.Ok[0].process
    && readResponse.Ok[0].process.id, differentProcessId, 'new reciprocal query index applied')



  // SCENARIO: update link field (no-op)
  const ieResp3 = await alice.call('planning', 'commitment', 'update_commitment', { commitment: updateCommitment })
  t.equal(ieResp3.Ok.commitment && ieResp3.Ok.commitment.inputOf, differentProcessId, 'update with same fields is no-op')
  await s.consistency()

  // ASSERT: test event fields
  readResponse = await alice.call('planning', 'commitment', 'get_commitment', { address: iCommitmentId })
  t.equal(readResponse.Ok.commitment && readResponse.Ok.commitment.inputOf, differentProcessId, 'field update no-op OK')



  // SCENARIO: remove link field
  const wipeEventInput = {
    id: iCommitmentId,
    inputOf: null,
  }
  const ieResp4 = await alice.call('planning', 'commitment', 'update_commitment', { commitment: wipeEventInput })
  t.equal(ieResp4.Ok.commitment && ieResp4.Ok.commitment.inputOf, undefined, 'update with null value erases field')
  await s.consistency()

  // ASSERT: test event fields
  readResponse = await alice.call('planning', 'commitment', 'get_commitment', { address: iCommitmentId })
  t.equal(readResponse.Ok.commitment && readResponse.Ok.commitment.inputOf, undefined, 'field erased successfully')

  // ASSERT: test event input query edge
  readResponse = await alice.call('planning', 'commitment', 'query_commitments', { params: { inputOf: differentProcessId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 0, 'field query index updated')

  // ASSERT: test process input query edge
  readResponse = await alice.call('observation', 'process', 'query_processes', { params: { committedInputs: iCommitmentId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 0, 'reciprocal field query index updated')



  // :TODO: attempt linking to nonexistent target (should this error, or happen regardless? Big question in distributed networks...)
  // :TODO: updates for fields when other values are present in the index array
})

runner.registerScenario('removing records with linked remote indexes clears them in associated records', async (s, t) => {
  const { alice } = await s.players({ alice: config }, true)

  // SCENARIO: write initial records
  const process = {
    name: 'context record for testing relationships',
  }
  const pResp = await alice.call('observation', 'process', 'create_process', { process })
  t.ok(pResp.Ok.process && pResp.Ok.process.id, 'record created successfully')
  await s.consistency()
  const processId = pResp.Ok.process.id

  const iIntent = {
    note: 'test input intent',
    inputOf: processId,
    ...testEventProps,
  }
  const iiResp = await alice.call('planning', 'intent', 'create_intent', { intent: iIntent })
  t.ok(iiResp.Ok.intent && iiResp.Ok.intent.id, 'input record created successfully')
  t.equal(iiResp.Ok.intent.inputOf, processId, 'field reference OK in write')
  await s.consistency()
  const iIntentId = iiResp.Ok.intent.id

  // ASSERT: test forward link field
  let readResponse = await alice.call('planning', 'intent', 'get_intent', { address: iIntentId })
  t.equal(readResponse.Ok.intent && readResponse.Ok.intent.inputOf, processId, 'field reference OK on read')

  // ASSERT: test reciprocal link field
  readResponse = await alice.call('observation', 'process', 'get_process', { address: processId })
  t.equal(readResponse.Ok.process
    && readResponse.Ok.process.intendedInputs
    && readResponse.Ok.process.intendedInputs[0], iIntentId, 'reciprocal field reference OK on read')

  // ASSERT: test commitment input query edge
  readResponse = await alice.call('planning', 'intent', 'query_intents', { params: { inputOf: processId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 1, 'field query index present')
  t.equal(readResponse.Ok && readResponse.Ok[0] && readResponse.Ok[0].intent && readResponse.Ok[0].intent.id, iIntentId, 'query index OK')

  // ASSERT: test process input query edge
  readResponse = await alice.call('observation', 'process', 'query_processes', { params: { intendedInputs: iIntentId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 1, 'reciprocal query index present')
  t.equal(readResponse.Ok && readResponse.Ok[0] && readResponse.Ok[0].process && readResponse.Ok[0].process.id, processId, 'reciprocal query index OK')



  // SCENARIO: wipe associated record
  const delResp = await alice.call('planning', 'intent', 'delete_intent', { address: iIntentId })
  t.ok(delResp.Ok, 'input record deleted')
  await s.consistency()

  // ASSERT: test forward link field
  readResponse = await alice.call('planning', 'intent', 'get_intent', { address: iIntentId })
  t.equal(readResponse.Err && readResponse.Err.Internal, 'No entry at this address', 'record deletion OK')

  // ASSERT: test reciprocal link field
  readResponse = await alice.call('observation', 'process', 'get_process', { address: processId })
  t.equal(readResponse.Ok.process
    && readResponse.Ok.process.intendedInputs.length, 0, 'reciprocal field reference removed')

  // ASSERT: test commitment input query edge
  readResponse = await alice.call('planning', 'intent', 'query_intents', { params: { inputOf: processId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 0, 'field query index removed')

  // ASSERT: test process input query edge
  readResponse = await alice.call('observation', 'process', 'query_processes', { params: { intendedInputs: iIntentId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 0, 'reciprocal query index removed')
})

runner.run()
