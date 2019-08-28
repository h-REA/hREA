/**
 * Fulfillment mutations
 *
 * @package: HoloREA
 * @since:   2019-08-28
 */

import { zomeFunction } from '../connection'
import {
  FulfillmentCreateParams,
  FulfillmentUpdateParams,
  FulfillmentResponse,
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const createHandler = zomeFunction('a1_planning', 'fulfillment', 'create_fulfillment')
const updateHandler = zomeFunction('a1_planning', 'fulfillment', 'update_fulfillment')

// CREATE
interface CreateArgs {
  fulfillment: FulfillmentCreateParams,
}
type createHandler = (root: any, args: CreateArgs) => Promise<FulfillmentResponse>

export const createFulfillment: createHandler = async (root, args) => {
  return (await createHandler)(args)
}

// UPDATE
interface UpdateArgs {
  fulfillment: FulfillmentUpdateParams,
}
type updateHandler = (root: any, args: UpdateArgs) => Promise<FulfillmentResponse>

export const updateFulfillment: updateHandler = async (root, args) => {
  return (await updateHandler)(args)
}
