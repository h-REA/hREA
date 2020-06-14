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

export default (dnaConfig?: DNAIdMappings, conductorUri?: string) => {
  const runCreate = mapZomeFn(dnaConfig, conductorUri, 'proposal', 'proposal', 'create_proposal')
  const runUpdate = mapZomeFn(dnaConfig, conductorUri, 'proposal', 'proposal', 'update_proposal')
  const runDelete = mapZomeFn(dnaConfig, conductorUri, 'proposal', 'proposal', 'delete_proposal')

  const createProposal: createHandler = async (root, args) => {
    const adaptedArguments = {
      proposal: args.proposal
    }
    return runCreate(adaptedArguments)
  }

  const updateProposal: updateHandler = async (root, args) => {
    const adaptedArguments = {
      proposal: args.proposal
    }
    return runUpdate(adaptedArguments)
  }

  const deleteProposal: deleteHandler = async (root, args) => {
    return runDelete({ address: args.id })
  }

  return {
    createProposal,
    updateProposal,
    deleteProposal,
  }
}
