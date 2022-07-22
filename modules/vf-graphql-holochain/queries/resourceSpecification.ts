/**
 * Top-level queries relating to ResourceSpecification
 *
 * @package: HoloREA
 * @since:   2019-09-12
 */

import { DNAIdMappings, ReadParams } from '../types.js'
import { mapZomeFn } from '../connection.js'

import {
  ResourceSpecification, ResourceSpecificationConnection, ResourceSpecificationResponse,
} from '@valueflows/vf-graphql'
import { PagingParams } from '../resolvers/zomeSearchInputTypes.js'

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const readOne = mapZomeFn<ReadParams, ResourceSpecificationResponse>(dnaConfig, conductorUri, 'specification', 'resource_specification', 'get_resource_specification')
  const readAll = mapZomeFn<PagingParams, ResourceSpecificationConnection>(dnaConfig, conductorUri, 'specification', 'resource_specification_index', 'read_all_resource_specifications')

  return {
    resourceSpecification: async (root, args): Promise<ResourceSpecification> => {
      return (await readOne({ address: args.id })).resourceSpecification
    },
    resourceSpecifications: async (root, args: PagingParams): Promise<ResourceSpecificationConnection> => {
      return await readAll(args)
    },
  }
}
