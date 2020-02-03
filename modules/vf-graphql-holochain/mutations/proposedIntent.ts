/**
 * Mutations for manipulating process specification
 *
 * @package: HoloREA
 * @since:   2019-09-12
 */

import { zomeFunction } from '../connection'
import {
  ProposedIntentCreateParams,
  ProposedIntentUpdateParams,
  ProposedIntentResponse,
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const createHandler = zomeFunction('proposal', 'proposed_intent', 'create_proposed_intent')
const updateHandler = zomeFunction('proposal', 'proposed_intent', 'update_proposed_intent')
const deleteHandler = zomeFunction('proposal', 'proposed_intent', 'delete_proposed_intent')

// CREATE
interface CreateArgs {
  proposedIntent: ProposedIntentCreateParams,
}
type createHandler = (root: any, args: CreateArgs) => Promise<ProposedIntentResponse>

export const createProposedIntent: createHandler = async (root, args) => {
  const adaptedArguments = {
    proposed_intent: args.proposedIntent
  }
  return createHandler(adaptedArguments)
}

// UPDATE
interface UpdateArgs {
    proposedIntent: ProposedIntentUpdateParams,
}
type updateHandler = (root: any, args: UpdateArgs) => Promise<ProposedIntentResponse>

export const updateProposedIntent: updateHandler = async (root, args) => {
  const adaptedArguments = {
    proposed_intent: args.proposedIntent
  }
  return updateHandler(adaptedArguments)
}

// DELETE
type deleteHandler = (root: any, args: { id: string }) => Promise<boolean>

export const deleteProposedIntent: deleteHandler = async (root, args) => {
  return deleteHandler({ address: args.id })
}
