const {
  getDNA,
  buildOrchestrator,
} = require('../init')

const runner = buildOrchestrator({
  planning: getDNA('planning'),
})

runner.registerScenario('record deletion API', async (s, t, { planning }) => {
  // write records
  const commitment = {
    note: 'a commitment to provide something',
  }
  const commitmentResponse = await planning.call('commitment', 'create_commitment', { commitment })
  t.ok(commitmentResponse.Ok.commitment && commitmentResponse.Ok.commitment.id, 'commitment created successfully')
  await s.consistent()
  const commitmentId = commitmentResponse.Ok.commitment.id

  // attempt retrieval
  let readResp = await planning.call('commitment', 'get_commitment', { address: commitmentId })
  t.equal(readResp.Ok.commitment.id, commitmentId, 'record not retrievable')

  // perform deletion
  const delResp = await planning.call('commitment', 'delete_commitment', { address: commitmentId })
  t.ok(delResp.Ok, 'record deleted successfully')
  await s.consistent()

  // attempt retrieval
  readResp = await planning.call('commitment', 'get_commitment', { address: commitmentId })
  t.equal(readResp.Err.Internal, 'No entry at this address', 'record not retrievable once deleted')
})

runner.registerScenario('Cannot delete records of a different type via zome API deletion handlers', async (s, t, { planning }) => {
  // SCENARIO: write records
  const commitment = {
    note: 'a commitment to provide something',
  }
  const commitmentResponse = await planning.call('commitment', 'create_commitment', { commitment })
  t.ok(commitmentResponse.Ok.commitment && commitmentResponse.Ok.commitment.id, 'commitment created successfully')
  await s.consistent()
  const commitmentId = commitmentResponse.Ok.commitment.id

  const fulfillment = {
    fulfills: commitmentId,
    fulfilledBy: commitmentId,  // erroneous but doesn't matter for now
    note: 'fulfillment indicating the relationship',
  }
  const fulfillmentResp = await planning.call('fulfillment', 'create_fulfillment', { fulfillment })
  t.ok(fulfillmentResp.Ok.fulfillment && fulfillmentResp.Ok.fulfillment.id, 'fulfillment created successfully')
  await s.consistent()
  const fulfillmentId = fulfillmentResp.Ok.fulfillment.id

  // attempt to delete commitment via fulfillment deletion API
  const delResp = await planning.call('fulfillment', 'delete_fulfillment', { address: commitmentId })
  t.equal(delResp.Err.ValidationFailed, 'incorrect record type specified for deletion')
})

runner.run()
