const {
  getDNA,
  buildConfig,
  buildRunner,
  buildPlayer,
} = require('../init')

const runner = buildRunner()

const config = buildConfig({
  proposal: getDNA('proposal'),
  agent: getDNA('agent'),
}, {})

const exampleProposal = {
  name: 'String',
  hasBeginning: '2019-11-19T00:00:00.056Z',
  hasEnd: '2019-11-19T00:00:00.056Z',
  unitBased: true,
  created: '2019-11-19T00:00:00.056Z',
  note: 'note',
}

runner.registerScenario('ProposedTo record API', async (s, t) => {
  const alice = await buildPlayer(s, 'alice', config)

  const agentAddress = (await alice.graphQL(`{
    myAgent {
      id
    }
  }`)).data.myAgent.id

  let proposalRes = await alice.graphQL(`
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

  await s.consistency()

  let createResp = await alice.graphQL(`
    mutation($p: ID!, $pTo: ID!) {
      res: proposeTo(proposed: $p,proposedTo: $pTo) {
        proposedTo {
          id
        }
      }
    }
  `, {
    p: proposalID,
    pTo: agentAddress,
  })
  await s.consistency()
  t.ok(createResp.data.res.proposedTo.id, 'record created')

  const psID = createResp.data.res.proposedTo.id
  let getResp = await alice.graphQL(`
    query($id: ID!) {
      res: proposal(id: $id) {
        id
        publishedTo {
          id
          proposedTo {
            id
          }
        }
      }
    }
  `, {
    id: proposalID,
  })

  t.equal(getResp.data.res.id, proposalID, 'Proposal fetch succesful')
  t.equal(getResp.data.res.publishedTo[0].id, psID, 'proposedTo fetching from proposal succesful')
  t.equal(getResp.data.res.publishedTo[0].proposedTo.id, agentAddress, 'agent fetching from proposedTo succesful')

  const deleteResult = await alice.graphQL(`
    mutation($id: ID!) {
      res: deleteProposedTo(id: $id)
    }
  `, {
    id: psID,
  })
  await s.consistency()

  t.equal(deleteResult.data.res, true)

  const queryForDeleted = await alice.graphQL(`
    query {
      res: proposal(id: "${proposalID}") {
        id
        publishedTo {
          id
          proposedTo {
            id
          }
        }
      }
    }
  `)

  t.equal(queryForDeleted.data.res.publishedTo.length, 0, 'record ref removed upon deletion')
})

runner.run()
