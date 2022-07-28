/**
 * Metadata resolvers
 *
 * @package: vf-graphql-holochain
 * @since:   2022-07-28
 */

import { Agent, Revision } from '@valueflows/vf-graphql'
import { VfModule, DNAIdMappings, DEFAULT_VF_MODULES } from '../types.js'
import { mapZomeFn } from '../connection.js'

import agentQueries, { AgentWithTypeResponse } from '../queries/agent.js'

interface ByPubKey {
  agentPubKey: string,
}

interface RawRevision {
  id: string,
  time: Date,
  agentPubKey: string,
}

export default (enabledVFModules: VfModule[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri: string) => {
  const hasAgent = -1 !== enabledVFModules.indexOf(VfModule.Agent)

  const whoisAgent = mapZomeFn<ByPubKey, AgentWithTypeResponse>(dnaConfig, conductorUri, 'agent', 'agent', 'whois')

  return hasAgent ? {
    author: async (record: RawRevision): Promise<Agent> => {
      const agent = (await whoisAgent({ agentPubKey: record.agentPubKey })).agent
      agent['__typename'] = agent.agentType
      return agent as Agent
    }
  } : {}
}
