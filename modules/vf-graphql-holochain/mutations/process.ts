/**
 * Mutations for manipulating processes
 *
 * @package: HoloREA
 * @since:   2019-09-12
 */

import { zomeFunction } from '../connection'
import {
  ProcessCreateParams,
  ProcessUpdateParams,
  ProcessResponse,
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const createHandler = zomeFunction('observation', 'process', 'create_process')
const updateHandler = zomeFunction('observation', 'process', 'update_process')
const deleteHandler = zomeFunction('observation', 'process', 'delete_process')

// CREATE
interface CreateArgs {
  process: ProcessCreateParams,
}
type createHandler = (root: any, args: CreateArgs) => Promise<ProcessResponse>

export const createProcess: createHandler = async (root, args) => {
  return createHandler(args)
}

// UPDATE
interface UpdateArgs {
  process: ProcessUpdateParams,
}
type updateHandler = (root: any, args: UpdateArgs) => Promise<ProcessResponse>

export const updateProcess: updateHandler = async (root, args) => {
  return updateHandler(args)
}

// DELETE
type deleteHandler = (root: any, args: { id: string }) => Promise<boolean>

export const deleteProcess: deleteHandler = async (root, args) => {
  return deleteHandler({ address: args.id })
}
