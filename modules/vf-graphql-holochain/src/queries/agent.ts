import {
    AgentConnection,
    AgentEdge,
    Agent,
  } from '@valueflows/vf-graphql'
  import { PagingParams } from '../types.js'
  import { getOne, getAll } from './helpers.js'

export default (cell: any) => {
    return {
        agents: async (root, args: PagingParams): Promise<AgentConnection> => {
            const output = await getAll(cell, 'agent', args)
            const formatted = output as AgentConnection
            return formatted
        },
        agent: async (root, args: { id: string }): Promise<Agent> => {
            const entry = await getOne(cell, 'agent', args)
            return entry as Agent
        },
        organizations: async (root, args: PagingParams): Promise<AgentConnection> => {
            const output = await getAll(cell, 'organization', args)
            return output as AgentConnection
        },
        organization: async (root, args: { id: string }): Promise<Agent> => {
            const entry = await getOne(cell, 'agent', args)
            if (entry?.agentType !== 'Organization') {
                throw new Error('Agent is not an organization')
            }
            return entry as Agent
        },
        people: async (root, args: PagingParams): Promise<AgentConnection> => {
            const output = await getAll(cell, 'person', args)
            return output as AgentConnection
        },
        person: async (root, args: { id: string }): Promise<Agent> => {
            const entry = await getOne(cell, 'agent', args)
            if (entry?.agentType !== 'Person') {
                throw new Error('Agent is not a person')
            }
            return entry as Agent
        }
    }
}
