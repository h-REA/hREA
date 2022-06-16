/**
 * Agent queries
 *
 * :TODO: wire into Personas hApp and replace generated agent names with serving of profile data
 *
 * @package: Holo-REA
 * @since:   2020-02-19
 */

import { DNAIdMappings, injectTypename } from '../types.js'
import { mapZomeFn, serializeHash, deserializeHash } from '../connection.js'

import {
  Agent
} from '@valueflows/vf-graphql'
import { AgentPubKey } from '@holochain/client'

export interface RegistrationQueryParams {
  pubKey: AgentPubKey,
}

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {

  const readMyAgent = mapZomeFn<null, AgentPubKey>(dnaConfig, conductorUri, 'agent', 'agent_registration', 'get_my_agent_pubkey')
  const readAllAgents = mapZomeFn<null, AgentPubKey[]>(dnaConfig, conductorUri, 'agent', 'agent_registration', 'get_registered')
  // special 'true' at the end is for skipEncodeDecode, because of the way this zome handles serialization and inputs
  // which is different from others
  const agentExists = mapZomeFn<RegistrationQueryParams, boolean>(dnaConfig, conductorUri, 'agent', 'agent_registration', 'is_registered', true)

  // read mapped DNA hash in order to construct VF-native IDs from DNA-local HC IDs
  const mappedDNA = dnaConfig['agent'] ? serializeHash(dnaConfig['agent'][0]) : null

  return {
    // :TODO: is myAgent always a 'Person' in Holochain, or will we allow users to act in an Organization context directly?
    myAgent: injectTypename('Person', async (root, args): Promise<Agent> => {
      const agentPubKey = serializeHash(await readMyAgent(null))
      // :TODO: wire to Personas hApp
      return {
        id: `${agentPubKey}:${mappedDNA}`,
        revisionId: '',
        name: `Agent ${agentPubKey.substr(2, 4)}`,
        meta: {
          retrievedRevision: {
            id: '',
          }
        }
      }
    }),

    // :TODO: this and the associated functionality in 'get_registered' needs to be revisited
    // or potentially integrated from other projects affording similar functionality.
    agents: async (root, args): Promise<Agent[]> => {
      return (await readAllAgents(null)).map(agentAddress => ({
        // :TODO: wire to Personas hApp
        id: `${serializeHash(agentAddress)}:${mappedDNA}`,
        revisionId: '',
        name: `Agent ${serializeHash(agentAddress).substr(2, 4)}`,
        __typename: 'Person',  // :SHONK:
        meta: {
          retrievedRevision: {
            id: ''
          }
        }
      }))
    },

    agent: injectTypename('Person', async (root, { id }): Promise<Agent> => {
      const rawAgentPubKey = deserializeHash(id.split(':')[0])
      const isAgent = await agentExists({ pubKey: rawAgentPubKey })

      if (!isAgent) {
        throw new Error('No agent exists with that ID')
      }
      return {
        id,
        revisionId: '',
        name: `Agent ${id.substr(2, 4)}`,
        meta: {
          retrievedRevision: {
            id: '',
          }
        }
      }
    }),
  }
}
