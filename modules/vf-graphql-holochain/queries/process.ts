/**
 * Top-level queries relating to Processes
 *
 * @package: HoloREA
 * @since:   2019-09-12
 */

import { DNAIdMappings, ReadParams } from '../types.js'
import { mapZomeFn } from '../connection.js'

import {
  Process, ProcessConnection, ProcessResponse,
} from '@valueflows/vf-graphql'
import { PagingParams } from '../resolvers/zomeSearchInputTypes.js'

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const readOne = mapZomeFn<ReadParams, ProcessResponse>(dnaConfig, conductorUri, 'observation', 'process', 'get_process')
  const readAll = mapZomeFn<PagingParams, ProcessConnection>(dnaConfig, conductorUri, 'observation', 'process_index', 'read_all_processs')

  return {
    process: async (root, args): Promise<Process> => {
      return (await readOne({ address: args.id })).process
    },
    processes: async (root, args: PagingParams): Promise<ProcessConnection> => {
      return await readAll(args)
    },
  }
}
