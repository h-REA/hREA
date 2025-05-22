import {
    AgentConnection,
    AgentEdge,
    Agent,
  } from '@valueflows/vf-graphql'
  import { PagingParams } from '../types'
  import { getOne, getAll } from './helpers'

export default (cell: any) => {
    return {
        agents: async (root, args: PagingParams): Promise<AgentConnection> => {
            const output = await getAll(cell, 'agent', args)
            const formatted = output as AgentConnection
            return formatted
        },
        agent: async (root, args: { id: string }): Promise<Agent> => {
            const entry = await getOne(cell, 'get_agent', args)
            return entry as Agent
        },
        organizations: async (root, args: PagingParams): Promise<AgentConnection> => {
            const output = await getAll(cell, 'organization', args)
            return output as AgentConnection
        },
        organization: async (root, args: { id: string }): Promise<Agent> => {
            const entry = await getOne(cell, 'agent', args)
            if (entry?.agentType !== 'organization') {
                return entry as Agent
            } else {
                throw new Error('Agent is not an organization')
            }
        },
        people: async (root, args: PagingParams): Promise<AgentConnection> => {
            const output = await getAll(cell, 'person', args)
            return output as AgentConnection
        },
        person: async (root, args: { id: string }): Promise<Agent> => {
            const entry = await getOne(cell, 'agent', args)
            if (entry?.agentType !== 'person') {
                return entry as Agent
            } else {
                throw new Error('Agent is not a person')
            }
        }
    }
}
