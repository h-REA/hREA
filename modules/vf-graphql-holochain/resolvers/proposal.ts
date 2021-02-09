/**
 * Resolvers for Proposal fields
 *
 * @package: HoloREA
 * @since:   2019-08-27
 */

import { DNAIdMappings, DEFAULT_VF_MODULES } from '../types'
import { mapZomeFn } from '../connection'

import {
  Proposal,
  ProposedTo,
  ProposedIntent,
} from '@valueflows/vf-graphql'

const extractProposedTo = (data): ProposedTo => data.proposedTo
const extractProposedIntent = (data): ProposedIntent => data.proposedIntent

export default (enabledVFModules: string[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri?: string) => {
  const readProposedTo = mapZomeFn(dnaConfig, conductorUri, 'proposal', 'proposed_to', 'get_proposed_to')
  const readProposedIntent = mapZomeFn(dnaConfig, conductorUri, 'proposal', 'proposed_intent', 'get_proposed_intent')

  return {
    publishes: async (record: Proposal): Promise<ProposedIntent[]> => {
      return (await Promise.all((record.publishes || []).map((address)=>readProposedIntent({address})))).map(extractProposedIntent)
    },

    publishedTo: async (record: Proposal): Promise<ProposedTo[]> => {
      return (await Promise.all((record.publishedTo || []).map((address)=>readProposedTo({address})))).map(extractProposedTo)
    },
  }
}
