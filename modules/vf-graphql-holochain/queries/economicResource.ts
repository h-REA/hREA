/**
 * Top-level queries relating to Economic Resources
 *
 * @package: HoloREA
 * @since:   2019-10-31
 */

import { DNAIdMappings } from '../types'
import { mapZomeFn } from '../connection'

import {
  EconomicResource,
  EconomicResourceConnection,
} from '@valueflows/vf-graphql'

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const readOne = mapZomeFn(dnaConfig, conductorUri, 'observation', 'economic_resource', 'get_economic_resource')
  const readAll = mapZomeFn(dnaConfig, conductorUri, 'observation', 'economic_resource_index', 'get_all_economic_resources')

  return {
    economicResource: async (root, args): Promise<EconomicResource> => {
      return (await readOne({ address: args.id })).economicResource
    },

    economicResources: async (root, args): Promise<EconomicResourceConnection> => {
      return await readAll(null)
    },
  }
}
