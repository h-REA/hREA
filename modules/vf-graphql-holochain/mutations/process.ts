/**
 * Mutations for manipulating processes
 *
 * @package: HoloREA
 * @since:   2019-09-12
 */

import { DNAIdMappings } from '../types'
import { mapZomeFn } from '../connection'
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

export default (dnaConfig: DNAIdMappings, conductorUri?: string, traceAppSignals?: AppSignalCb) => {
  const createHandler = mapZomeFn(dnaConfig, conductorUri, 'observation', 'process', 'create_process')
  const updateHandler = mapZomeFn(dnaConfig, conductorUri, 'observation', 'process', 'update_process')
  const deleteHandler = mapZomeFn(dnaConfig, conductorUri, 'observation', 'process', 'delete_process')

  const createProcess: createHandler = async (root, args) => {
    return createHandler(args)
  }

  const updateProcess: updateHandler = async (root, args) => {
    return updateHandler(args)
  }

  const deleteProcess: deleteHandler = async (root, args) => {
    return deleteHandler({ address: args.id })
  }

  return {
    createProcess,
    updateProcess,
    deleteProcess,
  }
}
