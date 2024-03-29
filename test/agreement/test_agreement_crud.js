import test from 'tape'
import { pause } from '@holochain/tryorama'
import { buildPlayer } from '../init.js'

const exampleEntry = {
  name: 'test agreement',
  created: new Date(),
  note: 'just testing, nothing was rly agreed',
}
const exampleEntry2 = {
  name: 'test agreement 2',
  created: new Date(),
  note: 'another test',
}
const updatedExampleEntry = {
  name: 'updated agreement',
  created: new Date(Date.now() + 3600000),
  note: 'updated the agreement to something else',
}

test('Agreement record API', async (t) => {
  // display the filename for context in the terminal and use .warn
  // to override the tap testing log filters
  console.warn(`\n\n${import.meta.url}`)
  const alice = await buildPlayer(['agreement'])

  try {
    let createResp = await alice.graphQL(
      `
      mutation($rs: AgreementCreateParams!, $rs2: AgreementCreateParams!) {
        res: createAgreement(agreement: $rs) {
          agreement {
            id
            revisionId
          }
        }
        res2: createAgreement(agreement: $rs2) {
          agreement {
            id
            revisionId
          }
        }
      }
    `,
      {
        rs: exampleEntry,
        rs2: exampleEntry2,
      },
    )
    await pause(100)
    t.ok(createResp.data.res.agreement.id, 'record created')
    const aId = createResp.data.res.agreement.id
    const a2Id = createResp.data.res2.agreement.id
    const r1Id = createResp.data.res.agreement.revisionId

    let getResp = await alice.graphQL(
      `
      query($id: ID!) {
        res: agreement(id: $id) {
          id
          revisionId
          name
          created
          note
        }
      }
    `,
      {
        id: aId,
      },
    )
    t.deepLooseEqual(
      { ...getResp.data.res }, { id: aId, revisionId: r1Id, ...exampleEntry },
      'record read OK',
    )

    const queryAllAgreements = await alice.graphQL(`
      query {
        res: agreements {
          edges {
            node {
              id
            }
          }
        }
      }
    `,
    )

    t.equal(queryAllAgreements.data.res.edges.length, 2, 'query for all agreements OK')
    t.deepEqual(queryAllAgreements.data.res.edges[1].node.id, aId, 'query for all agreements, first agreement in order OK')
    t.deepEqual(queryAllAgreements.data.res.edges[0].node.id, a2Id, 'query for all agreements, second agreement in order OK')
    const updateResp = await alice.graphQL(
      `
      mutation($rs: AgreementUpdateParams!) {
        res: updateAgreement(agreement: $rs) {
          agreement {
            id
            revisionId
          }
        }
      }
    `,
      {
        rs: { revisionId: r1Id, ...updatedExampleEntry },
      },
    )
    await pause(100)
    t.equal(updateResp.data.res.agreement.id, aId, 'record updated')
    const r2Id = updateResp.data.res.agreement.revisionId

    // now we fetch the Entry again to check that the update was successful
    const updatedGetResp = await alice.graphQL(
      `
      query($id: ID!) {
        res: agreement(id: $id) {
          id
          revisionId
          created
          name
          note
        }
      }
    `,
      {
        id: aId,
      },
    )
    t.deepLooseEqual(
      updatedGetResp.data.res,
      {
        id: aId,
        revisionId: r2Id,
        created: exampleEntry.created,
        ...updatedExampleEntry,
      },
      'record updated OK',
    )

    const deleteResult = await alice.graphQL(
      `
      mutation($id: ID!) {
        res: deleteAgreement(revisionId: $id)
      }
    `,
      {
        id: r2Id,
      },
    )
    await pause(100)
    t.equal(deleteResult.data.res, true)

    const queryForDeleted = await alice.graphQL(
      `
      query($id: ID!) {
        res: agreement(id: $id) {
          id
        }
      }
    `,
      {
        id: aId,
      },
    )
    t.equal(
      queryForDeleted.errors.length,
      1,
      'querying deleted record is an error',
    )
    t.notEqual(
      -1,
      queryForDeleted.errors[0].message.indexOf('No entry at this address'),
      'correct error reported',
    )
  } catch (e) {
    await alice.scenario.cleanUp()
    throw e
  }
  await alice.scenario.cleanUp()
})
