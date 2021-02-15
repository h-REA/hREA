/**
 * Mutations for manipulating process specification
 *
 * @package: HoloREA
 * @since:   2019-09-12
 */

import { DNAIdMappings } from '../types'
import { mapZomeFn } from '../connection'
import { deleteHandler } from './'

import {
  ProposedIntentResponse,
} from '@valueflows/vf-graphql'

export type createHandler = (root: any, args) => Promise<ProposedIntentResponse>

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const runCreate = mapZomeFn(dnaConfig, conductorUri, 'proposal', 'proposed_intent', 'create_proposed_intent')
  const runDelete = mapZomeFn(dnaConfig, conductorUri, 'proposal', 'proposed_intent', 'delete_proposed_intent')

  const proposeIntent: createHandler = async (root, args) => {
    return runCreate({ proposed_intent: args })
  }

  const deleteProposedIntent: deleteHandler = async (root, args) => {
    return runDelete({ address: args.id })
  }

  return {
    proposeIntent,
    deleteProposedIntent,
  }
}
