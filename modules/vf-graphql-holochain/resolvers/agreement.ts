/**
 * Resolvers for agreement fields
 *
 * @package: Holo-REA
 * @since:   2020-06-19
 */

import { DNAIdMappings, DEFAULT_VF_MODULES } from '../types'
import { mapZomeFn } from '../connection'

import {
  Agreement,
  Commitment,
  EconomicEvent,
} from '@valueflows/vf-graphql'

export default (enabledVFModules: string[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri: string) => {
  const hasObservation = -1 !== enabledVFModules.indexOf("observation")
  const hasPlanning = -1 !== enabledVFModules.indexOf("planning")

  const queryCommitments = mapZomeFn(dnaConfig, conductorUri, 'planning', 'commitment_index', 'query_commitments')
  const queryEvents = mapZomeFn(dnaConfig, conductorUri, 'observation', 'economic_event_index', 'query_events')

  return Object.assign(
    (hasPlanning ? {
      commitments: async (record: Agreement): Promise<Commitment[]> => {
        return (await queryCommitments({ params: { clauseOf: record.id } })).map(({ commitment }) => commitment)
      },
    } : {}),
    (hasObservation ? {
      economicEvents: async (record: Agreement): Promise<EconomicEvent[]> => {
        return (await queryEvents({ params: { realizationOf: record.id } })).map(({ economicEvent }) => economicEvent)
      },
    } : {}),
  )
}
