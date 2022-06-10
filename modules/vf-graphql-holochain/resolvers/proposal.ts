/**
 * Resolvers for Proposal fields
 *
 * @package: HoloREA
 * @since:   2019-08-27
 */

import { DNAIdMappings, DEFAULT_VF_MODULES, VfModule, ReadParams, ProposedIntentAddress, AddressableIdentifier } from '../types'
import { mapZomeFn } from '../connection'

import {
  Proposal,
  ProposedTo,
  ProposedIntent,
  ProposedToResponse,
  ProposedIntentResponse,
} from '@valueflows/vf-graphql'

const extractProposedTo = (data): ProposedTo => data.proposedTo
const extractProposedIntent = (data): ProposedIntent => data.proposedIntent

export default (enabledVFModules: VfModule[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri: string) => {
  const hasAgent = -1 !== enabledVFModules.indexOf(VfModule.Agent)
  const readProposedTo = mapZomeFn<ReadParams, ProposedToResponse>(dnaConfig, conductorUri, 'proposal', 'proposed_to', 'get_proposed_to')
  const readProposedIntent = mapZomeFn<ReadParams, ProposedIntentResponse>(dnaConfig, conductorUri, 'proposal', 'proposed_intent', 'get_proposed_intent')

  return Object.assign({
    publishes: async (record: { publishes: ProposedIntentAddress[] }): Promise<ProposedIntent[]> => {
      return (await Promise.all((record.publishes || []).map((address)=>readProposedIntent({address})))).map(extractProposedIntent)
    },
  },
  (hasAgent ? {
    publishedTo: async (record: { publishedTo: AddressableIdentifier[] }): Promise<ProposedTo[]> => {
      return (await Promise.all((record.publishedTo || []).map((address)=>readProposedTo({address})))).map(extractProposedTo)
    },
   } : {})
  )
}
