/**
 * Top-level queries relating to Proposal
 *
 * @package: HoloREA
 * @since:   2019-09-12
 */

import { DNAIdMappings, ReadParams } from '../types.js'
import { mapZomeFn } from '../connection.js'

import {
  Proposal,
  ProposedTo,
  ProposedIntent,
  ProposalResponse,
} from '@valueflows/vf-graphql'

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const readOne = mapZomeFn<ReadParams, ProposalResponse>(dnaConfig, conductorUri, 'proposal', 'proposal', 'get_proposal')

  return {
    proposal: async (root, args): Promise<Proposal> => {
      return (await readOne({ address: args.id })).proposal
    },
  }
}
