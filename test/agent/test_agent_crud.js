import test from 'tape'
import { pause } from '@holochain/tryorama'
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
  // display the filename for context in the terminal and use .warn
  // to override the tap testing log filters
  console.warn(`\n\n${import.meta.url}`)
  const alice = await buildPlayer(['agent'])
  try {
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
    let pId = createResp.data.res.agent.id
    let r1Id = createResp.data.res.agent.revisionId

    let myAgentResp = await alice.graphQL(
      `
      query {
        res: myAgent {
          id
          revisionId
        }
      }
    `,
    )
    await pause(100)
    t.equal(
      myAgentResp.errors.length,
      1,
      'getting my agent before agent is created is an error',
    )

    let associateMyAgent = await alice.graphQL(
      `
      mutation($rs: ID!) {
        res: associateMyAgent(agentId: $rs)
      }
    `,
      {
        rs: pId,
      },
    )
    await pause(100)
    t.ok(associateMyAgent.data.res, 'able to associate agent with agentpubkey')

    myAgentResp = await alice.graphQL(
      `
      query {
        res: myAgent {
          id
          revisionId
        }
      }
    `,
    )
    await pause(100)

    t.deepLooseEqual(
      { ...myAgentResp.data.res }, { id: pId, revisionId: r1Id },
      'read my Agent OK',
    )

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
    let personId = pId
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
    let r2PersonId = r2Id
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

    // test query read all

    let readAllResult = await alice.graphQL(
      `
      query {
        agents: agents {
          edges {
            node {
              id
            }
          }
        }
        people: people {
          edges {
            node {
              id
            }
          }
        }
        organizations: organizations {
          edges {
            node {
              id
            }
          }
        }
      }
      `,
    )

    await pause(100)

    t.equal(readAllResult.data.agents.edges.length, 2, 'query for all agents OK')
    t.deepEqual(readAllResult.data.agents.edges[1].node.id, personId, 'query for all agents right person in order OK')
    t.deepEqual(readAllResult.data.agents.edges[0].node.id, pId, 'query for all agents right organization in order OK')
    t.equal(readAllResult.data.people.edges.length, 1, 'query for all people OK')
    t.deepEqual(readAllResult.data.people.edges[0].node.id, personId, 'query for all people right person OK')
    t.equal(readAllResult.data.organizations.edges.length, 1, 'query for all organizations OK')
    t.deepEqual(readAllResult.data.organizations.edges[0].node.id, pId, 'query for all organizations right organization OK')

    let deleteResult = await alice.graphQL(
      `
      mutation($id: ID!) {
        res: deletePerson(revisionId: $id)
      }
    `,
      {
        id: r2PersonId,
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
        id: personId,
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
  } catch (e) {
    await alice.scenario.cleanUp()
    throw e
  }
  await alice.scenario.cleanUp()
})
