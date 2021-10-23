const {
  buildConfig,
  buildRunner,
  buildPlayer,
  mockIdentifier,
  mockAgentId,
} = require('../init')

const runner = buildRunner()

const config = buildConfig()

const testEventProps = {
  action: 'raise',
  resourceClassifiedAs: ['some-resource-type'],
  resourceQuantity: { hasNumericalValue: 1, hasUnit: mockIdentifier(false) },
  provider: mockAgentId(false),
  receiver: mockAgentId(false),
  due: '2019-11-19T04:29:55.056Z',
}

runner.registerScenario('fields with default values set are stored on creation', async (s, t) => {
  const { cells: [planning] } = await buildPlayer(s, config, ['planning'])

  const commitment = {
    note: 'test event',
    ...testEventProps,
  }

  const createResponse = await planning.call('commitment', 'create_commitment', { commitment })

  t.ok(createResponse.commitment && createResponse.commitment.id, 'record created successfully')
  t.equal(createResponse.commitment.finished, false, 'default value assigned on creation')

  await s.consistency()

  const readResponse = await planning.call('commitment', 'get_commitment', { address: createResponse.commitment.id })

  t.equal(readResponse.commitment.finished, false, 'default value present upon reading')
})

runner.run()
