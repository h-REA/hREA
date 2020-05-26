/**
 * Agent queries
 *
 * :TODO: wire into Personas hApp and replace generated agent names with serving of profile data
 *
 * @package: Holo-REA
 * @since:   2020-02-19
 */

import { zomeFunction } from '../connection'
import { injectTypename } from '../types'

import {
  Agent
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const readMyAgent = zomeFunction('agent', 'agent_registration', 'get_my_agent')
const readAllAgents = zomeFunction('agent', 'agent_registration', 'get_registered_agents')
const agentExists = zomeFunction('agent', 'agent_registration', 'is_registered_agent')

// :TODO: is myAgent always a 'Person' in Holochain, or will we allow users to act in an Organization context directly?
export const myAgent = injectTypename('Person', async (root, args): Promise<Agent> => {
  const agentData: Agent = (await readMyAgent({})).agent
  // :TODO: wire to Personas hApp
  agentData['name'] = `Agent ${agentData.id.substr(0, 4)}`
  return agentData
})

export const allAgents = async (root, args): Promise<Agent[]> => {
  return (await readAllAgents({})).map(agentAddress => ({
    // :TODO: wire to Personas hApp
    id: agentAddress,
    name: `Agent ${agentAddress.substr(2, 4)}`,
  }))
}

export const agent = async (root, { id }): Promise<Agent> => {
  const isAgent = await agentExists({ address: id })

  if (!isAgent) {
    throw new Error('No agent exists with that ID')
  }
  return {
    id,
    name: `Agent ${id}`,
  }
}
