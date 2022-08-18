/**
 * Resolvers for Process fields
 *
 * @package: HoloREA
 * @since:   2019-09-12
 */

import { DNAIdMappings, injectTypename, DEFAULT_VF_MODULES, VfModule, ReadParams, ByRevision, ProcessSpecificationAddress, AgentAddress, AddressableIdentifier } from '../types.js'
import { mapZomeFn, extractEdges } from '../connection.js'

import {
  Process,
  ProcessResponse,
  EconomicEvent,
  Commitment,
  Intent,
  ProcessSpecification,
  Plan,
  EconomicEventConnection,
  CommitmentConnection,
  IntentConnection,
  ProcessSpecificationResponse,
  Agent,
  AccountingScope
} from '@valueflows/vf-graphql'
import planQueries from '../queries/plan.js'
import { CommitmentSearchInput, EconomicEventSearchInput, IntentSearchInput } from './zomeSearchInputTypes.js'
import { AgentResponse } from '../mutations/agent.js'

export default (enabledVFModules: VfModule[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri: string) => {
  const hasHistory = -1 !== enabledVFModules.indexOf(VfModule.History)
  const hasObservation = -1 !== enabledVFModules.indexOf(VfModule.Observation)
  const hasProcessSpecification = -1 !== enabledVFModules.indexOf(VfModule.ProcessSpecification)
  const hasCommitment = -1 !== enabledVFModules.indexOf(VfModule.Commitment)
  const hasIntent = -1 !== enabledVFModules.indexOf(VfModule.Intent)
  const hasPlan = -1 !== enabledVFModules.indexOf(VfModule.Plan)
  const hasAgent = -1 !== enabledVFModules.indexOf(VfModule.Agent)

  const readRevision = mapZomeFn<ByRevision, ProcessResponse>(dnaConfig, conductorUri, 'observation', 'process', 'get_revision')
  const readEvents = mapZomeFn<EconomicEventSearchInput, EconomicEventConnection>(dnaConfig, conductorUri, 'observation', 'economic_event_index', 'query_economic_events')
  const readCommitments = mapZomeFn<CommitmentSearchInput, CommitmentConnection>(dnaConfig, conductorUri, 'planning', 'commitment_index', 'query_commitments')
  const readIntents = mapZomeFn<IntentSearchInput, IntentConnection>(dnaConfig, conductorUri, 'planning', 'intent_index', 'query_intents')
  const readProcessBasedOn = mapZomeFn<ReadParams, ProcessSpecificationResponse>(dnaConfig, conductorUri, 'specification', 'process_specification', 'get_process_specification')
  const readPlan = planQueries(dnaConfig, conductorUri)['plan']
  const readAgent = mapZomeFn<ReadParams, AgentResponse>(dnaConfig, conductorUri, 'agent', 'agent', 'get_agent')

  return Object.assign(
    {
      nextProcesses: () => {
        throw new Error('resolver unimplemented')
      },
      previousProcesses: () => {
        throw new Error('resolver unimplemented')
      },
    },
    (hasObservation ? {
      observedInputs: injectTypename('EconomicEvent', async (record: Process): Promise<EconomicEvent[]> => {
        const results = await readEvents({ params: { inputOf: record.id } })
        return extractEdges(results)
      }),

      observedOutputs: injectTypename('EconomicEvent', async (record: Process): Promise<EconomicEvent[]> => {
        const results = await readEvents({ params: { outputOf: record.id } })
        return extractEdges(results)
      }),

      unplannedInputs: () => {
        throw new Error('resolver unimplemented')
      },
      unplannedOutputs: () => {
        throw new Error('resolver unimplemented')
      },
      previous: () => {
        throw new Error('resolver unimplemented')
      },
      next: () => {
        throw new Error('resolver unimplemented')
      },
    } : {}),
    (hasCommitment ? {
      committedInputs: injectTypename('Commitment', async (record: Process): Promise<Commitment[]> => {
        const results = await readCommitments({ params: { inputOf: record.id } })
        return extractEdges(results)
      }),

      committedOutputs: injectTypename('Commitment', async (record: Process): Promise<Commitment[]> => {
        const results = await readCommitments({ params: { outputOf: record.id } })
        return extractEdges(results)
      }),
    } : {}),
    (hasIntent ? {
      intendedInputs: async (record: Process): Promise<Intent[]> => {
        const results = await readIntents({ params: { inputOf: record.id } })
        return extractEdges(results)
      },

      intendedOutputs: async (record: Process): Promise<Intent[]> => {
        const results = await readIntents({ params: { outputOf: record.id } })
        return extractEdges(results)
      },
    } : {}),
    (hasProcessSpecification ? {
      basedOn: async (record: { basedOn: ProcessSpecificationAddress }): Promise<ProcessSpecification> => {
        return (await readProcessBasedOn({ address: record.basedOn })).processSpecification
      },
    } : {}),
    (hasPlan ? {
      plannedWithin: async (record: Process): Promise<Plan> => {
        return (await readPlan(record, { id: record.plannedWithin }))
      },
    } : {}),
    (hasAgent ? {
      involvedAgents: () => {
        throw new Error('resolver unimplemented')
      },
      workingAgents: async (record: { workingAgents: AgentAddress[] }): Promise<Agent[]> => {
        return (await Promise.all((record.workingAgents || []).map((address)=>readAgent({address})))).map((agentResponse) => agentResponse.agent)
      },
      inScopeOf: async (record: { inScopeOf: AgentAddress[] }): Promise<AccountingScope[]> => {
        return (await Promise.all((record.inScopeOf || []).map((address)=>readAgent({address})))).map((agentResponse) => agentResponse.agent)
      },
    } : {}),
    (hasHistory ? {
      revision: async (record: Process, args: { revisionId: AddressableIdentifier }): Promise<Process> => {
        return (await readRevision(args)).process
      },
    } : {}),
  )
}
