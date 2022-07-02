/**
 * Top-level queries relating to Economic Resources
 *
 * @package: HoloREA
 * @since:   2019-10-31
 */

import { DNAIdMappings, EconomicResourceAddress, ReadParams } from '../types.js'
import { mapZomeFn } from '../connection.js'

import {
  EconomicResource,
  EconomicResourceConnection,
  EconomicResourceResponse,
} from '@valueflows/vf-graphql'
import { PagingParams } from '../resolvers/zomeSearchInputTypes.js'

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const readOne = mapZomeFn<ReadParams, EconomicResourceResponse>(dnaConfig, conductorUri, 'observation', 'economic_resource', 'get_economic_resource')
  const readAll = mapZomeFn<PagingParams, EconomicResourceConnection>(dnaConfig, conductorUri, 'observation', 'economic_resource_index', 'read_all_economic_resources')

  return {
    economicResource: async (root, args: { id: EconomicResourceAddress }): Promise<EconomicResource> => {
      return (await readOne({ address: args.id })).economicResource
    },

    economicResources: async (root, args: PagingParams): Promise<EconomicResourceConnection> => {
      return await readAll(args)
    },
  }
}
