/**
 * Resolvers for Process fields
 *
 * @package: HoloREA
 * @since:   2019-09-12
 */

import { DNAIdMappings, injectTypename, DEFAULT_VF_MODULES, VfModule, ReadParams } from '../types'
import { mapZomeFn, extractEdges } from '../connection'

import {
  Process,
  EconomicEvent,
  Commitment,
  Intent,
  ProcessSpecification,
  Plan,
  EconomicEventConnection,
  CommitmentConnection,
  IntentConnection,
  ProcessSpecificationResponse
} from '@valueflows/vf-graphql'
import planQueries from '../queries/plan'
import { CommitmentSearchInput, EconomicEventSearchInput, IntentSearchInput } from './zomeSearchInputTypes'

export default (enabledVFModules: VfModule[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri: string) => {
  const hasKnowledge = -1 !== enabledVFModules.indexOf(VfModule.Knowledge)
  const hasPlanning = -1 !== enabledVFModules.indexOf(VfModule.Planning)
  const hasPlan = -1 !== enabledVFModules.indexOf(VfModule.Plan)

  const readEvents = mapZomeFn<EconomicEventSearchInput, EconomicEventConnection>(dnaConfig, conductorUri, 'observation', 'economic_event_index', 'query_economic_events')
  const readCommitments = mapZomeFn<CommitmentSearchInput, CommitmentConnection>(dnaConfig, conductorUri, 'planning', 'commitment_index', 'query_commitments')
  const readIntents = mapZomeFn<IntentSearchInput, IntentConnection>(dnaConfig, conductorUri, 'planning', 'intent_index', 'query_intents')
  const readProcessBasedOn = mapZomeFn<ReadParams, ProcessSpecificationResponse>(dnaConfig, conductorUri, 'specification', 'process_specification', 'get_process_specification')
  const readPlan = planQueries(dnaConfig, conductorUri)['plan']

  return Object.assign(
    {
      inputs: injectTypename('EconomicEvent', async (record: Process): Promise<EconomicEvent[]> => {
        const results = await readEvents({ params: { inputOf: record.id } })
        return extractEdges(results)
      }),

      outputs: injectTypename('EconomicEvent', async (record: Process): Promise<EconomicEvent[]> => {
        const results = await readEvents({ params: { outputOf: record.id } })
        return extractEdges(results)
      }),
    },
    (hasPlanning ? {
      committedInputs: injectTypename('Commitment', async (record: Process): Promise<Commitment[]> => {
        const results = await readCommitments({ params: { inputOf: record.id } })
        return extractEdges(results)
      }),

      committedOutputs: injectTypename('Commitment', async (record: Process): Promise<Commitment[]> => {
        const results = await readCommitments({ params: { outputOf: record.id } })
        return extractEdges(results)
      }),

      intendedInputs: async (record: Process): Promise<Intent[]> => {
        const results = await readIntents({ params: { inputOf: record.id } })
        return extractEdges(results)
      },

      intendedOutputs: async (record: Process): Promise<Intent[]> => {
        const results = await readIntents({ params: { outputOf: record.id } })
        return extractEdges(results)
      },
    } : {}),
    (hasKnowledge ? {
      basedOn: async (record: Process): Promise<ProcessSpecification> => {
        return (await readProcessBasedOn({ address: record.basedOn })).processSpecification
      },
    } : {}),
    (hasPlan ? {
      plannedWithin: async (record: Process): Promise<Plan> => {
        return (await readPlan(record, { id: record.plannedWithin }))
      },
    } : {}),
  )
}
