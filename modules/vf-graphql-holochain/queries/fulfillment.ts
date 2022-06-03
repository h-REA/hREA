/**
 * Top-level queries relating to Fulfillments
 *
 * @package: HoloREA
 * @since:   2019-08-28
 */

import { DNAIdMappings } from '../types'
import { mapZomeFn } from '../connection'

import {
  Fulfillment, FulfillmentResponse,
} from '@valueflows/vf-graphql'

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const readOne = mapZomeFn<ReadParams, FulfillmentResponse>(dnaConfig, conductorUri, 'planning', 'fulfillment', 'get_fulfillment')

  return {
    fulfillment: async (root, args): Promise<Fulfillment> => {
      return (await (await readOne)({ address: args.id })).fulfillment
    },
  }
}
