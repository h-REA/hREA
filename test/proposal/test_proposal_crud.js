const {
  getDNA,
  buildConfig,
  buildRunner,
  buildPlayer,
} = require('../init')

const runner = buildRunner()

const config = buildConfig({
  proposal: getDNA('proposal'),
}, {})

const exampleEntry = {
  name: "String",
  hasBeginning: '2019-11-19T00:00:00.056Z',
  hasEnd: '2019-11-19T00:00:00.056Z',
  inScopeOf: null,
  unitBased: true,
  created: '2019-11-19T00:00:00.056Z',
  note: 'note'
}
const updatedExampleEntry = {
  name: "String2",
  hasBeginning: '2020-11-19T00:00:00.056Z',
  hasEnd: '2020-11-19T00:00:00.056Z',
  inScopeOf: null,
  unitBased: false,
  note: 'note2'
}

const dateAdapter = obj => ({
  ...obj,
  hasBeginning: (new Date(obj.hasBeginning)).toDateString(),
  hasEnd: (new Date(obj.hasEnd)).toDateString(),
  created: (new Date(obj.created)).toDateString(),
})

const grepMe = obj => console.log( 'grepme: ', JSON.stringify( obj ) )

runner.registerScenario('Proposal record API', async (s, t) => {
  const alice = await buildPlayer(s, 'alice', config)

  let createResp = await alice.graphQL(`
    mutation($rs: ProposalCreateParams!) {
      res: createProposal(proposal: $rs) {
        proposal {
          id
        }
      }
    }
  `, {
    rs: exampleEntry,
  })
  await s.consistency()
  t.ok(createResp.data.res.proposal.id, 'record created')
  const psId = createResp.data.res.proposal.id

  let getResp = await alice.graphQL(`
    query($id: ID!) {
      res: proposal(id: $id) {
        id
        name
        hasBeginning
        hasEnd
        inScopeOf
        unitBased
        created
        note
      }
    }
  `, {
    id: psId,
  })
  t.deepEqual(dateAdapter(getResp.data.res), dateAdapter({ 'id': psId, ...exampleEntry }), 'record read OK')
  const updateResp = await alice.graphQL(`
    mutation($rs: ProposalUpdateParams!) {
      res: updateProposal(proposal: $rs) {
        proposal {
          id
        }
      }
    }
  `, {
    rs: { id: psId, ...updatedExampleEntry },
  })
  await s.consistency()
  t.equal(updateResp.data.res.proposal.id, psId, 'record updated')

  // now we fetch the Entry again to check that the update was successful
  const updatedGetResp = await alice.graphQL(`
    query($id: ID!) {
      res: proposal(id: $id) {
        id
        created
        name
        hasBeginning
        hasEnd
        inScopeOf
        unitBased
        note
      }
    }
  `, {
    id: psId,
  })
  t.deepEqual(updatedGetResp.data.res, { id: psId, created: exampleEntry.created, ...updatedExampleEntry }, 'record updated OK')

  const deleteResult = await alice.graphQL(`
    mutation($id: String!) {
      res: deleteProposal(id: $id)
    }
  `, {
    id: psId,
  })
  await s.consistency()

  t.equal(deleteResult.data.res, true)

  const queryForDeleted = await alice.graphQL(`
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
})

runner.run()
