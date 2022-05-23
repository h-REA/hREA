/**
 * plan mutations
 *
 * @package: HoloREA
 * @since:   2019-05-27
 */

import { DNAIdMappings } from '../types'
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
  const runCreate = mapZomeFn(dnaConfig, conductorUri, 'planning', 'plan', 'create_plan')
  const runUpdate = mapZomeFn(dnaConfig, conductorUri, 'planning', 'plan', 'update_plan')
  const runDelete = mapZomeFn(dnaConfig, conductorUri, 'planning', 'plan', 'delete_plan')

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

