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
  ProcessConnection
} from '@valueflows/vf-graphql'


export default (enabledVFModules: VfModule[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri: string) => {

  const hasObservation = -1 !== enabledVFModules.indexOf(VfModule.Observation)
  const hasPlanning = -1 !== enabledVFModules.indexOf(VfModule.Planning)

  const readProcesses = mapZomeFn(dnaConfig, conductorUri, 'observation', 'process_index', 'query_processes')
  const queryCommitments = mapZomeFn(dnaConfig, conductorUri, 'planning', 'commitment_index', 'query_commitments')

  return Object.assign(
    (hasObservation ? {
      // WIP: processes to be completed soon
      // processes: async (record: Plan): Promise<Process[]> => {
      processes: async (record: Plan): Promise<ProcessConnection> => {
        console.log('process query input:', record)
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

