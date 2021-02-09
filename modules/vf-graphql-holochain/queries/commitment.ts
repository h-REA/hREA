/**
 * Top-level queries relating to Commitments
 *
 * @package: HoloREA
 * @since:   2019-08-28
 */

import { DNAIdMappings, injectTypename } from '../types'
import { mapZomeFn } from '../connection'

import {
  Commitment,
} from '@valueflows/vf-graphql'

export default (dnaConfig: DNAIdMappings, conductorUri?: string) => {
  const readOne = mapZomeFn(dnaConfig, conductorUri, 'planning', 'commitment', 'get_commitment')

  return {
    commitment: injectTypename('Commitment', async (root, args): Promise<Commitment> => {
      const { id } = args
      return (await (await readOne)({ address: id })).commitment
    }),
  }
}
