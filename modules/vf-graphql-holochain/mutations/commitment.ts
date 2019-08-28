/**
 * Commitment mutations
 *
 * @package: HoloREA
 * @since:   2019-08-28
 */

import { zomeFunction } from '../connection'
import {
  CommitmentCreateParams,
  CommitmentUpdateParams,
  CommitmentResponse,
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const createHandler = zomeFunction('a1_planning', 'commitment', 'create_commitment')
const updateHandler = zomeFunction('a1_planning', 'commitment', 'update_commitment')

// CREATE
interface CreateArgs {
  commitment: CommitmentCreateParams,
}
type createHandler = (root: any, args: CreateArgs) => Promise<CommitmentResponse>

export const createCommitment: createHandler = async (root, args) => {
  return (await createHandler)(args)
}

// UPDATE
interface UpdateArgs {
  commitment: CommitmentUpdateParams,
}
type updateHandler = (root: any, args: UpdateArgs) => Promise<CommitmentResponse>

export const updateCommitment: updateHandler = async (root, args) => {
  return (await updateHandler)(args)
}
