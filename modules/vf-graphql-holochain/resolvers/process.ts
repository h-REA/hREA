/**
 * Resolvers for Process fields
 *
 * @package: HoloREA
 * @since:   2019-09-12
 */

import { DNAIdMappings, injectTypename, DEFAULT_VF_MODULES } from '../types'
import { mapZomeFn } from '../connection'

import {
  Process,
  EconomicEvent,
  Commitment,
  Intent,
  ProcessSpecification
} from '@valueflows/vf-graphql'

export default (enabledVFModules: string[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri: string) => {
  const hasKnowledge = -1 !== enabledVFModules.indexOf("knowledge")
  const hasPlanning = -1 !== enabledVFModules.indexOf("planning")

  const readEvents = mapZomeFn(dnaConfig, conductorUri, 'observation', 'economic_event', 'query_events')
  const readCommitments = mapZomeFn(dnaConfig, conductorUri, 'planning', 'commitment', 'query_commitments')
  const readIntents = mapZomeFn(dnaConfig, conductorUri, 'planning', 'intent', 'query_intents')
  const readProcessBasedOn = mapZomeFn(dnaConfig, conductorUri, 'specification', 'process_specification', 'get_process_specification')

  return Object.assign(
    {
      inputs: injectTypename('EconomicEvent', async (record: Process): Promise<EconomicEvent[]> => {
        return (await readEvents({ params: { inputOf: record.id } })).map(({ economicEvent }) => economicEvent)
      }),

      outputs: injectTypename('EconomicEvent', async (record: Process): Promise<EconomicEvent[]> => {
        return (await readEvents({ params: { outputOf: record.id } })).map(({ economicEvent }) => economicEvent)
      }),
    },
    (hasPlanning ? {
      committedInputs: injectTypename('Commitment', async (record: Process): Promise<Commitment[]> => {
        return (await readCommitments({ params: { inputOf: record.id } })).map(({ commitment }) => commitment)
      }),

      committedOutputs: injectTypename('Commitment', async (record: Process): Promise<Commitment[]> => {
        return (await readCommitments({ params: { outputOf: record.id } })).map(({ commitment }) => commitment)
      }),

      intendedInputs: async (record: Process): Promise<Intent[]> => {
        return (await readIntents({ params: { inputOf: record.id } })).map(({ intent }) => intent)
      },

      intendedOutputs: async (record: Process): Promise<Intent[]> => {
        return (await readIntents({ params: { outputOf: record.id } })).map(({ intent }) => intent)
      },
    } : {}),
    (hasKnowledge ? {
      basedOn: async (record: Process): Promise<ProcessSpecification> => {
        return (await readProcessBasedOn({ address: record.basedOn })).processSpecification
      },
    } : {}),
  )
}
