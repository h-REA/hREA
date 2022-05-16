/**
 * Resolvers for Satisfaction record relationships
 *
 * @package: HoloREA
 * @since:   2019-08-31
 */

import { DNAIdMappings, addTypename, DEFAULT_VF_MODULES, VfModule } from '../types'
import { mapZomeFn, remapCellId } from '../connection'

import {
  Satisfaction,
  EventOrCommitment,
  Intent,
} from '@valueflows/vf-graphql'

async function extractRecordsOrFail (query): Promise<any> {
  const val = await query
  if (!val || !val.edges || !val.edges.length || !val.edges[0].node) {
    throw new Error('Reference not found')
  }
  return val.edges[0].node
}

export default (enabledVFModules: VfModule[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri: string) => {
  const hasObservation = -1 !== enabledVFModules.indexOf(VfModule.Observation)

  const readEvents = mapZomeFn(dnaConfig, conductorUri, 'observation', 'economic_event_index', 'query_economic_events')
  const readCommitments = mapZomeFn(dnaConfig, conductorUri, 'planning', 'commitment_index', 'query_commitments')
  const readIntents = mapZomeFn(dnaConfig, conductorUri, 'planning', 'intent_index', 'query_intents')

  return {
    satisfiedBy: async (record: Satisfaction): Promise<EventOrCommitment> => {
      const associatedId = remapCellId(record.id, record.satisfiedBy)
      // :NOTE: this presumes a satisfaction will never be erroneously linked to 2 records
      return (
        await Promise.all([
          extractRecordsOrFail(readCommitments({ params: { satisfies: associatedId } }))
            .then(addTypename('Commitment'))
            .catch((e) => e),
        ].concat(hasObservation ? [
          extractRecordsOrFail(readEvents({ params: { satisfies: associatedId } }))
            .then(addTypename('EconomicEvent'))
            .catch((e) => e),
        ] : []))
      )
      .filter(r => !(r instanceof Error))
      .pop()
    },

    satisfies: async (record: Satisfaction): Promise<Intent> => {
      const results = await readIntents({ params: { satisfiedBy: record.id } })
      return results.edges.pop()['node']
    },
  }
}
