/**
 * Fulfillment mutations
 *
 * @package: HoloREA
 * @since:   2019-08-28
 */

import { DNAIdMappings } from '../types'
import { mapZomeFn } from '../connection'
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

export default (dnaConfig: DNAIdMappings, conductorUri?: string, traceAppSignals?: AppSignalCb) => {
  const runCreate = mapZomeFn(dnaConfig, conductorUri, 'planning', 'fulfillment', 'create_fulfillment')
  const runUpdate = mapZomeFn(dnaConfig, conductorUri, 'planning', 'fulfillment', 'update_fulfillment')
  const runDelete = mapZomeFn(dnaConfig, conductorUri, 'planning', 'fulfillment', 'delete_fulfillment')

  const createFulfillment: createHandler = async (root, args) => {
    return runCreate(args)
  }

  const updateFulfillment: updateHandler = async (root, args) => {
    return runUpdate(args)
  }

  const deleteFulfillment: deleteHandler = async (root, args) => {
    return runDelete({ address: args.id })
  }

  return {
    createFulfillment,
    updateFulfillment,
    deleteFulfillment,
  }
}
