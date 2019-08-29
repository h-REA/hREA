/**
 * Resolvers for Fulfillment fields
 *
 * @package: HoloREA
 * @author:  pospi <pospi@spadgos.com>
 * @since:   2019-08-27
 */

import { zomeFunction } from '../connection'

import {
  Fulfillment,
  EconomicEvent,
  Commitment,
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const readEvents = zomeFunction('a1_observation', 'economic_event', 'query_events')
const readCommitments = zomeFunction('a1_planning', 'commitment', 'query_commitments')

export const fulfilledBy = async (record: Fulfillment): Promise<EconomicEvent> => {
  return (await (await readEvents)({ fulfills: record.id })).map(({ economicEvent }) => economicEvent)[0]
}

export const fulfills = async (record: Fulfillment): Promise<Commitment> => {
  return (await (await readCommitments)({ fulfilled_by: record.id })).map(({ commitment }) => commitment)[0]
}
