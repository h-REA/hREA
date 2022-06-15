/**
 * Mutations for manipulating unit
 *
 * @package: HoloREA
 * @since:   2019-09-12
 */

import { ByRevision, DNAIdMappings } from '../types.js'
import { mapZomeFn } from '../connection.js'
import { deleteHandler } from './'

import {
  UnitCreateParams,
  UnitUpdateParams,
  UnitResponse,
} from '@valueflows/vf-graphql'

export interface CreateArgs {
  unit: UnitCreateParams,
}
export type createHandler = (root: any, args: CreateArgs) => Promise<UnitResponse>

export interface UpdateArgs {
    unit: UnitUpdateParams,
}
export type updateHandler = (root: any, args: UpdateArgs) => Promise<UnitResponse>

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const runCreate = mapZomeFn<CreateArgs, UnitResponse>(dnaConfig, conductorUri, 'specification', 'unit', 'create_unit')
  const runUpdate = mapZomeFn<UpdateArgs, UnitResponse>(dnaConfig, conductorUri, 'specification', 'unit', 'update_unit')
  const runDelete = mapZomeFn<ByRevision, boolean>(dnaConfig, conductorUri, 'specification', 'unit', 'delete_unit')

  const createUnit: createHandler = async (root, args) => {
    return runCreate(args)
  }

  const updateUnit: updateHandler = async (root, args) => {
    return runUpdate(args)
  }

  const deleteUnit: deleteHandler = async (root, args) => {
    return runDelete(args)
  }

  return {
    createUnit,
    updateUnit,
    deleteUnit,
  }
}
