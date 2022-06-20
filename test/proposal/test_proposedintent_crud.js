import test from 'tape'
import { pause } from '@holochain/tryorama'
import {
  buildPlayer,
  mockAddress,
  sortById,
} from '../init.js'

const exampleProposal = {
  name: 'String',
  hasBeginning: new Date('2019-11-19T00:00:00.056Z'),
  hasEnd: new Date('2019-11-19T00:00:00.056Z'),
  unitBased: true,
  created: new Date('2019-11-19T00:00:00.056Z'),
  note: 'note',
}

const exampleIntent = {
  action: 'move',
  provider: mockAddress(),
}

test('ProposedIntent external link', async (t) => {
  const alice = await buildPlayer(['proposal', 'planning', 'agent'])
  try {
    const { graphQL } = alice
    /*
    * the next code is only for getting an intent and a proposal to link to the proposedIntent.
    * the idea is to verify the intent linking by getting Proposal->ProposedIntent->Intent
    */

    // intent creation
    let intentRes = await graphQL(`
      mutation($rs: IntentCreateParams!) {
        res: createIntent(intent: $rs) {
          intent {
            id
          }
        }
      }
    `, {
      rs: exampleIntent,
    })
    await pause(100)
    const intentAdress = intentRes.data.res.intent.id
    t.ok(intentAdress, 'can create intent')

    // proposal creation
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
    await pause(100)
    let proposalAdress = proposalRes.data.res.proposal.id
    t.ok(proposalAdress, 'can create proposal')

    proposalRes = await graphQL(`
    query($id: ID!) {
      res: proposal(id: $id) {
        id
        publishes {
          id
        }
      }
    }
    `, {
      id: proposalAdress,
    })
    t.equal(proposalRes.data.res.id, proposalAdress, 'proposal read OK')
    t.equal(proposalRes.data.res.publishes.length, 0, 'proposedIntent list empty')

    let proposeIntentResp = await graphQL(`
      mutation($pIn: ID!, $ps: ID!, $re: Boolean) {
        res: proposeIntent(publishedIn: $pIn, publishes: $ps, reciprocal: $re) {
          proposedIntent {
            id
            revisionId
          }
        }
      }
    `, {
      pIn: proposalAdress, // Proposal Address
      ps: intentAdress, // Intent Address
      re: true,
    })
    await pause(100)
    t.ok(proposeIntentResp.data.res.proposedIntent.id, 'can propose')
    const proposedIntentAdress = proposeIntentResp.data.res.proposedIntent.id
    const proposedIntentRev = proposeIntentResp.data.res.proposedIntent.revisionId

    let getResp = await graphQL(`
      query($id: ID!) {
        res: proposal(id: $id) {
          id
          publishes {
            id
            publishes {
              id
            }
          }
        }
      }
    `, {
      id: proposalAdress,
    })
    t.equal(getResp.data.res.id, proposalAdress, 'proposal fetch succesful')
    t.equal(getResp.data.res.publishes.length, 1, 'proposedIntent count as expected')
    t.equal(getResp.data.res.publishes[0].id, proposedIntentAdress, 'proposedIntent fetching from proposal succesful')
    t.equal(getResp.data.res.publishes[0].publishes.id, intentAdress, 'intent fetching from proposedIntent succesful')

    // another intent
    intentRes = await graphQL(`
      mutation($rs: IntentCreateParams!) {
        res: createIntent(intent: $rs) {
          intent {
            id
          }
        }
      }
    `, {
      rs: {
        hasPointInTime: new Date('2019-11-19T00:00:00.056Z'),
        ...exampleIntent,
      },
    })
    await pause(100)
    const intentAdress2 = intentRes.data.res.intent.id
    t.ok(intentAdress2, 'can create intent')

    // another proposed intent
    let proposeIntentResp2 = await graphQL(`
      mutation($pIn: ID!, $ps: ID!, $re: Boolean) {
        res: proposeIntent(publishedIn: $pIn, publishes: $ps, reciprocal: $re) {
          proposedIntent {
            id
            revisionId
          }
        }
      }
    `, {
      pIn: proposalAdress, // Proposal Address
      ps: intentAdress2, // second Intent Address
      re: true,
    })
    await pause(100)
    t.ok(proposeIntentResp2.data.res.proposedIntent.id, 'can propose')
    const proposedIntentAdress2 = proposeIntentResp2.data.res.proposedIntent.id
    const proposedIntentRev2 = proposeIntentResp2.data.res.proposedIntent.revisionId

    getResp = await graphQL(`
      query($id: ID!) {
        res: proposal(id: $id) {
          id
          publishes {
            id
            publishes {
              id
            }
          }
        }
      }
    `, {
      id: proposalAdress,
    })
    t.equal(getResp.data.res.id, proposalAdress, 'proposal fetch succesful')
    t.equal(getResp.data.res.publishes.length, 2, 'proposedIntent count as expected')

    // :TODO: remove client-side sorting when deterministic time-ordered indexing is implemented
    const sortedPIIds = [{ id: proposedIntentAdress }, { id: proposedIntentAdress2 }].sort(sortById)
    getResp.data.res.publishes.sort(sortById)

    const sortedIIds = [{ id: intentAdress }, { id: intentAdress2 }].sort(sortById)
    const sortedPublishesIds = [
      { id: getResp.data.res.publishes[0].publishes.id },
      { id: getResp.data.res.publishes[1].publishes.id },
    ].sort(sortById)

    t.equal(getResp.data.res.publishes[0].id, sortedPIIds[0].id, 'proposedIntent B fetching from proposal succesful')
    t.equal(getResp.data.res.publishes[1].id, sortedPIIds[1].id, 'proposedIntent A fetching from proposal succesful')
    t.equal(sortedPublishesIds[0].id, sortedIIds[0].id, 'intent B fetching from proposedIntent succesful')
    t.equal(sortedPublishesIds[1].id, sortedIIds[1].id, 'intent A fetching from proposedIntent succesful')

    await graphQL(`
      mutation($in: ID!) {
        res: deleteProposedIntent(revisionId: $in)
      }
    `, {
      in: proposedIntentRev,
    })
    await pause(100)

    getResp = await graphQL(`
      query($id: ID!) {
        res: proposal(id: $id) {
          id
          publishes {
            id
            publishes {
              id
            }
          }
        }
      }
    `, {
      id: proposalAdress,
    })
    t.equal(getResp.data.res.id, proposalAdress, 'proposal fetch after delete succesful')
    t.equal(getResp.data.res.publishes.length, 1, 'proposedIntent count as expected after delete')
    t.equal(getResp.data.res.publishes[0].id, proposedIntentAdress2, 'proposedIntent fetching from proposal after delete succesful')
    t.equal(getResp.data.res.publishes[0].publishes.id, intentAdress2, 'intent fetching from proposedIntent after delete succesful')

    await graphQL(`
      mutation($in: ID!) {
        res: deleteProposedIntent(revisionId: $in)
      }
    `, {
      in: proposedIntentRev2,
    })
    await pause(100)

    getResp = await graphQL(`
    query($id: ID!) {
      res: proposal(id: $id) {
        id
        publishes {
          id
          publishes{
            id
          }
        }
      }
    }
    `, {
      id: proposalAdress,
    })
    t.equal(getResp.data.res.id, proposalAdress, 'proposal fetch after deleting all relationships succesful')
    t.equal(getResp.data.res.publishes.length, 0, 'proposedIntent array emptied as appropriate')
  } catch (e) {
    await alice.scenario.cleanUp()
    throw e
  }
  await alice.scenario.cleanUp()
})
