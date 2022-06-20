import test from 'tape'
import { pause } from '@holochain/tryorama'
import { buildPlayer } from '../init.js'

const exampleEntry = {
  name: 'test agreement',
  created: new Date(),
  note: 'just testing, nothing was rly agreed',
}
const updatedExampleEntry = {
  name: 'updated agreement',
  created: new Date(Date.now() + 3600000),
  note: 'updated the agreement to something else',
}

test('Agreement record API', async (t) => {
  const alice = await buildPlayer(['agreement'])

  try {
    let createResp = await alice.graphQL(
      `
      mutation($rs: AgreementCreateParams!) {
        res: createAgreement(agreement: $rs) {
          agreement {
            id
            revisionId
          }
        }
      }
    `,
      {
        rs: exampleEntry,
      },
    )
    await pause(100)
    t.ok(createResp.data.res.agreement.id, 'record created')
    const aId = createResp.data.res.agreement.id
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
      getResp.data.res, { id: aId, revisionId: r1Id, ...exampleEntry },
      'record read OK',
    )

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
