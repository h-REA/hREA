const {
  getDNA,
  buildConfig,
  buildRunner,
  buildPlayer,
  mockIdentifier,
  mockAgentId,
} = require('../init')

const runner = buildRunner()

const config = buildConfig({
  planning: getDNA('planning'),
})

const testEventProps = {
  action: 'raise',
  resourceClassifiedAs: ['some-resource-type'],
  resourceQuantity: { hasNumericalValue: 1, hasUnit: mockIdentifier(false) },
  provider: mockAgentId(false),
  receiver: mockAgentId(false),
  due: '2019-11-19T04:29:55.056Z',
}

runner.registerScenario('record deletion API', async (s, t) => {
  const { cells: [planning] } = await buildPlayer(s, config, ['planning'])

  // write records
  const commitment = {
    note: 'a commitment to provide something',
    ...testEventProps,
  }
  const commitmentResponse = await planning.call('commitment', 'create_commitment', { commitment })
  t.ok(commitmentResponse.commitment && commitmentResponse.commitment.id, 'commitment created successfully')
  await s.consistency()
  const commitmentId = commitmentResponse.commitment.id

  // attempt retrieval
  let readResp = await planning.call('commitment', 'get_commitment', { address: commitmentId })
  t.deepEqual(readResp.commitment.id, commitmentId, 'record retrievable')

  // perform deletion
  const delResp = await planning.call('commitment', 'delete_commitment', { revisionId: commitmentResponse.commitment.revisionId })
  t.ok(delResp, 'record deleted successfully')
  await s.consistency()

  // attempt retrieval
  try {
    await planning.call('commitment', 'get_commitment', { address: commitmentId })
  } catch (err) {
    t.ok(err.data.data.includes('No entry at this address'), 'record not retrievable once deleted')
  }
})

const runner2 = buildRunner()

runner2.registerScenario('Cannot delete records of a different type via zome API deletion handlers', async (s, t) => {
  const { cells: [planning] } = await buildPlayer(s, config, ['planning'])

  // SCENARIO: write records
  const commitment = {
    note: 'a commitment to provide something',
    ...testEventProps,
  }
  const commitmentResponse = await planning.call('commitment', 'create_commitment', { commitment })
  t.ok(commitmentResponse.commitment && commitmentResponse.commitment.id, 'commitment created successfully')
  await s.consistency()
  const commitmentId = commitmentResponse.commitment.id

  const satisfaction = {
    satisfies: commitmentId, // erroneous but doesn't matter for now
    satisfiedBy: commitmentId,
    note: 'satisfaction indicating the relationship',
  }
  const satisfactionResp = await planning.call('satisfaction', 'create_satisfaction', { satisfaction })
  t.ok(satisfactionResp.satisfaction && satisfactionResp.satisfaction.id, 'satisfaction created successfully')
  await s.consistency()

  // attempt to delete commitment via satisfaction deletion API
  try {
    await planning.call('satisfaction', 'delete_satisfaction', { revisionId: commitmentResponse.commitment.revisionId })
  } catch (err) {
    t.ok(err.data.data.includes('Could not convert entry to requested type'), 'records not deleteable via IDs of incorrect type')
  }
})

runner.run()
runner2.run()
