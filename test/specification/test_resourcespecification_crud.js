import test from 'tape'
import { pause } from '@holochain/tryorama'
import {
  buildPlayer,
} from '../init.js'

const exampleEntry = {
  name: 'TRE',
  image: 'https://holochain.org/something',
  note: 'test resource specification',
}
const exampleEntry2 = {
  name: '2',
  image: 'https://holochain.org/something',
  note: 'test resource specification number 2',
}
const updatedExampleEntry = {
  name: 'QUA',
  image: 'https://holochain.org/something-else',
  note: 'test resource specification updated',
}

test('ResourceSpecification record API', async (t) => {
  const alice = await buildPlayer(['specification'])
  try {
    let createResp = await alice.graphQL(`
      mutation($rs: ResourceSpecificationCreateParams!, $rs2: ResourceSpecificationCreateParams!) {
        res: createResourceSpecification(resourceSpecification: $rs) {
          resourceSpecification {
            id
            revisionId
          }
        }
        res2: createResourceSpecification(resourceSpecification: $rs2) {
          resourceSpecification {
            id
            revisionId
          }
        }
      }
      `, {
      rs: exampleEntry,
      rs2: exampleEntry,
    })
    await pause(100)

    t.ok(createResp.data.res.resourceSpecification.id, 'record created')
    const rsId = createResp.data.res.resourceSpecification.id
    const rs2Id = createResp.data.res2.resourceSpecification.id
    const rsRev = createResp.data.res.resourceSpecification.revisionId

    const getResp = await alice.graphQL(`
      query($id: ID!) {
        res: resourceSpecification(id: $id) {
          id
          name
          image
          note
        }
      }
      `, {
      id: rsId,
    })

    t.deepLooseEqual(getResp.data.res, { id: rsId, ...exampleEntry }, 'record read OK')

    const queryAllResourceSpecifications = await alice.graphQL(`
      query {
        res: resourceSpecifications {
          edges {
            node {
              id
            }
          }
        }
      }
    `,
    )

    t.equal(queryAllResourceSpecifications.data.res.edges.length, 2, 'query for all resource specifications OK')
    t.deepEqual(queryAllResourceSpecifications.data.res.edges[1].node.id, rsId, 'query for all RSs, first RS in order OK')
    t.deepEqual(queryAllResourceSpecifications.data.res.edges[0].node.id, rs2Id, 'query for all RSs, second RS in order OK')
    const updateResp = await alice.graphQL(`
      mutation($rs: ResourceSpecificationUpdateParams!) {
        res: updateResourceSpecification(resourceSpecification: $rs) {
          resourceSpecification {
            id
            revisionId
          }
        }
      }
      `, {
      rs: { revisionId: rsRev, ...updatedExampleEntry },
    })
    const updatedRsRevId = updateResp.data.res.resourceSpecification.revisionId
    await pause(100)

    t.equal(updateResp.data.res.resourceSpecification.id, rsId, 'record update OK')

    // now we fetch the Entry again to check that the update was successful
    const updatedGetResp = await alice.graphQL(`
      query($id: ID!) {
        res: resourceSpecification(id: $id) {
          id
          revisionId
          name
          image
          note
        }
      }
    `, {
      id: rsId,
    })

    t.deepLooseEqual(updatedGetResp.data.res, { id: rsId, revisionId: updatedRsRevId, ...updatedExampleEntry }, 'record properties updated')

    const deleteResult = await alice.graphQL(`
      mutation($revisionId: ID!) {
        res: deleteResourceSpecification(revisionId: $revisionId)
      }
    `, {
      revisionId: updatedRsRevId,
    })
    await pause(100)

    t.equal(deleteResult.data.res, true)

    const queryForDeleted = await alice.graphQL(`
      query($id: ID!) {
        res: resourceSpecification(id: $id) {
          id
          name
          image
          note
        }
      }
    `, {
      id: rsId,
    })

    t.equal(queryForDeleted.errors.length, 1, 'querying deleted record is an error')
    t.notEqual(-1, queryForDeleted.errors[0].message.indexOf('No entry at this address'), 'correct error reported')
  } catch (e) {
    await alice.scenario.cleanUp()
    throw e
  }
  await alice.scenario.cleanUp()
})
