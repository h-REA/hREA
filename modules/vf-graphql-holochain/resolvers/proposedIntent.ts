/**
 * Resolvers for Proposal fields
 *
 * @package: HoloREA
 * @since:   2019-08-27
 */

import { DNAIdMappings } from '../types'
import { mapZomeFn } from '../connection'

import {
  Proposal,
  Intent,
  ProposedIntent,
} from '@valueflows/vf-graphql'

export default (dnaConfig?: DNAIdMappings, conductorUri?: string) => {
  const readProposal = mapZomeFn(dnaConfig, conductorUri, 'proposal', 'proposal', 'get_proposal')
  const readIntent = mapZomeFn(dnaConfig, conductorUri, 'planning', 'intent', 'get_intent')

  return {
    publishedIn: async (record: ProposedIntent): Promise<Proposal> => {
      return (await readProposal({address:record.publishedIn})).proposal
    },

    publishes: async (record: ProposedIntent): Promise<Intent> => {
      return (await readIntent({address:record.publishes})).intent
    },
  }
}
