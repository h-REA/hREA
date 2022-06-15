/**
 * Commitment mutations
 *
 * @package: HoloREA
 * @since:   2019-08-28
 */

import { ByRevision, DNAIdMappings } from '../types.js'
import { mapZomeFn } from '../connection.js'
import { deleteHandler } from './'

import {
  CommitmentCreateParams,
  CommitmentUpdateParams,
  CommitmentResponse,
} from '@valueflows/vf-graphql'

export interface CreateArgs {
  commitment: CommitmentCreateParams,
}
export type createHandler = (root: any, args: CreateArgs) => Promise<CommitmentResponse>

export interface UpdateArgs {
  commitment: CommitmentUpdateParams,
}
export type updateHandler = (root: any, args: UpdateArgs) => Promise<CommitmentResponse>

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const runCreate = mapZomeFn<CreateArgs, CommitmentResponse>(dnaConfig, conductorUri, 'planning', 'commitment', 'create_commitment')
  const runUpdate = mapZomeFn<UpdateArgs, CommitmentResponse>(dnaConfig, conductorUri, 'planning', 'commitment', 'update_commitment')
  const runDelete = mapZomeFn<ByRevision, boolean>(dnaConfig, conductorUri, 'planning', 'commitment', 'delete_commitment')

  const createCommitment: createHandler = async (root, args) => {
    return runCreate(args)
  }

  const updateCommitment: updateHandler = async (root, args) => {
    return runUpdate(args)
  }

  const deleteCommitment: deleteHandler = async (root, args) => {
    return runDelete(args)
  }

  return {
    createCommitment,
    updateCommitment,
    deleteCommitment,
  }
}
