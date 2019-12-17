/**
 * Mutations for manipulating process specification
 *
 * @package: HoloREA
 * @since:   2019-09-12
 */

import { zomeFunction } from '../connection'
import {
  ProcessSpecificationCreateParams,
  ProcessSpecificationUpdateParams,
  ProcessSpecificationResponse,
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const createHandler = zomeFunction('specification', 'process_specification', 'create_process_specification')
const updateHandler = zomeFunction('specification', 'process_specification', 'update_process_specification')
const deleteHandler = zomeFunction('specification', 'process_specification', 'delete_process_specification')

// CREATE
interface CreateArgs {
  processSpecification: ProcessSpecificationCreateParams,
}
type createHandler = (root: any, args: CreateArgs) => Promise<ProcessSpecificationResponse>

export const createProcessSpecification: createHandler = async (root, args) => {
  const adaptedArguments = {
    process_specification: args.processSpecification
  }
  return createHandler(adaptedArguments)
}

// UPDATE
interface UpdateArgs {
    processSpecification: ProcessSpecificationUpdateParams,
}
type updateHandler = (root: any, args: UpdateArgs) => Promise<ProcessSpecificationResponse>

export const updateProcessSpecification: updateHandler = async (root, args) => {
  const adaptedArguments = {
    process_specification: args.processSpecification
  }
  return updateHandler(adaptedArguments)
}

// DELETE
type deleteHandler = (root: any, args: { id: string }) => Promise<boolean>

export const deleteProcessSpecification: deleteHandler = async (root, args) => {
  return deleteHandler({ address: args.id })
}
