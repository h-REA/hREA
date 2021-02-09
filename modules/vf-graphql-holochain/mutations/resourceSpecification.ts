/**
 * Mutations for manipulating resource specification
 *
 * @package: HoloREA
 * @since:   2019-09-12
 */

import { DNAIdMappings } from '../types'
import { mapZomeFn } from '../connection'
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

export default (dnaConfig: DNAIdMappings, conductorUri?: string) => {
  const runCreate = mapZomeFn(dnaConfig, conductorUri, 'specification', 'resource_specification', 'create_resource_specification')
  const runUpdate = mapZomeFn(dnaConfig, conductorUri, 'specification', 'resource_specification', 'update_resource_specification')
  const runDelete = mapZomeFn(dnaConfig, conductorUri, 'specification', 'resource_specification', 'delete_resource_specification')

  const createResourceSpecification: createHandler = async (root, args) => {
    return runCreate({
      resource_specification: args.resourceSpecification
    })
  }

  const updateResourceSpecification: updateHandler = async (root, args) => {
    return runUpdate({
      resource_specification: args.resourceSpecification
    })
  }

  const deleteResourceSpecification: deleteHandler = async (root, args) => {
    return runDelete({ address: args.id })
  }

  return {
    createResourceSpecification,
    updateResourceSpecification,
    deleteResourceSpecification,
  }
}
