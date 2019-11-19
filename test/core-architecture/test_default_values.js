const {
  getDNA,
  buildConfig,
  buildRunner,
} = require('../init')

const runner = buildRunner()

const config = buildConfig({
  planning: getDNA('planning'),
})

const testEventProps = {
  action: 'produce',
  resourceClassifiedAs: ['some-resource-type'],
  resourceQuantity: { numericValue: 1, unit: 'dangling-unit-todo-tidy-up' },
  provider: 'agentid-1-todo',
  receiver: 'agentid-2-todo',
  due: '2019-11-19T04:29:55.056Z',
}

runner.registerScenario('fields with default values set are stored on creation', async (s, t) => {
  const { planning } = await s.players({ planning: config }, true)

  const commitment = {
    note: 'test event',
    ...testEventProps,
  }

  const createResponse = await planning.call('planning', 'commitment', 'create_commitment', { commitment })

  t.ok(createResponse.Ok.commitment && createResponse.Ok.commitment.id, 'record created successfully')
  t.equal(createResponse.Ok.commitment.finished, false, 'default value assigned on creation')

  await s.consistency()

  const readResponse = await planning.call('planning', 'commitment', 'get_commitment', { address: createResponse.Ok.commitment.id })

  t.equal(readResponse.Ok.commitment.finished, false, 'default value present upon reading')
})

runner.run()
