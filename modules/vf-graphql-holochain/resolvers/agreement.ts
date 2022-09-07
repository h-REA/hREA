/**
 * Resolvers for agreement fields
 *
 * @package: hREA
 * @since:   2020-06-19
 */

import { DNAIdMappings, DEFAULT_VF_MODULES, VfModule, AgentAddress, ReadParams, ByRevision, AddressableIdentifier } from '../types.js'
import { extractEdges, mapZomeFn } from '../connection.js'

import {
  Agent,
  AgentConnection,
  Agreement,
  AgreementResponse,
  Commitment,
  CommitmentConnection,
  EconomicEvent,
  EconomicEventConnection,
} from '@valueflows/vf-graphql'
import { CommitmentSearchInput, EconomicEventSearchInput } from './zomeSearchInputTypes.js'
import { AgentResponse } from '../mutations/agent'

const extractAgent = (data): Agent => data.agent

export default (enabledVFModules: VfModule[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri: string) => {
  const hasHistory = -1 !== enabledVFModules.indexOf(VfModule.History)
  const hasObservation = -1 !== enabledVFModules.indexOf(VfModule.Observation)
  const hasCommitment = -1 !== enabledVFModules.indexOf(VfModule.Commitment)
  const hasAgent = -1 !== enabledVFModules.indexOf(VfModule.Agent)

  const readRevision = mapZomeFn<ByRevision, AgreementResponse>(dnaConfig, conductorUri, 'agreement', 'agreement', 'get_revision')
  const queryCommitments = mapZomeFn<CommitmentSearchInput,CommitmentConnection>(dnaConfig, conductorUri, 'planning', 'commitment_index', 'query_commitments')
  const queryEvents = mapZomeFn<EconomicEventSearchInput, EconomicEventConnection>(dnaConfig, conductorUri, 'observation', 'economic_event_index', 'query_economic_events')
  const readAgent = mapZomeFn<ReadParams, AgentResponse>(dnaConfig, conductorUri, 'agent', 'agent', 'get_agent')

  return Object.assign(
    (hasCommitment ? {
      commitments: async (record: Agreement): Promise<Commitment[]> => {
        const commitments = await queryCommitments({ params: { clauseOf: record.id } })
        return extractEdges(commitments)
      },
    } : {}),
    (hasObservation ? {
      economicEvents: async (record: Agreement): Promise<EconomicEvent[]> => {
        const economicEvents = await queryEvents({ params: { realizationOf: record.id } })
        return extractEdges(economicEvents)
      },
      unplannedEconomicEvents: async (record: Agreement): Promise<EconomicEvent[]> => {
        throw new Error('resolver unimplemented')
      },
    } : {}),
    (hasAgent ? {
      involvedAgents: async (record: { involvedAgents: AgentAddress[] }): Promise<Agent[]> => {
        return (await Promise.all((record.involvedAgents || []).map((address)=>readAgent({address})))).map(extractAgent)
      },
    } : {}),
    (hasHistory ? {
      revision: async (record: Agreement, args: { revisionId: AddressableIdentifier }): Promise<Agreement> => {
        return (await readRevision(args)).agreement
      },
    } : {}),
  )
}
