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
  Agent,
} from '@valueflows/vf-graphql'

import agentQueries from '../queries/agent'

export default (enabledVFModules: string[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri?: string) => {
  const hasAgent = -1 !== enabledVFModules.indexOf("agent")

  const readProposal = mapZomeFn(dnaConfig, conductorUri, 'proposal', 'proposal', 'get_proposal')
  const readAgent = agentQueries(dnaConfig, conductorUri)['agent']

  return Object.assign(
    {
      proposed: async (record: ProposedTo): Promise<Proposal> => {
        return (await readProposal({address:record.proposed})).proposal
      },
    },
    (hasAgent ? {
      proposedTo: async (record: ProposedTo): Promise<Agent> => {
        return readAgent(record, { id: record.proposedTo })
      },
    } : {}),
  )
}
