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

test('updating remote link fields syncs fields and associated indexes', async (t) => {
  // display the filename for context in the terminal and use .warn
  // to override the tap testing log filters
  console.warn(`\n\n${import.meta.url}`)
  const alice = await buildPlayer(['observation', 'planning'])
  try {
    const { cells: [observation, planning] } = alice

    // SCENARIO: write initial records
    const process = {
      name: 'context record for testing relationships',
    }
    const pResp = await observation.call('process', 'create_process', { process })
    t.ok(pResp.process && pResp.process.id, 'target record created successfully')
    await pause(100)
    const processId = pResp.process.id

    const process2 = {
      name: 'second context record for testing relationships',
    }
    const pResp2 = await observation.call('process', 'create_process', { process: process2 })
    t.ok(pResp2.process && pResp2.process.id, 'secondary record created successfully')
    await pause(100)
    const differentProcessId = pResp2.process.id

    const iCommitment = {
      note: 'test input commitment',
      inputOf: processId,
      ...testEventProps,
    }
    const icResp = await planning.call('commitment', 'create_commitment', { commitment: iCommitment })
    t.ok(icResp.commitment && icResp.commitment.id, 'input record created successfully')
    t.deepLooseEqual(icResp.commitment.inputOf, processId, 'field reference OK in write')
    await pause(100)
    const iCommitmentId = icResp.commitment.id
    const iCommitmentRevisionId = icResp.commitment.revisionId

    // ASSERT: test forward link field
    let readResponse = await planning.call('commitment', 'get_commitment', { address: iCommitmentId })
    t.deepLooseEqual(readResponse.commitment && readResponse.commitment.inputOf, processId, 'field reference OK on read')

    // ASSERT: test reciprocal link field
    readResponse = await observation.call('process', 'get_process', { address: processId })
    t.deepLooseEqual(readResponse.process &&
      readResponse.process.committedInputs &&
      readResponse.process.committedInputs[0], iCommitmentId, 'reciprocal field reference OK on read')

    // ASSERT: test commitment input query edge
    readResponse = await planning.call('commitment_index', 'query_commitments', { params: { inputOf: processId } })
    t.equal(readResponse.edges && readResponse.edges.length, 1, 'field query index present')
    t.deepLooseEqual(readResponse.edges && readResponse.edges[0] && readResponse.edges[0].node && readResponse.edges[0].node.id, iCommitmentId, 'query index OK')

    // ASSERT: test process input query edge
    readResponse = await observation.call('process_index', 'query_processes', { params: { committedInputs: iCommitmentId } })
    t.equal(readResponse.edges && readResponse.edges.length, 1, 'reciprocal query index present')
    t.deepLooseEqual(readResponse.edges && readResponse.edges[0] && readResponse.edges[0].node && readResponse.edges[0].node.id, processId, 'reciprocal query index OK')

    // SCENARIO: update link field
    const updateCommitment = {
      id: iCommitmentId,
      revisionId: iCommitmentRevisionId,
      inputOf: differentProcessId,
    }
    const ieResp2 = await planning.call('commitment', 'update_commitment', { commitment: updateCommitment })
    t.deepLooseEqual(ieResp2.commitment && ieResp2.commitment.inputOf, differentProcessId, 'record link field updated successfully')
    await pause(100)

    // ASSERT: test commitment fields
    readResponse = await planning.call('commitment', 'get_commitment', { address: iCommitmentId })
    t.ok(readResponse.commitment && readResponse.commitment.inputOf, 'field reference OK on read')
    t.deepLooseEqual(readResponse.commitment && readResponse.commitment.inputOf, differentProcessId, 'field updated successfully')

    // ASSERT: test new commitment input query edge
    readResponse = await planning.call('commitment_index', 'query_commitments', { params: { inputOf: differentProcessId } })
    t.deepLooseEqual(readResponse.edges && readResponse.edges[0] &&
      readResponse.edges[0].node &&
      readResponse.edges[0].node.id, iCommitmentId, 'new field query index applied')

    // ASSERT: test stale commitment input query edge
    readResponse = await planning.call('commitment_index', 'query_commitments', { params: { inputOf: processId } })
    t.equal(readResponse.edges && readResponse.edges.length, 0, 'stale field query index removed')

    // ASSERT: test process input query edge
    readResponse = await observation.call('process_index', 'query_processes', { params: { committedInputs: iCommitmentId } })
    t.equal(readResponse.edges && readResponse.edges.length, 1, 'reciprocal query index count ok')
    t.deepLooseEqual(readResponse.edges && readResponse.edges[0] &&
      readResponse.edges[0].node &&
      readResponse.edges[0].node.id, differentProcessId, 'new reciprocal query index applied')

    // SCENARIO: update link field (no-op)
    const ieResp3 = await planning.call('commitment', 'update_commitment', { commitment: updateCommitment })
    t.deepLooseEqual(ieResp3.commitment && ieResp3.commitment.inputOf, differentProcessId, 'update with same fields is no-op')
    await pause(100)
    const ieResp3RevisionId = ieResp3.commitment.revisionId

    // ASSERT: test event fields
    readResponse = await planning.call('commitment', 'get_commitment', { address: iCommitmentId })
    t.deepLooseEqual(readResponse.commitment && readResponse.commitment.inputOf, differentProcessId, 'field update no-op OK')

    // SCENARIO: remove link field
    const wipeEventInput = {
      id: iCommitmentId,
      revisionId: ieResp3RevisionId,
      action: 'lower',
      inputOf: null,
    }
    let ieResp4
    try {
      ieResp4 = await planning.call('commitment', 'update_commitment', { commitment: wipeEventInput })
    } catch (e) {
      // to create a failure, pretend that we still have what was there
      ieResp4 = { commitment: { inputOf: differentProcessId } }
      console.error(e)
    }
    t.equal(ieResp4.commitment && ieResp4.commitment.inputOf, undefined, 'update with null value erases field')
    await pause(100)

    // ASSERT: test event fields
    readResponse = await planning.call('commitment', 'get_commitment', { address: iCommitmentId })
    t.equal(readResponse.commitment && readResponse.commitment.inputOf, undefined, 'field erased successfully')

    // ASSERT: test event input query edge
    readResponse = await planning.call('commitment_index', 'query_commitments', { params: { inputOf: differentProcessId } })
    t.equal(readResponse.edges && readResponse.edges.length, 0, 'field query index updated')

    // ASSERT: test process input query edge
    readResponse = await observation.call('process_index', 'query_processes', { params: { committedInputs: iCommitmentId } })
    t.equal(readResponse.edges && readResponse.edges.length, 0, 'reciprocal field query index updated')

    // :TODO: attempt linking to nonexistent target (should this error, or happen regardless? Big question in distributed networks...)
    // :TODO: updates for fields when other values are present in the index array
  } catch (e) {
    await alice.scenario.cleanUp()
    throw e
  }
  await alice.scenario.cleanUp()
})
