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
  Intent,
  ProposedIntent,
} from '@valueflows/vf-graphql'

export default (enabledVFModules: string[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri?: string, traceAppSignals?: AppSignalCb) => {
  const hasPlanning = -1 !== enabledVFModules.indexOf("planning")

  const readProposal = mapZomeFn(dnaConfig, conductorUri, 'proposal', 'proposal', 'get_proposal')
  const readIntent = mapZomeFn(dnaConfig, conductorUri, 'planning', 'intent', 'get_intent')

  return Object.assign(
    {
      publishedIn: async (record: ProposedIntent): Promise<Proposal> => {
        return (await readProposal({address:record.publishedIn})).proposal
      },
    },
    (hasPlanning ? {
      publishes: async (record: ProposedIntent): Promise<Intent> => {
        return (await readIntent({address:record.publishes})).intent
      },
    } : {}),
  )
}
