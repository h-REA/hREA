const {
  getDNA,
  buildOrchestrator,
} = require('../init')

const runner = buildOrchestrator({
  planning: getDNA('planning'),
})

runner.registerScenario('fields with default values set are stored on creation', async (s, t, { planning }) => {
  const commitment = {
    note: 'test event',
  }

  const createResponse = await planning.call('commitment', 'create_commitment', { commitment })

  t.ok(createResponse.Ok.commitment && createResponse.Ok.commitment.id, 'record created successfully')
  t.equal(createResponse.Ok.commitment.finished, false, 'default value assigned on creation')

  await s.consistent()

  const readResponse = await planning.call('commitment', 'get_commitment', { address: createResponse.Ok.commitment.id })

  t.equal(readResponse.Ok.commitment.finished, false, 'default value present upon reading')
})

runner.run()
