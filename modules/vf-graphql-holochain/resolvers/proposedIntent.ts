/**
 * Resolvers for Proposal fields
 *
 * @package: HoloREA
 * @since:   2019-08-27
 */

import { DNAIdMappings, DEFAULT_VF_MODULES, VfModule, ReadParams, ProposalAddress, IntentAddress } from '../types.js'
import { mapZomeFn } from '../connection.js'

import {
  Proposal,
  Intent,
  ProposedIntent,
  ProposalResponse,
  IntentResponse,
} from '@valueflows/vf-graphql'

export default (enabledVFModules: VfModule[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri: string) => {
  const hasIntent = -1 !== enabledVFModules.indexOf(VfModule.Intent)

  const readProposal = mapZomeFn<ReadParams, ProposalResponse>(dnaConfig, conductorUri, 'proposal', 'proposal', 'get_proposal')
  const readIntent = mapZomeFn<ReadParams, IntentResponse>(dnaConfig, conductorUri, 'planning', 'intent', 'get_intent')

  return Object.assign(
    {
      publishedIn: async (record: { publishedIn: ProposalAddress }): Promise<Proposal> => {
        return (await readProposal({address:record.publishedIn})).proposal
      },
    },
    (hasIntent ? {
      publishes: async (record: { publishes: IntentAddress }): Promise<Intent> => {
        return (await readIntent({address:record.publishes})).intent
      },
    } : {}),
  )
}
