/**
 * Top-level queries relating to ProcessSpecification
 *
 * @package: HoloREA
 * @since:   2019-09-12
 */

import { DNAIdMappings, ReadParams } from '../types.js'
import { mapZomeFn } from '../connection.js'

import {
  ProcessSpecification, ProcessSpecificationConnection, ProcessSpecificationResponse,
} from '@valueflows/vf-graphql'
import { PagingParams } from '../resolvers/zomeSearchInputTypes.js'

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const readOne = mapZomeFn<ReadParams, ProcessSpecificationResponse>(dnaConfig, conductorUri, 'specification', 'process_specification', 'get_process_specification')
  const readAll = mapZomeFn<PagingParams, ProcessSpecificationConnection>(dnaConfig, conductorUri, 'specification', 'process_specification_index', 'read_all_process_specifications')

  return {
    processSpecification: async (root, args): Promise<ProcessSpecification> => {
      return (await readOne({ address: args.id })).processSpecification
    },
    processSpecifications: async (root, args: PagingParams): Promise<ProcessSpecificationConnection> => {
      return await readAll(args)
    },
  }
}
