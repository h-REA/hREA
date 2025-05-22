import {
    AgentCreateParams,
    OrganizationResponse,
} from "@valueflows/vf-graphql"
import { createEntry, updateEntry, deleteEntry } from "./helpers"

export default (cell: any) => {
  return {
    createOrganization: async (root, args) => {
      const createAgentArgs: AgentCreateParams = {
          ...args.organization,
          agent_type: 'Organization',
      }
      return await createEntry(cell, 'agent', {agent: createAgentArgs}) as OrganizationResponse
    },
    deleteOrganization: async (root, args) => {
      return await deleteEntry(cell, 'agent', args)
    },
    updateOrganization: async (root, args) => {
      const updateAgentArgs: AgentCreateParams = {
          ...args.organization,
          agent_type: 'Organization',
      }
      return await updateEntry(cell, 'agent', {agent: updateAgentArgs}) as OrganizationResponse
    },
    createPerson: async (root, args) => {
      const createAgentArgs: AgentCreateParams = {
          ...args.person,
          agent_type: 'Person',
      }
      return await createEntry(cell, 'agent', {agent: createAgentArgs}) as OrganizationResponse
    },
    deletePerson: async (root, args) => {
      return await deleteEntry(cell, 'agent', args)
    },
    updatePerson: async (root, args) => {
      const updateAgentArgs: AgentCreateParams = {
          ...args.person,
          agent_type: 'Person',
      }
      return await updateEntry(cell, 'agent', {agent: updateAgentArgs}) as OrganizationResponse
    },
  }
}