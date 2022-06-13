/**
 * Fulfillment mutations
 *
 * @package: HoloREA
 * @since:   2019-08-28
 */

import { ByRevision, DNAIdMappings } from '../types.js'
import { mapZomeFn } from '../connection.js'
import { deleteHandler } from './'

import {
  FulfillmentCreateParams,
  FulfillmentUpdateParams,
  FulfillmentResponse,
} from '@valueflows/vf-graphql'

export interface CreateArgs {
  fulfillment: FulfillmentCreateParams,
}
export type createHandler = (root: any, args: CreateArgs) => Promise<FulfillmentResponse>

export interface UpdateArgs {
  fulfillment: FulfillmentUpdateParams,
}
export type updateHandler = (root: any, args: UpdateArgs) => Promise<FulfillmentResponse>

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const runCreate = mapZomeFn<CreateArgs, FulfillmentResponse>(dnaConfig, conductorUri, 'planning', 'fulfillment', 'create_fulfillment')
  const runUpdate = mapZomeFn<UpdateArgs, FulfillmentResponse>(dnaConfig, conductorUri, 'planning', 'fulfillment', 'update_fulfillment')
  const runDelete = mapZomeFn<ByRevision, boolean>(dnaConfig, conductorUri, 'planning', 'fulfillment', 'delete_fulfillment')

  const createFulfillment: createHandler = async (root, args) => {
    return runCreate(args)
  }

  const updateFulfillment: updateHandler = async (root, args) => {
    return runUpdate(args)
  }

  const deleteFulfillment: deleteHandler = async (root, args) => {
    return runDelete(args)
  }

  return {
    createFulfillment,
    updateFulfillment,
    deleteFulfillment,
  }
}
