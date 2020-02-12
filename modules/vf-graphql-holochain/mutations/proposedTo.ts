/**
 * Mutations for manipulating process specification
 *
 * @package: HoloREA
 * @since:   2019-09-12
 */

import { zomeFunction } from '../connection'
import {
  ProposedToResponse,
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const createHandler = zomeFunction('proposal', 'proposed_to', 'create_proposed_to')
const deleteHandler = zomeFunction('proposal', 'proposed_to', 'delete_proposed_to')

// CREATE
type createHandler = (root: any, args) => Promise<ProposedToResponse>

export const createProposedTo: createHandler = async (root, args) => {
  const adaptedArguments = {
    proposed_to: {
      proposed: args.proposed,
      proposedTo: args.proposedTo
    }
  }
  return createHandler(adaptedArguments)
}

// DELETE
type deleteHandler = (root: any, args: { id: string }) => Promise<boolean>

export const deleteProposedTo: deleteHandler = async (root, args) => {
  return deleteHandler({ address: args.id })
}
