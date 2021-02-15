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
} from '@valueflows/vf-graphql'

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const readOne = mapZomeFn(dnaConfig, conductorUri, 'observation', 'economic_resource', 'get_resource')
  const readAll = mapZomeFn(dnaConfig, conductorUri, 'observation', 'economic_resource', 'get_all_resources')

  return {
    economicResource: async (root, args): Promise<EconomicResource> => {
      return (await readOne({ address: args.id })).economicResource
    },

    economicResources: async (root, args): Promise<EconomicResource[]> => {
      return (await readAll({})).map(e => e.economicResource)
    },
  }
}
