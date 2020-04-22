/**
 * Mutations for manipulating process specification
 *
 * @package: HoloREA
 * @since:   2019-09-12
 */

import { zomeFunction } from '../connection'
import {
  ProposalCreateParams,
  ProposalUpdateParams,
  ProposalResponse,
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const createHandler = zomeFunction('proposal', 'proposal', 'create_proposal')
const updateHandler = zomeFunction('proposal', 'proposal', 'update_proposal')
const deleteHandler = zomeFunction('proposal', 'proposal', 'delete_proposal')

// CREATE
interface CreateArgs {
  proposal: ProposalCreateParams,
}
type createHandler = (root: any, args: CreateArgs) => Promise<ProposalResponse>

export const createProposal: createHandler = async (root, args) => {
  const adaptedArguments = {
    proposal: args.proposal
  }
  return createHandler(adaptedArguments)
}

// UPDATE
interface UpdateArgs {
    proposal: ProposalUpdateParams,
}
type updateHandler = (root: any, args: UpdateArgs) => Promise<ProposalResponse>

export const updateProposal: updateHandler = async (root, args) => {
  const adaptedArguments = {
    proposal: args.proposal
  }
  return updateHandler(adaptedArguments)
}

// DELETE
type deleteHandler = (root: any, args: { id: string }) => Promise<boolean>

export const deleteProposal: deleteHandler = async (root, args) => {
  return deleteHandler({ address: args.id })
}
