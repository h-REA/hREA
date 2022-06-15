/**
 * Mutations for manipulating processes
 *
 * @package: HoloREA
 * @since:   2019-09-12
 */

import { ByRevision, DNAIdMappings } from '../types.js'
import { mapZomeFn } from '../connection.js'
import { deleteHandler } from './'

import {
  ProcessCreateParams,
  ProcessUpdateParams,
  ProcessResponse,
} from '@valueflows/vf-graphql'

export interface CreateArgs {
  process: ProcessCreateParams,
}
export type createHandler = (root: any, args: CreateArgs) => Promise<ProcessResponse>

export interface UpdateArgs {
  process: ProcessUpdateParams,
}
export type updateHandler = (root: any, args: UpdateArgs) => Promise<ProcessResponse>

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const createHandler = mapZomeFn<CreateArgs, ProcessResponse>(dnaConfig, conductorUri, 'observation', 'process', 'create_process')
  const updateHandler = mapZomeFn<UpdateArgs, ProcessResponse>(dnaConfig, conductorUri, 'observation', 'process', 'update_process')
  const deleteHandler = mapZomeFn<ByRevision, boolean>(dnaConfig, conductorUri, 'observation', 'process', 'delete_process')

  const createProcess: createHandler = async (root, args) => {
    return createHandler(args)
  }

  const updateProcess: updateHandler = async (root, args) => {
    return updateHandler(args)
  }

  const deleteProcess: deleteHandler = async (root, args) => {
    return deleteHandler(args)
  }

  return {
    createProcess,
    updateProcess,
    deleteProcess,
  }
}
