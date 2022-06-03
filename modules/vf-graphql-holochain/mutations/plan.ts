/**
 * plan mutations
 *
 * @package: hREA
 * @since:   2022-05-23
 */

import { ByRevision, DNAIdMappings } from '../types'
import { mapZomeFn } from '../connection'
import { deleteHandler } from './'

import {
  PlanCreateParams,
  PlanUpdateParams,
  PlanResponse,
} from '@valueflows/vf-graphql'

export interface CreateArgs {
    plan: PlanCreateParams,
}
export type createHandler = (root: any, args: CreateArgs) => Promise<PlanResponse>

export interface UpdateArgs {
  plan: PlanUpdateParams,
}
export type updateHandler = (root: any, args: UpdateArgs) => Promise<PlanResponse>

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const runCreate = mapZomeFn<CreateArgs, PlanResponse>(dnaConfig, conductorUri, 'plan', 'plan', 'create_plan')
  const runUpdate = mapZomeFn<UpdateArgs, PlanResponse>(dnaConfig, conductorUri, 'plan', 'plan', 'update_plan')
  const runDelete = mapZomeFn<ByRevision, boolean>(dnaConfig, conductorUri, 'plan', 'plan', 'delete_plan')

  const createPlan: createHandler = async (root, args) => {
    return runCreate(args)
  }

  const updatePlan: updateHandler = async (root, args) => {
    return runUpdate(args)
  }

  const deletePlan: deleteHandler = async (root, args) => {
    return runDelete(args)
  }

  return {
    createPlan,
    updatePlan,
    deletePlan,
  }
}

