/**
 * Mutations for manipulating process specification
 *
 * @package: HoloREA
 * @since:   2019-09-12
 */

import { ByRevision, DNAIdMappings } from '../types'
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

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const runCreate = mapZomeFn<CreateArgs, ProcessSpecificationResponse>(dnaConfig, conductorUri, 'specification', 'process_specification', 'create_process_specification')
  const runUpdate = mapZomeFn<UpdateArgs, ProcessSpecificationResponse>(dnaConfig, conductorUri, 'specification', 'process_specification', 'update_process_specification')
  const runDelete = mapZomeFn<ByRevision, boolean>(dnaConfig, conductorUri, 'specification', 'process_specification', 'delete_process_specification')

  const createProcessSpecification: createHandler = async (root, args) => {
    return runCreate(args)
  }

  const updateProcessSpecification: updateHandler = async (root, args) => {
    return runUpdate(args)
  }

  const deleteProcessSpecification: deleteHandler = async (root, args) => {
    return runDelete(args)
  }

  return {
    createProcessSpecification,
    updateProcessSpecification,
    deleteProcessSpecification,
  }
}
