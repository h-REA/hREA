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

runner.registerScenario('updating remote link fields syncs fields and associated indexes', async (s, t, { observation, planning }) => {
  // SCENARIO: write initial records
  const process = {
    name: 'context process for testing relationships',
  }
  const pResp = await observation.call('process', 'create_process', { process })
  t.ok(pResp.Ok.process && pResp.Ok.process.id, 'process created successfully')
  await s.consistent()
  const processId = pResp.Ok.process.id

  const process2 = {
    name: 'second context process for testing relationships',
  }
  const pResp2 = await observation.call('process', 'create_process', { process: process2 })
  t.ok(pResp2.Ok.process && pResp2.Ok.process.id, 'process created successfully')
  await s.consistent()
  const differentProcessId = pResp2.Ok.process.id

  const iCommitment = {
    note: 'test input commitment',
    inputOf: processId,
  }
  const icResp = await planning.call('commitment', 'create_commitment', { commitment: iCommitment })
  t.ok(icResp.Ok.commitment && icResp.Ok.commitment.id, 'input record created successfully')
  t.equal(icResp.Ok.commitment.inputOf, processId, 'field reference OK in write')
  await s.consistent()
  const iCommitmentId = icResp.Ok.commitment.id

  // ASSERT: test forward link field
  let readResponse = await planning.call('commitment', 'get_commitment', { address: iCommitmentId })
  t.equal(readResponse.Ok.commitment && readResponse.Ok.commitment.inputOf, processId, 'field reference OK on read')

  // ASSERT: test reciprocal link field
  readResponse = await observation.call('process', 'get_process', { address: processId })
  t.equal(readResponse.Ok.process
    && readResponse.Ok.process.committedInputs
    && readResponse.Ok.process.committedInputs[0], iCommitmentId, 'reciprocal field reference OK on read')

  // ASSERT: test commitment input query edge
  readResponse = await planning.call('commitment', 'query_commitments', { params: { inputOf: processId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 1, 'field query index present')
  t.equal(readResponse.Ok && readResponse.Ok[0] && readResponse.Ok[0].commitment && readResponse.Ok[0].commitment.id, iCommitmentId, 'query index OK')

  // ASSERT: test process input query edge
  readResponse = await observation.call('process', 'query_processes', { params: { committedInputs: iCommitmentId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 1, 'reciprocal query index present')
  t.equal(readResponse.Ok && readResponse.Ok[0] && readResponse.Ok[0].process && readResponse.Ok[0].process.id, processId, 'reciprocal query index OK')



  // SCENARIO: update link field
  const updateCommitment = {
    id: iCommitmentId,
    inputOf: differentProcessId,
  }
  const ieResp2 = await planning.call('commitment', 'update_commitment', { commitment: updateCommitment })
  t.equal(ieResp2.Ok.commitment && ieResp2.Ok.commitment.inputOf, differentProcessId, 'record link field updated successfully')
  await s.consistent()

  // ASSERT: test commitment fields
  readResponse = await planning.call('commitment', 'get_commitment', { address: iCommitmentId })
  t.ok(readResponse.Ok.commitment && readResponse.Ok.commitment.inputOf, 'field reference OK on read')
  t.equal(readResponse.Ok.commitment && readResponse.Ok.commitment.inputOf, differentProcessId, 'field updated successfully')

  // ASSERT: test new commitment input query edge
  readResponse = await planning.call('commitment', 'query_commitments', { params: { inputOf: differentProcessId } })
  t.equal(readResponse.Ok && readResponse.Ok[0]
    && readResponse.Ok[0].commitment
    && readResponse.Ok[0].commitment.id, iCommitmentId, 'new field query index applied')

  // ASSERT: test stale commitment input query edge
  readResponse = await planning.call('commitment', 'query_commitments', { params: { inputOf: processId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 0, 'stale field query index removed')

  // ASSERT: test process input query edge
  readResponse = await observation.call('process', 'query_processes', { params: { committedInputs: iCommitmentId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 1, 'reciprocal query index count ok')
  t.equal(readResponse.Ok && readResponse.Ok[0]
    && readResponse.Ok[0].process
    && readResponse.Ok[0].process.id, differentProcessId, 'new reciprocal query index applied')



  // SCENARIO: update link field (no-op)
  const ieResp3 = await planning.call('commitment', 'update_commitment', { commitment: updateCommitment })
  t.equal(ieResp3.Ok.commitment && ieResp3.Ok.commitment.inputOf, differentProcessId, 'update with same fields is no-op')
  await s.consistent()

  // ASSERT: test event fields
  readResponse = await planning.call('commitment', 'get_commitment', { address: iCommitmentId })
  t.equal(readResponse.Ok.commitment && readResponse.Ok.commitment.inputOf, differentProcessId, 'field update no-op OK')



  // SCENARIO: remove link field
  const wipeEventInput = {
    id: iCommitmentId,
    inputOf: null,
  }
  const ieResp4 = await planning.call('commitment', 'update_commitment', { commitment: wipeEventInput })
  t.equal(ieResp4.Ok.commitment && ieResp4.Ok.commitment.inputOf, undefined, 'update with null value erases field')
  await s.consistent()

  // ASSERT: test event fields
  readResponse = await planning.call('commitment', 'get_commitment', { address: iCommitmentId })
  t.equal(readResponse.Ok.commitment && readResponse.Ok.commitment.inputOf, undefined, 'field erased successfully')

  // ASSERT: test event input query edge
  readResponse = await planning.call('commitment', 'query_commitments', { params: { inputOf: differentProcessId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 0, 'field query index updated')

  // ASSERT: test process input query edge
  readResponse = await observation.call('process', 'query_processes', { params: { committedInputs: iCommitmentId } })
  t.equal(readResponse.Ok && readResponse.Ok.length, 0, 'process query index updated')



  // :TODO: attempt linking to nonexistent target (should this error, or happen regardless? Big question in distributed networks...)
  // :TODO: updates for fields when other values are present in the index array
})

runner.run()
