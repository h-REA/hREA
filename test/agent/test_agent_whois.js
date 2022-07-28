import test from 'tape'
import { pause } from '@holochain/tryorama'
import { buildPlayer } from '../init.js'

const examplePerson = {
  agentType: 'person',
  name: 'test person',
  image: 'https://image.png',
  note: 'test person note',
}

test('Agent whois API', async (t) => {
  const alice = await buildPlayer(['agent'])
  try {
    const { cells: [agent] } = alice

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
  } catch (e) {
    await alice.scenario.cleanUp()
    throw e
  }
  await alice.scenario.cleanUp()
})
