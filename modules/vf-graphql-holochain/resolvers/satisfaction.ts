/**
 * Resolvers for Satisfaction record relationships
 *
 * @package: HoloREA
 * @since:   2019-08-31
 */

import { zomeFunction } from '../connection'

import {
  Satisfaction,
  EventOrCommitment,
  Intent,
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const readEvents = zomeFunction('observation', 'economic_event', 'query_events')
const readCommitments = zomeFunction('planning', 'commitment', 'query_commitments')
const readIntents = zomeFunction('planning', 'intent', 'query_intents')

async function extractRecordsOrFail(query, subfieldId: string): Promise<any> {
  const val = await query
  if (!val || !val.length || !val[0][subfieldId]) {
    throw new Error('Reference not found')
  }
  return val[0][subfieldId]
}

export const satisfiedBy = async (record: Satisfaction): Promise<EventOrCommitment> => {
  // :NOTE: this presumes a satisfaction will never be erroneously linked to 2 records
  return Promise.race([
    extractRecordsOrFail((await (await readEvents)({ satisfies: record.id })), 'economicEvent'),
    extractRecordsOrFail((await (await readCommitments)({ satisfies: record.id })), 'commitment'),
  ])
}

export const satisfies = async (record: Satisfaction): Promise<Intent> => {
  return (await (await readIntents)({ satisfied_by: record.id })).map(({ intent }) => intent)[0]
}
