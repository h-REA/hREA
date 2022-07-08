import test from 'tape'
import { pause } from '@holochain/tryorama'
import {
  buildPlayer,
} from '../init.js'

const exampleEntry = {
  name: 'TPE',
  note: 'test process specification',
}
const exampleEntry2 = {
  name: 'T2',
  note: 'test 2',
}
const updatedExampleEntry = {
  name: 'UPE',
  note: 'updated process specification',
}

test('ProcessSpecification record API', async (t) => {
  const alice = await buildPlayer(['specification'])
  try {
    let createResp = await alice.graphQL(`
      mutation($rs: ProcessSpecificationCreateParams!, $rs2: ProcessSpecificationCreateParams!) {
        res: createProcessSpecification(processSpecification: $rs) {
          processSpecification {
            id
            revisionId
          }
        }
        res2: createProcessSpecification(processSpecification: $rs2) {
          processSpecification {
            id
            revisionId
          }
        }
      }
    `, {
      rs: exampleEntry,
      rs2: exampleEntry2,
    })
    await pause(100)

    t.ok(createResp.data.res.processSpecification.id, 'record created')
    const psId = createResp.data.res.processSpecification.id
    const ps2Id = createResp.data.res2.processSpecification.id
    const psRev = createResp.data.res.processSpecification.revisionId

    let getResp = await alice.graphQL(`
      query($id: ID!) {
        res: processSpecification(id: $id) {
          id
          name
          note
        }
      }
    `, {
      id: psId,
    })

    t.deepLooseEqual(getResp.data.res, { 'id': psId, ...exampleEntry }, 'record read OK')

    const queryAllProcessSpecifications = await alice.graphQL(`
      query {
        res: processSpecifications {
          edges {
            node {
              id
            }
          }
        }
      }
    `,
    )

    t.equal(queryAllProcessSpecifications.data.res.edges.length, 2, 'query for all process specifications OK')
    t.deepEqual(queryAllProcessSpecifications.data.res.edges[1].node.id, psId, 'query for all PSs, first PS in order OK')
    t.deepEqual(queryAllProcessSpecifications.data.res.edges[0].node.id, ps2Id, 'query for all PSs, second PS in order OK')
    const updateResp = await alice.graphQL(`
      mutation($rs: ProcessSpecificationUpdateParams!) {
        res: updateProcessSpecification(processSpecification: $rs) {
          processSpecification {
            id
            revisionId
          }
        }
      }
    `, {
      rs: { revisionId: psRev, ...updatedExampleEntry },
    })
    await pause(100)

    t.equal(updateResp.data.res.processSpecification.id, psId, 'record updated')
    const updatedPsRevId = updateResp.data.res.processSpecification.revisionId

    // now we fetch the Entry again to check that the update was successful
    const updatedGetResp = await alice.graphQL(`
      query($id: ID!) {
        res: processSpecification(id: $id) {
          id
          revisionId
          name
          note
        }
      }
    `, {
      id: psId,
    })
    t.deepLooseEqual(updatedGetResp.data.res, { id: psId, revisionId: updatedPsRevId, ...updatedExampleEntry }, 'record updated OK')

    const deleteResult = await alice.graphQL(`
      mutation($revisionId: ID!) {
        res: deleteProcessSpecification(revisionId: $revisionId)
      }
    `, {
      revisionId: updatedPsRevId,
    })
    await pause(100)

    t.equal(deleteResult.data.res, true)

    const queryForDeleted = await alice.graphQL(`
      query($id: ID!) {
        res: processSpecification(id: $id) {
          id
        }
      }
    `, {
      id: psId,
    })

    t.equal(queryForDeleted.errors.length, 1, 'querying deleted record is an error')
    t.notEqual(-1, queryForDeleted.errors[0].message.indexOf('No entry at this address'), 'correct error reported')
  } catch (e) {
    await alice.scenario.cleanUp()
    throw e
  }
  await alice.scenario.cleanUp()
})
