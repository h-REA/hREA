/**
 * Mutations for manipulating proposals
 *
 * @package: HoloREA
 * @since:   2019-09-12
 */

import { ByRevision, DNAIdMappings } from '../types.js'
import { mapZomeFn } from '../connection.js'
import { deleteHandler } from './'

import {
  ProposalCreateParams,
  ProposalUpdateParams,
  ProposalResponse,
} from '@valueflows/vf-graphql'

export interface CreateArgs {
  proposal: ProposalCreateParams,
}
export type createHandler = (root: any, args: CreateArgs) => Promise<ProposalResponse>

export interface UpdateArgs {
    proposal: ProposalUpdateParams,
}
export type updateHandler = (root: any, args: UpdateArgs) => Promise<ProposalResponse>

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const runCreate = mapZomeFn<CreateArgs, ProposalResponse>(dnaConfig, conductorUri, 'proposal', 'proposal', 'create_proposal')
  const runUpdate = mapZomeFn<UpdateArgs, ProposalResponse>(dnaConfig, conductorUri, 'proposal', 'proposal', 'update_proposal')
  const runDelete = mapZomeFn<ByRevision, boolean>(dnaConfig, conductorUri, 'proposal', 'proposal', 'delete_proposal')

  const createProposal: createHandler = async (root, args) => {
    return runCreate(args)
  }

  const updateProposal: updateHandler = async (root, args) => {
    return runUpdate(args)
  }

  const deleteProposal: deleteHandler = async (root, args) => {
    return runDelete(args)
  }

  return {
    createProposal,
    updateProposal,
    deleteProposal,
  }
}
