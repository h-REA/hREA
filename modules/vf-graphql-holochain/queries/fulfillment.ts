/**
 * Top-level queries relating to Fulfillments
 *
 * @package: HoloREA
 * @since:   2019-08-28
 */

import { DNAIdMappings, ReadParams } from '../types.js'
import { mapZomeFn } from '../connection.js'

import {
  Fulfillment, FulfillmentConnection, FulfillmentResponse,
} from '@valueflows/vf-graphql'
import { PagingParams } from '../resolvers/zomeSearchInputTypes.js'

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const readOne = mapZomeFn<ReadParams, FulfillmentResponse>(dnaConfig, conductorUri, 'planning', 'fulfillment', 'get_fulfillment')
  const readAll = mapZomeFn<PagingParams, FulfillmentConnection>(dnaConfig, conductorUri, 'planning', 'fulfillment_index', 'read_all_fulfillments')

  return {
    fulfillment: async (root, args): Promise<Fulfillment> => {
      return (await (await readOne)({ address: args.id })).fulfillment
    },
    fulfillments: async (root, args: PagingParams): Promise<FulfillmentConnection> => {
      return await readAll(args)
    },
  }
}
