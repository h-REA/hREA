/**
 * Mutations for manipulating resource specification
 *
 * @package: HoloREA
 * @since:   2019-09-12
 */

import { ByRevision, DNAIdMappings } from '../types.js'
import { mapZomeFn } from '../connection.js'
import { deleteHandler } from './'

import {
  ResourceSpecificationCreateParams,
  ResourceSpecificationUpdateParams,
  ResourceSpecificationResponse,
} from '@valueflows/vf-graphql'

export interface CreateArgs {
  resourceSpecification: ResourceSpecificationCreateParams,
}
export type createHandler = (root: any, args: CreateArgs) => Promise<ResourceSpecificationResponse>

export interface UpdateArgs {
  resourceSpecification: ResourceSpecificationUpdateParams,
}
export type updateHandler = (root: any, args: UpdateArgs) => Promise<ResourceSpecificationResponse>

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const runCreate = mapZomeFn<CreateArgs, ResourceSpecificationResponse>(dnaConfig, conductorUri, 'specification', 'resource_specification', 'create_resource_specification')
  const runUpdate = mapZomeFn<UpdateArgs, ResourceSpecificationResponse>(dnaConfig, conductorUri, 'specification', 'resource_specification', 'update_resource_specification')
  const runDelete = mapZomeFn<ByRevision, boolean>(dnaConfig, conductorUri, 'specification', 'resource_specification', 'delete_resource_specification')

  const createResourceSpecification: createHandler = async (root, args) => {
    return runCreate(args)
  }

  const updateResourceSpecification: updateHandler = async (root, args) => {
    return runUpdate(args)
  }

  const deleteResourceSpecification: deleteHandler = async (root, args) => {
    return runDelete(args)
  }

  return {
    createResourceSpecification,
    updateResourceSpecification,
    deleteResourceSpecification,
  }
}
