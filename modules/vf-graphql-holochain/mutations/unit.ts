/**
 * Mutations for manipulating unit
 *
 * @package: HoloREA
 * @since:   2019-09-12
 */

import { zomeFunction } from '../connection'
import {
  UnitCreateParams,
  UnitUpdateParams,
  UnitResponse,
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const createHandler = zomeFunction('specification', 'unit', 'create_unit')
const updateHandler = zomeFunction('specification', 'unit', 'update_unit')
const deleteHandler = zomeFunction('specification', 'unit', 'delete_unit')

// CREATE
interface CreateArgs {
  unit: UnitCreateParams,
}
type createHandler = (root: any, args: CreateArgs) => Promise<UnitResponse>

export const createUnit: createHandler = async (root, args) => {
  return createHandler(args)
}

// UPDATE
interface UpdateArgs {
    unit: UnitUpdateParams,
}
type updateHandler = (root: any, args: UpdateArgs) => Promise<UnitResponse>

export const updateUnit: updateHandler = async (root, args) => {
  return updateHandler(args)
}

// DELETE
type deleteHandler = (root: any, args: { id: string }) => Promise<boolean>

export const deleteUnit: deleteHandler = async (root, args) => {
  return deleteHandler({ address: args.id })
}
