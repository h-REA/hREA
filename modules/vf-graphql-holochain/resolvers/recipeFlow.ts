/**
 * Recipe Flow record reference resolvers
 *
 * @package: HoloREA
 * @since:   2019-08-31
 */

import { DNAIdMappings, DEFAULT_VF_MODULES, VfModule, ReadParams, ById, ByRevision, ProposedIntentAddress, ResourceSpecificationAddress, AddressableIdentifier, AgentAddress } from '../types.js'
import { extractEdges, mapZomeFn } from '../connection.js'

import {
  Maybe,
  Agent,
  RecipeFlow,
  RecipeFlowResponse,
  Satisfaction,
  Process,
  ResourceSpecification,
  ProposedIntent,
  Action,
  SatisfactionConnection,
  ProcessConnection,
  ProposedIntentResponse,
  ResourceSpecificationResponse,
  AccountingScope,
} from '@valueflows/vf-graphql'

import agentQueries from '../queries/agent.js'
import { ProcessSearchInput, SatisfactionSearchInput } from './zomeSearchInputTypes.js'

const extractProposedIntent = (data): ProposedIntent => data.proposedIntent

export default (enabledVFModules: VfModule[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri: string) => {
  const hasHistory = -1 !== enabledVFModules.indexOf(VfModule.History)
  const hasAgent = -1 !== enabledVFModules.indexOf(VfModule.Agent)
  const hasSatisfaction = -1 !== enabledVFModules.indexOf(VfModule.Satisfaction)
  const hasResourceSpecification = -1 !== enabledVFModules.indexOf(VfModule.ResourceSpecification)
  const hasAction = -1 !== enabledVFModules.indexOf(VfModule.Action)
  const hasProcess = -1 !== enabledVFModules.indexOf(VfModule.Process)
  const hasProposal = -1 !== enabledVFModules.indexOf(VfModule.Proposal)
  const hasObservation = -1 !== enabledVFModules.indexOf(VfModule.Observation)

  const readRevision = mapZomeFn<ByRevision, RecipeFlowResponse>(dnaConfig, conductorUri, 'planning', 'recipe_flow', 'get_revision')
  const readSatisfactions = mapZomeFn<SatisfactionSearchInput, SatisfactionConnection>(dnaConfig, conductorUri, 'planning', 'satisfaction_index', 'query_satisfactions')
  const readProcesses = mapZomeFn<ProcessSearchInput, ProcessConnection>(dnaConfig, conductorUri, 'observation', 'process_index', 'query_processes')
  const readProposedIntent = mapZomeFn<ReadParams, ProposedIntentResponse>(dnaConfig, conductorUri, 'proposal', 'proposed_intent', 'get_proposed_intent')
  const readResourceSpecification = mapZomeFn<ReadParams, ResourceSpecificationResponse>(dnaConfig, conductorUri, 'specification', 'resource_specification', 'get_resource_specification')
  const readAction = mapZomeFn<ById, Action>(dnaConfig, conductorUri, 'specification', 'action', 'get_action')
  const readAgent = agentQueries(dnaConfig, conductorUri)['agent']

  return Object.assign(
    (hasSatisfaction ? {
      satisfiedBy: async (record: RecipeFlow): Promise<Satisfaction[]> => {
        const results = await readSatisfactions({ params: { satisfies: record.id } })
        return extractEdges(results)
      },
    } : {}),
    (hasAgent ? {
      provider: async (record: RecipeFlow): Promise<Maybe<Agent>> => {
        return record.provider ? readAgent(record, { id: record.provider }) : null
      },

      receiver: async (record: RecipeFlow): Promise<Maybe<Agent>> => {
        return record.receiver ? readAgent(record, { id: record.receiver }) : null
      },
      inScopeOf: async (record: { inScopeOf: AgentAddress[] }): Promise<AccountingScope[]> => {
        return (await Promise.all((record.inScopeOf || []).map((address)=>readAgent(record, {address}))))
      },
    } : {}),
    (hasProcess ? {
      inputOf: async (record: RecipeFlow): Promise<Process> => {
        const results = await readProcesses({ params: { intendedInputs: record.id } })
        return results.edges.pop()!['node']
      },

      outputOf: async (record: RecipeFlow): Promise<Process> => {
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
    (hasObservation ? {
      resourceInventoriedAs: () => {
        throw new Error('resolver unimplemented')
      },
    } : {}),
    (hasHistory ? {
      revision: async (record: RecipeFlow, args: { revisionId: AddressableIdentifier }): Promise<RecipeFlow> => {
        return (await readRevision(args)).recipe_flow
      },
    } : {}),
  )
}
