/**
 * Top-level queries relating to Fulfillments
 *
 * @package: HoloREA
 * @since:   2019-08-28
 */

import { DNAIdMappings, ReadParams } from '../types.js'
import { mapZomeFn } from '../connection.js'

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
