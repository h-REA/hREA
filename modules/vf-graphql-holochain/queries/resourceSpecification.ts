/**
 * Top-level queries relating to ResourceSpecification
 *
 * @package: HoloREA
 * @since:   2019-09-12
 */

import { DNAIdMappings } from '../types'
import { mapZomeFn } from '../connection'

import {
  ResourceSpecification,
} from '@valueflows/vf-graphql'

export default (dnaConfig?: DNAIdMappings, conductorUri?: string) => {
  const readOne = mapZomeFn(dnaConfig, conductorUri, 'specification', 'resource_specification', 'get_resource_specification')

  return {
    resourceSpecification: async (root, args): Promise<ResourceSpecification> => {
      return (await readOne({ address: args.id })).resourceSpecification
    },
  }
}
