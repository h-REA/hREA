/**
 * Resolvers for Plan fields
 *
 * @package: HoloREA
 * @since:   2019-08-27
 */

import { DNAIdMappings, DEFAULT_VF_MODULES, VfModule, AgentAddress } from '../types'
import { extractEdges, mapZomeFn } from '../connection'

import {
  Process,
  PlanProcessFilterParams,
  Plan,
  Commitment,
  ProcessConnection,
  CommitmentConnection,
  AccountingScope
} from '@valueflows/vf-graphql'
import { CommitmentSearchInput, ProcessSearchInput } from './zomeSearchInputTypes'
import agentQueries from '../queries/agent'


export default (enabledVFModules: VfModule[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri: string) => {

  const hasProcess = -1 !== enabledVFModules.indexOf(VfModule.Process)
  const hasCommitment = -1 !== enabledVFModules.indexOf(VfModule.Commitment)
  const hasAgent = -1 !== enabledVFModules.indexOf(VfModule.Agent)

  const readProcesses = mapZomeFn<ProcessSearchInput, ProcessConnection>(dnaConfig, conductorUri, 'observation', 'process_index', 'query_processes')
  const queryCommitments = mapZomeFn<CommitmentSearchInput, CommitmentConnection>(dnaConfig, conductorUri, 'planning', 'commitment_index', 'query_commitments')
  const readAgent = agentQueries(dnaConfig, conductorUri)['agent']

  return Object.assign(
    (hasProcess ? {
      processes: async (record: Plan): Promise<ProcessConnection> => {
        const results = await readProcesses({ params: { plannedWithin: record.id } })
        return results
      },
    } : {}),
    (hasCommitment ? {
      independentDemands: async (record: Plan): Promise<Commitment[]> => {
        const commitments = await queryCommitments({ params: { independentDemandOf: record.id } })
        return extractEdges(commitments)
      },
    } : {}),
    (hasAgent ? {
      inScopeOf: async (record: { inScopeOf: AgentAddress[] }): Promise<AccountingScope[]> => {
        return (await Promise.all((record.inScopeOf || []).map((address)=>readAgent(record, {address}))))
      },
    } : {}),
  )
}

