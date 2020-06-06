/**
 * Resolvers for Satisfaction record relationships
 *
 * @package: HoloREA
 * @since:   2019-08-31
 */

import { DNAIdMappings, addTypename } from '../types'
import { mapZomeFn } from '../connection'

import {
  Satisfaction,
  EventOrCommitment,
  Intent,
} from '@valueflows/vf-graphql'

async function extractRecordsOrFail (query, subfieldId: string): Promise<any> {
  const val = await query
  if (!val || !val.length || !val[0][subfieldId]) {
    throw new Error('Reference not found')
  }
  return val[0][subfieldId]
}

export default (dnaConfig?: DNAIdMappings, conductorUri?: string) => {
  const readEvents = mapZomeFn(dnaConfig, conductorUri, 'observation', 'economic_event', 'query_events')
  const readCommitments = mapZomeFn(dnaConfig, conductorUri, 'planning', 'commitment', 'query_commitments')
  const readIntents = mapZomeFn(dnaConfig, conductorUri, 'planning', 'intent', 'query_intents')

  return {
    satisfiedBy: async (record: Satisfaction): Promise<EventOrCommitment> => {
      // :NOTE: this presumes a satisfaction will never be erroneously linked to 2 records
      return (
        await Promise.all([
          extractRecordsOrFail(readEvents({ params: { satisfies: record.id } }), 'economicEvent')
            .then(addTypename('EconomicEvent'))
            .catch((e) => e),
          extractRecordsOrFail(readCommitments({ params: { satisfies: record.id } }), 'commitment')
            .then(addTypename('Commitment'))
            .catch((e) => e),
        ])
      )
      .filter(r => !(r instanceof Error))
      .pop()
    },

    satisfies: async (record: Satisfaction): Promise<Intent> => {
      return (await readIntents({ params: { satisfiedBy: record.id } })).pop()['intent']
    },
  }
}
