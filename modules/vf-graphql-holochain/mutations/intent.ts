/**
 * Intent mutations
 *
 * @package: HoloREA
 * @since:   2019-08-31
 */

import { zomeFunction } from '../connection'
import {
  IntentCreateParams,
  IntentUpdateParams,
  IntentResponse,
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const createHandler = zomeFunction('planning', 'intent', 'create_intent')
const updateHandler = zomeFunction('planning', 'intent', 'update_intent')
const deleteHandler = zomeFunction('planning', 'intent', 'delete_intent')

// CREATE
interface CreateArgs {
  intent: IntentCreateParams,
}
type createHandler = (root: any, args: CreateArgs) => Promise<IntentResponse>

export const createIntent: createHandler = async (root, args) => {
  return (await createHandler)(args)
}

// UPDATE
interface UpdateArgs {
  intent: IntentUpdateParams,
}
type updateHandler = (root: any, args: UpdateArgs) => Promise<IntentResponse>

export const updateIntent: updateHandler = async (root, args) => {
  return (await updateHandler)(args)
}

// DELETE
type deleteHandler = (root: any, args: { id: string }) => Promise<boolean>

export const deleteIntent: deleteHandler = async (root, args) => {
  return (await deleteHandler)({ address: args.id })
}
