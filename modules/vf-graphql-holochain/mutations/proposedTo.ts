/**
 * Mutations for manipulating process specification
 *
 * @package: HoloREA
 * @since:   2019-09-12
 */

import { zomeFunction } from '../connection'
import {
  ProposedToCreateParams,
  ProposedToUpdateParams,
  ProposedToResponse,
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const createHandler = zomeFunction('proposal', 'proposed_to', 'create_proposed_to')
const updateHandler = zomeFunction('proposal', 'proposed_to', 'update_proposed_to')
const deleteHandler = zomeFunction('proposal', 'proposed_to', 'delete_proposed_to')

// CREATE
interface CreateArgs {
  proposedTo: ProposedToCreateParams,
}
type createHandler = (root: any, args: CreateArgs) => Promise<ProposedToResponse>

export const createProposedTo: createHandler = async (root, args) => {
  const adaptedArguments = {
    proposed_to: args.proposedTo
  }
  return createHandler(adaptedArguments)
}

// UPDATE
interface UpdateArgs {
    proposedTo: ProposedToUpdateParams,
}
type updateHandler = (root: any, args: UpdateArgs) => Promise<ProposedToResponse>

export const updateProposedTo: updateHandler = async (root, args) => {
  const adaptedArguments = {
    proposed_to: args.proposedTo
  }
  return updateHandler(adaptedArguments)
}

// DELETE
type deleteHandler = (root: any, args: { id: string }) => Promise<boolean>

export const deleteProposedTo: deleteHandler = async (root, args) => {
  return deleteHandler({ address: args.id })
}
