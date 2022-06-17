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
const exampleOrganization = {
  name: 'test organization',
  image: 'https://org.png',
  classifiedAs: ['org'],
  note: 'test organization note',
}
const updatedOrganization = {
  name: 'updated organization',
  image: 'https://org2.png',
  classifiedAs: ['org2'],
  note: 'updated the organization to something else',
}

test('Agent record API', async (t) => {
  const alice = await buildPlayer(['agent'])

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
  let pId = createResp.data.res.agent.id
  let r1Id = createResp.data.res.agent.revisionId

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
      res2: person(id: $id) {
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
    'record agent read OK',
  )
  t.deepLooseEqual(
    { ...getResp.data.res2 }, { id: pId, revisionId: r1Id, ...examplePerson },
    'record person read OK',
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
  let r2Id = updateResp.data.res.agent.revisionId

  // now we fetch the Entry again to check that the update was successful
  let updatedGetResp = await alice.graphQL(
    `
    query($id: ID!) {
      res: agent(id: $id) {
        id
        revisionId
        name
        image
        note
      }
      res2: person(id: $id) {
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
    'record agent updated OK',
  )
  t.deepLooseEqual(
    { ...updatedGetResp.data.res2 },
    {
      id: pId,
      revisionId: r2Id,
      ...updatedPerson,
    },
    'record person updated OK',
  )

  await pause(100)
  let deleteResult = await alice.graphQL(
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

  let queryForDeleted = await alice.graphQL(
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

  let createOrgResp = await alice.graphQL(
    `
    mutation($rs: OrganizationCreateParams!) {
      res: createOrganization(organization: $rs) {
        agent {
          id
          revisionId
        }
      }
    }
  `,
    {
      rs: exampleOrganization,
    },
  )
  await pause(100)
  t.ok(createOrgResp.data.res.agent.id, 'record created')
  console.log('created agent: ', createOrgResp.data.res.agent)
  pId = createOrgResp.data.res.agent.id
  r1Id = createOrgResp.data.res.agent.revisionId

  let getOrgResp = await alice.graphQL(
    `
    query($id: ID!) {
      res: agent(id: $id) {
        id
        revisionId
        name
        image
        note
      }
      res2: organization(id: $id) {
        id
        revisionId
        name
        image
        classifiedAs
        note
      }
    }
  `,
    {
      id: pId,
    },
  )
  t.deepLooseEqual(
    { ...getOrgResp.data.res, classifiedAs: exampleOrganization.classifiedAs }, { id: pId, revisionId: r1Id, ...exampleOrganization },
    'record agent read OK',
  )
  t.deepLooseEqual(
    { ...getOrgResp.data.res2 }, { id: pId, revisionId: r1Id, ...exampleOrganization },
    'record organization read OK',
  )

  const updateOrgResp = await alice.graphQL(
    `
    mutation($rs: OrganizationUpdateParams!) {
      res: updateOrganization(organization: $rs) {
        agent {
          id
          revisionId
          name
          image
          classifiedAs
          note
        }
      }
    }
  `,
    {
      rs: { revisionId: r1Id, ...updatedOrganization },
    },
  )
  await pause(100)
  t.equal(updateOrgResp.data.res.agent.id, pId, 'record updated')
  console.log('update agent', updateOrgResp.data.res.agent)
  r2Id = updateOrgResp.data.res.agent.revisionId

  // now we fetch the Entry again to check that the update was successful
  updatedGetResp = await alice.graphQL(
    `
    query($id: ID!) {
      res: agent(id: $id) {
        id
        revisionId
        name
        image
        note
      }
      res2: organization(id: $id) {
        id
        revisionId
        name
        image
        classifiedAs
        note
      }
    }
  `,
    {
      id: pId,
    },
  )
  t.deepLooseEqual(
    { ...updatedGetResp.data.res, classifiedAs: updatedOrganization.classifiedAs },
    {
      id: pId,
      revisionId: r2Id,
      ...updatedOrganization,
    },
    'record agent updated OK',
  )
  console.log('person queried even though organization', updatedGetResp.data.res)
  t.deepLooseEqual(
    { ...updatedGetResp.data.res2 },
    {
      id: pId,
      revisionId: r2Id,
      ...updatedOrganization,
    },
    'record Organization updated OK',
  )

  await pause(100)
  deleteResult = await alice.graphQL(
    `
    mutation($id: ID!) {
      res: deleteOrganization(revisionId: $id)
    }
  `,
    {
      id: r2Id,
    },
  )
  await pause(100)
  t.equal(deleteResult.data.res, true)

  queryForDeleted = await alice.graphQL(
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
