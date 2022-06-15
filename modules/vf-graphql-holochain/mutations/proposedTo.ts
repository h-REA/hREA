/**
 * Mutations for manipulating proposed to
 *
 * @package: HoloREA
 * @since:   2019-09-12
 */

import { AgentAddress, ByRevision, DNAIdMappings, ProposalAddress } from '../types.js'
import { mapZomeFn } from '../connection.js'
import { deleteHandler } from './'

import {
  ProposedToResponse,
} from '@valueflows/vf-graphql'

export interface CreateParams {
  proposedTo: CreateRequest,
}
interface CreateRequest {
    proposed: ProposalAddress,
    proposedTo: AgentAddress,
}
export type createHandler = (root: any, args: CreateRequest) => Promise<ProposedToResponse>

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const runCreate = mapZomeFn<CreateParams, ProposedToResponse>(dnaConfig, conductorUri, 'proposal', 'proposed_to', 'create_proposed_to')
  const runDelete = mapZomeFn<ByRevision, boolean>(dnaConfig, conductorUri, 'proposal', 'proposed_to', 'delete_proposed_to')

  const proposeTo: createHandler = async (root, args) => {
    return runCreate({ proposedTo: args })
  }

  const deleteProposedTo: deleteHandler = async (root, args) => {
    return runDelete(args)
  }

  return {
    proposeTo,
    deleteProposedTo,
  }
}
