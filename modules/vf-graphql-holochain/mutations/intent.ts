/**
 * Intent mutations
 *
 * @package: HoloREA
 * @since:   2019-08-31
 */

import { DNAIdMappings } from '../types'
import { mapZomeFn } from '../connection'
import { deleteHandler } from './'

import {
  IntentCreateParams,
  IntentUpdateParams,
  IntentResponse,
} from '@valueflows/vf-graphql'

export interface CreateArgs {
  intent: IntentCreateParams,
}
export type createHandler = (root: any, args: CreateArgs) => Promise<IntentResponse>

export interface UpdateArgs {
  intent: IntentUpdateParams,
}
export type updateHandler = (root: any, args: UpdateArgs) => Promise<IntentResponse>

export default (dnaConfig: DNAIdMappings, conductorUri?: string) => {
  const runCreate = mapZomeFn(dnaConfig, conductorUri, 'planning', 'intent', 'create_intent')
  const runUpdate = mapZomeFn(dnaConfig, conductorUri, 'planning', 'intent', 'update_intent')
  const runDelete = mapZomeFn(dnaConfig, conductorUri, 'planning', 'intent', 'delete_intent')

  const createIntent: createHandler = async (root, args) => {
    return runCreate(args)
  }

  const updateIntent: updateHandler = async (root, args) => {
    return runUpdate(args)
  }

  const deleteIntent: deleteHandler = async (root, args) => {
    return runDelete({ address: args.id })
  }

  return {
    createIntent,
    updateIntent,
    deleteIntent,
  }
}
