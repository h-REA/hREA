/**
 * Top-level queries relating to Commitments
 *
 * @package: HoloREA
 * @since:   2019-08-28
 */

import { zomeFunction } from '../connection'

import {
  Commitment,
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const readCommitment = zomeFunction('planning', 'commitment', 'get_commitment')

// Read a single commitment by ID
export const commitment = async (root, args): Promise<Commitment> => {
  const { id } = args
  return (await (await readCommitment)({ address: id })).commitment
}
