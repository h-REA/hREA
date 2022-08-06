import test from 'tape'
import { pause } from '@holochain/tryorama'
import { buildPlayer } from '../init.js'

const examplePerson = {
  agentType: 'Person',
  name: 'test person',
  image: 'https://image.png',
  note: 'test person note',
}

test('Agent whois API', async (t) => {
  const alice = await buildPlayer(['agent'])
  try {
    const { cells: [agent], graphQL } = alice

    // test low-level whois API for direct RPC calls
    const createResp = await agent.call('agent', 'create_agent', { agent: examplePerson })
    await pause(100)
    t.ok(createResp.agent.id, 'agent record created')
    let aId = createResp.agent.id

    const agentPubKey = alice.player.agentPubKey

    let err
    try {
      await agent.call('agent', 'whois', { agentPubKey })
    } catch (e) {
      err = e
    }
    t.notEqual(err.data.data.indexOf('No Agent data is associated'), -1, 'query before agent association is an error')

    let associateResp = await agent.call('agent', 'associate_my_agent', { agentAddress: aId })
    t.ok(associateResp, 'associated agent profile')

    const whoisResult = await agent.call('agent', 'whois', { agentPubKey })
    t.deepEqual(whoisResult.agent.id, aId, 'agent whois query successful after association')

    // test high level record metadata API to verify GraphQL resolvers
    const gqlResult = await graphQL(`
      query {
        myAgent {
          id
          revisionId
          meta {
            retrievedRevision {
              id
              time
              author {
                id
              }
            }
          }
        }
      }
    `)
    t.ok(gqlResult.data.myAgent.id, 'myAgent retrieval OK')
    t.equal(gqlResult.data.myAgent.id, gqlResult.data.myAgent.meta.retrievedRevision.author.id, 'author metadata resolves to authoring agent profile')
    t.equal(gqlResult.data.myAgent.revisionId, gqlResult.data.myAgent.meta.retrievedRevision.id, 'revision IDs match')
    t.ok(gqlResult.data.myAgent.meta.retrievedRevision.time instanceof Date, 'revision time returned as Date object')
  } catch (e) {
    await alice.scenario.cleanUp()
    throw e
  }
  await alice.scenario.cleanUp()
})
