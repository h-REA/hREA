/**
 * Top-level queries relating to ProcessSpecification
 *
 * @package: HoloREA
 * @since:   2019-09-12
 */

import { DNAIdMappings, ReadParams } from '../types.js'
import { mapZomeFn } from '../connection.js'

import {
  ProcessSpecification, ProcessSpecificationResponse,
} from '@valueflows/vf-graphql'

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const readOne = mapZomeFn<ReadParams, ProcessSpecificationResponse>(dnaConfig, conductorUri, 'specification', 'process_specification', 'get_process_specification')

  return {
    processSpecification: async (root, args): Promise<ProcessSpecification> => {
      return (await readOne({ address: args.id })).processSpecification
    },
  }
}
