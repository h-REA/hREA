/**
 * Resolvers for Fulfillment fields
 *
 * @package: HoloREA
 * @since:   2019-08-27
 */

import { DNAIdMappings, injectTypename } from '../types'
import { mapZomeFn } from '../connection'

import {
  Fulfillment,
  EconomicEvent,
  Commitment,
} from '@valueflows/vf-graphql'

export default (dnaConfig?: DNAIdMappings, conductorUri?: string) => {
  const readEvents = mapZomeFn(dnaConfig, conductorUri, 'observation', 'economic_event', 'query_events')
  const readCommitments = mapZomeFn(dnaConfig, conductorUri, 'planning', 'commitment', 'query_commitments')

  return {
    fulfilledBy: injectTypename('EconomicEvent', async (record: Fulfillment): Promise<EconomicEvent> => {
      return (await readEvents({ params: { fulfills: record.id } })).pop()['economicEvent']
    }),

    fulfills: injectTypename('Commitment', async (record: Fulfillment): Promise<Commitment> => {
      return (await readCommitments({ params: { fulfilledBy: record.id } })).pop()['commitment']
    }),
  }
}
