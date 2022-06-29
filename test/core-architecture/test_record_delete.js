import test from 'tape'
import { pause } from '@holochain/tryorama'
import {
  buildPlayer,
  mockIdentifier,
  mockAddress,
} from '../init.js'

const testEventProps = {
  action: 'raise',
  resourceClassifiedAs: ['some-resource-type'],
  resourceQuantity: { hasNumericalValue: 1, hasUnit: mockIdentifier(false) },
  provider: mockAddress(false),
  receiver: mockAddress(false),
  due: '2019-11-19T04:29:55.056Z',
}

test('record deletion API', async (t) => {
  const alice = await buildPlayer(['planning'])
  const { cells: [planning] } = alice

  // write records
  const commitment = {
    note: 'a commitment to provide something',
    ...testEventProps,
  }
  const commitmentResponse = await planning.call('commitment', 'create_commitment', { commitment })
  t.ok(commitmentResponse.commitment && commitmentResponse.commitment.id, 'commitment created successfully')
  await pause(100)
  const commitmentId = commitmentResponse.commitment.id

  // attempt retrieval
  let readResp = await planning.call('commitment', 'get_commitment', { address: commitmentId })
  t.deepLooseEqual(readResp.commitment.id, commitmentId, 'record retrievable')

  // perform deletion
  const delResp = await planning.call('commitment', 'delete_commitment', { revisionId: commitmentResponse.commitment.revisionId })
  t.ok(delResp, 'record deleted successfully')
  await pause(100)

  // attempt retrieval
  try {
    await planning.call('commitment', 'get_commitment', { address: commitmentId })
  } catch (err) {
    t.ok(err.data.data.includes('No entry at this address'), 'record not retrievable once deleted')
  }

  await alice.scenario.cleanUp()
})

test('Cannot delete records of a different type via zome API deletion handlers', async (t) => {
  const alice = await buildPlayer(['planning'])
  const { cells: [planning] } = alice

  // SCENARIO: write records
  const commitment = {
    note: 'a commitment to provide something',
    ...testEventProps,
  }
  const commitmentResponse = await planning.call('commitment', 'create_commitment', { commitment })
  t.ok(commitmentResponse.commitment && commitmentResponse.commitment.id, 'commitment created successfully')
  await pause(100)
  const commitmentId = commitmentResponse.commitment.id

  const satisfaction = {
    satisfies: commitmentId, // erroneous but doesn't matter for now
    satisfiedBy: commitmentId,
    note: 'satisfaction indicating the relationship',
  }
  const satisfactionResp = await planning.call('satisfaction', 'create_satisfaction', { satisfaction })
  t.ok(satisfactionResp.satisfaction && satisfactionResp.satisfaction.id, 'satisfaction created successfully')
  await pause(100)

  // attempt to delete commitment via satisfaction deletion API
  try {
    await planning.call('satisfaction', 'delete_satisfaction', { revisionId: commitmentResponse.commitment.revisionId })
  } catch (err) {
    t.ok(err.data.data.includes('Could not convert entry to requested type'), 'records not deleteable via IDs of incorrect type')
  }

  await alice.scenario.cleanUp()
})
