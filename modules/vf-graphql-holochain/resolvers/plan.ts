/**
 * Resolvers for Plan fields
 *
 * @package: HoloREA
 * @since:   2019-08-27
 */

import { DNAIdMappings, DEFAULT_VF_MODULES, VfModule } from '../types'
import { extractEdges, mapZomeFn } from '../connection'

import {
  Process,
  PlanProcessFilterParams,
  Plan,
  Commitment,
  ProcessConnection,
  CommitmentConnection
} from '@valueflows/vf-graphql'
import { CommitmentSearchInput, ProcessSearchInput } from './zomeSearchInputTypes'


export default (enabledVFModules: VfModule[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri: string) => {

  const hasObservation = -1 !== enabledVFModules.indexOf(VfModule.Observation)
  const hasPlanning = -1 !== enabledVFModules.indexOf(VfModule.Planning)

  const readProcesses = mapZomeFn<ProcessSearchInput, ProcessConnection>(dnaConfig, conductorUri, 'observation', 'process_index', 'query_processes')
  const queryCommitments = mapZomeFn<CommitmentSearchInput, CommitmentConnection>(dnaConfig, conductorUri, 'planning', 'commitment_index', 'query_commitments')

  return Object.assign(
    (hasObservation ? {
      processes: async (record: Plan): Promise<ProcessConnection> => {
        const results = await readProcesses({ params: { plannedWithin: record.id } })
        return results
      },
    } : {}),
    (hasPlanning ? {
      independentDemands: async (record: Plan): Promise<Commitment[]> => {
        const commitments = await queryCommitments({ params: { independentDemandOf: record.id } })
        return extractEdges(commitments)
      },
    } : {}),
  )
}

