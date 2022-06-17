import test from 'tape'
import { pause } from '@connoropolous/tryorama'
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

test('fields with default values set are stored on creation', async (t) => {
  const alice = await buildPlayer(['planning'])
  const { cells: [planning] } = alice

  const commitment = {
    note: 'test event',
    ...testEventProps,
  }

  const createResponse = await planning.call('commitment', 'create_commitment', { commitment })

  t.ok(createResponse.commitment && createResponse.commitment.id, 'record created successfully')
  t.equal(createResponse.commitment.finished, false, 'default value assigned on creation')

  await pause(100)

  const readResponse = await planning.call('commitment', 'get_commitment', { address: createResponse.commitment.id })

  t.equal(readResponse.commitment.finished, false, 'default value present upon reading')

  await alice.scenario.cleanUp()
})
