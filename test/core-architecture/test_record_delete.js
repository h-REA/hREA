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
  action: 'raise',
  resourceClassifiedAs: ['some-resource-type'],
  resourceQuantity: { hasNumericalValue: 1, hasUnit: 'dangling-unit-todo-tidy-up' },
  provider: 'agentid-1-todo',
  receiver: 'agentid-2-todo',
  due: '2019-11-19T04:29:55.056Z',
}

runner.registerScenario('record deletion API', async (s, t) => {
  const { planning } = await s.players({ planning: config }, true)

  // write records
  const commitment = {
    note: 'a commitment to provide something',
    ...testEventProps,
  }
  const commitmentResponse = await planning.call('planning', 'commitment', 'create_commitment', { commitment })
  t.ok(commitmentResponse.Ok.commitment && commitmentResponse.Ok.commitment.id, 'commitment created successfully')
  await s.consistency()
  const commitmentId = commitmentResponse.Ok.commitment.id

  // attempt retrieval
  let readResp = await planning.call('planning', 'commitment', 'get_commitment', { address: commitmentId })
  t.equal(readResp.Ok.commitment.id, commitmentId, 'record not retrievable')

  // perform deletion
  const delResp = await planning.call('planning', 'commitment', 'delete_commitment', { address: commitmentId })
  t.ok(delResp.Ok, 'record deleted successfully')
  await s.consistency()

  // attempt retrieval
  readResp = await planning.call('planning', 'commitment', 'get_commitment', { address: commitmentId })
  t.equal(readResp.Err.Internal, 'No entry at this address', 'record not retrievable once deleted')
})

runner.registerScenario('Cannot delete records of a different type via zome API deletion handlers', async (s, t) => {
  const { planning } = await s.players({ planning: config }, true)

  // SCENARIO: write records
  const commitment = {
    note: 'a commitment to provide something',
    ...testEventProps,
  }
  const commitmentResponse = await planning.call('planning', 'commitment', 'create_commitment', { commitment })
  t.ok(commitmentResponse.Ok.commitment && commitmentResponse.Ok.commitment.id, 'commitment created successfully')
  await s.consistency()
  const commitmentId = commitmentResponse.Ok.commitment.id

  const fulfillment = {
    fulfills: commitmentId,
    fulfilledBy: commitmentId,  // erroneous but doesn't matter for now
    note: 'fulfillment indicating the relationship',
  }
  const fulfillmentResp = await planning.call('planning', 'fulfillment', 'create_fulfillment', { fulfillment })
  t.ok(fulfillmentResp.Ok.fulfillment && fulfillmentResp.Ok.fulfillment.id, 'fulfillment created successfully')
  await s.consistency()
  const fulfillmentId = fulfillmentResp.Ok.fulfillment.id

  // attempt to delete commitment via fulfillment deletion API
  const delResp = await planning.call('planning', 'fulfillment', 'delete_fulfillment', { address: commitmentId })
  t.equal(delResp.Err.ValidationFailed, 'incorrect record type specified for deletion')
})

runner.run()
