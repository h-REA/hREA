/**
 * Top-level queries relating to Commitments
 *
 * @package: HoloREA
 * @since:   2019-08-28
 */

import { DNAIdMappings, injectTypename, ReadParams } from '../types.js'
import { mapZomeFn } from '../connection.js'

import {
  Commitment, CommitmentResponse,
} from '@valueflows/vf-graphql'

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const readOne = mapZomeFn<ReadParams, CommitmentResponse>(dnaConfig, conductorUri, 'planning', 'commitment', 'get_commitment')

  return {
    commitment: injectTypename('Commitment', async (root, args): Promise<Commitment> => {
      const { id } = args
      return (await (await readOne)({ address: id })).commitment
    }),
  }
}
