/**
 * Mutations for manipulating resource specification
 *
 * @package: HoloREA
 * @since:   2019-09-12
 */

import { zomeFunction } from '../connection'
import {
  ResourceSpecificationCreateParams,
  ResourceSpecificationUpdateParams,
  ResourceSpecificationResponse,
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const createHandler = zomeFunction('specification', 'resource_specification', 'create_resource_specification')
const updateHandler = zomeFunction('specification', 'resource_specification', 'update_resource_specification')
const deleteHandler = zomeFunction('specification', 'resource_specification', 'delete_resource_specification')

// CREATE
interface CreateArgs {
  resourceSpecification: ResourceSpecificationCreateParams,
}
type createHandler = (root: any, args: CreateArgs) => Promise<ResourceSpecificationResponse>

export const createResourceSpecification: createHandler = async (root, args) => {
  return createHandler({
    resource_specification: args.resourceSpecification
  })
}

// UPDATE
interface UpdateArgs {
    resourceSpecification: ResourceSpecificationUpdateParams,
}
type updateHandler = (root: any, args: UpdateArgs) => Promise<ResourceSpecificationResponse>

export const updateResourceSpecification: updateHandler = async (root, args) => {
  return updateHandler(args)
}

// DELETE
type deleteHandler = (root: any, args: { id: string }) => Promise<boolean>

export const deleteResourceSpecification: deleteHandler = async (root, args) => {
  return deleteHandler({ address: args.id })
}
