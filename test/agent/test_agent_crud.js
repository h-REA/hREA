import test from 'tape'
import { pause } from '@connoropolous/tryorama'
import { buildPlayer } from '../init.js'

const examplePerson = {
  name: 'test person',
  image: 'https://image.png',
  note: 'test person note',
}
const updatedPerson = {
  name: 'updated person',
  image: 'https://image2.png',
  note: 'updated the person to something else',
}

test('Agent record API', async (t) => {
  const alice = await buildPlayer(['agent'])
  const { cells: [observation] } = alice

  // const pResp = await observation.call('agent', 'create_agent', { agent: examplePerson })
  // console.log('direct: ', pResp)

  let createResp = await alice.graphQL(
    `
    mutation($rs: AgentCreateParams!) {
      res: createPerson(person: $rs) {
        agent {
          id
          revisionId
        }
      }
    }
  `,
    {
      rs: examplePerson,
    },
  )
  await pause(100)
  t.ok(createResp.data.res.agent.id, 'record created')
  console.log('created agent: ', createResp.data.res.agent)
  const pId = createResp.data.res.agent.id
  const r1Id = createResp.data.res.agent.revisionId

  let getResp = await alice.graphQL(
    `
    query($id: ID!) {
      res: agent(id: $id) {
        id
        revisionId
        name
        image
        note
      }
    }
  `,
    {
      id: pId,
    },
  )
  t.deepLooseEqual(
    { ...getResp.data.res }, { id: pId, revisionId: r1Id, ...examplePerson },
    'record read OK',
  )

  const updateResp = await alice.graphQL(
    `
    mutation($rs: AgentUpdateParams!) {
      res: updatePerson(person: $rs) {
        agent {
          id
          revisionId
          name
          image
          note
        }
      }
    }
  `,
    {
      rs: { revisionId: r1Id, ...updatedPerson },
    },
  )
  await pause(100)
  t.equal(updateResp.data.res.agent.id, pId, 'record updated')
  console.log('update agent', updateResp.data.res.agent)
  const r2Id = updateResp.data.res.agent.revisionId

  // now we fetch the Entry again to check that the update was successful
  const updatedGetResp = await alice.graphQL(
    `
    query($id: ID!) {
      res: agent(id: $id) {
        id
        revisionId
        name
        image
        note
      }
    }
  `,
    {
      id: pId,
    },
  )
  t.deepLooseEqual(
    { ...updatedGetResp.data.res },
    {
      id: pId,
      revisionId: r2Id,
      ...updatedPerson,
    },
    'record updated OK',
  )

  const deleteResult = await alice.graphQL(
    `
    mutation($id: ID!) {
      res: deletePerson(revisionId: $id)
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
      res: agent(id: $id) {
        id
      }
    }
  `,
    {
      id: pId,
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

  await alice.scenario.cleanUp()
})
