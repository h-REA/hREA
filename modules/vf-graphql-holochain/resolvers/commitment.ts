/**
 * Resolvers for Commitment fields
 *
 * @package: HoloREA
 * @since:   2019-08-28
 */

import { zomeFunction } from '../connection'

import {
  Commitment,
  Fulfillment,
  Satisfaction,
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const readFulfillments = zomeFunction('planning', 'fulfillment', 'query_fulfillments')
const readSatisfactions = zomeFunction('planning', 'satisfaction', 'query_satisfactions')

export const fulfilledBy = async (record: Commitment): Promise<[Fulfillment]> => {
  return (await (await readFulfillments)({ commitment: record.id })).map(({ fulfillment }) => fulfillment)
}

export const satisfies = async (record: Commitment): Promise<[Satisfaction]> => {
  return (await (await readSatisfactions)({ satisfied_by: record.id })).map(({ satisfaction }) => satisfaction)
}
