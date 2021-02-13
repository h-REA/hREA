/**
 * Mutations for manipulating process specification
 *
 * @package: HoloREA
 * @since:   2019-09-12
 */

import { DNAIdMappings } from '../types'
import { mapZomeFn } from '../connection'
import { deleteHandler } from './'

import {
  ProcessSpecificationCreateParams,
  ProcessSpecificationUpdateParams,
  ProcessSpecificationResponse,
} from '@valueflows/vf-graphql'

export interface CreateArgs {
  processSpecification: ProcessSpecificationCreateParams,
}
export type createHandler = (root: any, args: CreateArgs) => Promise<ProcessSpecificationResponse>

export interface UpdateArgs {
    processSpecification: ProcessSpecificationUpdateParams,
}
export type updateHandler = (root: any, args: UpdateArgs) => Promise<ProcessSpecificationResponse>

export default (dnaConfig: DNAIdMappings, conductorUri?: string, traceAppSignals?: AppSignalCb) => {
  const runCreate = mapZomeFn(dnaConfig, conductorUri, 'specification', 'process_specification', 'create_process_specification')
  const runUpdate = mapZomeFn(dnaConfig, conductorUri, 'specification', 'process_specification', 'update_process_specification')
  const runDelete = mapZomeFn(dnaConfig, conductorUri, 'specification', 'process_specification', 'delete_process_specification')

  const createProcessSpecification: createHandler = async (root, args) => {
    return runCreate({
      process_specification: args.processSpecification
    })
  }

  const updateProcessSpecification: updateHandler = async (root, args) => {
    return runUpdate({
      process_specification: args.processSpecification
    })
  }

  const deleteProcessSpecification: deleteHandler = async (root, args) => {
    return runDelete({ address: args.id })
  }

  return {
    createProcessSpecification,
    updateProcessSpecification,
    deleteProcessSpecification,
  }
}
