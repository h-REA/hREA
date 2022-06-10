/**
 * Resolvers for agent fields
 *
 * @package: Holo-REA
 * @since:   2020-05-28
 */

import {
  Process,
  PlanProcessFilterParams,
  Plan,
  Commitment,
  ProcessConnection,
  CommitmentConnection,
  Agent,
  IntentConnection,
  EconomicEventConnection,
  EconomicResourceConnection,
  PlanConnection,
  ProposalConnection
} from '@valueflows/vf-graphql'
import { extractEdges, mapZomeFn } from '../connection'
import { DNAIdMappings, DEFAULT_VF_MODULES, VfModule } from '../types'
import { CommitmentSearchInput, EconomicEventSearchInput, EconomicResourceSearchInput, IntentSearchInput, PlanSearchInput, ProcessSearchInput, ProposalSearchInput } from './zomeSearchInputTypes'


export default (enabledVFModules: VfModule[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri: string) => {

  const hasProcess = -1 !== enabledVFModules.indexOf(VfModule.Process)
  const hasCommitment = -1 !== enabledVFModules.indexOf(VfModule.Commitment)
  const hasIntent = -1 !== enabledVFModules.indexOf(VfModule.Intent)
  const hasObservation = -1 !== enabledVFModules.indexOf(VfModule.Observation)
  const hasPlan = -1 !== enabledVFModules.indexOf(VfModule.Plan)
  const hasProposal = -1 !== enabledVFModules.indexOf(VfModule.Proposal)

  const readProcesses = mapZomeFn<ProcessSearchInput, ProcessConnection>(dnaConfig, conductorUri, 'observation', 'process_index', 'query_processes')
  const queryCommitments = mapZomeFn<CommitmentSearchInput, CommitmentConnection>(dnaConfig, conductorUri, 'planning', 'commitment_index', 'query_commitments')
  const queryIntents = mapZomeFn<IntentSearchInput, IntentConnection>(dnaConfig, conductorUri, 'planning', 'intent_index', 'query_intents')
  const queryEconomicEvents = mapZomeFn<EconomicEventSearchInput, EconomicEventConnection>(dnaConfig, conductorUri, 'observation', 'economic_event_index', 'query_economic_events')
  const queryEconomicResources = mapZomeFn<EconomicResourceSearchInput, EconomicResourceConnection>(dnaConfig, conductorUri, 'observation', 'economic_resource_index', 'query_economic_resources')
  const queryPlans = mapZomeFn<PlanSearchInput, PlanConnection>(dnaConfig, conductorUri, 'plan', 'plan_index', 'query_plans')
  const queryProposals = mapZomeFn<ProposalSearchInput, ProposalConnection>(dnaConfig, conductorUri, 'plan', 'plan_index', 'query_plans')

  return Object.assign(
    (hasProcess ? {
      processes: async (record: Agent): Promise<ProcessConnection> => {
        const results = await readProcesses({ params: { inScopeOf: record.id } })
        return results
      },
    } : {}),
    (hasCommitment ? {
      commitments: async (record: Agent): Promise<CommitmentConnection> => {
        const commitments = await queryCommitments({ params: { inScopeOf: record.id } })
        return commitments
      },
    } : {}),
    (hasIntent ? {
      intents: async (record: Agent): Promise<IntentConnection> => {
        const intents = await queryIntents({ params: { inScopeOf: record.id } })
        return intents
      },
    } : {}),
    (hasObservation ? {
      economicEvents: async (record: Agent): Promise<EconomicEventConnection> => {
        const economicEvents = await queryEconomicEvents({ params: { inScopeOf: record.id } })
        return economicEvents
      },
      inventoriedEconomicResources: async (record: Agent): Promise<EconomicResourceConnection> => {
        const economicResources = await queryEconomicResources({ params: { primaryAccountable: record.id } })
        return economicResources
      },
    } : {}),
    (hasPlan ? {
      plans: async (record: Agent): Promise<PlanConnection> => {
        const plans = await queryPlans({ params: { inScopeOf: record.id } })
        return plans
      },
    } : {}),
    (hasProposal ? {
      proposals: async (record: Agent): Promise<ProposalConnection> => {
        const proposals = await queryProposals({ params: { inScopeOf: record.id } })
        return proposals
      },
    } : {}),
  )
}

