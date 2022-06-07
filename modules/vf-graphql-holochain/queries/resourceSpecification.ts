/**
 * Top-level queries relating to ResourceSpecification
 *
 * @package: HoloREA
 * @since:   2019-09-12
 */

import { DNAIdMappings, ReadParams } from '../types'
import { mapZomeFn } from '../connection'

import {
  ResourceSpecification, ResourceSpecificationResponse,
} from '@valueflows/vf-graphql'

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const readOne = mapZomeFn<ReadParams, ResourceSpecificationResponse>(dnaConfig, conductorUri, 'specification', 'resource_specification', 'get_resource_specification')

  return {
    resourceSpecification: async (root, args): Promise<ResourceSpecification> => {
      return (await readOne({ address: args.id })).resourceSpecification
    },
  }
}
