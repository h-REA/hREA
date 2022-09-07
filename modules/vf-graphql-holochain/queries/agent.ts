/**
 * Agent queries
 *
 * :TODO: wire into Personas hApp and replace generated agent names with serving of profile data
 *
 * @package: hREA
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
import { AgentSearchInput, PagingParams } from '../resolvers/zomeSearchInputTypes.js'

export interface RegistrationQueryParams {
  pubKey: AgentPubKey,
}
export type AgentWithType = Agent & { agentType: string }
export interface AgentWithTypeResponse {
  agent: AgentWithType
}
export interface AgentEdgeWithTypeEdge extends Omit<AgentEdge, 'node'> {
  node: AgentWithType
}
export interface AgentConnectionWithType extends Omit<AgentConnection, 'edges'> {
  edges: AgentEdgeWithTypeEdge[]
}

export function addAgentTypename(record: AgentWithType) {
  record['__typename'] = record.agentType.toLowerCase() === 'person' ? 'Person' : (
    record.agentType.toLowerCase() === 'organization' ? 'Organization' : record.agentType
  )
  return record as Agent
}

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {

  //assumes there is a link from agentPubKey to a Person entry, but what if link cannot be resolved?
  const readMyAgent = mapZomeFn<null, AgentResponse>(dnaConfig, conductorUri, 'agent', 'agent', 'get_my_agent')
  const readAgent = mapZomeFn<ReadParams, AgentWithTypeResponse>(dnaConfig, conductorUri, 'agent', 'agent', 'get_agent')
  const readAll = mapZomeFn<PagingParams, AgentConnectionWithType>(dnaConfig, conductorUri, 'agent', 'agent_index', 'read_all_agents')
  const readAllAgentType = mapZomeFn<AgentSearchInput, AgentConnection>(dnaConfig, conductorUri, 'agent', 'agent_index', 'query_agents')

  const agentRelationship = () => {
    throw new Error('query unimplemented')
  }
  const agentRelationships = () => {
    throw new Error('query unimplemented')
  }
  const agentRelationshipRole = () => {
    throw new Error('query unimplemented')
  }
  const agentRelationshipRoles = () => {
    throw new Error('query unimplemented')
  }

  return {
    // :TODO: is myAgent always a 'Person' in Holochain, or will we allow users to act in an Organization context directly?
    myAgent: injectTypename('Person', async (root, args): Promise<Agent> => {
      return (await readMyAgent(null)).agent
    }),
    agent: async (root, args): Promise<Agent> => {
      return addAgentTypename((await readAgent({ address: args.id })).agent)
    },
    organization: injectTypename('Organization', async (root, args): Promise<Organization> => {
      return ((await readAgent({ address: args.id })).agent) as Organization
      // TODO: type check if person or organization and provide error if person
    }),
    person: injectTypename('Person', async (root, args): Promise<Person> => {
      return ((await readAgent({ address: args.id })).agent) as Person
      // TODO: type check if person or organization and provide error if organization
    }),
    agents: async (root, args: PagingParams): Promise<AgentConnection> => {
      const agents = await readAll(args)
      return {
        pageInfo: agents.pageInfo,
        edges: agents.edges.map(({ cursor, node }) => {
          return { cursor, node: addAgentTypename(node) }
        })
      } as AgentConnection
    },
    organizations: async (root, args: PagingParams): Promise<OrganizationConnection> => {
      const agents = await readAllAgentType({ params: { agentType: 'Organization' } })
      return (agents as OrganizationConnection)
    },
    people: async (root, args: PagingParams): Promise<PersonConnection> => {
      const agents = await readAllAgentType({ params: { agentType: 'Person' } })
      return (agents as PersonConnection)
    },
    agentRelationship,
    agentRelationships,
    agentRelationshipRole,
    agentRelationshipRoles,
  }
}
