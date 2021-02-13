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
  ProposedToResponse,
} from '@valueflows/vf-graphql'

export type createHandler = (root: any, args) => Promise<ProposedToResponse>

export default (dnaConfig: DNAIdMappings, conductorUri?: string, traceAppSignals?: AppSignalCb) => {
  const runCreate = mapZomeFn(dnaConfig, conductorUri, 'proposal', 'proposed_to', 'create_proposed_to')
  const runDelete = mapZomeFn(dnaConfig, conductorUri, 'proposal', 'proposed_to', 'delete_proposed_to')

  const proposeTo: createHandler = async (root, args) => {
    return runCreate({ proposed_to: args })
  }

  const deleteProposedTo: deleteHandler = async (root, args) => {
    return runDelete({ address: args.id })
  }

  return {
    proposeTo,
    deleteProposedTo,
  }
}
