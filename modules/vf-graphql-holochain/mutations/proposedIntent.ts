/**
 * Mutations for manipulating proposed intents
 *
 * @package: HoloREA
 * @since:   2019-09-12
 */

import { ByRevision, DNAIdMappings, IntentAddress, ProposalAddress } from '../types'
import { mapZomeFn } from '../connection'
import { deleteHandler } from './'

import {
  ProposedIntentResponse,
} from '@valueflows/vf-graphql'
import { CreateCloneCellRequest } from '@holochain/client'

export interface CreateParams {
  proposedIntent: CreateRequest,
}
interface CreateRequest {
  publishedIn: ProposalAddress,
  publishes: IntentAddress,
  reciprocal: boolean,
}
export type createHandler = (root: any, args: CreateRequest) => Promise<ProposedIntentResponse>

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const runCreate = mapZomeFn<CreateParams, ProposedIntentResponse>(dnaConfig, conductorUri, 'proposal', 'proposed_intent', 'create_proposed_intent')
  const runDelete = mapZomeFn<ByRevision, boolean>(dnaConfig, conductorUri, 'proposal', 'proposed_intent', 'delete_proposed_intent')

  const proposeIntent: createHandler = async (root, args) => {
    return runCreate({ proposedIntent: args })
  }

  const deleteProposedIntent: deleteHandler = async (root, args) => {
    return runDelete(args)
  }

  return {
    proposeIntent,
    deleteProposedIntent,
  }
}
