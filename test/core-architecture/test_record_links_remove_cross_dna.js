import test from 'tape'
import { pause } from '@holochain/tryorama'
import {
  buildPlayer,
  mockAddress,
  mockIdentifier,
} from '../init.js'

const testEventProps = {
  action: 'consume',
  resourceClassifiedAs: ['some-resource-type'],
  resourceQuantity: { hasNumericalValue: 1, hasUnit: mockIdentifier(false) },
  provider: mockAddress(false),
  receiver: mockAddress(false),
  due: '2019-11-19T04:29:55.056Z',
}

test('removing records with linked remote indexes clears them in associated records', async (t) => {
  const alice = await buildPlayer(['observation', 'planning'])
  try {
    const { cells: [observation, planning] } = alice

    // SCENARIO: write initial records
    const process = {
      name: 'context record for testing relationships',
    }
    const pResp = await observation.call('process', 'create_process', { process })
    t.ok(pResp.process && pResp.process.id, 'record created successfully')
    await pause(100)
    const processId = pResp.process.id

    const iIntent = {
      note: 'test input intent',
      inputOf: processId,
      ...testEventProps,
    }
    const iiResp = await planning.call('intent', 'create_intent', { intent: iIntent })
    t.ok(iiResp.intent && iiResp.intent.id, 'input record created successfully')
    t.deepLooseEqual(iiResp.intent.inputOf, processId, 'field reference OK in write')
    await pause(100)
    const iIntentId = iiResp.intent.id
    const iIntentRevisionId = iiResp.intent.revisionId

    // ASSERT: test forward link field
    let readResponse = await planning.call('intent', 'get_intent', { address: iIntentId })
    t.deepLooseEqual(readResponse.intent && readResponse.intent.inputOf, processId, 'field reference OK on read')

    // ASSERT: test reciprocal link field
    readResponse = await observation.call('process', 'get_process', { address: processId })
    t.deepLooseEqual(readResponse.process &&
      readResponse.process.intendedInputs &&
      readResponse.process.intendedInputs[0], iIntentId, 'reciprocal field reference OK on read')

    // ASSERT: test commitment input query edge
    readResponse = await planning.call('intent_index', 'query_intents', { params: { inputOf: processId } })
    t.equal(readResponse.edges && readResponse.edges.length, 1, 'field query index present')
    t.deepLooseEqual(readResponse.edges && readResponse.edges[0] && readResponse.edges[0].node && readResponse.edges[0].node.id, iIntentId, 'query index OK')

    // ASSERT: test process input query edge
    readResponse = await observation.call('process_index', 'query_processes', { params: { intendedInputs: iIntentId } })
    t.equal(readResponse.edges && readResponse.edges.length, 1, 'reciprocal query index present')
    t.deepLooseEqual(readResponse.edges && readResponse.edges[0] && readResponse.edges[0].node && readResponse.edges[0].node.id, processId, 'reciprocal query index OK')

    // SCENARIO: wipe associated record
    await planning.call('intent', 'delete_intent', { revisionId: iIntentRevisionId })
    await pause(100)

    // ASSERT: test forward link field
    try {
      readResponse = await planning.call('intent', 'get_intent', { address: iIntentId })
    } catch (err) {
      t.ok(err.data.data.includes('No entry at this address'), 'record deletion OK')
    }

    // ASSERT: test reciprocal link field
    readResponse = await observation.call('process', 'get_process', { address: processId })
    t.equal(readResponse.process.intendedInputs, undefined, 'reciprocal field reference removed')

    // ASSERT: test commitment input query edge
    readResponse = await planning.call('intent_index', 'query_intents', { params: { inputOf: processId } })
    t.equal(readResponse.edges && readResponse.edges.length, 0, 'field query index removed')

    // ASSERT: test process input query edge
    readResponse = await observation.call('process_index', 'query_processes', { params: { intendedInputs: iIntentId } })
    t.equal(readResponse.edges && readResponse.edges.length, 0, 'reciprocal query index removed')
  } catch (e) {
    await alice.scenario.cleanUp()
    throw e
  }
  await alice.scenario.cleanUp()
})
