/**
 * Top-level queries relating to Proposal
 *
 * @package: HoloREA
 * @since:   2019-09-12
 */

import { DNAIdMappings } from '../types'
import { mapZomeFn } from '../connection'

import {
  Proposal,
  ProposedTo,
  ProposedIntent,
} from '@valueflows/vf-graphql'

export default (dnaConfig: DNAIdMappings, conductorUri?: string) => {
  const readOne = mapZomeFn(dnaConfig, conductorUri, 'proposal', 'proposal', 'get_proposal')

  return {
    proposal: async (root, args): Promise<Proposal> => {
      return (await readOne({ address: args.id })).proposal
    },
  }
}
