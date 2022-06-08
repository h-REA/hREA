/**
 * Intent record reference resolvers
 *
 * @package: HoloREA
 * @since:   2019-08-31
 */

import { DNAIdMappings, DEFAULT_VF_MODULES, VfModule, ReadParams, ById, ProposedIntentAddress, ResourceSpecificationAddress, AddressableIdentifier } from '../types'
import { extractEdges, mapZomeFn } from '../connection'

import {
  Maybe,
  Agent,
  Intent,
  Satisfaction,
  Process,
  ResourceSpecification,
  ProposedIntent,
  Action,
  SatisfactionConnection,
  ProcessConnection,
  ProposedIntentResponse,
  ResourceSpecificationResponse,
} from '@valueflows/vf-graphql'

import agentQueries from '../queries/agent'
import { ProcessSearchInput, SatisfactionSearchInput } from './zomeSearchInputTypes'

const extractProposedIntent = (data): ProposedIntent => data.proposedIntent

export default (enabledVFModules: VfModule[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri: string) => {
  const hasAgent = -1 !== enabledVFModules.indexOf(VfModule.Agent)
  const hasSatisfaction = -1 !== enabledVFModules.indexOf(VfModule.Satisfaction)
  const hasResourceSpecification = -1 !== enabledVFModules.indexOf(VfModule.ResourceSpecification)
  const hasAction = -1 !== enabledVFModules.indexOf(VfModule.Action)
  const hasProcess = -1 !== enabledVFModules.indexOf(VfModule.Process)
  const hasProposal = -1 !== enabledVFModules.indexOf(VfModule.Proposal)

  const readSatisfactions = mapZomeFn<SatisfactionSearchInput, SatisfactionConnection>(dnaConfig, conductorUri, 'planning', 'satisfaction_index', 'query_satisfactions')
  const readProcesses = mapZomeFn<ProcessSearchInput, ProcessConnection>(dnaConfig, conductorUri, 'observation', 'process_index', 'query_processes')
  const readProposedIntent = mapZomeFn<ReadParams, ProposedIntentResponse>(dnaConfig, conductorUri, 'proposal', 'proposed_intent', 'get_proposed_intent')
  const readResourceSpecification = mapZomeFn<ReadParams, ResourceSpecificationResponse>(dnaConfig, conductorUri, 'specification', 'resource_specification', 'get_resource_specification')
  const readAction = mapZomeFn<ById, Action>(dnaConfig, conductorUri, 'specification', 'action', 'get_action')
  const readAgent = agentQueries(dnaConfig, conductorUri)['agent']

  return Object.assign(
    (hasSatisfaction ? {
      satisfiedBy: async (record: Intent): Promise<Satisfaction[]> => {
        const results = await readSatisfactions({ params: { satisfies: record.id } })
        return extractEdges(results)
      },
    } : {}),
    (hasAgent ? {
      provider: async (record: Intent): Promise<Maybe<Agent>> => {
        return record.provider ? readAgent(record, { id: record.provider }) : null
      },

      receiver: async (record: Intent): Promise<Maybe<Agent>> => {
        return record.receiver ? readAgent(record, { id: record.receiver }) : null
      },
    } : {}),
    (hasProcess ? {
      inputOf: async (record: Intent): Promise<Process> => {
        const results = await readProcesses({ params: { intendedInputs: record.id } })
        return results.edges.pop()!['node']
      },

      outputOf: async (record: Intent): Promise<Process> => {
        const results = await readProcesses({ params: { intendedOutputs: record.id } })
        return results.edges.pop()!['node']
      },
    } : {}),
    (hasProposal ? {
      publishedIn: async (record: { publishedIn: ProposedIntentAddress[] }): Promise<ProposedIntent[]> => {
        return (await Promise.all((record.publishedIn || []).map((address)=>readProposedIntent({address})))).map(extractProposedIntent)
      },
    } : {}),
    (hasResourceSpecification ? {
      resourceConformsTo: async (record: { resourceConformsTo: ResourceSpecificationAddress }): Promise<ResourceSpecification> => {
        return (await readResourceSpecification({ address: record.resourceConformsTo })).resourceSpecification
      },
    } : {}),
    (hasAction ? {
      action: async (record: { action: AddressableIdentifier }): Promise<Action> => {
        return (await readAction({ id: record.action }))
      },
    } : {}),
  )
}
