/**
 * Resolvers for Fulfillment fields
 *
 * @package: HoloREA
 * @since:   2019-08-27
 */

import { zomeFunction } from '../connection'
import { injectTypename } from '../types'

import {
  Fulfillment,
  EconomicEvent,
  Commitment,
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const readEvents = zomeFunction('observation', 'economic_event', 'query_events')
const readCommitments = zomeFunction('planning', 'commitment', 'query_commitments')

export const fulfilledBy = injectTypename('EconomicEvent', async (record: Fulfillment): Promise<EconomicEvent> => {
  return (await readEvents({ params: { fulfills: record.id } })).pop()['economicEvent']
})

export const fulfills = injectTypename('Commitment', async (record: Fulfillment): Promise<Commitment> => {
  return (await readCommitments({ params: { fulfilledBy: record.id } })).pop()['commitment']
})
