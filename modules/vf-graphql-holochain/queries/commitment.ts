/**
 * Top-level queries relating to Commitments
 *
 * @package: HoloREA
 * @since:   2019-08-28
 */

import { DNAIdMappings, injectTypename, ReadParams } from '../types.js'
import { mapZomeFn } from '../connection.js'

import {
  Commitment, CommitmentConnection, CommitmentResponse,
} from '@valueflows/vf-graphql'
import { PagingParams } from '../resolvers/zomeSearchInputTypes.js'

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const readOne = mapZomeFn<ReadParams, CommitmentResponse>(dnaConfig, conductorUri, 'planning', 'commitment', 'get_commitment')
  const readAll = mapZomeFn<PagingParams, CommitmentConnection>(dnaConfig, conductorUri, 'planning', 'commitment_index', 'read_all_commitments')

  return {
    commitment: injectTypename('Commitment', async (root, args): Promise<Commitment> => {
      const { id } = args
      return (await (await readOne)({ address: id })).commitment
    }),
    commitments: async (root, args: PagingParams): Promise<CommitmentConnection> => {
      return await readAll(args)
    },
  }
}
