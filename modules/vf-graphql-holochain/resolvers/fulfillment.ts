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
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const readEvents = zomeFunction('a1_observation', 'economic_event', 'query_events')
const readCommitments = zomeFunction('a1_planning', 'commitment', 'query_commitments')

export const fulfilledBy = async ({ id }: Fulfillment) => {
  const resp = await (await readEvents)({ fulfills: id })
  return resp
}

export const fulfills = async ({ id }: Fulfillment) => {
  const resp = await (await readCommitments)({ fulfilledBy: id })
  return resp
}
