import test from 'tape'
import { pause } from '@connoropolous/tryorama'
import {
  buildPlayer,
  mockAddress,
} from '../init.js'

const exampleProposal = {
  name: 'String',
  hasBeginning: new Date('2019-11-19T00:00:00.056Z'),
  hasEnd: new Date('2019-11-19T00:00:00.056Z'),
  unitBased: true,
  created: new Date('2019-11-19T00:00:00.056Z'),
  note: 'note',
}

test('ProposedTo record API', async (t) => {
  const alice = await buildPlayer(['proposal', 'agent'])
  const { graphQL } = alice

  const agentAddress = mockAddress()

  let proposalRes = await graphQL(`
    mutation($rs: ProposalCreateParams!) {
      res: createProposal(proposal: $rs) {
        proposal {
          id
        }
      }
    }
  `, {
    rs: exampleProposal,
  })

  let proposalID = proposalRes.data.res.proposal.id

  await pause(100)

  let createResp = await graphQL(`
    mutation($p: ID!, $pTo: ID!) {
      res: proposeTo(proposed: $p,proposedTo: $pTo) {
        proposedTo {
          id
          revisionId
        }
      }
    }
  `, {
    p: proposalID,
    pTo: agentAddress,
  })
  await pause(100)
  t.ok(createResp.data.res.proposedTo.id, 'record created')

  const psID = createResp.data.res.proposedTo.id
  const psRev = createResp.data.res.proposedTo.revisionId

  // Re-instate this below once agent indexing is done
  // query($id: ID!) {
  //   res: proposal(id: $id) {
  //     id
  //     publishedTo {
  //       id
  //       proposedTo {
  //         id
  //       }
  //     }
  //   }
  // }
  let getResp = await graphQL(`
    query($id: ID!) {
      res: proposal(id: $id) {
        id
        publishedTo {
          id
        }
      }
    }
  `, {
    id: proposalID,
  })

  t.equal(getResp.data.res.id, proposalID, 'Proposal fetch succesful')
  console.log('getResp', getResp)
  t.equal(getResp.data.res.publishedTo[0].id, psID, 'proposedTo fetching from proposal succesful')
  // re-instate this below once agent indexing is done
  // t.equal(getResp.data.res.publishedTo[0].proposedTo.id, agentAddress, 'agent fetching from proposedTo succesful')

  const deleteResult = await graphQL(`
    mutation($id: ID!) {
      res: deleteProposedTo(revisionId: $id)
    }
  `, {
    id: psRev,
  })
  await pause(100)

  t.equal(deleteResult.data.res, true)

  const queryForDeleted = await graphQL(`
    query {
      res: proposal(id: "${proposalID}") {
        id
        publishedTo {
          id
        }
      }
    }
  `)

  t.equal(queryForDeleted.data.res.publishedTo.length, 0, 'record ref removed upon deletion')

  await alice.scenario.cleanUp()
})
