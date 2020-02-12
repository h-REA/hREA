/**
 * Mutations for manipulating process specification
 *
 * @package: HoloREA
 * @since:   2019-09-12
 */

import { zomeFunction } from '../connection'
import {
  ProposedIntentResponse,
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const createHandler = zomeFunction('proposal', 'proposed_intent', 'create_proposed_intent')
const deleteHandler = zomeFunction('proposal', 'proposed_intent', 'delete_proposed_intent')

// CREATE
type createHandler = (root: any, args) => Promise<ProposedIntentResponse>

export const createProposedIntent: createHandler = async (root, args) => {
  const adaptedArguments = {
    proposed_intent: {
      publishedIn: args.publishedIn,
      publishes: args.publishes
    }
  }
  return createHandler(adaptedArguments)
}

// DELETE
type deleteHandler = (root: any, args: { id: string }) => Promise<boolean>

export const deleteProposedIntent: deleteHandler = async (root, args) => {
  return deleteHandler({ address: args.id })
}
