/**
 * Resolvers for EconomicEvent fields
 *
 * @package: HoloREA
 * @since:   2019-08-27
 */

import { zomeFunction } from '../connection'

import {
  EconomicEvent,
  Fulfillment,
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const readFulfillments = zomeFunction('a1_observation', 'fulfillment', 'query_fulfillments')

export const fulfills = async (record: EconomicEvent): Promise<[Fulfillment]> => {
  return (await (await readFulfillments)({ economic_event: record.id })).map(({ fulfillment }) => fulfillment)
}
