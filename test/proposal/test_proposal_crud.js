import test from 'tape'
import { pause } from '@holochain/tryorama'
import {
  buildPlayer,
} from '../init.js'

const exampleEntry = {
  name: 'String',
  hasBeginning: new Date('2019-11-19T00:00:00.056Z'),
  hasEnd: new Date('2019-11-19T00:00:00.056Z'),
  unitBased: true,
  created: new Date('2019-11-19T00:00:00.056Z'),
  note: 'note',
}
const exampleEntry2 = {
  name: 'entry 2',
  hasBeginning: new Date('2019-11-19T00:00:00.056Z'),
  hasEnd: new Date('2019-11-19T00:00:00.056Z'),
  unitBased: true,
  created: new Date('2019-11-19T00:00:00.056Z'),
  note: 'note',
}
const updatedExampleEntry = {
  name: 'String2',
  hasBeginning: new Date('2020-11-19T00:00:00.056Z'),
  hasEnd: new Date('2020-11-19T00:00:00.056Z'),
  unitBased: false,
  note: 'note2',
}

test('Proposal record API', async (t) => {
  // display the filename for context in the terminal and use .warn
  // to override the tap testing log filters
  console.warn(`\n\n${import.meta.url}`)
  const alice = await buildPlayer(['proposal'])
  try {
    const { graphQL } = alice

    let createResp = await graphQL(`
      mutation($rs: ProposalCreateParams!) {
        res: createProposal(proposal: $rs) {
          proposal {
            id
            revisionId
          }
        }
      }
    `, {
      rs: exampleEntry,
    })
    await pause(100)
    t.ok(createResp.data.res.proposal.id, 'record created')
    const psId = createResp.data.res.proposal.id
    const psRev = createResp.data.res.proposal.revisionId

    let getResp = await graphQL(`
      query($id: ID!) {
        res: proposal(id: $id) {
          id
          revisionId
          name
          hasBeginning
          hasEnd
          unitBased
          created
          note
        }
      }
    `, {
      id: psId,
    })
    t.deepLooseEqual(getResp.data.res, { 'id': psId, 'revisionId': psRev, ...exampleEntry }, 'record read OK')
    const updateResp = await graphQL(`
      mutation($rs: ProposalUpdateParams!) {
        res: updateProposal(proposal: $rs) {
          proposal {
            id
            revisionId
          }
        }
      }
    `, {
      rs: { revisionId: psRev, ...updatedExampleEntry },
    })
    await pause(100)
    t.equal(updateResp.data.res.proposal.id, psId, 'record ID consistent')
    t.notEqual(updateResp.data.res.proposal.revisionId, psRev, 'record updated')
    const psRev2 = updateResp.data.res.proposal.revisionId

    // now we fetch the Entry again to check that the update was successful
    const updatedGetResp = await graphQL(`
      query($id: ID!) {
        res: proposal(id: $id) {
          id
          revisionId
          created
          name
          hasBeginning
          hasEnd
          unitBased
          note
        }
      }
    `, {
      id: psId,
    })
    t.deepLooseEqual(updatedGetResp.data.res, { id: psId, revisionId: psRev2, created: exampleEntry.created, ...updatedExampleEntry }, 'record updated OK')

    // query all test

    createResp = await graphQL(`
      mutation($rs: ProposalCreateParams!) {
        res: createProposal(proposal: $rs) {
          proposal {
            id
            revisionId
          }
        }
      }
    `, {
      rs: exampleEntry2,
    })
    await pause(100)
    t.ok(createResp.data.res.proposal.id, 'second record created')
    const ps2Id = createResp.data.res.proposal.id
    const ps2Rev = createResp.data.res.proposal.revisionId

    const queryAllProposals = await graphQL(`
      query {
        res: proposals {
          edges {
            node {
              id
            }
          }
        }
      }
    `,
    )

    t.equal(queryAllProposals.data.res.edges.length, 2, 'query for all proposals OK')
    t.deepEqual(queryAllProposals.data.res.edges[1].node.id, psId, 'query for all proposals, first proposal in order OK')
    t.deepEqual(queryAllProposals.data.res.edges[0].node.id, ps2Id, 'query for all proposals, second proposal in order OK')

    const deleteResult = await graphQL(`
      mutation($revisionId: ID!) {
        res: deleteProposal(revisionId: $revisionId)
      }
    `, {
      revisionId: psRev2,
    })
    await pause(100)

    t.equal(deleteResult.data.res, true)

    const queryForDeleted = await graphQL(`
      query($id: ID!) {
        res: proposal(id: $id) {
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
