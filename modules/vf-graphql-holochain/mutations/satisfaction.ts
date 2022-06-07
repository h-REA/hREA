/**
 * Satisfaction mutations
 *
 * @package: HoloREA
 * @since:   2019-08-31
 */

import { ByRevision, DNAIdMappings } from '../types'
import { mapZomeFn } from '../connection'
import { deleteHandler } from './'

import {
  SatisfactionCreateParams,
  SatisfactionUpdateParams,
  SatisfactionResponse,
} from '@valueflows/vf-graphql'

export interface CreateArgs {
  satisfaction: SatisfactionCreateParams,
}
export type createHandler = (root: any, args: CreateArgs) => Promise<SatisfactionResponse>

export interface UpdateArgs {
  satisfaction: SatisfactionUpdateParams,
}
export type updateHandler = (root: any, args: UpdateArgs) => Promise<SatisfactionResponse>

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const runCreate = mapZomeFn<CreateArgs, SatisfactionResponse>(dnaConfig, conductorUri, 'planning', 'satisfaction', 'create_satisfaction')
  const runUpdate = mapZomeFn<UpdateArgs, SatisfactionResponse>(dnaConfig, conductorUri, 'planning', 'satisfaction', 'update_satisfaction')
  const runDelete = mapZomeFn<ByRevision, boolean>(dnaConfig, conductorUri, 'planning', 'satisfaction', 'delete_satisfaction')

  const createSatisfaction: createHandler = async (root, args) => {
    return runCreate(args)
  }

  const updateSatisfaction: updateHandler = async (root, args) => {
    return runUpdate(args)
  }

  const deleteSatisfaction: deleteHandler = async (root, args) => {
    return runDelete(args)
  }

  return {
    createSatisfaction,
    updateSatisfaction,
    deleteSatisfaction,
  }
}
