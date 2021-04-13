/**
 * Agent queries
 *
 * :TODO: wire into Personas hApp and replace generated agent names with serving of profile data
 *
 * @package: Holo-REA
 * @since:   2020-02-19
 */

import { DNAIdMappings, injectTypename } from '../types'
import { mapZomeFn } from '../connection'

import {
  Agent
} from '@valueflows/vf-graphql'

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const readMyAgent = mapZomeFn(dnaConfig, conductorUri, 'agent', 'agent_registration', 'get_my_agent')
  const readAllAgents = mapZomeFn(dnaConfig, conductorUri, 'agent', 'agent_registration', 'get_registered_agents')
  const agentExists = mapZomeFn(dnaConfig, conductorUri, 'agent', 'agent_registration', 'is_registered_agent')

  return {
    // :TODO: is myAgent always a 'Person' in Holochain, or will we allow users to act in an Organization context directly?
    myAgent: injectTypename('Person', async (root, args): Promise<Agent> => {
      const agentData: Agent = (await readMyAgent(null)).agent
      // :TODO: wire to Personas hApp
      agentData['name'] = `Agent ${agentData.id.substr(2, 4)}`
      return agentData
    }),

    agents: async (root, args): Promise<Agent[]> => {
      return (await readAllAgents(null)).map(agentAddress => ({
        // :TODO: wire to Personas hApp
        id: agentAddress,
        name: `Agent ${agentAddress.substr(2, 4)}`,
        __typename: 'Person',  // :SHONK:
      }))
    },

    agent: injectTypename('Person', async (root, { id }): Promise<Agent> => {
      const isAgent = await agentExists({ address: id })

      if (!isAgent) {
        throw new Error('No agent exists with that ID')
      }
      return {
        id,
        name: `Agent ${id.substr(2, 4)}`,
      }
    }),
  }
}
