/**
 * Resolvers for Proposal fields
 *
 * @package: HoloREA
 * @since:   2019-08-27
 */

import { DNAIdMappings, DEFAULT_VF_MODULES, VfModule, ReadParams, ByRevision, ProposedIntentAddress, AddressableIdentifier, AgentAddress } from '../types.js'
import { mapZomeFn } from '../connection.js'

import {
  Proposal,
  ProposalResponse,
  ProposedTo,
  ProposedIntent,
  ProposedToResponse,
  ProposedIntentResponse,
  AccountingScope,
} from '@valueflows/vf-graphql'
import agentQueries from '../queries/agent.js'

const extractProposedTo = (data): ProposedTo => data.proposedTo
const extractProposedIntent = (data): ProposedIntent => data.proposedIntent

export default (enabledVFModules: VfModule[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri: string) => {
  const hasHistory = -1 !== enabledVFModules.indexOf(VfModule.History)
  const hasAgent = -1 !== enabledVFModules.indexOf(VfModule.Agent)
  const hasIntent = -1 !== enabledVFModules.indexOf(VfModule.Intent)

  const readRevision = mapZomeFn<ByRevision, ProposalResponse>(dnaConfig, conductorUri, 'proposal', 'proposal', 'get_revision')
  const readProposedTo = mapZomeFn<ReadParams, ProposedToResponse>(dnaConfig, conductorUri, 'proposal', 'proposed_to', 'get_proposed_to')
  const readProposedIntent = mapZomeFn<ReadParams, ProposedIntentResponse>(dnaConfig, conductorUri, 'proposal', 'proposed_intent', 'get_proposed_intent')
  const readAgent = agentQueries(dnaConfig, conductorUri)['agent']

  return Object.assign(
    {
      publishes: async (record: { publishes: ProposedIntentAddress[] }): Promise<ProposedIntent[]> => {
        return (await Promise.all((record.publishes || []).map((address)=>readProposedIntent({address})))).map(extractProposedIntent)
      }
    },
    (hasAgent ? {
      publishedTo: async (record: { publishedTo: AddressableIdentifier[] }): Promise<ProposedTo[]> => {
        return (await Promise.all((record.publishedTo || []).map((address)=>readProposedTo({address})))).map(extractProposedTo)
      },
      inScopeOf: async (record: { inScopeOf: AgentAddress[] }): Promise<AccountingScope[]> => {
        return (await Promise.all((record.inScopeOf || []).map((address)=>readAgent(record, {address}))))
      },
    } : {}),
    (hasIntent ? {
      primaryIntents: () => {
        throw new Error('resolver unimplemented')
      },
      reciprocalIntents: () => {
        throw new Error('resolver unimplemented')
      },
    } : {}),
    (hasHistory ? {
      revision: async (record: Proposal, args: { revisionId: AddressableIdentifier }): Promise<Proposal> => {
        return (await readRevision(args)).proposal
      },
    } : {}),
  )
}
