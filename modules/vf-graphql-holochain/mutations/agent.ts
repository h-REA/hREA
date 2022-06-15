/**
 * agent mutations
 *
 * @package: hREA
 * @since:   2022-06-08
 */

import { ByRevision, DNAIdMappings } from '../types.js'
import { mapZomeFn } from '../connection.js'
import { deleteHandler } from './'

import {
  AgentCreateParams,
  AgentUpdateParams,
  PersonResponse,
  OrganizationCreateParams,
  OrganizationUpdateParams,
  OrganizationResponse,
  AccountingScope,
  Person,
} from '@valueflows/vf-graphql'

// export type AgentResponse = OrganizationResponse
export interface AgentResponse {
    agent: AccountingScope,
}
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
            //@ts-ignore `type` field doesn't exist yet
            type: 'person',
        }
    }
    return (await runCreateAgent(createAgentArgs)) as PersonResponse
  }

  const updatePerson: updatePersonHandler = async (root, args) => {
    const updateAgentArgs: AgentUpdateArgs = {
        agent: {
            ...args.person,
        }
    }
    return ( await runUpdateAgent(updateAgentArgs)) as PersonResponse
  }

  const deletePerson: deleteHandler = async (root, args) => {
    return runDeleteAgent(args)
  }

  const createOrganization: createOrganizationHandler = async (root, args) => {
    const createAgentArgs: AgentCreateArgs = {
        agent: {
            ...args.organization,
            //@ts-ignore `type` field doesn't exist yet
            type: 'organization',
        }
    }
    return (await runCreateAgent(createAgentArgs)) as OrganizationResponse
  }

  const updateOrganization: updateOrganizationHandler = async (root, args) => {
    const updateAgentArgs: AgentUpdateArgs = {
        agent: {
            ...args.organization,
        }
    }
    return (await runUpdateAgent(updateAgentArgs)) as OrganizationResponse
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


