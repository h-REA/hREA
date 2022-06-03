/**
 * Mutations for manipulating proposed to
 *
 * @package: HoloREA
 * @since:   2019-09-12
 */

import { AgentAddress, ByRevision, DNAIdMappings, ProposalAddress } from '../types'
import { mapZomeFn } from '../connection'
import { deleteHandler } from './'

import {
  ProposedToResponse,
} from '@valueflows/vf-graphql'

export interface CreateArgs {
  proposedTo: {
    proposed: AgentAddress,
    publishes: ProposalAddress,
    reciprocal: boolean,
  },
}
export type createHandler = (root: any, args) => Promise<ProposedToResponse>

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const runCreate = mapZomeFn<CreateArgs, ProposedToResponse>(dnaConfig, conductorUri, 'proposal', 'proposed_to', 'create_proposed_to')
  const runDelete = mapZomeFn<ByRevision, boolean>(dnaConfig, conductorUri, 'proposal', 'proposed_to', 'delete_proposed_to')

  const proposeTo: createHandler = async (root, args) => {
    return runCreate(args)
  }

  const deleteProposedTo: deleteHandler = async (root, args) => {
    return runDelete(args)
  }

  return {
    proposeTo,
    deleteProposedTo,
  }
}
