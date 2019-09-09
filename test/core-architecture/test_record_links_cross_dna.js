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

runner.registerScenario('updating remote link fields changes field and associated index', async (s, t, { observation, planning }) => {
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
})

runner.run()
