/**
 * Agent queries
 *
 * :TODO: wire into Personas hApp and replace generated agent names with serving of profile data
 *
 * @package: Holo-REA
 * @since:   2020-02-19
 */

import { DNAIdMappings, injectTypename, ReadParams } from '../types.js'
import { mapZomeFn, serializeHash, deserializeHash } from '../connection.js'

import {
  AccountingScope,
  Agent,
  AgentConnection,
  AgentEdge,
  Organization,
  OrganizationConnection,
  Person,
  PersonConnection
} from '@valueflows/vf-graphql'
import { AgentPubKey } from '@holochain/client'
import { AgentResponse } from '../mutations/agent'
import { PagingParams } from '../resolvers/zomeSearchInputTypes.js'

export interface RegistrationQueryParams {
  pubKey: AgentPubKey,
}
export type AgentWithType = Agent & { agentType: string }
export interface AgentEdgeWithTypeEdge extends Omit<AgentEdge, 'node'> {
  node: AgentWithType
}
export interface AgentConnectionWithType extends Omit<AgentConnection, 'edges'> {
  edges: AgentEdgeWithTypeEdge[]
}

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {

  //assumes there is a link from agentPubKey to a Person entry, but what if link cannot be resolved?
  const readMyAgent = mapZomeFn<null, AgentResponse>(dnaConfig, conductorUri, 'agent', 'agent', 'get_my_agent')
  const readAgent = mapZomeFn<ReadParams, AgentResponse>(dnaConfig, conductorUri, 'agent', 'agent', 'get_agent')
  const readAll = mapZomeFn<PagingParams, AgentConnectionWithType>(dnaConfig, conductorUri, 'agent', 'agent_index', 'read_all_agents')

  return {
    // :TODO: is myAgent always a 'Person' in Holochain, or will we allow users to act in an Organization context directly?
    myAgent: injectTypename('Person', async (root, args): Promise<Agent> => {
      return (await readMyAgent(null)).agent
    }),

    // TODO: not totally sure on the significance of the `__typename` field injection, but once the `type` field is added to Agent, we could conditionally inject `Person` or `Organization`
    agent: injectTypename('Person', async (root, args): Promise<Agent> => {
      return (await readAgent({ address: args.id })).agent
    }),
    organization: injectTypename('Organization', async (root, args): Promise<Organization> => {
      return ((await readAgent({ address: args.id })).agent) as Organization
      // TODO: type check if person or organization and provide error if person
    }),
    person: injectTypename('Person', async (root, args): Promise<Person> => {
      return ((await readAgent({ address: args.id })).agent) as Person
      // TODO: type check if person or organization and provide error if organization
    }),
    agents: async (root, args: PagingParams): Promise<AgentConnection> => {
      let agents = (await readAll(args)) as AgentConnection
      agents.edges = agents.edges.map((agentEdge) => {
        agentEdge.node['__typename'] = 'Person'
        return agentEdge
      })
      return agents
    },
    organizations: async (root, args: PagingParams): Promise<OrganizationConnection> => {
      let agents = await readAll(args)
      agents.edges = agents.edges.filter((agentEdge) => agentEdge.node.agentType === 'Organization')
      return (agents as OrganizationConnection)
    },
    people: async (root, args: PagingParams): Promise<PersonConnection> => {
      let agents = await readAll(args)
      agents.edges = agents.edges.filter((agentEdge) => agentEdge.node.agentType === 'Person')
      return (agents as PersonConnection)
    },
  }
}
