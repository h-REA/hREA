/**
 * Resolvers for Commitment fields
 *
 * @package: HoloREA
 * @since:   2019-08-28
 */

import { zomeFunction } from '../connection'

import {
  Commitment,
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const readFulfillments = zomeFunction('a1_planning', 'fulfillment', 'query_fulfillments')

export const fulfilledBy = async (record: Commitment) => {
  return (await (await readFulfillments)({ commitment: record.id })).map(({ fulfillment }) => fulfillment)
}
