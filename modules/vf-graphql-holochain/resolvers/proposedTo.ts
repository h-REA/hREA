/**
 * Resolvers for Proposal fields
 *
 * @package: HoloREA
 * @since:   2019-08-27
 */

import { DNAIdMappings, DEFAULT_VF_MODULES, VfModule, ReadParams, ProposalAddress, AgentAddress } from '../types'
import { mapZomeFn } from '../connection'

import {
  Proposal,
  ProposedTo,
  Agent,
  ProposalResponse,
} from '@valueflows/vf-graphql'

import agentQueries from '../queries/agent'

export default (enabledVFModules: VfModule[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri: string) => {
  const hasAgent = -1 !== enabledVFModules.indexOf(VfModule.Agent)

  const readProposal = mapZomeFn<ReadParams, ProposalResponse>(dnaConfig, conductorUri, 'proposal', 'proposal', 'get_proposal')
  const readAgent = agentQueries(dnaConfig, conductorUri)['agent']

  return Object.assign(
    {
      proposed: async (record: { proposed: ProposalAddress }): Promise<Proposal> => {
        return (await readProposal({address:record.proposed})).proposal
      },
    },
    (hasAgent ? {
      proposedTo: async (record: { proposedTo: AgentAddress}): Promise<Agent> => {
        return readAgent(record, { id: record.proposedTo })
      },
    } : {}),
  )
}
