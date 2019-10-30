/**
 * Resolvers for Satisfaction record relationships
 *
 * @package: HoloREA
 * @since:   2019-08-31
 */

import { zomeFunction } from '../connection'
import { addTypename } from '../types'

import {
  Satisfaction,
  EventOrCommitment,
  Intent,
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const readEvents = zomeFunction('observation', 'economic_event', 'query_events')
const readCommitments = zomeFunction('planning', 'commitment', 'query_commitments')
const readIntents = zomeFunction('planning', 'intent', 'query_intents')

async function extractRecordsOrFail (query, subfieldId: string): Promise<any> {
  const val = await query
  if (!val || !val.length || !val[0][subfieldId]) {
    throw new Error('Reference not found')
  }
  return val[0][subfieldId]
}

export const satisfiedBy = async (record: Satisfaction): Promise<EventOrCommitment> => {
  // :NOTE: this presumes a satisfaction will never be erroneously linked to 2 records
  return Promise.race([
    extractRecordsOrFail(readEvents({ params: { satisfies: record.id } }), 'economicEvent')
      .then(addTypename('EconomicEvent')),
    extractRecordsOrFail(readCommitments({ params: { satisfies: record.id } }), 'commitment')
      .then(addTypename('Commitment')),
  ])
}

export const satisfies = async (record: Satisfaction): Promise<Intent> => {
  return (await readIntents({ params: { satisfiedBy: record.id } })).pop()['intent']
}
