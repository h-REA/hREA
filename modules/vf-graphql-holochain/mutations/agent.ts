/**
 * agent mutations
 *
 * @package: hREA
 * @since:   2022-06-08
 */

import { ByRevision, DNAIdMappings } from '../types'
import { mapZomeFn } from '../connection'
import { deleteHandler } from './'

import {
  AgentCreateParams,
  AgentUpdateParams,
  PersonResponse,
  OrganizationCreateParams,
  OrganizationUpdateParams,
  OrganizationResponse,
} from '@valueflows/vf-graphql'

export type AgentResponse = OrganizationResponse
export interface PersonCreateArgs {
    person: AgentCreateParams,
}
export type createPersonHandler = (root: any, args: PersonCreateArgs) => Promise<PersonResponse>

export interface PersonUpdateArgs {
  person: AgentUpdateParams,
}
export type updatePersonHandler = (root: any, args: PersonUpdateArgs) => Promise<PersonResponse>

export interface OrganizationCreateArgs {
    organization: OrganizationCreateParams,
}
export type createOrganizationHandler = (root: any, args: OrganizationCreateArgs) => Promise<OrganizationResponse>

export interface OrganizationUpdateArgs {
  organization: OrganizationUpdateParams,
}
export type updateOrganizationHandler = (root: any, args: OrganizationUpdateArgs) => Promise<OrganizationResponse>

export interface AgentCreateArgs {
    agent: OrganizationCreateParams,
}
export interface AgentUpdateArgs {
    agent: OrganizationUpdateParams,
}

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const runCreateAgent = mapZomeFn<AgentCreateArgs, AgentResponse>(dnaConfig, conductorUri, 'agent', 'agent', 'create_agent')
  const runUpdateAgent = mapZomeFn<AgentUpdateArgs, AgentResponse>(dnaConfig, conductorUri, 'agent', 'agent', 'update_agent')
  const runDeleteAgent = mapZomeFn<ByRevision, boolean>(dnaConfig, conductorUri, 'agent', 'agent', 'delete_agent')

  const createPerson: createPersonHandler = async (root, args) => {
    const createAgentArgs: AgentCreateArgs = {
        agent: {
            ...args.person,
            type: 'person',
        }
    }
    return runCreateAgent(createAgentArgs)
  }

  const updatePerson: updatePersonHandler = async (root, args) => {
    const updateAgentArgs: AgentUpdateArgs = {
        agent: {
            ...args.person,
        }
    }
    return runUpdateAgent(updateAgentArgs)
  }

  const deletePerson: deleteHandler = async (root, args) => {
    return runDeleteAgent(args)
  }

  const createOrganization: createOrganizationHandler = async (root, args) => {
    const createAgentArgs: AgentCreateArgs = {
        agent: {
            ...args.organization,
            type: 'organization',
        }
    }
    return runCreateAgent(createAgentArgs)
  }

  const updateOrganization: updateOrganizationHandler = async (root, args) => {
    const updateAgentArgs: AgentUpdateArgs = {
        agent: {
            ...args.organization,
        }
    }
    return runUpdateAgent(updateAgentArgs)
  }

  const deleteOrganization: deleteHandler = async (root, args) => {
    return runDeleteAgent(args)
  }

  return {
    createPerson,
    updatePerson,
    deletePerson,
    createOrganization,
    updateOrganization,
    deleteOrganization,
  }
}


